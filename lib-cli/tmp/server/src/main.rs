
#[macro_use]
extern crate lazy_static;

#[path = "../producer/producer.rs"]
pub mod producer;

use fiber_transport_server::{
    server::Server,
};

use fiber::{
    logger::{
        LogLevel,
    },
};

use producer::{
    userlogin_request,
    userlogin_request::{
        ObserverRequest as UserLoginRequestObserver,
    },
    users_request::{
        ObserverRequest as UsersRequestObserver,
    },
    message_request,
    message_request::{
        ObserverRequest as MessageRequestObserver,
    },
    messages_request::{
        ObserverRequest as MessagesRequestObserver,
    },
    serverevents_userkickoff,
    serverevents_userkickoff::{
        ObserverEvent as ServerEventsUserKickOffEvent,
    },
    default_connected_event::{
        ObserverEvent as ConnectedEvent,
    },
    default_disconnected_event::{
        ObserverEvent as DisconnectedEvent,
    },
    consumer_identification::Filter,
    protocol as Protocol,
    consumer::Cx,
    Control,
};
use std::sync::{
    Arc,
    RwLock
};
use regex::Regex;
use std::time::{
    SystemTime,
    UNIX_EPOCH
};
use uuid::Uuid;
use futures::{
    executor,
};
use tokio::{
    join,
    runtime::Runtime,
};

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::DefaultLogger;

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Producer".to_owned(), Some(5));
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

#[allow(unused_variables)]
#[allow(non_snake_case)]
impl UserLoginRequestObserver {
    fn conclusion<WrappedCustomContext>(
        request: &Protocol::UserLogin::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<userlogin_request::Conclusion, Protocol::UserLogin::Err> {
        Ok(producer::userlogin_request::Conclusion::Accept(
            producer::protocol::UserLogin::Accepted {
                uuid: cx.uuid().to_string(),
            },
        ))
    }
    
    fn Accept<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (
            (Filter, Protocol::Events::UserConnected),
            (Filter, Protocol::Events::Message)
        ),
        String
    > {
        match store::users.write() {
            Ok(mut users) => {
                users.insert(cx.uuid(), store::User {
                    name: request.username.clone(),
                    uuid: cx.uuid(),
                });
                if let Err(e) = executor::block_on(async move {
                    if let Err(e) = cx.assign(producer::protocol::Identification::AssignedKey {
                        uuid: Some(cx.uuid().to_string()),
                        auth: Some(true),
                    }, true) {
                        return Err(format!("Fail to assign client due error: {}", e));
                    }
                    Ok::<(), String>(())
                }) {
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
                        Ok((
                            (filter.clone(), producer::protocol::Events::UserConnected {
                                uuid: cx.uuid().to_string(),
                                username: "----".to_string(),
                            }),
                            (filter, producer::protocol::Events::Message {
                                user: "".to_owned(),
                                message: msg,
                                timestamp: tm.as_secs(),
                                uuid: cx.uuid().to_string(),
                            }),
                        ))
                    },
                    Err(e) => Err(format!("Fail write message due error: {}", e)),
                }
            },
            Err(e) => Err(format!("Fail write user due error: {}", e)),
        }
    }
    
    fn Deny<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (),
        String
    > {
        Err(String::from("Deny method isn't implemented"))
    }

}
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl UsersRequestObserver {
    fn response<WrappedCustomContext>(
        request: &Protocol::Users::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<Protocol::Users::Response, Protocol::Users::Err> {
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
#[allow(non_snake_case)]
impl MessageRequestObserver {
    fn conclusion<WrappedCustomContext>(
        request: &Protocol::Message::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<message_request::Conclusion, Protocol::Message::Err> {
        let re = Regex::new(r"[<>]").unwrap();
        if re.is_match(&request.message) {
            Ok(producer::message_request::Conclusion::Deny(
                producer::protocol::Message::Denied {
                    reason: "Symbols < and > cannot be used".to_owned(),
                },
            ))
        } else {
            Ok(producer::message_request::Conclusion::Accept(
                producer::protocol::Message::Accepted {
                    uuid: cx.uuid().to_string(),
                },
            ))
        }
    }
    
    fn Accept<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::Message::Request,
    ) -> Result<
        (Filter, Protocol::Events::Message),
        String
    > {
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
                Ok((Filter {
                    uuid: Some((cx.uuid(), producer::consumer_identification::Condition::NotEqual)),
                    assign: Some(true),
                    filter: None,
                }, producer::protocol::Events::Message {
                    user: request.user.clone(),
                    message: request.message.clone(),
                    timestamp: tm.as_secs(),
                    uuid: cx.uuid().to_string(),
                }))
            },
            Err(e) => Err(format!("{}", e))
        }
    }
    
    fn Deny<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::Message::Request,
    ) -> Result<
        (),
        String
    > {
        Err(String::from("Deny method isn't implemented"))
    }

}
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl MessagesRequestObserver {
    fn response<WrappedCustomContext>(
        request: &Protocol::Messages::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<Protocol::Messages::Response, Protocol::Messages::Err> {
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
}

impl ConnectedEvent {
    fn handler<WrappedCustomContext>(
        _uuid: Uuid,
        _ucx: WrappedCustomContext,
        _broadcast: &dyn Fn(Filter, producer::broadcast::Broadcast) -> Result<(), String>,
    ) -> () {
        // Implementation
    }
}

impl DisconnectedEvent {
    fn handler<WrappedCustomContext>(
        uuid: Uuid,
        _ucx: WrappedCustomContext,
        broadcast: &dyn Fn(Filter, producer::broadcast::Broadcast) -> Result<(), String>,
    ) -> () {
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
                            if let Err(e) = broadcast(filter.clone(), producer::broadcast::Broadcast::EventsUserDisconnected(
                                producer::protocol::Events::UserDisconnected {
                                    username: user.name,
                                    uuid: uuid.to_string(),
                                }
                            )) {
                                println!("Fail to send broadcasting. Error: {}", e);
                            }
                            // match (producer::protocol::Events::UserDisconnected {
                            //     username: user.name,
                            //     uuid: uuid.to_string(),
                            // }).pack(0, Some(uuid.to_string())) {
                            //     Ok(buffer) => {
                            //         if let Err(e) = broadcast(filter.clone(), buffer) {
                            //             println!("Fail to send broadcasting. Error: {}", e);
                            //         }
                            //     },
                            //     Err(e) => {
                            //         println!("broadcasting error: {}", e);
                            //     }
                            // };
                            // match (producer::protocol::Events::Message {
                            //     user: "".to_owned(),
                            //     message: msg,
                            //     timestamp: tm.as_secs(),
                            //     uuid: uuid.to_string(),
                            // }).pack(0, Some(uuid.to_string())) {
                            //     Ok(buffer) => {
                            //         if let Err(e) = broadcast(filter.clone(), buffer) {
                            //             println!("Fail to send broadcasting. Error: {}", e);
                            //         }
                            //     },
                            //     Err(e) => {
                            //         println!("broadcasting error: {}", e);
                            //     }
                            // };
                        },
                        Err(e) => {
                            println!("Cannot get access to messages due error: {}", e)
                        }
                    }
                } else {
                    println!("No {} user has been found", uuid);
                }
            },
            Err(e) => {
                println!("{}", e)
            }
        };
    }
}

impl ServerEventsUserKickOffEvent {
    fn handler<WrappedCustomContext>(
        event: &Protocol::ServerEvents::UserKickOff,
        ucx: WrappedCustomContext,
        control: Control,
    ) -> Option<Vec<(Filter, serverevents_userkickoff::Broadcast)>> {
        // Implementation
        None      
    }
}


#[allow(non_snake_case)]
impl producer::ProducerEventsHolder {

    fn Connected(uuid: Uuid) {
        println!("=========> {} has been connected!", uuid);
    }

}

fn main() {
    match fiber::tools::LOGGER_SETTINGS.lock() {
        Ok(mut settings) => settings.set_level(LogLevel::Verb),
        Err(e) => println!("Fail set log level due error: {}", e),
    };
    let server: Server = Server::new(String::from("127.0.0.1:8080"));
    let ucx = CustomContext {};
    producer::init_and_start(server, ucx, None);
    // let rt  = match Runtime::new() {
    //     Ok(rt) => rt,
    //     Err(e) => {
    //         panic!(e);
    //     },
    // };
    // rt.block_on( async move {
    //     let (thread, control) = producer::init(server, ucx);
    //     let kickoff_task = async move {
    //         tokio::time::sleep(std::time::Duration::from_millis(20000)).await;
    //         control.events.KickOffEvent.send(producer::KickOffEvent::Event {
    //             reason: String::from("Test")
    //         });
    //         tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    //     };
    //     join!(
    //         thread,
    //         kickoff_task,
    //     );
    // });
}
