#[macro_use]
extern crate lazy_static;

#[path = "../producer/src/lib.rs"]
pub mod producer;

use fiber::logger::LogLevel;
use fiber_transport_server::server::Server;
use fiber_transport_server::{ErrorResponse, Request, Response};
use producer::UserLoginObserver::{
    Observer as UserLoginObserver,
    ObserverRequest as UserLoginObserverRequest,
    AcceptBroadcasting as UserLoginAcceptBroadcasting,
};
use producer::UsersObserver::{Observer as UsersObserver, ObserverRequest as UsersObserverRequest};
use producer::MessageObserver::{
    Observer as MessageObserver,
    ObserverRequest as MessageObserverRequest,
    AcceptBroadcasting as MessageAcceptBroadcasting,
};
use producer::MessagesObserver::{
    Observer as MessagesObserver, ObserverRequest as MessagesObserverRequest,
};
/*
use producer::EventDisconnected::{
    Controller as EventDisconnectedController, Observer as EventDisconnectedObserver,
};*/
use producer::consumer_identification::Filter;
use producer::*;
use std::sync::{Arc, RwLock};
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// use std::thread::spawn;

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::DefaultLogger;

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Producer".to_owned(), None);
    }
}

#[allow(non_upper_case_globals)]
pub mod store {
    use std::collections::HashMap;
    use uuid::Uuid;
    use std::sync::{RwLock};

    #[derive(Clone, Debug)]
    pub struct User {
        pub name: String,
        pub uuid: Uuid,
    }

    #[derive(Clone, Debug)]
    pub struct Message {
        pub name: String,
        pub uuid: Uuid,
        pub message: String,
        pub timestamp: u64,
    }
    lazy_static! {
        pub static ref users: RwLock<HashMap<Uuid, User>> = RwLock::new(HashMap::new());
        pub static ref messages: RwLock<HashMap<Uuid, Message>> = RwLock::new(HashMap::new());
    }
}

#[derive(Clone)]
struct CustomContext {}

impl CustomContext {}

type WrappedCustomContext = Arc<RwLock<CustomContext>>;

struct ProducerInstance {}

impl Producer<Server, WrappedCustomContext> for ProducerInstance {

    fn disconnected(uuid: Uuid, _ucx: WrappedCustomContext) -> Result<producer::EventDisconnectedBroadcasting, String> {
        match store::users.write() {
            Ok(mut users) => {
                if let Some(user) = users.remove(&uuid) {
                    let filter = Filter {
                        uuid: Some((uuid.clone(), producer::consumer_identification::Condition::NotEqual)),
                        assign: Some(true),
                        filter: None,
                    };
                    let start = SystemTime::now();
                    let tm = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    let msg = format!("{} left chat", user.name);
                    match store::messages.write() {
                        Ok(mut messages) => {
                            messages.insert(Uuid::new_v4(), store::Message {
                                name: "".to_owned(),
                                uuid: uuid,
                                message: msg.clone(),
                                timestamp: tm.as_secs(),
                            });
                            Ok(producer::EventDisconnectedBroadcasting {
                                UserDisconnected: (
                                    filter.clone(),
                                    producer::protocol::Events::UserDisconnected {
                                        username: user.name,
                                        uuid: uuid.to_string(),
                                }),
                                Message: Some((filter,
                                    producer::protocol::Events::Message {
                                        user: "".to_owned(),
                                        message: msg,
                                        timestamp: tm.as_secs(),
                                        uuid: uuid.to_string(),
                                }))
                            })

                        },
                        Err(e) => Err(format!("Cannot get access to messages due error: {}", e))
                    }
                } else {
                    Err(format!("No {} user has been found", uuid))
                }
            },
            Err(e) => Err(format!("{}", e))
        }
    }

}

#[allow(unused_variables)]
impl UserLoginObserver for UserLoginObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserLogin::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
    ) -> Result<producer::UserLoginObserver::Conclusion, producer::protocol::UserLogin::Err> {
        Ok(producer::UserLoginObserver::Conclusion::Accept(
            producer::protocol::UserLogin::Accepted {
                uuid: cx.uuid().to_string(),
            },
        ))
    }

    fn Accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::UserLogin::Request,
    ) -> Result<UserLoginAcceptBroadcasting, String> {
        match store::users.write() {
            Ok(mut users) => {
                users.insert(cx.uuid(), store::User {
                    name: request.username.clone(),
                    uuid: cx.uuid(),
                });
                if let Err(e) = cx.assign(producer::protocol::Identification::AssignedKey {
                    uuid: Some(cx.uuid().to_string()),
                    auth: Some(true),
                }, true) {
                    return Err(format!("Fail to assign client due error: {}", e));
                }
                let start = SystemTime::now();
                let tm = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let msg = format!("{} join chat. Welcome {}!", request.username, request.username);
                match store::messages.write() {
                    Ok(mut messages) => {
                        messages.insert(Uuid::new_v4(), store::Message {
                            name: "".to_owned(),
                            uuid: cx.uuid(),
                            message: msg.clone(),
                            timestamp: tm.as_secs(),
                        });
                        let filter = Filter {
                            uuid: Some((cx.uuid(), producer::consumer_identification::Condition::NotEqual)),
                            assign: Some(true),
                            filter: None,
                        };
                        Ok(UserLoginAcceptBroadcasting {
                            UserConnected: (filter.clone(), producer::protocol::Events::UserConnected {
                                uuid: cx.uuid().to_string(),
                                username: "----".to_string(),
                            }),
                            Message: Some((filter, producer::protocol::Events::Message {
                                user: "".to_owned(),
                                message: msg,
                                timestamp: tm.as_secs(),
                                uuid: cx.uuid().to_string(),
                            })),
                        })
                    },
                    Err(e) => Err(format!("Fail write message due error: {}", e)),
                }
            },
            Err(e) => Err(format!("Fail write user due error: {}", e)),
        }
    }

}


#[allow(unused_variables)]
impl UsersObserver for UsersObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::Users::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
    ) -> Result<producer::protocol::Users::Response, producer::protocol::Users::Err> {
        match store::users.read() {
            Ok(users) => Ok(producer::protocol::Users::Response {
                users: users
                    .values()
                    .cloned()
                    .map(|user| producer::protocol::Users::User {
                        name: user.name,
                        uuid: user.uuid.to_string(),
                    })
                    .collect(),
            }),
            Err(e) => Err(producer::protocol::Users::Err {
                error: format!("{}", e) 
            })
        }
    }

}

#[allow(unused_variables)]
impl MessageObserver for MessageObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::Message::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
    ) -> Result<producer::MessageObserver::Conclusion, producer::protocol::Message::Err> {
        let re = Regex::new(r"[<>]").unwrap();
        if re.is_match(&request.message) {
            Ok(producer::MessageObserver::Conclusion::Deny(
                producer::protocol::Message::Denied {
                    reason: "Symbols < and > cannot be used".to_owned(),
                },
            ))
        } else {
            Ok(producer::MessageObserver::Conclusion::Accept(
                producer::protocol::Message::Accepted {
                    uuid: cx.uuid().to_string(),
                },
            ))
        }
    }

    fn Accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::Message::Request,
    ) -> Result<MessageAcceptBroadcasting, String> {
        let start = SystemTime::now();
        let tm = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        match store::messages.write() {
            Ok(mut messages) => {
                messages.insert(Uuid::new_v4(), store::Message {
                    name: request.user.clone(),
                    uuid: cx.uuid(),
                    message: request.message.clone(),
                    timestamp: tm.as_secs(),
                });
                let start = SystemTime::now();
                let tm = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                Ok(MessageAcceptBroadcasting {
                    Message: (Filter {
                        uuid: Some((cx.uuid(), producer::consumer_identification::Condition::NotEqual)),
                        assign: Some(true),
                        filter: None,
                    }, producer::protocol::Events::Message {
                        user: request.user,
                        message: request.message,
                        timestamp: tm.as_secs(),
                        uuid: cx.uuid().to_string(),
                    })
                })
            },
            Err(e) => Err(format!("{}", e))
        }
    }

}

#[allow(unused_variables)]
impl MessagesObserver for MessagesObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::Messages::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
    ) -> Result<producer::protocol::Messages::Response, producer::protocol::Messages::Err> {
        match store::messages.read() {
            Ok(messages) => {
                let mut msgs: Vec<producer::protocol::Messages::Message> = messages
                    .values()
                    .cloned()
                    .map(|msg| producer::protocol::Messages::Message {
                        timestamp: msg.timestamp,
                        user: msg.name,
                        uuid: msg.uuid.to_string(),
                        message: msg.message,
                    })
                    .collect();
                msgs.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
                Ok(producer::protocol::Messages::Response {
                    messages: msgs
                })
            },
            Err(e) => Err(producer::protocol::Messages::Err {
                error: format!("{}", e)
            })
        }
    }

    fn Response<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::Messages::Request,
    ) -> Result<(), String> {
        Ok(())
        // Remove
    }
}
        /*

#[allow(unused_variables)]
impl EventDisconnectedController for EventDisconnectedObserver {
    fn disconnected<WrappedCustomContext>(
        uuid: Uuid,
        ucx: WrappedCustomContext,
    ) -> Result<(), String> {
        match store::users.write() {
            Ok(mut users) => {
                if let Some(user) = users.remove(&uuid) {
                    let filter = Filter {
                        uuid: Some((uuid.clone(), producer::consumer_identification::Condition::NotEqual)),
                        assign: Some(true),
                        filter: None,
                    };
                    let start = SystemTime::now();
                    let tm = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    let msg = format!("{} left chat", user.name);
                    match store::messages.write() {
                        Ok(mut messages) => {
                            messages.insert(Uuid::new_v4(), store::Message {
                                name: "".to_owned(),
                                uuid: uuid.clone(),
                                message: msg.clone(),
                                timestamp: tm.as_secs(),
                            });
                            let filter = Filter {
                                uuid: Some((uuid.clone(), producer::consumer_identification::Condition::NotEqual)),
                                assign: Some(true),
                                filter: None,
                            };
                            if let Err(e) = broadcasting(
                                filter,
                                Broadcasting::Message(producer::protocol::Events::Message {
                                    user: "".to_owned(),
                                    message: msg,
                                    timestamp: tm.as_secs(),
                                    uuid: uuid.to_string(),
                                }),
                            ) {
                                println!("Fail broadcast due error: {}", e);
                            }
                        },
                        Err(e) => {}
                    };
                    broadcasting(
                        filter,
                        Broadcasting::UserDisconnected(producer::protocol::Events::UserDisconnected {
                            username: user.name,
                            uuid: uuid.to_string(),
                        }),
                    )
                } else {
                    Err(format!("No {} user has been found", uuid))
                }
            },
            Err(e) => Err(format!("{}", e))
        }
        Ok(())
    }
}
        */

fn main() {
    match fiber::tools::LOGGER_SETTINGS.lock() {
        Ok(mut settings) => settings.set_level(LogLevel::Info),
        Err(e) => println!("Fail set log level due error: {}", e),
    };
    /*
    spawn(move || {
        let server: Server = Server::new(String::from("127.0.0.1:8080"));
        let ucx = CustomContext {};
        // let mut producer: ProducerInstance = ProducerInstance {};
        let _feedback = match ProducerInstance::listen(server, Arc::new(RwLock::new(ucx)), None) {
            Ok(feedback) => feedback,
            Err(e) => panic!(e),
        };
    });
    */
    let mut server: Server = Server::new(String::from("127.0.0.1:8080"));
    if let Err(e) = server
        .handshake(|_: &Request, res: Response| -> Result<Response, ErrorResponse> { Ok(res) })
    {
        println!("Fail to assign handshake hadler due error: {}", e);
    }
    let ucx = CustomContext {};
    let producer: ProducerInstance = ProducerInstance {};
    let _feedback = match producer.listen(server, Arc::new(RwLock::new(ucx))) {
        Ok(feedback) => loop {
            match feedback.events.recv() {
                Ok(m) => match m {
                    producer::ProducerEvents::Connected((_uuid, _ucx)) => {
                        println!(">>>>>> Connected");
                    }
                    producer::ProducerEvents::ServerDown => {
                        println!(">>>>>> ServerDown");
                    }
                    producer::ProducerEvents::Disconnected(_uuid) => {
                        println!(">>>>>> Disconnected");
                    }
                    producer::ProducerEvents::InternalError(e) => {
                        println!(">>>>>> InternalError: {}", e);
                    }
                    producer::ProducerEvents::EmitError(e) => {
                        println!(">>>>>> EmitError: {}", e);
                    }
                    producer::ProducerEvents::EventError(e) => {
                        println!(">>>>>> EventError: {}", e);
                    }
                    producer::ProducerEvents::EventChannelError(e) => {
                        println!(">>>>>> EventChannelError: {}", e);
                    }
                    producer::ProducerEvents::ConnectionError(e) => {
                        println!(">>>>>> ConnectionError: {}", e);
                    }
                    producer::ProducerEvents::ServerError(e) => {
                        println!("ServerError: {}", e);
                    }
                    producer::ProducerEvents::BroadcastingError(e) => {
                        println!("BroadcastingError: {}", e);
                    }
                    producer::ProducerEvents::Reading(e) => {
                        println!("Reading: {}", e);
                    }
                    producer::ProducerEvents::EventListenError(e) => {
                        println!("EventListenError: {}", e);
                    }
                    producer::ProducerEvents::NotAssignedConsumer(e) => {
                        println!("NotAssignedConsumer: {}", e);
                    }
                },
                Err(e) => {
                    panic!("Error on events: {:?}", e);
                }
            }
        },
        Err(e) => panic!(e),
    };
}
