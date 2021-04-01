#[macro_use]
extern crate lazy_static;

#[path = "../producer/src/lib.rs"]
pub mod producer;

use fiber::logger::LogLevel;
use fiber_transport_server::server::Server;
use fiber_transport_server::{ErrorResponse, Request, Response};
use producer::UserLoginObserver::{
    Observer as UserLoginObserver, ObserverRequest as UserLoginObserverRequest,
};
use producer::UserLogoutObserver::{
    Observer as UserLogoutObserver, ObserverRequest as UserLogoutObserverRequest,
};
use producer::UsersObserver::{Observer as UsersObserver, ObserverRequest as UsersObserverRequest};
use producer::MessageObserver::{
    Observer as MessageObserver, ObserverRequest as MessageObserverRequest,
};
use producer::consumer_identification::Filter;
use producer::EventUserConnected::{
    Controller as EventUserConnectedController, Observer as EventUserConnectedObserver,
};
use producer::*;
use std::sync::{Arc, RwLock};
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};

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

impl Producer<Server, WrappedCustomContext> for ProducerInstance {}

#[allow(unused_variables)]
impl UserLoginObserver for UserLoginObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserLogin::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(
            producer::protocol::UserLogin::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserLoginObserver::Conclusion, String> {
        Ok(producer::UserLoginObserver::Conclusion::Accept(
            producer::protocol::UserLogin::Accepted {
                uuid: cx.uuid().to_string(),
            },
        ))
    }

    fn accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::UserLogin::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(
            producer::protocol::UserLogin::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        match store::users.write() {
            Ok(mut users) => {
                users.insert(cx.uuid(), store::User {
                    name: request.username,
                    uuid: cx.uuid(),
                });
            },
            Err(e) => {}
        };
        Ok(())
    }

    fn broadcast<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::UserLogin::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(
            producer::protocol::UserLogin::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        let filter = Filter {
            key: Some(producer::protocol::Identification::SelfKey {
                id: None,
                uuid: Some(cx.uuid().to_string()),
                location: None,
            }),
            assigned: None,
            condition: producer::consumer_identification::EFilterMatchCondition::NotEqual,
        };
        broadcast(
            filter,
            Broadcasting::UserConnected(producer::protocol::Events::UserConnected {
                uuid: cx.uuid().to_string(),
                username: "----".to_string(),
            }),
        )
    }
}

#[allow(unused_variables)]
impl UserLogoutObserver for UserLogoutObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserLogout::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(
            producer::protocol::UserLogout::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserLogoutObserver::Conclusion, String> {
        println!("GOOOD");
        Err(String::from("conclusion method isn't implemented"))
    }
}

#[allow(unused_variables)]
impl UsersObserver for UsersObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::Users::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(
            producer::protocol::Users::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UsersObserver::Conclusion, String> {
        match store::users.read() {
            Ok(users) => Ok(producer::UsersObserver::Conclusion::Response(
                producer::protocol::Users::Response {
                    users: users
                        .values()
                        .cloned()
                        .map(|user| producer::protocol::Users::User {
                            name: user.name,
                            uuid: user.uuid.to_string(),
                        })
                        .collect(),
                },
            )),
            Err(e) => Err(format!("{}", e))
        }
    }

    fn response<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::Users::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(
            producer::protocol::Users::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[allow(unused_variables)]
impl MessageObserver for MessageObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::Message::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(
            producer::protocol::Message::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::MessageObserver::Conclusion, String> {
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

    fn accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::Message::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(
            producer::protocol::Message::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        let start = SystemTime::now();
        let tm = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        match store::messages.write() {
            Ok(mut messages) => {
                messages.insert(cx.uuid(), store::Message {
                    name: request.user,
                    uuid: cx.uuid(),
                    message: request.message,
                    timestamp: tm.as_secs(),
                });
            },
            Err(e) => {}
        };
        Ok(())
    }

    fn broadcast<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::Message::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(
            producer::protocol::Message::Err,
        ) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        let filter = Filter {
            key: Some(producer::protocol::Identification::SelfKey {
                id: None,
                uuid: Some(cx.uuid().to_string()),
                location: None,
            }),
            assigned: None,
            condition: producer::consumer_identification::EFilterMatchCondition::NotEqual,
        };
        let start = SystemTime::now();
        let tm = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        broadcast(
            filter,
            Broadcasting::Message(producer::protocol::Events::Message {
                user: request.user,
                message: request.message,
                timestamp: tm.as_secs(),
            }),
        )
    }
}

#[allow(unused_variables)]
impl EventUserConnectedController for EventUserConnectedObserver {
    fn connected<WrappedCustomContext>(
        event: &producer::EventUserConnected::Event,
        ucx: WrappedCustomContext,
        broadcasting: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("connected handler isn't implemented"))
    }
}

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
                    producer::ProducerEvents::Connected(_ucx) => {
                        println!(">>>>>> Connected");
                    }
                    producer::ProducerEvents::ServerDown => {
                        println!(">>>>>> ServerDown");
                    }
                    producer::ProducerEvents::Disconnected => {
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
