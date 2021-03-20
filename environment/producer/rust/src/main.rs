#[path = "../producer/src/lib.rs"]
pub mod producer;

use fiber_transport_server::server::{ Server };
use fiber_transport_server::{ ErrorResponse, Request, Response };
use producer::UserJoinObserver::{
    Observer as UserJoinObserver, ObserverRequest as UserJoinObserverRequest,
};
use producer::UserSignInObserver::{
    Observer as UserSignInObserver, ObserverRequest as UserSignInObserverRequest,
};
use producer::EventUserConnected::{
    Controller as EventUserConnectedController, Observer as EventUserConnectedObserver,
};
use producer::*;
use producer::consumer_identification::EFilterMatchCondition;
use std::sync::{Arc, RwLock};
// use std::thread::spawn;

#[derive(Clone)]
struct CustomContext {}

impl CustomContext {}

type WrappedCustomContext = Arc<RwLock<CustomContext>>;

struct ProducerInstance {}

impl Producer<Server, WrappedCustomContext> for ProducerInstance {}

#[allow(unused_variables)]
impl UserJoinObserver for UserJoinObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserJoin::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(producer::protocol::UserJoin::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserJoinObserver::Conclusion, String> {
        println!("GOOOD");
        Err(String::from("conclusion method isn't implemented"))
    }
}

#[allow(unused_variables)]
impl UserSignInObserver for UserSignInObserverRequest {
    fn conclusion<WrappedCustomContext>(
        request: producer::protocol::UserSignIn::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: WrappedCustomContext,
        error: &dyn Fn(producer::protocol::UserSignIn::Err) -> Result<(), producer::observer::RequestObserverErrors>,
    ) -> Result<producer::UserSignInObserver::Conclusion, String> {
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
            producer::protocol::Identification::Key,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("connected handler isn't implemented"))
    }
}

fn main() {
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
        let mut producer: ProducerInstance = ProducerInstance {};
        let _feedback = match producer.listen(server, Arc::new(RwLock::new(ucx))) {
            Ok(feedback) => loop {
                match feedback.events.recv() {
                    Ok(m) => {
                        match m {
                            producer::ProducerEvents::Connected(_ucx) => {
                                println!("Connected");
                            },
                            producer::ProducerEvents::ServerDown => {
                                println!("ServerDown");
                            },
                            producer::ProducerEvents::Disconnected => {
                                println!("Disconnected");
                            },
                            producer::ProducerEvents::InternalError(e) => {
                                println!("InternalError: {}", e);
                            },
                            producer::ProducerEvents::EmitError(e) => {
                                println!("EmitError: {}", e);
                            },
                            producer::ProducerEvents::EventError(e) => {
                                println!("EventError: {}", e);
                            },
                            producer::ProducerEvents::EventChannelError(e) => {
                                println!("EventChannelError: {}", e);
                            },
                            producer::ProducerEvents::ConnectionError(e) => {
                                println!("ConnectionError: {}", e);
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
    
