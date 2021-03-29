#[macro_use]
extern crate lazy_static;

#[path = "../producer/src/lib.rs"]
pub mod producer;

use fiber::logger::{ LogLevel };
use fiber_transport_server::server::{ Server };
use fiber_transport_server::{ ErrorResponse, Request, Response };
use producer::UserLoginObserver::{
    Observer as UserLoginObserver, ObserverRequest as UserLoginObserverRequest,
};
use producer::UserLogoutObserver::{
    Observer as UserLogoutObserver, ObserverRequest as UserLogoutObserverRequest,
};
use producer::EventUserConnected::{
    Controller as EventUserConnectedController, Observer as EventUserConnectedObserver,
};
use producer::*;
use producer::consumer_identification::Filter;
use std::sync::{Arc, RwLock};
// use std::thread::spawn;

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::{ DefaultLogger };

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Producer".to_owned(), None);
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
        error: &dyn Fn(producer::protocol::UserLogin::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserLoginObserver::Conclusion, String> {
        Ok(producer::UserLoginObserver::Conclusion::Accept(
            producer::protocol::UserLogin::Accepted { uuid: cx.uuid().to_string() }
        ))
    }

    fn accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::UserLogin::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(producer::protocol::UserLogin::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn broadcast<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn producer::consumer_context::Context,
        ucx: UCX,
        request: producer::protocol::UserLogin::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(producer::protocol::UserLogin::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[allow(unused_variables)]
impl UserLogoutObserver for UserLogoutObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserLogout::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(producer::protocol::UserLogout::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserLogoutObserver::Conclusion, String> {
        println!("GOOOD");
        Err(String::from("conclusion method isn't implemented"))
    }
}

#[allow(unused_variables)]
impl EventUserConnectedController for EventUserConnectedObserver {
    fn connected<WrappedCustomContext>(
        event: &producer::EventUserConnected::Event,
        ucx: WrappedCustomContext,
        broadcasting: &dyn Fn(
            Filter,
            Broadcasting,
        ) -> Result<(), String>,
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
        if let Err(e) = server.handshake(|_: &Request, res: Response| -> Result<Response, ErrorResponse> {
            Ok(res)
        }) {
            println!("Fail to assign handshake hadler due error: {}", e);
        }
        let ucx = CustomContext {};
        let producer: ProducerInstance = ProducerInstance {};
        let _feedback = match producer.listen(server, Arc::new(RwLock::new(ucx))) {
            Ok(feedback) => loop {
                match feedback.events.recv() {
                    Ok(m) => {
                        match m {
                            producer::ProducerEvents::Connected(_ucx) => {
                                println!(">>>>>> Connected");
                            },
                            producer::ProducerEvents::ServerDown => {
                                println!(">>>>>> ServerDown");
                            },
                            producer::ProducerEvents::Disconnected => {
                                println!(">>>>>> Disconnected");
                            },
                            producer::ProducerEvents::InternalError(e) => {
                                println!(">>>>>> InternalError: {}", e);
                            },
                            producer::ProducerEvents::EmitError(e) => {
                                println!(">>>>>> EmitError: {}", e);
                            },
                            producer::ProducerEvents::EventError(e) => {
                                println!(">>>>>> EventError: {}", e);
                            },
                            producer::ProducerEvents::EventChannelError(e) => {
                                println!(">>>>>> EventChannelError: {}", e);
                            },
                            producer::ProducerEvents::ConnectionError(e) => {
                                println!(">>>>>> ConnectionError: {}", e);
                            },
                            producer::ProducerEvents::ServerError(e) => {
                                println!("ServerError: {}", e);
                            },
                            producer::ProducerEvents::Reading(e) => {
                                println!("Reading: {}", e);
                            },
                            producer::ProducerEvents::EventListenError(e) => {
                                println!("EventListenError: {}", e);
                            },
                            producer::ProducerEvents::NotAssignedConsumer(e) => {
                                println!("NotAssignedConsumer: {}", e);
                            }
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
    
