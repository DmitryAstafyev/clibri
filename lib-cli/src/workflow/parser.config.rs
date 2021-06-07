use super::{
    Target,
    ENext,
    EntityParser,
    EntityOut,
    Protocol,
    INTERNAL_SERVICE_GROUP,
    Field,
    PrimitiveTypes,
};

mod key_words {
    pub const PRODUCER: &str = "Producer";
    pub const CONSUMER: &str = "Consumer";
    pub const SELF_KEY: &str = "SelfKey";
    pub const ASSIGNED_KEY: &str = "AssignedKey";
    pub const ALIAS: &str = "&config";
}

pub mod names {
    pub const DEFAULT_SELF_KEY_REQUEST_STRUCT: &str = "SelfKeyRequest";
    pub const DEFAULT_SELF_KEY_RESPONSE_STRUCT: &str = "SelfKeyResponse";
    pub const DEFAULT_ASSIGNED_KEY_STRUCT: &str = "AssignedKey";
}

#[derive(Debug, PartialEq, Clone)]
enum EExpectation {
    Word,
    ValueDelimiter,
    Semicolon,
    Comma,
    PathDelimiter,
    Open,
}

#[derive(Debug, Clone)]
enum Pending {
    Nothing,
    Producer,
    Consumer,
    SelfKey(String),
    AssignedKey(String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub producer: Vec<Target>,
    pub consumer: Vec<Target>,
    pub self_key: Option<String>,
    pub self_key_response: String,
    pub assigned_key: Option<String>,
    closed: bool,
    expectation: Vec<EExpectation>,
    pending: Pending,
    prev: Option<ENext>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {

    pub fn new() -> Self {
        Self {
            producer: vec![],
            consumer: vec![],
            self_key: None,
            self_key_response: format!("{}.{}", INTERNAL_SERVICE_GROUP, names::DEFAULT_SELF_KEY_RESPONSE_STRUCT),
            assigned_key: None,
            closed: false,
            expectation: vec![EExpectation::Open],
            pending: Pending::Nothing,
            prev: None,
        }
    }

    fn add_producer(&mut self, alias: String) -> Result<(), String> {
        let target: Target = if alias == *"rust" {
            Target::Rust
        } else if alias == *"typescript" {
            Target::TypeScript
        } else {
            return Err(format!("Unknown producer target {}", alias));
        };
        if self.producer.contains(&target) {
            Err(format!("Target {} has been added already to producer", alias))
        } else {
            self.producer.push(target);
            Ok(())
        }
    }

    fn add_consumer(&mut self, alias: String) -> Result<(), String> {
        let target: Target = if alias == *"rust" {
            Target::Rust
        } else if alias == *"typescript" {
            Target::TypeScript
        } else {
            return Err(format!("Unknown consumer target {}", alias));
        };
        if self.consumer.contains(&target) {
            Err(format!("Target {} has been added already to consumer", alias))
        } else {
            self.consumer.push(target);
            Ok(())
        }
    }

    fn set_self_key(&mut self, key: String) -> Result<(), String> {
        if let Some(value) = self.self_key.as_ref() {
            Err(format!("Self key is already set to {}", value))
        } else {
            self.self_key = Some(key);
            Ok(())
        }
    }

    fn set_assigned_key(&mut self, key: String) -> Result<(), String> {
        if let Some(value) = self.assigned_key.as_ref() {
            Err(format!("Assigned key is already set to {}", value))
        } else {
            self.assigned_key = Some(key);
            Ok(())
        }
    }

    fn close(&mut self, protocol: &mut Protocol) -> Result<(), String> {
        if let Some(self_key) = self.self_key.as_ref() {
            if protocol.find_by_str_path(0, self_key).is_none() {
                return Err(format!("Self key {} isn't defined in protocol", self_key));
            }
            protocol.add_service_struct(names::DEFAULT_SELF_KEY_RESPONSE_STRUCT.to_owned(), vec![
                Field::create_not_assigned_primitive(String::from("uuid"), PrimitiveTypes::ETypes::Estr, false),
            ]);
        } else {
            println!("Self Key for consumer isn't defined. Basic implementation would be included.");
            protocol.add_service_struct(names::DEFAULT_SELF_KEY_REQUEST_STRUCT.to_owned(), vec![
                Field::create_not_assigned_primitive(String::from("uuid"), PrimitiveTypes::ETypes::Estr, true),
            ]);
            protocol.add_service_struct(names::DEFAULT_SELF_KEY_RESPONSE_STRUCT.to_owned(), vec![
                Field::create_not_assigned_primitive(String::from("uuid"), PrimitiveTypes::ETypes::Estr, false),
            ]);
            self.self_key = Some(format!("{}.{}", INTERNAL_SERVICE_GROUP, names::DEFAULT_SELF_KEY_REQUEST_STRUCT));
        }
        if let Some(assigned_key) = self.assigned_key.as_ref() {
            if protocol.find_by_str_path(0, assigned_key).is_none() {
                return Err(format!("Assigned key {} isn't defined in protocol", assigned_key));
            }
        } else {
            protocol.add_service_struct(names::DEFAULT_ASSIGNED_KEY_STRUCT.to_owned(), vec![]);
            self.assigned_key = Some(format!("{}.{}", INTERNAL_SERVICE_GROUP, names::DEFAULT_ASSIGNED_KEY_STRUCT));
            println!("Assigned Key for consumer isn't defined. Basic implementation would be included.");
        }
        if self.producer.is_empty() {
            Err(String::from("No targets for producer has been found"))
        } else if self.consumer.is_empty() {
            Err(String::from("No targets for consumer has been found"))
        } else {
            self.closed = true;
            self.prev = None;
            Ok(())
        }
    }

    pub fn get_self(&self) -> Result<String, String> {
        if let Some(self_key) = self.self_key.as_ref() {
            Ok(String::from(self_key))
        } else {
            Err(String::from("Self key isn't defined for workflow"))
        }
    }

    pub fn get_assigned(&self) -> Result<String, String> {
        if let Some(assigned_key) = self.assigned_key.as_ref() {
            Ok(String::from(assigned_key))
        } else {
            Err(String::from("Assigned key isn't defined for workflow"))
        }
    }

}

impl EntityParser for Config {

    fn open(word: String) -> Option<Self> {
        if word == key_words::ALIAS {
            Some(Config::new())
        } else {
            None
        }
    }

    fn next(&mut self, enext: ENext, protocol: &mut Protocol) -> Result<usize, String> {
        fn is_in(src: &[EExpectation], target: &EExpectation) -> bool {
            src.iter().any(|e| e == target)
        }
        let prev = enext.clone();
        match enext {
            ENext::Open(offset) => {
                if is_in(&self.expectation, &EExpectation::Open) {
                    self.expectation = vec![EExpectation::Word];
                    Ok(offset)
                } else {
                    Err(format!("Symbol isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::Word((word, offset, _next_char)) => {
                if is_in(&self.expectation, &EExpectation::Word) {
                    match &self.pending {
                        Pending::Nothing => {
                            if word == key_words::PRODUCER {
                                if !self.producer.is_empty() {
                                    return Err(String::from("Producer targets cannot be defined multiple times"));
                                }
                                self.pending = Pending::Producer;
                            } else if word == key_words::CONSUMER {
                                if !self.consumer.is_empty() {
                                    return Err(String::from("Consumer targets cannot be defined multiple times"));
                                }
                                self.pending = Pending::Consumer;
                            } else if word == key_words::ASSIGNED_KEY {
                                if self.assigned_key.is_some() && !matches!(prev, ENext::ValueDelimiter(_) | ENext::PathDelimiter(_)) {
                                    return Err(String::from("Assigned Key is already defined"));
                                }
                                self.pending = Pending::AssignedKey(String::new());
                            } else if word == key_words::SELF_KEY {
                                if self.self_key.is_some() && !matches!(prev, ENext::ValueDelimiter(_) | ENext::PathDelimiter(_)) {
                                    return Err(String::from("Self Key is already defined"));
                                }
                                self.pending = Pending::SelfKey(String::new());
                            } else {
                                return Err(format!("Unexpected keyword: {}", word));
                            }
                            self.expectation = vec![EExpectation::ValueDelimiter];
                        },
                        Pending::Producer => match self.add_producer(word) {
                            Ok(_) => {
                                self.expectation = vec![EExpectation::Word, EExpectation::Comma, EExpectation::Semicolon];
                            },
                            Err(e) => {
                                return Err(e);
                            },
                        },
                        Pending::Consumer => match self.add_consumer(word) {
                            Ok(_) => {
                                self.expectation = vec![EExpectation::Word, EExpectation::Comma, EExpectation::Semicolon];
                            },
                            Err(e) => {
                                return Err(e);
                            },
                        },
                        Pending::AssignedKey(path_to_struct) => {
                            self.pending = Pending::AssignedKey(format!("{}{}{}",
                                path_to_struct,
                                if path_to_struct.is_empty() { "" } else { "." },
                                word
                            ));
                            self.expectation = vec![EExpectation::Word, EExpectation::PathDelimiter, EExpectation::Semicolon];
                        },
                        Pending::SelfKey(path_to_struct) => {
                            self.pending = Pending::SelfKey(format!("{}{}{}",
                                path_to_struct,
                                if path_to_struct.is_empty() { "" } else { "." },
                                word
                            ));
                            self.expectation = vec![EExpectation::Word, EExpectation::PathDelimiter, EExpectation::Semicolon];
                        }
                    };
                    self.prev = Some(prev);
                    Ok(offset)
                } else {
                    Err(format!("Symbol isn't expected. Expectation: {:?}", self.expectation))
                }
            },
            ENext::PathDelimiter(offset) => if is_in(&self.expectation, &EExpectation::PathDelimiter) {
                self.expectation = vec![EExpectation::Word];
                Ok(offset)
            } else {
                Err(format!("Symbol . isn't expected. Expectation: {:?}", self.expectation))
            },
            ENext::ValueDelimiter(offset) => if is_in(&self.expectation, &EExpectation::ValueDelimiter) {
                self.expectation = vec![EExpectation::Word];
                Ok(offset)
            } else {
                Err(format!("Symbol : isn't expected. Expectation: {:?}", self.expectation))
            },
            ENext::Semicolon(offset) => if is_in(&self.expectation, &EExpectation::Semicolon) {
                match self.pending.clone() {
                    Pending::Nothing => {
                        return Err(format!("Symbol ; isn't expected. Expectation: {:?}", self.expectation));
                    },
                    Pending::Consumer => if self.consumer.is_empty() {
                        self.print();
                        return Err(String::from("No alias was provided for consumer"))
                    },
                    Pending::Producer => if self.producer.is_empty() {
                        return Err(String::from("No alias was provided for producer"))
                    },
                    Pending::AssignedKey(path_to_struct) => if let Err(e) = self.set_assigned_key(path_to_struct) {
                        return Err(e);
                    },
                    Pending::SelfKey(path_to_struct) => if let Err(e) = self.set_self_key(path_to_struct) {
                        return Err(e);
                    },
                };
                self.pending = Pending::Nothing;
                self.expectation = vec![EExpectation::Word];
                Ok(offset)
            } else {
                Err(format!("Symbol ; isn't expected. Expectation: {:?}", self.expectation))
            },
            ENext::Close(offset) => if let Err(e) = self.close(protocol) {
                Err(e)
            } else {
                Ok(offset)
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
        EntityOut::Config(self.clone())
    }

}