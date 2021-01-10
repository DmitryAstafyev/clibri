#[path = "./observer.request.rs"]
pub mod request_observer;

#[path = "./observer.event.rs"]
pub mod event_observer;

#[path = "./observer.broadcast.rs"]
pub mod broadcast_observer;

#[path = "./broadcast.rs"]
pub mod broadcast;

#[path = "./events.holder.rs"]
pub mod events_holder;

#[path = "./context.rs"]
pub mod context;

#[path = "./buffer.rs"]
pub mod buffer;

#[path = "./package.rs"]
pub mod package;

#[path = "./protocol.rs"]
pub mod protocol;

#[path = "./consumer.rs"]
pub mod consumer;

use request_observer::{ RequestObserver as RequestObserverTrait, Observer as RequestObserver};
use broadcast_observer::{ BroadcastObserver as BroadcastObserverTrait, Observer as BroadcastObserver };
use event_observer::{ EventObserver as EventObserverTrait, Observer as EventObserver, EventObserverErrors};
use events_holder:: { EventsHolder };
use context::*;
use consumer::{ Consumer };
use std::cmp::{ PartialEq, Eq };
use fiber_transport_server::connection_context::{ ConnectionContext };
use fiber_transport_server::server::{ Server, ServerEvents };
use std::thread;
use std::thread::spawn;
use uuid::Uuid;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock, Mutex};
use std::{time::Duration};
use std::collections::HashMap;

/*
use std::collections::{ HashMap };
use uuid::Uuid;
*/

pub enum Messages {
    UserSingInRequest(UserSingInRequest),
}

#[derive(Debug, Clone)]
pub struct Protocol {

}

impl protocol::Protocol<Messages> for Protocol {

    fn get_msg(&self, id: u32, buffer: &[u8]) -> Result<Messages, String> {
        Ok(Messages::UserSingInRequest(UserSingInRequest {
            login: String::from("login"),
            email: String::from("email"),
        }))
    }

}

pub struct Identification {
    pub uuid: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserSingInRequest {
    pub login: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserSingInBroadcast {
    login: String,
}

impl Encodable for UserSingInBroadcast {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct UserSingInResponse {
    error: Option<String>,
}

impl Encodable for UserSingInResponse {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserSingInConclusion {
    Accept,
    Deny,
}

pub struct UserSingInEvents {
    pub accept: EventObserver<UserSingInRequest, Identification, UserSingInConclusion>,
    pub broadcast: BroadcastObserver<UserSingInRequest, UserSingInBroadcast, Identification>,
    pub deny: EventObserver<UserSingInRequest, Identification, UserSingInConclusion>,
}

impl Default for UserSingInEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl UserSingInEvents {

    pub fn new() -> Self {
        UserSingInEvents {
            accept: EventObserver::new(),
            broadcast: BroadcastObserver::new(),
            deny: EventObserver::new(),
        }
    }

}

impl EventsHolder<UserSingInRequest, Identification, UserSingInConclusion> for UserSingInEvents {
    fn emit(
        &mut self,
        conclusion: UserSingInConclusion,
        cx: &mut dyn Context<Identification>,
        request: UserSingInRequest,
    ) -> Result<(), EventObserverErrors> {
        match conclusion {
            UserSingInConclusion::Accept => {
                if let Err(e) = self.accept.emit(conclusion, cx, request.clone()) {
                    return Err(e);
                }
                if let Err(e) = self.broadcast.emit(cx, request) {
                    return Err(EventObserverErrors::ErrorOnBroadcasting(e));
                }
                Ok(())
            },
            UserSingInConclusion::Deny => self.deny.emit(conclusion, cx, request),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserDisconnected {
    login: String,
}

impl Encodable for UserDisconnected {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

pub enum Broadcasting {
    UserDisconnected(UserDisconnected)
}

#[allow(non_snake_case)]
pub struct Producer {
    server: Server,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
    pub UserSingIn: Arc<RwLock<RequestObserver<UserSingInRequest, UserSingInResponse, Identification, UserSingInConclusion, UserSingInEvents>>>,
    // pub UserSingIn: RequestObserver<UserSingInRequest, UserSingInResponse, Identification, UserSingInConclusion, UserSingInEvents>,
    
}

impl Producer {

    pub fn new(server: Server) -> Self {
        Producer {
            server,
            consumers: Arc::new(RwLock::new(HashMap::new())),
            UserSingIn: Arc::new(RwLock::new(RequestObserver::new(UserSingInEvents::new()))),
        }
    }

    pub fn listen(&mut self) -> Result<(), String> {
        let (tx_channel, rx_channel): (
            Sender<ServerEvents>,
            Receiver<ServerEvents>,
        ) = mpsc::channel();
        let consumers = self.consumers.clone();
        let UserSingIn = self.UserSingIn.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match rx_channel.try_recv() {
                    Ok(event) => match event {
                        ServerEvents::Connected(uuid, cx) => match consumers.write() {
                            Ok(mut consumers) => {
                                let consumer = consumers.entry(uuid).or_insert_with(Consumer::new);
                            },
                            Err(e) => {},
                        },
                        ServerEvents::Disconnected(uuid, cx) => match consumers.write() {
                            Ok(mut consumers) => {
                                consumers.remove(&uuid);
                            },
                            Err(e) => {},
                        },
                        ServerEvents::Received(uuid, cx, buffer) => match consumers.write() {
                            Ok(mut consumers) => {
                                if let Some(consumer) = consumers.get_mut(&uuid) {
                                    match consumer.read(buffer) {
                                        Ok(message) => match message {
                                            Messages::UserSingInRequest(request) => {
                                                match UserSingIn.write() {
                                                    Ok(mut UserSingIn) => {
                                                        let mut cx = consumer.get_cx();
                                                        UserSingIn.emit(&mut cx, request);
                                                        println!("");        
                                                    },
                                                    Err(e) => {},
                                                }
                                            }
                                        },
                                        Err(e) => {},
                                    }
                                }
                            },
                            Err(e) => {},
                        },
                        ServerEvents::Error(uuid, e) => {

                        },
                    },
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                }
            }
        });
        match self.server.listen(tx_channel) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn broadcast(&mut self, ident: Identification, broadcast: Broadcasting) -> Result<(), String> {
        Ok(())
    }

}

fn test() {
    let server: Server = Server::new(String::from("127.0.0.1:8080"));
    let producer: Producer = Producer::new(server);
    // (Fn(Request, &mut dyn Context<Identification>) -> Result<(Response, Conclusion), String>)
    match producer.UserSingIn.write() {
        Ok(mut UserSingIn) => {
            UserSingIn.subscribe(&on_UserSingInRequest);
        },
        Err(e) => {},
    };
}



#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        
        assert_eq!(true, false);
    }
}
