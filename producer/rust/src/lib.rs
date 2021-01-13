#[path = "./traits/observer.rs"]
pub mod observer;

#[path = "./buffer.rs"]
pub mod buffer;

#[path = "./package.rs"]
pub mod package;

#[path = "./protocol.rs"]
pub mod protocol;

#[path = "./consumer/consumer.rs"]
pub mod consumer;

#[path = "./consumer/consumer.identification.rs"]
pub mod consumer_identification;

#[path = "./consumer/consumer.context.rs"]
pub mod consumer_context;

#[path = "./declarations/observer.UserSingInRequest.rs"]
pub mod DeclUserSingInRequest;

#[path = "./declarations/observer.UserJoinRequest.rs"]
pub mod DeclUserJoinRequest;

#[path = "./implementations/observer.UserSingInRequest.rs"]
pub mod ImplUserSingInRequest;

#[path = "./implementations/observer.UserJoinRequest.rs"]
pub mod ImplUserJoinRequest;

use consumer::Consumer;
use consumer_context::*;
use consumer_identification::EFilterMatchCondition;
use DeclUserJoinRequest::UserJoinObserver;
use DeclUserSingInRequest::UserSingInObserver;
use ImplUserJoinRequest::UserJoinRequest;
use ImplUserSingInRequest::UserSingInRequest;

use fiber::server::context::ConnectionContext;
use fiber::server::events::ServerEvents;
use fiber::server::server::Server as ServerTrait;
use fiber_transport_server::connection_context::ConnectionContext as ServerConnectionContext;
use fiber_transport_server::server::Server;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use uuid::Uuid;
/*
use std::collections::{ HashMap };
use uuid::Uuid;
*/

pub enum Messages {
    UserSingInRequest(UserSingInRequest),
    UserJoinRequest(UserJoinRequest),
}

#[derive(Debug, Clone)]
pub struct Protocol {}

impl protocol::Protocol<Messages> for Protocol {
    fn get_msg(&self, id: u32, buffer: &[u8]) -> Result<Messages, String> {
        Ok(Messages::UserSingInRequest(UserSingInRequest {
            login: String::from("login"),
            email: String::from("email"),
        }))
    }
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
pub struct UserDisconnected {
    login: String,
}

impl Encodable for UserDisconnected {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

pub enum Broadcasting {
    UserDisconnected(UserDisconnected),
}

#[allow(non_snake_case)]
pub struct Producer<S, CX>
where
    S: ServerTrait<CX>,
    CX: ConnectionContext + Send + Sync,
{
    server: S,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>,
    pub UserSingIn: Arc<RwLock<ImplUserSingInRequest::ObserverRequest>>,
    pub UserJoin: Arc<RwLock<ImplUserJoinRequest::ObserverRequest>>,
}

impl<S, CX: 'static> Producer<S, CX>
where
    S: ServerTrait<CX>,
    CX: ConnectionContext + Send + Sync,
{
    pub fn new(server: S) -> Self {
        Producer {
            server,
            consumers: Arc::new(RwLock::new(HashMap::new())),
            UserSingIn: Arc::new(RwLock::new(ImplUserSingInRequest::ObserverRequest::new())),
            UserJoin: Arc::new(RwLock::new(ImplUserJoinRequest::ObserverRequest::new())),
        }
    }

    #[allow(non_snake_case)]
    pub fn listen(&mut self) -> Result<(), String> {
        let (tx_channel, rx_channel): (Sender<ServerEvents<CX>>, Receiver<ServerEvents<CX>>) =
            mpsc::channel();
        let consumers_ref = self.consumers.clone();
        let UserSingIn = self.UserSingIn.clone();
        let UserJoin = self.UserJoin.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match rx_channel.try_recv() {
                    Ok(event) => match event {
                        ServerEvents::Connected(uuid, cx) => match consumers_ref.write() {
                            Ok(mut storage) => {
                                let consumer = storage
                                    .entry(uuid)
                                    .or_insert(Consumer::new(cx, consumers_ref.clone()));
                            }
                            Err(e) => {}
                        },
                        ServerEvents::Disconnected(uuid, cx) => match consumers_ref.write() {
                            Ok(mut consumers) => {
                                consumers.remove(&uuid);
                            }
                            Err(e) => {}
                        },
                        ServerEvents::Received(uuid, cx, buffer) => match consumers_ref.write() {
                            Ok(mut consumers) => {
                                if let Some(consumer) = consumers.get_mut(&uuid) {
                                    let broadcast = |filter: HashMap<String, String>, condition: EFilterMatchCondition, broadcast: Broadcasting| {
                                        Self::Broadcast(consumers_ref.clone(), filter, condition, broadcast)
                                    };
                                    match consumer.read(buffer) {
                                        Ok(message) => match message {
                                            Messages::UserSingInRequest(request) => {
                                                match UserSingIn.write() {
                                                    Ok(mut UserSingIn) => {
                                                        if let Err(e) = UserSingIn.emit(
                                                            consumer.get_cx(),
                                                            request,
                                                            &broadcast,
                                                        ) {
                                                            // TODO: error channel
                                                            println!("{:?}", e);
                                                        }
                                                    }
                                                    Err(e) => {}
                                                }
                                            }
                                            Messages::UserJoinRequest(request) => {
                                                match UserJoin.write() {
                                                    Ok(mut UserJoin) => {
                                                        if let Err(e) = UserJoin.emit(
                                                            consumer.get_cx(),
                                                            request,
                                                            &broadcast,
                                                        ) {
                                                            // TODO: error channel
                                                            println!("{:?}", e);
                                                        }
                                                    }
                                                    Err(e) => {}
                                                }
                                            }
                                        },
                                        Err(e) => {}
                                    }
                                }
                            }
                            Err(e) => {}
                        },
                        ServerEvents::Error(uuid, e) => {}
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

    pub fn broadcast(
        &mut self,
        filter: HashMap<String, String>,
        condition: EFilterMatchCondition,
        broadcast: Broadcasting,
    ) -> Result<(), String> {
        match self.consumers.write() {
            Ok(consumers) => match broadcast {
                Broadcasting::UserDisconnected(mut msg) => match msg.abduct() {
                    Ok(buffer) => {
                        let mut errors: Vec<String> = vec![];
                        for (uuid, consumer) in consumers.iter() {
                            if let Err(e) =
                                consumer.send_if(buffer.clone(), filter.clone(), condition.clone())
                            {
                                errors.push(format!(
                                    "Fail to send data to {}, due error: {}",
                                    uuid, e
                                ));
                            }
                        }
                        if errors.is_empty() {
                            Ok(())
                        } else {
                            Err(errors.join("\n"))
                        }
                    }
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn Broadcast(
        consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>,
        filter: HashMap<String, String>,
        condition: EFilterMatchCondition,
        broadcast: Broadcasting,
    ) -> Result<(), String> {
        match consumers.write() {
            Ok(consumers) => match broadcast {
                Broadcasting::UserDisconnected(mut msg) => match msg.abduct() {
                    Ok(buffer) => {
                        let mut errors: Vec<String> = vec![];
                        for (uuid, consumer) in consumers.iter() {
                            if let Err(e) =
                                consumer.send_if(buffer.clone(), filter.clone(), condition.clone())
                            {
                                errors.push(format!(
                                    "Fail to send data to {}, due error: {}",
                                    uuid, e
                                ));
                            }
                        }
                        if errors.is_empty() {
                            Ok(())
                        } else {
                            Err(errors.join("\n"))
                        }
                    }
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(format!("{}", e)),
        }
    }
}

fn test() {
    let server: Server = Server::new(String::from("127.0.0.1:8080"));
    let mut producer: Producer<Server, ServerConnectionContext> = Producer::new(server);
    producer.listen();
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(true, false);
    }
}
