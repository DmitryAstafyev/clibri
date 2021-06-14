use super::{
    ENext,
    EntityParser,
    EntityOut,
    Protocol
};

mod key_words {
    pub const ALIAS: &str = "@broadcast";
}

#[derive(Debug, PartialEq, Clone)]
enum EExpectation {
    Word,
    Semicolon,
    PathDelimiter,
    Open,
    Close,
    Arrow,
}

#[derive(Debug, Clone)]
enum Pending {
    Nothing,
    Broadcast(String),
}

#[derive(Debug, Clone)]
pub struct Broadcast {
    pub reference: String,
    pub optional: bool,
}

impl Broadcast {
    pub fn new(reference: String, optional: bool) -> Self {
        Self {
            reference,
            optional,
        }
    }

    pub fn validate(&self, protocol: &Protocol) -> bool {
        if protocol.find_by_str_path(0, &self.reference).is_none() {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone)]
pub struct Broadcasts {
    pub broadcasts: Vec<Broadcast>,
    expectation: Vec<EExpectation>,
    pending: Pending,
    closed: bool,
}

impl Broadcasts {
    pub fn new() -> Self {
        Self {
            broadcasts: vec![],
            expectation: vec![EExpectation::Open],
            pending: Pending::Nothing,
            closed: false,
        }
    }

    fn close(&mut self, protocol: &Protocol) -> Result<(), String> {
        if self.broadcasts.is_empty() {
            return Err(String::from("Event without any broadcast messages doesn't make sense"))
        }
        for broadcast in self.broadcasts.iter() {
            if !broadcast.validate(protocol) {
                return Err(format!("Broadcast object/struct {} isn't defined in protocol", broadcast.reference));
            }
        }
        self.closed = true;
        self.pending = Pending::Nothing;
        self.expectation = vec![];
        Ok(())
    }

    pub fn next_broadcast(&mut self) -> Option<Broadcast> {
        if self.broadcasts.is_empty() {
            None
        } else {
            Some(self.broadcasts.remove(0))
        }
    }

}

impl EntityParser for Broadcasts {
    
    fn open(word: String) -> Option<Self> {
        if word == key_words::ALIAS {
            Some(Broadcasts::new())
        } else {
            None
        }
    }

    fn next(&mut self, enext: ENext, protocol: &mut Protocol) -> Result<usize, String> {
        fn is_in(src: &[EExpectation], target: &EExpectation) -> bool {
            src.iter().any(|e| e == target)
        }
        match enext {
            ENext::Open(offset) => {
                if is_in(&self.expectation, &EExpectation::Open) {
                    /* USECASES:
                               |
                    @broadcast {
                        > Events.Message;
                        > Events.UserConnected;
                        > Events.UserDisconnected;
                    }
                    */
                    self.pending = Pending::Nothing;
                    self.expectation = vec![EExpectation::Arrow];
                    Ok(offset)
                } else {
                    Err(format!("Symbol Open isn't expected. Expectation: {:?}.", self.expectation))
                }
            },
            ENext::Word((word, offset, _next_char)) => {
                match self.pending.clone() {
                    Pending::Broadcast(path_to_struct) => {
                        /* USECASES:
                        @broadcast {
                              |      |
                            > Events.Message;
                              |      |
                            > Events.UserConnected;
                              |      |
                            > Events.UserDisconnected;
                        }
                        */
                        self.pending = Pending::Broadcast(format!("{}{}{}",
                            path_to_struct,
                            if path_to_struct.is_empty() { "" } else { "." },
                            word
                        ));
                        self.expectation = vec![
                            EExpectation::Word,
                            EExpectation::PathDelimiter,
                            EExpectation::Semicolon,
                        ];
                    },
                    _ => {
                        return Err(format!("Unexpected word {}", word));
                    }
                };
                Ok(offset)
            },
            ENext::PathDelimiter(offset) => {
                if is_in(&self.expectation, &EExpectation::PathDelimiter) {
                    /* USECASES:
                    @broadcast {
                                |
                        > Events.Message;
                                |
                        > Events.UserConnected;
                                |
                        > Events.UserDisconnected;
                    }
                    */
                    self.expectation = vec![EExpectation::Word];
                    Ok(offset)
                } else {
                    Err(format!("Symbol . isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Semicolon(offset) => {
                if is_in(&self.expectation, &EExpectation::Semicolon) {
                    match self.pending.clone() {
                        Pending::Broadcast(path_to_struct) => {
                            /* USECASES:
                            @broadcast {
                                                |
                                > Events.Message;
                                                      |
                                > Events.UserConnected;
                                                         |
                                > Events.UserDisconnected;
                            }
                            */
                            if path_to_struct.is_empty() {
                                Err(String::from("Broadcast reference cannot be empty"))
                            } else {
                                self.broadcasts.push(Broadcast::new(path_to_struct, false));
                                self.expectation = vec![
                                    EExpectation::Arrow,
                                    EExpectation::Close,
                                ];
                                self.pending = Pending::Nothing;
                                Ok(offset)
                            }
                        },
                        _ => Err(String::from("Symbol ; expected only after request definition."))
                    }
                } else {
                    Err(format!("Symbol ; isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Arrow(offset) => {
                if is_in(&self.expectation, &EExpectation::Arrow) {
                    match self.pending.clone() {
                        Pending::Nothing => {
                            /* USECASES:
                            @broadcast {
                                |
                                > Events.Message;
                                |
                                > Events.UserConnected;
                                |
                                > Events.UserDisconnected;
                            }
                            */
                            self.pending = Pending::Broadcast(String::new());
                            self.expectation = vec![
                                EExpectation::Word,
                                EExpectation::PathDelimiter,
                            ];
                            Ok(offset)
                        },
                        _ => Err(format!("Incorrect position of >. Pending: {:?}", self.pending)),
                    }
                } else {
                    Err(format!("Symbol > isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Close(offset) => {
                if is_in(&self.expectation, &EExpectation::Close) {
                    match self.pending.clone() {
                        Pending::Nothing => {
                            /* USECASES:
                            @broadcast {
                                > Events.Message;
                                > Events.UserConnected;
                                > Events.UserDisconnected;
                            |
                            }
                            */
                            if let Err(e) = self.close(protocol) {
                                return Err(e);
                            }
                            Ok(offset)
                        },
                        _ => {
                            Err(String::from("Fail to close event. Position of close isn't correct."))
                        },
                    }
                } else {
                    Err(format!("Symbol CLOSE isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            _ => {
                Err(format!("Isn't expected value: {:?}", enext))
            }
        }
    }

    fn closed(&self) -> bool {
        self.closed
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn extract(&mut self) -> EntityOut {
        EntityOut::Broadcasts(self.clone())
    }

}