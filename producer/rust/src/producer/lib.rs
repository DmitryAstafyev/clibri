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
pub mod DeclUserSingInRequest;

#[allow(non_snake_case)]
#[path = "./declarations/observer.UserJoinRequest.rs"]
pub mod DeclUserJoinRequest;

#[allow(non_snake_case)]
#[allow(non_snake_case)]
#[path = "./declarations/observer.event.UserConnected.rs"]
pub mod DeclEventUserConnected;

#[allow(non_snake_case)]
#[path = "./implementations/observer.UserSingInRequest.rs"]
pub mod ImplUserSingInRequest;

#[allow(non_snake_case)]
#[path = "./implementations/observer.UserJoinRequest.rs"]
pub mod ImplUserJoinRequest;

#[allow(non_snake_case)]
#[path = "./implementations/observer.event.UserConnected.rs"]
pub mod ImplEventUserConnected;

use consumer::{Consumer};
use consumer_identification::EFilterMatchCondition;
use protocol as Protocol;
use Protocol::{ StructEncode };
use DeclUserJoinRequest::{ UserJoinObserver };
use DeclUserSingInRequest::UserSingInObserver;
use DeclEventUserConnected::EventUserConnected;
use logger::{ Logger };

use fiber::server::context::ConnectionContext;
use fiber::server::events::ServerEvents;
use fiber::server::server::Server as ServerTrait;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use uuid::Uuid;

pub enum Broadcasting {
    UserDisconnected(Protocol::UserDisconnected),
}

pub enum ProducerEvents<UCX: Send + Sync> {
    InternalError(String),
    EmitError(String),
    ServerError(String),
    Reading(String),
    Connected(Arc<RwLock<UCX>>),
    Disconnected,
}

pub struct DefaultLogger {

}

impl Logger for DefaultLogger {}

pub fn broadcasting<CX: ConnectionContext + Send + Sync,>(
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

#[allow(non_snake_case)]
pub struct Producer<S, CX, UCX>
where
    S: ServerTrait<CX>,
    CX: ConnectionContext + Send + Sync,
    UCX: 'static + Send + Sync
{
    server: S,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>,
    events: Sender<ProducerEvents<UCX>>,
    logger: &'static (dyn Logger + Send + Sync),
    pub UserSingIn: Option<ImplUserSingInRequest::ObserverRequest<UCX>>,
    pub UserJoin: Option<ImplUserJoinRequest::ObserverRequest<UCX>>,
    pub EventUserConnected: ImplEventUserConnected::EventObserver<UCX>,
}

impl<S, CX: 'static, UCX: 'static> Producer<S, CX, UCX>
where
    S: ServerTrait<CX>,
    CX: ConnectionContext + Send + Sync,
    UCX: Send + Sync,
{
    pub fn new(server: S, logger: Option<&'static (dyn Logger + Send + Sync)>) -> (Self, Receiver<ProducerEvents<UCX>>) {
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
            UserSingIn: Some(ImplUserSingInRequest::ObserverRequest::new()),
            UserJoin: Some(ImplUserJoinRequest::ObserverRequest::new()),
            EventUserConnected: ImplEventUserConnected::EventObserver::new(),
        }, receiver)
    }

    #[allow(non_snake_case)]
    pub fn listen(&mut self, ucx: UCX) -> Result<(), String> {
        let (tx_channel, rx_channel): (Sender<ServerEvents<CX>>, Receiver<ServerEvents<CX>>) =
            mpsc::channel();
        let consumers_ref = self.consumers.clone();
        let ucx = Arc::new(RwLock::new(ucx));
        self.EventUserConnected.listen(ucx.clone(), consumers_ref.clone());
        let UserSingIn = Arc::new(RwLock::new(if let Some(v) = self.UserSingIn.take() { v } else {
            return Err(String::from("Cannot get instance of UserSingIn"))
        }));
        let UserJoin = Arc::new(RwLock::new(if let Some(v) = self.UserJoin.take() { v } else {
            return Err(String::from("Cannot get instance of UserJoin"))
        }));
        let events = self.events.clone();
        let logger = self.logger;
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                // TODO: here we can use recv as well instread try_recv
                match rx_channel.recv() {
                    Ok(event) => {
                        let consumers_ref = consumers_ref.clone();
                        let ucx = ucx.clone();
                        let UserSingIn = UserSingIn.clone();
                        let UserJoin = UserJoin.clone();
                        let events = events.clone();
                        spawn(move || {
                            match event {
                                ServerEvents::Connected(uuid, cx) => match consumers_ref.write() {
                                    Ok(mut storage) => {
                                        let _consumer = storage
                                            .entry(uuid)
                                            .or_insert(Consumer::new(cx, consumers_ref.clone()));
                                        if let Err(e) = events.send(ProducerEvents::Connected(ucx.clone())) {
                                            logger.err(&format!("{}", e));
                                        }
                                    }
                                    Err(e) => if let Err(e) = events.send(ProducerEvents::InternalError(format!("Fail to access to consumers due error: {}", e).to_owned())) {
                                        logger.err(&format!("{}", e));
                                    }
                                },
                                ServerEvents::Disconnected(uuid, _cx) => match consumers_ref.write() {
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
                                ServerEvents::Received(uuid, _cx, buffer) => match consumers_ref.write() {
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
                                            loop {
                                                if let Some(message) = consumer.next() {
                                                    match message {
                                                        Protocol::AvailableMessages::UserSingIn(Protocol::UserSingIn::AvailableMessages::Request(request)) => {
                                                            match UserSingIn.write() {
                                                                Ok(UserSingIn) => {
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
                                                } else {
                                                    break;
                                                }
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
        broadcasting(self.consumers.clone(), filter, condition, broadcast)
    }

}
