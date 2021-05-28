use super::{chars, ENext, EntityParser, EntityOut};

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
    Reference(String),
    Broadcast(String),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub reference: Option<String>,
    pub broadcasts: Vec<String>,
    expectation: Vec<EExpectation>,
    pending: Pending,
    closed: bool,
}

impl Event {
    pub fn new(reference: String) -> Self {
        Self {
            reference: None,
            broadcasts: vec![],
            expectation: vec![EExpectation::Word, EExpectation::PathDelimiter],
            pending: Pending::Reference(reference),
            closed: false,
        }
    }

    fn close(&mut self) {
        self.closed = true;
        self.pending = Pending::Nothing;
        self.expectation = vec![];
    }
}

impl EntityParser for Event {
    
    fn open(word: String) -> Option<Self> {
        if word.starts_with(chars::AT) {
            Some(Self::new(word[1..word.len() - 1].to_owned()))
        } else {
            None
        }
    }

    fn next(&mut self, enext: ENext) -> Result<usize, String> {
        fn is_in(src: &[EExpectation], target: &EExpectation) -> bool {
            src.iter().any(|e| e == target)
        }
        match enext {
            ENext::Open(offset) => {
                if is_in(&self.expectation, &EExpectation::Open) {
                    match self.pending.clone() {
                        Pending::Reference(path_to_struct) => if path_to_struct.is_empty() {
                            Err(String::from("Reference isn't defined"))
                        } else {
                            /* USECASES:
                                                  |
                            @ServerEvents.KickOff {
                                > Events.Message;
                                > Events.UserConnected;
                            }
                            */
                            self.reference = Some(path_to_struct);
                            self.pending = Pending::Nothing;
                            self.expectation = vec![EExpectation::Arrow];
                            Ok(offset)
                        },
                        _ => Err(String::from("Listing of broadcasts can be done only after Error would be defined."))
                    }
                } else {
                    Err(format!("Symbol Open isn't expected. Expectation: {:?}.", self.expectation))
                }
            },
            ENext::Word((word, offset, _next_char)) => {
                match self.pending.clone() {
                    Pending::Reference(path_to_struct) => {
                        /* USECASES:
                                      |
                        @ServerEvents.KickOff {
                            > Events.Message;
                            > Events.UserConnected;
                        }
                        */
                        self.pending = Pending::Reference(format!("{}{}{}",
                            path_to_struct,
                            if path_to_struct.is_empty() { "" } else { "." },
                            word
                        ));
                        self.expectation = vec![
                            EExpectation::Word,
                            EExpectation::PathDelimiter,
                            EExpectation::Open,
                            EExpectation::Semicolon,
                        ];
                    },
                    Pending::Broadcast(path_to_struct) => {
                        /* USECASES:
                        @ServerEvents.KickOff {
                              |      |
                            > Events.Message;
                              |      |
                            > Events.UserConnected;
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
                                 |
                    @ServerEvents.KickOff {
                                |
                        > Events.Message;
                                |
                        > Events.UserConnected;
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
                        Pending::Reference(path_to_struct) => {
                            if path_to_struct.is_empty() {
                                Err(String::from("To create event, reference to event's object/struct should defined"))
                            } else {
                                self.reference = Some(path_to_struct);
                                self.close();
                                Ok(offset)
                            }
                        },
                        Pending::Broadcast(path_to_struct) => {
                            /* USECASES:
                            @ServerEvents.KickOff {
                                                |
                                > Events.Message;
                                                      |
                                > Events.UserConnected;
                            }
                            */
                            if path_to_struct.is_empty() {
                                Err(String::from("Broadcast reference cannot be empty"))
                            } else {
                                self.broadcasts.push(path_to_struct);
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
                            @ServerEvents.KickOff {
                                |
                                > Events.Message;
                                |
                                > Events.UserConnected;
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
                            @ServerEvents.KickOff {
                                > Events.Message;
                                > Events.UserConnected;
                            |
                            }
                            */
                            if self.reference.is_none() {
                                return Err(String::from("Event cannot be create without reference to event object/struct"));
                            }
                            self.close();
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
        EntityOut::Event(self.clone())
    }

}