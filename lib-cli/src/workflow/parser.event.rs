use super::{broadcast::Broadcast, chars, ENext, EntityOut, EntityParser, Protocol};

mod defaults {
    pub const connected: &str = "connected";
    pub const disconnected: &str = "disconnected";
}
#[derive(Debug, PartialEq, Clone)]
enum EExpectation {
    Word,
    Semicolon,
    PathDelimiter,
    Open,
    Close,
    Arrow,
    Question,
}

#[derive(Debug, Clone)]
enum Pending {
    Nothing,
    Reference(String),
    Broadcast(Broadcast),
}

#[derive(Debug, Clone)]
pub struct Event {
    pub reference: Option<String>,
    pub broadcasts: Vec<Broadcast>,
    expectation: Vec<EExpectation>,
    pending: Pending,
    closed: bool,
}

impl Event {
    pub fn new(reference: String) -> Self {
        Self {
            reference: None,
            broadcasts: vec![],
            expectation: vec![
                EExpectation::Word,
                EExpectation::PathDelimiter,
                EExpectation::Open,
            ],
            pending: Pending::Reference(reference),
            closed: false,
        }
    }

    fn close(&mut self, protocol: &Protocol) -> Result<(), String> {
        if let Some(reference) = self.reference.as_ref() {
            if reference != defaults::connected
                && reference != defaults::disconnected
                && protocol.find_by_str_path(0, reference).is_none()
            {
                return Err(format!(
                    "Reference to event object/struct {} isn't defined in protocol",
                    reference
                ));
            }
        } else {
            return Err(String::from(
                "Reference to event object/struct should be defined",
            ));
        }
        if self.broadcasts.is_empty() {
            return Err(String::from(
                "Event without any broadcast messages doesn't make sense",
            ));
        }
        for broadcast in self.broadcasts.iter() {
            if protocol.find_by_str_path(0, &broadcast.reference).is_none() {
                return Err(format!(
                    "Broadcast object/struct {} isn't defined in protocol",
                    &broadcast.reference
                ));
            }
        }
        self.closed = true;
        self.pending = Pending::Nothing;
        self.expectation = vec![];
        Ok(())
    }

    pub fn get_reference(&self) -> Result<String, String> {
        if let Some(reference) = self.reference.as_ref() {
            Ok(String::from(reference))
        } else {
            Err(String::from(
                "Reference to object/struct for event isn't defined",
            ))
        }
    }

    pub fn as_filename(&self) -> Result<String, String> {
        if let Some(reference) = self.reference.as_ref() {
            Ok(format!(
                "{}.rs",
                String::from(reference).to_lowercase().replace(".", "_")
            ))
        } else {
            Err(String::from(
                "Reference to object/struct of event isn't defined for action",
            ))
        }
    }

    pub fn as_struct_path(&self) -> Result<String, String> {
        if let Some(reference) = self.reference.as_ref() {
            Ok(String::from(reference).replace(".", "::"))
        } else {
            Err(String::from(
                "Reference to object/struct of event isn't defined for action",
            ))
        }
    }

    pub fn as_mod_name(&self) -> Result<String, String> {
        if let Some(reference) = self.reference.as_ref() {
            Ok(String::from(reference).to_lowercase().replace(".", "_"))
        } else {
            Err(String::from(
                "Reference to object/struct of event isn't defined for action",
            ))
        }
    }

    pub fn as_struct_name(&self) -> Result<String, String> {
        if let Some(reference) = self.reference.as_ref() {
            Ok(String::from(reference).replace(".", ""))
        } else {
            Err(String::from(
                "Reference to object/struct event isn't defined for action",
            ))
        }
    }
}

impl EntityParser for Event {
    fn open(word: String) -> Option<Self> {
        if word.starts_with(chars::AT) {
            Some(Self::new(word[1..word.len()].to_owned()))
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
                    match self.pending.clone() {
                        Pending::Reference(path_to_struct) => {
                            if path_to_struct.is_empty() {
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
                            }
                        }
                        _ => Err(String::from(
                            "Listing of broadcasts can be done only after Error would be defined.",
                        )),
                    }
                } else {
                    Err(format!(
                        "Symbol Open isn't expected. Expectation: {:?}.",
                        self.expectation
                    ))
                }
            }
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
                        self.pending = Pending::Reference(format!(
                            "{}{}{}",
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
                    }
                    Pending::Broadcast(mut broadcast) => {
                        /* USECASES:
                        @ServerEvents.KickOff {
                              |      |
                            > Events.Message;
                              |      |
                            > Events.UserConnected;
                        }
                        */
                        broadcast.reference = format!(
                            "{}{}{}",
                            broadcast.reference,
                            if broadcast.reference.is_empty() {
                                ""
                            } else {
                                "."
                            },
                            word
                        );
                        self.pending = Pending::Broadcast(broadcast);
                        self.expectation = vec![
                            EExpectation::Word,
                            EExpectation::PathDelimiter,
                            EExpectation::Semicolon,
                            EExpectation::Question,
                        ];
                    }
                    _ => {
                        return Err(format!("Unexpected word {}", word));
                    }
                };
                Ok(offset)
            }
            ENext::Question(offset) => {
                if is_in(&self.expectation, &EExpectation::Question) {
                    match self.pending.clone() {
                        Pending::Broadcast(mut broadcast) => {
                            if !broadcast.reference.is_empty() {
                                /* USECASES:
                                @ServerEvents.KickOff {
                                                    |
                                    > Events.Message?;
                                                          |
                                    > Events.UserConnected?;
                                }
                                */
                                broadcast.optional = true;
                                self.pending = Pending::Broadcast(broadcast);
                                self.expectation = vec![EExpectation::Semicolon];
                                Ok(offset)
                            } else {
                                Err(format!(
                                    "Symbol ? isn't expected. Expectation: {:?}",
                                    self.expectation
                                ))
                            }
                        }
                        _ => Err(format!(
                            "Symbol ? isn't expected. Expectation: {:?}",
                            self.expectation
                        )),
                    }
                } else {
                    Err(format!(
                        "Symbol ? isn't expected. Expectation: {:?}",
                        self.expectation
                    ))
                }
            }
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
                    Err(format!(
                        "Symbol . isn't expected. Expectation: {:?}",
                        self.expectation
                    ))
                }
            }
            ENext::Semicolon(offset) => {
                if is_in(&self.expectation, &EExpectation::Semicolon) {
                    match self.pending.clone() {
                        Pending::Reference(path_to_struct) => {
                            if path_to_struct.is_empty() {
                                Err(String::from("To create event, reference to event's object/struct should defined"))
                            } else {
                                self.reference = Some(path_to_struct);
                                if let Err(e) = self.close(protocol) {
                                    return Err(e);
                                }
                                Ok(offset)
                            }
                        }
                        Pending::Broadcast(mut broadcast) => {
                            /* USECASES:
                            @ServerEvents.KickOff {
                                                |
                                > Events.Message;
                                                      |
                                > Events.UserConnected;
                            }
                            */
                            if broadcast.reference.is_empty() {
                                Err(String::from("Broadcast reference cannot be empty"))
                            } else {
                                self.broadcasts.push(broadcast);
                                self.expectation = vec![EExpectation::Arrow, EExpectation::Close];
                                self.pending = Pending::Nothing;
                                Ok(offset)
                            }
                        }
                        _ => Err(String::from(
                            "Symbol ; expected only after request definition.",
                        )),
                    }
                } else {
                    Err(format!(
                        "Symbol ; isn't expected. Expectation: {:?}",
                        self.expectation
                    ))
                }
            }
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
                            self.pending = Pending::Broadcast(Broadcast::new(String::new(), false));
                            self.expectation =
                                vec![EExpectation::Word, EExpectation::PathDelimiter];
                            Ok(offset)
                        }
                        _ => Err(format!(
                            "Incorrect position of >. Pending: {:?}",
                            self.pending
                        )),
                    }
                } else {
                    Err(format!(
                        "Symbol > isn't expected. Expectation: {:?}",
                        self.expectation
                    ))
                }
            }
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
                            if let Err(e) = self.close(protocol) {
                                return Err(e);
                            }
                            Ok(offset)
                        }
                        _ => Err(String::from(
                            "Fail to close event. Position of close isn't correct.",
                        )),
                    }
                } else {
                    Err(format!(
                        "Symbol CLOSE isn't expected. Expectation: {:?}",
                        self.expectation
                    ))
                }
            }
            _ => Err(format!("Isn't expected value: {:?}", enext)),
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
