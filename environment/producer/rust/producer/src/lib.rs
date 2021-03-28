#[path = "./traits/observer.rs"]
pub mod observer;

#[path = "./protocol/protocol.rs"]
pub mod protocol;

#[path = "./consumer/consumer.rs"]
pub mod consumer;

#[path = "./consumer/consumer.identification.rs"]
pub mod consumer_identification;

#[path = "./consumer/consumer.context.rs"]
pub mod consumer_context;

#[allow(non_snake_case)]
#[path = "./declarations/observer.UserLoginRequest.rs"]
pub mod UserLoginObserver;

#[allow(non_snake_case)]
#[path = "./declarations/observer.UserLogoutRequest.rs"]
pub mod UserLogoutObserver;

#[allow(non_snake_case)]
#[path = "./declarations/observer.event.UserConnected.rs"]
pub mod EventUserConnected;

use super::{ tools };
use consumer::Consumer;
use consumer_identification::Filter;
use protocol as Protocol;
use Protocol::{StructEncode, PackingStruct};

use fiber::server::events::ServerEvents;
use fiber::server::server::Server as ServerTrait;
use fiber::logger::{ Logger };
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
use uuid::Uuid;

pub enum Broadcasting {
    UserDisconnected(Protocol::Events::UserDisconnected),
}

pub enum ProducerEvents<UCX: 'static + Sync + Send + Clone> {
    InternalError(String),
    EmitError(String),
    EventError(String),
    EventChannelError(String),
    EventListenError(String),
    ConnectionError(String),
    NotAssignedConsumer(String),
    ServerError(String),
    ServerDown,
    Reading(String),
    Connected(UCX),
    Disconnected,
}

pub fn broadcasting(
    consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
    filter: Filter,
    broadcast: Broadcasting,
) -> Result<(), String> {
    match consumers.write() {
        Ok(consumers) => match broadcast {
            Broadcasting::UserDisconnected(mut msg) => match msg.abduct() {
                Ok(buffer) => {
                    let mut errors: Vec<String> = vec![];
                    for (uuid, consumer) in consumers.iter() {
                        if let Err(e) =
                            consumer.send_if(buffer.clone(), filter.clone())
                        {
                            errors.push(format!("Fail to send data to {}, due error: {}", uuid, e));
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

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Channel<UCX> where UCX: 'static + Sync + Send + Clone {
    pub events: Receiver<ProducerEvents<UCX>>,
    pub EventUserConnectedSender: Sender<EventUserConnected::Event>,
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
pub trait Producer<S, UCX>
where
    S: 'static + ServerTrait,
    UCX: 'static + Sync + Send + Clone,
{

    fn listen(
        &self,
        mut server: S,
        ucx: UCX,
    ) -> Result<Channel<UCX>, String> {
        let (tx_channel, rx_channel): (Sender<ServerEvents>, Receiver<ServerEvents>) =
            mpsc::channel();
        let (sender_tx_channel, sender_rx_channel): (
            Sender<(Vec<u8>, Option<Uuid>)>,
            Receiver<(Vec<u8>, Option<Uuid>)>,
        ) = mpsc::channel();
        let consumers_wp = Arc::new(RwLock::new(HashMap::new()));
        let (tx_feedback, rx_feedback): (
            Sender<ProducerEvents<UCX>>,
            Receiver<ProducerEvents<UCX>>,
        ) = mpsc::channel();
        let feedback = tx_feedback.clone();
        use EventUserConnected::Controller;
        let EventUserConnectedSender = match EventUserConnected::Observer::new().listen(
            ucx.clone(),
            consumers_wp.clone(),
            feedback.clone(),
        ) {
            Ok(sender) => sender,
            Err(e) => {
                if let Err(e) =
                    feedback.send(ProducerEvents::EventListenError(e.to_string()))
                {
                    tools::logger.err(&format!(
                        "Cannot start listen event EventUserConnected due {}",
                        e
                    ));
                }
                return Err(format!(
                    "Cannot start listen event EventUserConnected due {}",
                    e
                ));
            }
        };
        spawn(move || {
            let UserLogin = Arc::new(RwLock::new(UserLoginObserver::ObserverRequest::new()));
            let UserLogout = Arc::new(RwLock::new(UserLogoutObserver::ObserverRequest::new()));
            loop {
                match rx_channel.recv() {
                    Ok(event) => {
                        let consumers_wp = consumers_wp.clone();
                        let ucx = ucx.clone();
                        let UserLogin = UserLogin.clone();
                        let UserLogout = UserLogout.clone();
                        let feedback = feedback.clone();
                        let sender_tx_channel_wrapped =
                            Arc::new(Mutex::new(sender_tx_channel.clone()));
                        spawn(move || match event {
                            ServerEvents::Connected(uuid) => match consumers_wp.write() {
                                Ok(mut consumers) => {
                                    let _consumer = consumers.entry(uuid).or_insert_with(|| {
                                        tools::logger.debug(&format!("New Consumer would be added; uuid: {}", uuid));
                                        Consumer::new(
                                            uuid,
                                            consumers_wp.clone(),
                                            sender_tx_channel_wrapped.clone(),
                                        )
                                    });
                                    if let Err(e) =
                                        feedback.send(ProducerEvents::Connected(ucx.clone()))
                                    {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                                Err(e) => {
                                    if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                        format!("Fail to access to consumers due error: {}", e),
                                    )) {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                            },
                            ServerEvents::Disconnected(uuid) => match consumers_wp.write() {
                                Ok(mut consumers) => {
                                    consumers.remove(&uuid);
                                    if let Err(e) = feedback.send(ProducerEvents::Disconnected) {
                                        tools::logger.err(&format!("{}", e));
                                    } else {
                                        tools::logger.debug(&format!("Consumer uuid: {} disconnected and destroyed", uuid));
                                    }
                                }
                                Err(e) => {
                                    if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                        format!("Fail to access to consumers due error: {}", e)
                                            .to_owned(),
                                    )) {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                            },
                            ServerEvents::Received(uuid, buffer) => match consumers_wp.write() {
                                Ok(mut consumers) => {
                                    tools::logger.debug(&format!("New message has been received; uuid: {}; length: {}", uuid, buffer.len()));
                                    if let Some(consumer) = consumers.get_mut(&uuid) {
                                        let broadcast = |filter: Filter, broadcast: Broadcasting| {
                                                broadcasting(consumers_wp.clone(), filter, broadcast)
                                            };
                                        if let Err(e) = consumer.chunk(&buffer) {
                                            if let Err(e) = feedback.send(ProducerEvents::Reading(
                                                format!(
                                                    "Fail to read connection buffer due error: {}",
                                                    e
                                                )
                                                .to_owned(),
                                            )) {
                                                tools::logger.err(&format!("{}", e));
                                            }
                                        }
                                        while let Some((message, header)) = consumer.next() {
                                            match message {
                                                    Protocol::AvailableMessages::Identification(message) => if let Protocol::Identification::AvailableMessages::SelfKey(request) = message {
                                                        let uuid = consumer.set_key(request);
                                                        tools::logger.debug(&format!("{}:: identification is done", uuid));
                                                        if let Err(e) = match (Protocol::Identification::SelfKeyResponse { uuid }).pack(header.sequence) {
                                                            Ok(buffer) => if let Err(e) = consumer.send(buffer) {
                                                                Err(e)
                                                            } else {
                                                                Ok(())
                                                            },
                                                            Err(e) => Err(e),
                                                        } {
                                                            if let Err(e) = feedback.send(ProducerEvents::ConnectionError(format!("Fail to response for Identification due error: {:?}", e).to_owned())) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    message => if !consumer.assigned() {
                                                        if let Err(e) = feedback.send(ProducerEvents::NotAssignedConsumer(format!("Consumer ({}) didn't apply Identification", consumer.get_uuid()).to_owned())) {
                                                            tools::logger.err(&format!("{}", e));
                                                        }
                                                        // TODO: Consumer should be disconnected or some feedback should be to consumer
                                                        // it might be some option of producer like NonAssignedStratagy
                                                    } else {
                                                        match message {
                                                            Protocol::AvailableMessages::UserLogin(Protocol::UserLogin::AvailableMessages::Request(request)) => {
                                                                match UserLogin.write() {
                                                                    Ok(UserLogin) => {
                                                                        use UserLoginObserver::Observer;
                                                                        if let Err(e) = UserLogin.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            request,
                                                                            &broadcast,
                                                                        ) {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit UserLogin due error: {:?}", e).to_owned())) {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to UserLogin due error: {}", e).to_owned())) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            Protocol::AvailableMessages::UserLogout(Protocol::UserLogout::AvailableMessages::Request(request)) => {
                                                                match UserLogout.write() {
                                                                    Ok(UserLogout) => {
                                                                        use UserLogoutObserver::Observer;
                                                                        if let Err(e) = UserLogout.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            request,
                                                                            &broadcast,
                                                                        ) {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit Protocol::UserLogout::Request due error: {:?}", e).to_owned())) {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to UserLogout due error: {}", e).to_owned())) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            _ => {},
                                                        }
                                                    },
                                                };
                                        }
                                    } else {
                                        tools::logger.err(&format!("Fail to find consumer uuid: {}", uuid));
                                    }
                                }
                                Err(e) => {
                                    if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                        format!("Fail to access to consumers due error: {}", e)
                                            .to_owned(),
                                    )) {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                            },
                            ServerEvents::Error(uuid, e) => {
                                if let Err(e) = feedback.send(ProducerEvents::ConnectionError(
                                    format!("Connection {:?}: {}", uuid, e).to_owned(),
                                )) {
                                    tools::logger.err(&format!("{}", e));
                                }
                            }
                        });
                    }
                    Err(e) => {
                        tools::logger.err(&format!("{}", e));
                    }
                }
            }
        });
        let feedback = tx_feedback.clone();
        spawn(move || {
            match server.listen(tx_channel, sender_rx_channel) {
                Ok(()) => {
                    if let Err(e) = feedback.send(ProducerEvents::ServerDown) {
                        tools::logger.warn(&format!("{}", e));
                    }
                }
                Err(e) => {
                    if let Err(e) = feedback.send(ProducerEvents::ServerError(e)) {
                        tools::logger.err(&format!("{}", e));
                    }
                }
            };
        });
        Ok(Channel {
            events: rx_feedback,
            EventUserConnectedSender: EventUserConnectedSender,
        })
    }
}
