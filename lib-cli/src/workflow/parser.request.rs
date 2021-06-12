use super::{
    EntityOut,
    ENext,
    EntityParser,
    Protocol,
};

use std::{
    path::{
        Path,
    }
};

#[derive(Debug, PartialEq, Clone)]
enum EExpectation {
    Word,
    Semicolon,
    PathDelimiter,
    Open,
    Close,
    Arrow,
    OpenBracket,
    CloseBracket,
    Exclamation,
    Question,
}

#[derive(Debug, Clone)]
enum Pending {
    Nothing,
    Error(String),
    Request(String),
    Action(Action),
    Broadcast(String),
}

#[derive(Debug, Clone)]
pub struct ActionBroadcast {
    pub reference: String,
    pub optional: bool,
}

impl ActionBroadcast {

    pub fn new(reference: String, optional: bool) -> Self {
        Self {
            reference,
            optional,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    pub conclusion: Option<String>,
    pub response: Option<String>,
    /// (broadcast struct, is optional)
    pub broadcast: Vec<ActionBroadcast>,
    current: String,
    optional: bool,
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}

impl Action {
    pub fn new() -> Self {
        Self {
            conclusion: None,
            response: None,
            broadcast: vec![],
            current: String::new(),
            optional: false,
        }
    }

    fn add_broadcast(&mut self, broadcast: ActionBroadcast) -> Result<(), String> {
        if self.broadcast.iter().any(|b| b.reference == broadcast.reference) {
            Err(format!("Broadcast {} has been already added", broadcast.reference))
        } else {
            self.broadcast.push(broadcast);
            Ok(())
        }
    }

    fn valid(&self) -> Result<(), String> {
        if self.response.is_none() {
            Err(String::from("For request at least response should be defined."))
        } else if !self.current.is_empty() { 
            Err(format!("Cannot close action as soon as there is not accepted part: {}", self.current))
        } else {
            Ok(())
        }
    }

    pub fn get_conclusion(&self) -> Result<String, String> {
        if let Some(conclusion) = self.conclusion.as_ref() {
            Ok(String::from(conclusion))
        } else {
            Err(String::from("Conclusion name isn't defined"))
        }
    }

    fn get_dest_file_declaration(&self, base: &Path, request: &Request) -> Result<String, String> {
        let dest = base.join("observers");
        let request = request.get_request()?;
        Ok(String::from(dest.join(format!("{}.rs", request.to_lowercase().replace(".", "_"))).to_string_lossy()))
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request: Option<String>,
    pub error: Option<String>,
    pub actions: Vec<Action>,
    pub broadcasts: Vec<String>,
    expectation: Vec<EExpectation>,
    pending: Pending,
    closed: bool,
}

impl Request {
    pub fn new(request: String) -> Self {
        Self {
            request: None,
            error: None,
            actions: vec![],
            broadcasts: vec![],
            expectation: vec![EExpectation::Word, EExpectation::PathDelimiter, EExpectation::Exclamation],
            pending: Pending::Request(request),
            closed: false,
        }
    }

    fn close(&mut self, protocol: &Protocol) -> Result<(), String> {
        if let Some(request) = self.request.as_ref() {
            if protocol.find_by_str_path(0, request).is_none() {
                return Err(format!("Request {} isn't defined in protocol", request));
            }
        } else {
            return Err(String::from("For request should be defined at least reference to request object/struct"));
        }
        if let Some(error) = self.error.as_ref() {
            if protocol.find_by_str_path(0, error).is_none() {
                return Err(format!("Error {} isn't defined in protocol", error));
            }
        } else if !self.actions.is_empty() {
            return Err(String::from("As soon as request has actions, error usecase should be defined."));
        }
        for broadcast in self.broadcasts.iter() {
            if protocol.find_by_str_path(0, broadcast).is_none() {
                return Err(format!("Broadcast object/struct {} isn't defined in protocol", broadcast));
            }
        }
        for action in self.actions.iter() {
            if action.conclusion.is_none() && self.actions.len() != 1 {
                return Err(String::from("In case request has a multiple response, should be defined conclusion for each response"));
            }
            if let Some(response) = action.response.as_ref() {
                if protocol.find_by_str_path(0, response).is_none() {
                    return Err(format!("Response object/struct {} isn't defined in protocol", response));
                }
            } else {
                return Err(String::from("Response should have reference to response object/struct"));
            }
            for broadcast in action.broadcast.iter() {
                if protocol.find_by_str_path(0, &broadcast.reference).is_none() {
                    return Err(format!("Response broadcast object/struct {} isn't defined in protocol", broadcast.reference));
                }
            }
        }
        self.closed = true;
        self.pending = Pending::Nothing;
        self.expectation = vec![];
        Ok(())
    }

    pub fn get_request(&self) -> Result<String, String> {
        if let Some(request) = self.request.as_ref() {
            Ok(String::from(request))
        } else {
            Err(String::from("Reference to object/struct request isn't defined for action"))
        }
    }

    pub fn as_filename(&self) -> Result<String, String> {
        if let Some(request) = self.request.as_ref() {
            Ok(format!("{}.rs", String::from(request).to_lowercase().replace(".", "_")))
        } else {
            Err(String::from("Reference to object/struct request isn't defined for action"))
        }
    }

    pub fn as_struct_path(&self) -> Result<String, String> {
        if let Some(request) = self.request.as_ref() {
            Ok(String::from(request).replace(".", "::"))
        } else {
            Err(String::from("Reference to object/struct request isn't defined for action"))
        }
    }

    pub fn as_struct_name(&self) -> Result<String, String> {
        if let Some(request) = self.request.as_ref() {
            Ok(String::from(request).replace(".", ""))
        } else {
            Err(String::from("Reference to object/struct request isn't defined for action"))
        }
    }

    pub fn as_mod_name(&self) -> Result<String, String> {
        if let Some(request) = self.request.as_ref() {
            Ok(String::from(request).to_lowercase().replace(".", "_"))
        } else {
            Err(String::from("Reference to object/struct request isn't defined for action"))
        }
    }

    pub fn get_response(&self) -> Result<String, String> {
        if self.actions.is_empty() {
            Err(String::from("No any actions/responses are defined for this request"))
        } else {
            if let Some(response) = self.actions[0].response.as_ref() {
                Ok(response.to_string())
            } else {
                Err(String::from("Action doesn't have defined response"))
            }
        }
    }

    pub fn get_err(&self) -> Result<String, String> {
        if let Some(error) = self.error.as_ref() {
            Ok(error.to_string())
        } else {
            Err(String::from("Action doesn't have defined error"))
        }
    }

}

impl EntityParser for Request {
    
    fn open(word: String) -> Option<Self> {
        if word.chars().all(char::is_alphanumeric) {
            Some(Self::new(word))
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
                        Pending::Request(path_to_struct) => if path_to_struct.is_empty() {
                            Err(String::from("Request isn't defined"))
                        } else {
                            /* USECASES:
                                                   |
                            Event.UserDisconnected {
                                > ...;
                            }
                            */
                            self.request = Some(path_to_struct);
                            self.pending = Pending::Nothing;
                            self.expectation = vec![EExpectation::Arrow];
                            Ok(offset)
                        },
                        Pending::Error(path_to_struct) => if self.request.is_none() {
                            Err(String::from("Request isn't defined"))
                        } else {
                            /* USECASES:
                                                           |
                            Messages.Request !Messages.Err {
                                (...) > ...;
                            }
                            */
                            self.error = Some(path_to_struct);
                            self.pending = Pending::Nothing;
                            self.expectation = vec![EExpectation::OpenBracket];
                            Ok(offset)
                        },
                        _ => Err(String::from("Listing of conclusions can be done only after Error would be defined."))
                    }
                } else {
                    Err(format!("Symbol Open isn't expected. Expectation: {:?}.", self.expectation))
                }
            },
            ENext::Word((word, offset, _next_char)) => {
                match self.pending.clone() {
                    Pending::Nothing => {
                    }
                    Pending::Request(path_to_struct) => {
                        self.pending = Pending::Request(format!("{}{}{}",
                            path_to_struct,
                            if path_to_struct.is_empty() { "" } else { "." },
                            word
                        ));
                        self.expectation = vec![
                            EExpectation::Word,
                            EExpectation::PathDelimiter,
                            EExpectation::Semicolon,
                            EExpectation::Exclamation,
                            EExpectation::Open,
                        ];
                    },
                    Pending::Error(path_to_struct) => {
                        self.pending = Pending::Error(format!("{}{}{}",
                            path_to_struct,
                            if path_to_struct.is_empty() { "" } else { "." },
                            word
                        ));
                        self.expectation = vec![
                            EExpectation::Word,
                            EExpectation::PathDelimiter,
                            EExpectation::Open,
                        ];
                    },
                    Pending::Action(mut action) => {
                        if action.response.is_none() && action.conclusion.is_none() {
                            /* USECASES:
                            Message.Request !Message.Err {
                                 |
                                (Accept    > Message.Accepted) > Events.Message;
                                 |
                                (Deny      > Message.Denied);
                                 |        | 
                                (Messages.Response);
                            }
                            */
                            action.current = format!("{}{}{}",
                                action.current,
                                if action.current.is_empty() { "" } else { "." },
                                word
                            );
                            self.expectation = vec![
                                EExpectation::PathDelimiter,
                                EExpectation::Word,
                                EExpectation::Arrow,
                                EExpectation::CloseBracket,
                            ];
                        } else if action.response.is_none() && action.conclusion.is_some() {
                            /* USECASES:
                            Message.Request !Message.Err {
                                             |       |
                                (Accept    > Message.Accepted) > Events.Message;
                                             |       |
                                (Deny      > Message.Denied);
                            }
                            */
                            action.current = format!("{}{}{}",
                                action.current,
                                if action.current.is_empty() { "" } else { "." },
                                word
                            );
                            self.expectation = vec![
                                EExpectation::PathDelimiter,
                                EExpectation::Word,
                                EExpectation::CloseBracket,
                            ];
                        } else if action.response.is_some() && action.conclusion.is_some() {
                            /* USECASES:
                            Message.Request !Message.Err {
                                                                 |
                                (Accept    > Message.Accepted) > Events.Message;
                            }
                            */
                            action.current = format!("{}{}{}",
                                action.current,
                                if action.current.is_empty() { "" } else { "." },
                                word
                            );
                            self.expectation = vec![
                                EExpectation::PathDelimiter,
                                EExpectation::Word,
                                EExpectation::Semicolon,
                                EExpectation::Question,
                            ];
                        } else {
                            return Err(format!("Unexpected place for {}", word));
                        }
                        self.pending = Pending::Action(action);
                    },
                    Pending::Broadcast(path_to_struct) => {
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
                };
                Ok(offset)
            },
            ENext::Question(offset) => {
                if is_in(&self.expectation, &EExpectation::Question) {
                    match self.pending.clone() {
                        Pending::Action(mut action) => {
                            if action.response.is_some() && action.conclusion.is_some() {
                                /* USECASES:
                                Message.Request !Message.Err {
                                                                                   |
                                    (Accept    > Message.Accepted) > Events.Message?;
                                }
                                */
                                action.optional = true;
                                self.pending = Pending::Action(action);
                                self.expectation = vec![EExpectation::Semicolon];
                                Ok(offset)
                            } else {
                                Err(format!("Symbol ? isn't expected. Expectation: {:?}", self.expectation))
                            }
                        },
                        _ => {
                            Err(format!("Symbol ? isn't expected. Expectation: {:?}", self.expectation))

                        }
                    }
                } else {
                    Err(format!("Symbol ? isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::PathDelimiter(offset) => {
                if is_in(&self.expectation, &EExpectation::PathDelimiter) {
                    self.expectation = vec![EExpectation::Word];
                    Ok(offset)
                } else {
                    Err(format!("Symbol . isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Semicolon(offset) => {
                if is_in(&self.expectation, &EExpectation::Semicolon) {
                    match self.pending.clone() {
                        Pending::Request(path_to_struct) => {
                            if path_to_struct.is_empty() {
                                return Err(String::from("Cannot close request as soon as request isn't defined"));
                            }
                            self.request = Some(path_to_struct);
                            if let Err(e) = self.close(protocol) {
                                return Err(e);
                            }
                            Ok(offset)
                        },
                        Pending::Action(mut action) => {
                            if action.current.is_empty() {
                                /* USECASES:
                                Message.Request !Message.Err {
                                                                                    |
                                    (Accept    > Message.Accepted) > Events.Message?;
                                                                |
                                    (Deny      > Message.Denied);
                                                       |
                                    (Messages.Response);
                                }
                                */
                                self.expectation = vec![
                                    EExpectation::Close,
                                    EExpectation::OpenBracket,
                                ];
                                if let Err(e) = action.valid() {
                                    return Err(e);
                                }
                                self.actions.push(action);
                                self.pending = Pending::Nothing;
                            } else {
                                /* USECASES:
                                Message.Request !Message.Err {
                                                                                   |
                                    (Accept    > Message.Accepted) > Events.Message;
                                                                                   |
                                                                   > Events.Message;
                                }
                                Messages.Request !Messages.Err {
                                                                        |
                                    (Messages.Response) > Events.Message;
                                                                        |
                                                        > Events.Message;
                                }
                                */
                                if let Err(e) = action.add_broadcast(ActionBroadcast::new(action.current.clone(), action.optional)) {
                                    return Err(e);
                                }
                                action.optional = false;
                                action.current = String::new();
                                self.pending = Pending::Action(action);
                                self.expectation = vec![
                                    EExpectation::Close,
                                    EExpectation::OpenBracket,
                                    EExpectation::Arrow,
                                ];
                            }
                            Ok(offset)
                        },
                        Pending::Broadcast(path_to_struct) => {
                            /* USECASES:
                            Event.UserDisconnected {
                                                |
                                > Events.Message;
                                                |
                                > Events.Message;
                            }
                            */
                            self.broadcasts.push(path_to_struct);
                            self.expectation = vec![
                                EExpectation::Arrow,
                                EExpectation::Close,
                            ];
                            self.pending = Pending::Nothing;
                            Ok(offset)
                        },
                        _ => Err(String::from("Symbol ; expected only after request definition."))
                    }
                } else {
                    Err(format!("Symbol ; isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Exclamation(offset) => {
                if is_in(&self.expectation, &EExpectation::Exclamation) {
                    match self.pending.clone() {
                        Pending::Request(path_to_struct) => {
                            self.request = Some(path_to_struct);
                            self.pending = Pending::Error(String::new());
                            self.expectation = vec![
                                EExpectation::Word,
                            ];
                            Ok(offset)
                        },
                        _ => Err(String::from("Symbol ! expected only after request definition."))
                    }
                } else {
                    Err(format!("Symbol ! isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::OpenBracket(offset) => {
                if is_in(&self.expectation, &EExpectation::OpenBracket) {
                    match self.pending.clone() {
                        Pending::Nothing => {
                            /* USECASES:
                            Message.Request !Message.Err {
                                |
                                (Accept    > Message.Accepted) > Events.Message;
                                |
                                (Deny      > Message.Denied);
                                |
                                (Messages.Response);
                            }
                            */
                            self.pending = Pending::Action(Action::new());
                            self.expectation = vec![EExpectation::Word];
                            Ok(offset)
                        },
                        Pending::Action(action) => {
                            /* USECASES:
                            Message.Request !Message.Err {
                                |
                                (Deny      > Message.Denied);
                                |
                                (Messages.Response);
                            }
                            */
                            if let Err(e) = action.valid() {
                                Err(e)
                            } else {
                                self.actions.push(action);
                                self.pending = Pending::Action(Action::new());
                                self.expectation = vec![EExpectation::Word];
                                Ok(offset)
                            }
                        },
                        _ => Err(format!("Incorrect position to open conclusion. Pending: {:?}", self.pending)),
                    }
                } else {
                    Err(format!("Symbol ( isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::CloseBracket(offset) => {
                if is_in(&self.expectation, &EExpectation::CloseBracket) {
                    match self.pending.clone() {
                        Pending::Action(mut action) => {
                            /* USECASES:
                            Message.Request !Message.Err {
                                                             |
                                (Accept    > Message.Accepted) > Events.Message;
                                                            |
                                (Deny      > Message.Denied);
                                                  |
                                (Messages.Response);
                            }
                            */
                            if action.current.is_empty() {
                                return Err(String::from("Cannot close action without at least definition of response"));
                            } else if action.response.is_none() {
                                action.response = Some(action.current.clone());
                                action.current = String::new();
                            } else {
                                return Err(String::from("Cannot close action multiple times"));
                            }
                            self.pending = Pending::Action(action);
                            self.expectation = vec![
                                EExpectation::Arrow,
                                EExpectation::Semicolon,
                            ];
                            Ok(offset)
                        },
                        _ => Err(format!("Incorrect position for close conclusion. Pending: {:?}", self.pending)),
                    }
                } else {
                    Err(format!("Symbol ) isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Arrow(offset) => {
                if is_in(&self.expectation, &EExpectation::Arrow) {
                    match self.pending.clone() {
                        Pending::Nothing => {
                            /* USECASES:
                            Event.UserDisconnected {
                                |
                                > Events.Message;
                                > Events.Message;
                            }
                            */
                            self.pending = Pending::Broadcast(String::new());
                            self.expectation = vec![
                                EExpectation::Word,
                                EExpectation::PathDelimiter,
                            ];
                            Ok(offset)
                        },
                        Pending::Action(mut action) => {
                            if action.conclusion.is_none() && action.response.is_none() && !action.current.is_empty() {
                                /* USECASES:
                                Message.Request !Message.Err {
                                               |
                                    (Accept    > Message.Accepted) > Events.Message;
                                               |
                                    (Deny      > Message.Denied);
                                }
                                */
                                action.conclusion = Some(action.current);
                                action.current = String::new();
                                self.expectation = vec![
                                    EExpectation::Word,
                                ];
                            } else if action.response.is_some() && action.current.is_empty() {
                                /* USECASES:
                                Message.Request !Message.Err {
                                                                   |
                                    (Accept    > Message.Accepted) > Events.Message;
                                    (Deny      > Message.Denied);
                                }
                                Messages.Request !Messages.Err {
                                                        |
                                    (Messages.Response) > Events.Message;
                                }
                                */
                                self.expectation = vec![
                                    EExpectation::Word,
                                ];
                            }
                            self.pending = Pending::Action(action);
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
                        Pending::Action(action) => {
                            /* USECASES:
                            Message.Request !Message.Err {
                                (Accept    > Message.Accepted) > Events.Message;
                            |    
                            }
                            Messages.Request !Messages.Err {
                                (Messages.Response) > Events.Message;
                            |
                            }
                            Messages.Request !Messages.Err {
                                (Messages.Response);
                            |
                            }
                            */
                            if let Err(e) = action.valid() {
                                return Err(e);
                            }
                            self.actions.push(action);
                        },
                        Pending::Broadcast(path_to_struct) => {
                            /* USECASES:
                            Messages.Request {
                                > Events.Message;
                            |
                            }
                            */
                            if path_to_struct.is_empty() {
                                return Err(String::from("Fail to add broadcast without reference to struct"));
                            }
                            self.pending = Pending::Nothing;
                            self.broadcasts.push(path_to_struct);
                        },
                        _ => {

                        },
                    }
                    if let Err(e) = self.close(protocol) {
                        return Err(e);
                    }
                    Ok(offset)
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
        EntityOut::Request(self.clone())
    }

}