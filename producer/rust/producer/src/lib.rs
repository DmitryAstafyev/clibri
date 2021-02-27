#[path = "./traits/observer.rs"]
pub mod observer;

#[path = "./traits/logger.rs"]
pub mod logger;

#[path = "./protocol/protocol.rs"]
pub mod protocol;

#[path = "./consumer/consumer.rs"]
pub mod consumer;

#[path = "./consumer/consumer.identification.rs"]
pub mod consumer_identification;

#[path = "./consumer/consumer.context.rs"]
pub mod consumer_context;

#[allow(non_snake_case)]
#[path = "./declarations/observer.UserSingInRequest.rs"]
pub mod UserSingInObserver;

#[allow(non_snake_case)]
#[path = "./declarations/observer.UserJoinRequest.rs"]
pub mod UserJoinObserver;

#[allow(non_snake_case)]
#[path = "./declarations/observer.event.UserConnected.rs"]
pub mod EventUserConnected;

use consumer::{Consumer};
use consumer_identification::EFilterMatchCondition;
use protocol as Protocol;
use Protocol::StructEncode;
use logger::Logger;

use fiber::server::events::ServerEvents;
use fiber::server::server::Server as ServerTrait;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use uuid::Uuid;

pub struct Context {

}

pub enum Broadcasting {
    UserDisconnected(Protocol::UserDisconnected),
}

pub enum ProducerEvents {
    InternalError(String),
    EmitError(String),
    ServerError(String),
    Reading(String),
    Connected(Arc<RwLock<Context>>),
    Disconnected,
}

pub struct DefaultLogger {

}

impl Logger for DefaultLogger {}

pub fn broadcasting(
    consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
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

#[allow(non_snake_case)]
pub struct Producer<S>
where
    S: ServerTrait,
{
    server: S,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
    events: Sender<ProducerEvents>,
    logger: &'static (dyn Logger + Send + Sync),
    UserSingIn: UserSingInObserver::ObserverRequest,
    UserJoin: UserJoinObserver::ObserverRequest,
    EventUserConnected: EventUserConnected::Observer,
}

#[allow(non_snake_case)]
impl<S> Producer<S>
where
    S: ServerTrait,
{
    pub fn new(server: S, logger: Option<&'static (dyn Logger + Send + Sync)>) -> (Self, Receiver<ProducerEvents>) {
        let (sender, receiver) = mpsc::channel();
        let logs = if let Some(logger) = logger {
            logger
        } else {
            &(DefaultLogger {})
        };
        (Producer {
            server,
            consumers: Arc::new(RwLock::new(HashMap::new())),
            events: sender,
            logger: logs,
            UserSingIn: UserSingInObserver::ObserverRequest::new(),
            UserJoin: UserJoinObserver::ObserverRequest::new(),
            EventUserConnected: EventUserConnected::Observer::new(),
        }, receiver)
    }

    #[allow(non_snake_case)]
    pub fn listen(&mut self, ucx: Context) -> Result<(), String> {
        let (tx_channel, rx_channel): (Sender<ServerEvents>, Receiver<ServerEvents>) =
            mpsc::channel();
        let (sender_tx_channel, sender_rx_channel): (Sender<(Vec<u8>, Option<Uuid>)>, Receiver<(Vec<u8>, Option<Uuid>)>) =
            mpsc::channel();
        let consumers_ref = self.consumers.clone();
        let ucx = Arc::new(RwLock::new(ucx));
        {
            use EventUserConnected::EventsController;
            self.EventUserConnected.listen(ucx.clone(), consumers_ref.clone());
        }
        let UserSingIn = Arc::new(RwLock::new(self.UserSingIn.clone()));
        let UserJoin = Arc::new(RwLock::new(self.UserJoin.clone()));
        let events = self.events.clone();
        let logger = self.logger;
        spawn(move || {
            loop {
                match rx_channel.recv() {
                    Ok(event) => {
                        let consumers_ref = consumers_ref.clone();
                        let ucx = ucx.clone();
                        let UserSingIn = UserSingIn.clone();
                        let UserJoin = UserJoin.clone();
                        let events = events.clone();
                        let sender_tx_channel_wrapped = Arc::new(Mutex::new(sender_tx_channel.clone()));
                        spawn(move || {
                            match event {
                                ServerEvents::Connected(uuid) => match consumers_ref.write() {
                                    Ok(mut storage) => {
                                        let _consumer = storage
                                            .entry(uuid)
                                            .or_insert_with(|| Consumer::new(consumers_ref.clone(), sender_tx_channel_wrapped.clone()));
                                        if let Err(e) = events.send(ProducerEvents::Connected(ucx.clone())) {
                                            logger.err(&format!("{}", e));
                                        }
                                    }
                                    Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to consumers due error: {}", e).to_owned())) {
                                        logger.err(&format!("{}", e));
                                    }
                                },
                                ServerEvents::Disconnected(uuid) => match consumers_ref.write() {
                                    Ok(mut consumers) => {
                                        consumers.remove(&uuid);
                                        if let Err(e) = events.send(ProducerEvents::Disconnected) {
                                            logger.err(&format!("{}", e));
                                        }
                                    }
                                    Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to consumers due error: {}", e).to_owned())) {
                                        logger.err(&format!("{}", e));
                                    }
                                },
                                ServerEvents::Received(uuid, buffer) => match consumers_ref.write() {
                                    Ok(mut consumers) => {
                                        if let Some(consumer) = consumers.get_mut(&uuid) {
                                            let broadcast = |filter: HashMap<String, String>, condition: EFilterMatchCondition, broadcast: Broadcasting| {
                                                broadcasting(consumers_ref.clone(), filter, condition, broadcast)
                                            };
                                            if let Err(e) = consumer.chunk(&buffer) {
                                                if let Err(e) = events.send(ProducerEvents::Reading(format!("Fail to read connection buffer due error: {}", e).to_owned())) {
                                                    logger.err(&format!("{}", e));
                                                }
                                            }
                                            while let Some(message) = consumer.next() {
                                                match message {
                                                    Protocol::AvailableMessages::UserSingIn(Protocol::UserSingIn::AvailableMessages::Request(request)) => {
                                                        match UserSingIn.write() {
                                                            Ok(UserSingIn) => {
                                                                use UserSingInObserver::Observer;
                                                                if let Err(e) = UserSingIn.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = events.send(ProducerEvents::EmitError(format!("Fail to emit UserSingInRequest due error: {:?}", e).to_owned())) {
                                                                        logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to UserSingIn due error: {}", e).to_owned())) {
                                                                logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    Protocol::AvailableMessages::UserJoin(Protocol::UserJoin::AvailableMessages::Request(request)) => {
                                                        match UserJoin.write() {
                                                            Ok(UserJoin) => {
                                                                use UserJoinObserver::Observer;
                                                                if let Err(e) = UserJoin.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = events.send(ProducerEvents::EmitError(format!("Fail to emit Protocol::UserJoin::Request due error: {:?}", e).to_owned())) {
                                                                        logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to UserJoin due error: {}", e).to_owned())) {
                                                                logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    _ => {},
                                                };
                                            }
                                        }
                                    }
                                    Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to consumers due error: {}", e).to_owned())) {
                                        logger.err(&format!("{}", e));
                                    }
                                },
                                ServerEvents::Error(uuid, e) => if let Err(e) = events.send(ProducerEvents::ServerError(format!("Connection {:?}: {}", uuid, e).to_owned())) {
                                    logger.err(&format!("{}", e));
                                }
                            }
                        });
                    },
                    Err(e) => {
                        logger.err(&format!("{}", e));
                    }
                }
            }
        });
        match self.server.listen(tx_channel, sender_rx_channel) {
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
        broadcasting(self.consumers.clone(), filter, condition, broadcast)
    }

}
