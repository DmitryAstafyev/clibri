#[path = "./traits/observer.rs"]
pub mod observer;

#[path = "./protocol/protocol.rs"]
pub mod protocol;

#[path = "./consumer/consumer.rs"]
pub mod consumer;

#[path = "./consumer/consumer.identification.rs"]
pub mod consumer_identification;

#[allow(non_snake_case)]
#[path = "./observers/observer.UserLogin.rs"]
pub mod UserLoginObserver;

#[allow(non_snake_case)]
#[path = "./observers/observer.Users.rs"]
pub mod UsersObserver;

#[allow(non_snake_case)]
#[path = "./observers/observer.Message.rs"]
pub mod MessageObserver;

#[allow(non_snake_case)]
#[path = "./observers/observer.Messages.rs"]
pub mod MessagesObserver;
/*
#[allow(non_snake_case)]
#[path = "./observers/observer.event.Disconnected.rs"]
pub mod EventDisconnected;
*/
use super::{ tools };
use consumer::Consumer;
use consumer_identification::Filter;
use protocol as Protocol;
use Protocol::{PackingStruct};
use fiber::{
    server::{
        interface::Interface,
        events::Events,
    }
};
use async_channel::{
    self,
    Sender,
    Receiver,
};
use futures::{
    executor,
};
use fiber::logger::{ Logger };
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
use uuid::Uuid;

pub enum ProducerEvents<UCX: 'static + Sync + Send + Clone> {
    InternalError(String),
    EmitError(String),
    EventError(String),
    EventChannelError(String),
    EventListenError(String),
    ConnectionError(String),
    BroadcastingError(String),
    NotAssignedConsumer(String),
    ServerError(String),
    ServerDown,
    Reading(String),
    Connected((Uuid, UCX)),
    Disconnected(Uuid),
}

#[derive(Clone)]
pub enum ConsumersChannel {
    Add(Uuid),
    Remove(Uuid),
    SendByFilter((Filter, Vec<u8>)),
    SendTo((Uuid, Vec<u8>)),
    Assign((Uuid, Protocol::Identification::AssignedKey, bool)),
    Chunk((Uuid, Vec<u8>)),
}

pub async fn broadcasting(
    consumers: Sender<ConsumersChannel>,
    filter: Filter,
    buffer: Vec<u8>,
) -> Result<(), String> {
    if let Err(e) = consumers.send(ConsumersChannel::SendByFilter((filter, buffer))).await {
        Err(tools::logger.err(&format!("Fail to get access consumers channel due error: {}", e)))
    } else {
        Ok(())
    }
}

#[allow(non_snake_case)]
pub struct EventDisconnectedBroadcasting {
    pub UserDisconnected: (Filter, Protocol::Events::UserDisconnected),
    pub Message: Option<(Filter, Protocol::Events::Message)>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Channel<UCX> where UCX: 'static + Sync + Send + Clone {
    pub events: Receiver<ProducerEvents<UCX>>,
    consumers: Arc<Mutex<Sender<ConsumersChannel>>>,
}

impl<UCX: 'static + Sync + Send + Clone> Channel<UCX> {

    pub fn send(&self, filter: Filter, buffer: Vec<u8>) -> Result<(), String> {
        Err(String::from(""))
        /*
        match self.consumers.lock() {
            Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::SendByFilter((filter, buffer))) {
                Err(e.to_string())
            } else {
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
        */
    }

    pub fn assign(&self, uuid: Uuid, assigned: Protocol::Identification::AssignedKey, overwrite: bool) -> Result<(), String> {
        Err(String::from(""))
        /*
        match self.consumers.lock() {
            Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Assign((uuid, assigned, overwrite))) {
                Err(e.to_string())
            } else {
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
        */
    }

    pub fn drop(&self, uuid: Uuid) -> Result<(), String> {
        // TODO: drop connection of consumer
        Ok(())
    }
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
pub trait Producer<UCX>
where
    UCX: 'static + Sync + Send + Clone,
{

    fn listen<S: 'static + Interface + Sync + Send>(
        &self,
        mut server: S,
        // mut server: &'static mut (impl Interface + Sync + Send),
        ucx: UCX,
    ) -> Result<Channel<UCX>, String> {
        let (tx_channel, rx_channel): (Sender<Events>, Receiver<Events>) =
            async_channel::unbounded();
        let (sender_tx_channel, sender_rx_channel): (
            Sender<(Vec<u8>, Option<Uuid>)>,
            Receiver<(Vec<u8>, Option<Uuid>)>,
        ) = async_channel::unbounded();
        let (tx_feedback, rx_feedback): (
            Sender<ProducerEvents<UCX>>,
            Receiver<ProducerEvents<UCX>>,
        ) = async_channel::unbounded();
        let (tx_consumers, rx_consumers): (
            Sender<ConsumersChannel>,
            Receiver<ConsumersChannel>,
        ) = async_channel::unbounded();
        let consumers = Arc::new(Mutex::new(tx_consumers.clone()));
        let channel_consumers = Arc::new(Mutex::new(tx_consumers.clone()));
        let feedback = tx_feedback.clone();
        /*
        use EventDisconnected::{ Controller as EventDisconnectedController };
        let EventDisconnectedSender = match EventDisconnected::Observer::new().listen(
            ucx.clone(),
            consumers.clone(),
            feedback.clone(),
        ) {
            Ok(sender) => sender,
            Err(e) => {
                if let Err(e) =
                    feedback.send(ProducerEvents::EventListenError(e.to_string()))
                {
                    tools::logger.err(&format!(
                        "Cannot start listen event EventDisconnected due {}",
                        e
                    ));
                }
                return Err(format!(
                    "Cannot start listen event EventDisconnected due {}",
                    e
                ));
            }
        };
        */
        let css = tx_consumers.clone();
        spawn(move || {
            if let Err(e) = executor::block_on(async {
                loop {
                    let consumers = Arc::new(Mutex::new(css.clone()));
                    match rx_channel.recv().await {
                        Ok(event) => {
                            let feedback = feedback.clone();
                            match event {
                                Events::Connected(uuid) => match consumers.lock() {
                                    Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Add(uuid)).await {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    },
                                    Err(e) => {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Add: Fail to access to consumers channel due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    }
                                },
                                Events::Disconnected(uuid) => match consumers.lock() {
                                    Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Remove(uuid)).await {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    },
                                    Err(e) => {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Remove: Fail to access to consumers channel due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    }
                                },
                                Events::Received(uuid, buffer) => match consumers.lock() {
                                    Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Chunk((uuid, buffer))).await {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    },
                                    Err(e) => {
                                        if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                            tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers channel due error: {}", e)),
                                        )).await {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    }
                                },
                                Events::Error(uuid, e) => if let Err(e) = feedback.send(ProducerEvents::ConnectionError(
                                    tools::logger.err(&format!("Connection {:?}: {}", uuid, e)),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                }
                            }
                        },
                        Err(e) => {
                            tools::logger.err(&format!("{}", e));
                        }
                    }
                }
                Ok::<(), String>(())
            }) {
                // TODO: Error message
            }
        });
        let feedback = tx_feedback.clone();
        spawn(move || {
            match server.listen(tx_channel, sender_rx_channel) {
                Ok(()) => {
                    if let Err(e) = executor::block_on(async move {
                        if let Err(e) = feedback.send(ProducerEvents::ServerDown).await {
                            tools::logger.warn(&format!("{}", e));
                        }
                        Ok::<(), String>(())
                    }) {
                        // TODO: Error message
                    }
                },
                Err(e) => {
                    if let Err(e) = executor::block_on(async move {
                        if let Err(e) = feedback.send(ProducerEvents::ServerError(e)).await {
                            tools::logger.err(&format!("{}", e));
                        }
                        Ok::<(), String>(())
                    }) {
                        // TODO: Error message
                    }
                }
            };
        });
        let feedback = tx_feedback.clone();
        let ucx = ucx.clone();
        spawn(move || {
            if let Err(e) = executor::block_on(async {
                let store = Arc::new(RwLock::new(HashMap::new()));
                let sender_tx_channel = Arc::new(Mutex::new(sender_tx_channel.clone()));
                let UserLogin = Arc::new(RwLock::new(UserLoginObserver::ObserverRequest::new()));
                let Users = Arc::new(RwLock::new(UsersObserver::ObserverRequest::new()));
                let Message = Arc::new(RwLock::new(MessageObserver::ObserverRequest::new()));
                let Messages = Arc::new(RwLock::new(MessagesObserver::ObserverRequest::new()));
                loop {
                    let broadcast = |filter: Filter, buffer: Vec<u8>| {
                        async {
                            match broadcasting(tx_consumers.clone(), filter, buffer).await {
                                Ok(_) => Ok::<(), String>(()),
                                Err(e) => Err::<(), String>(e)
                            }
                        }
                    };
                    match rx_consumers.recv().await {
                        Ok(action) => match action {
                            ConsumersChannel::Add(uuid) => match store.write() {
                                Ok(mut store) => {
                                    let _consumer = store.entry(uuid).or_insert_with(|| {
                                        Consumer::new(
                                            uuid,
                                            consumers.clone(),
                                            sender_tx_channel.clone(),
                                        )
                                    });
                                    tools::logger.debug(&format!("New Consumer added; uuid: {}", uuid));
                                    if let Err(e) =
                                        feedback.send(ProducerEvents::Connected((uuid, ucx.clone()))).await
                                    {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                    if let Err(e) = Self::connected(uuid.clone(), ucx.clone()) {
                                        tools::logger.err(&e);
                                    }
                                }
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                }
                            },
                            ConsumersChannel::Remove(uuid) => match store.write() {
                                Ok(mut store) => {
                                    store.remove(&uuid);
                                    if let Err(e) = feedback.send(ProducerEvents::Disconnected(uuid)).await {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                    tools::logger.debug(&format!("Consumer uuid: {} disconnected and destroyed", uuid));
                                    match Self::disconnected(uuid.clone(), ucx.clone()) {
                                        Ok(mut msgs) => {
                                            match msgs.UserDisconnected.1.pack(0, Some(uuid.to_string())) {
                                                Ok(buffer) => if let Err(e) = broadcast(msgs.UserDisconnected.0, buffer).await {
                                                    if let Err(e) = feedback.send(ProducerEvents::BroadcastingError(
                                                        tools::logger.err(&format!("ConsumersChannel::Remove: cannot broadcast UserDisconnected due error: {}", e)),
                                                    )).await {
                                                        tools::logger.err(&format!("{}", e));
                                                    }
                                                },
                                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::BroadcastingError(
                                                    tools::logger.err(&format!("ConsumersChannel::Remove: cannot encode UserDisconnected due error: {}", e)),
                                                )).await {
                                                    tools::logger.err(&format!("{}", e));
                                                },
                                            }
                                            if let Some(mut msg) = msgs.Message {
                                                match msg.1.pack(0, Some(uuid.to_string())) {
                                                    Ok(buffer) => if let Err(e) = broadcast(msg.0, buffer).await {
                                                        if let Err(e) = feedback.send(ProducerEvents::BroadcastingError(
                                                            tools::logger.err(&format!("ConsumersChannel::Remove: cannot broadcast Message due error: {}", e)),
                                                        )).await {
                                                            tools::logger.err(&format!("{}", e));
                                                        }
                                                    },
                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::BroadcastingError(
                                                        tools::logger.err(&format!("ConsumersChannel::Remove: cannot encode Message due error: {}", e)),
                                                    )).await {
                                                        tools::logger.err(&format!("{}", e));
                                                    },
                                                };
                                            }
                                        },
                                        Err(e) => { tools::logger.err(&e); },
                                    };
                                },
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                }
                            },
                            ConsumersChannel::SendByFilter((filter, buffer)) => match store.write() {
                                Ok(store) => {
                                    let mut errors: Vec<String> = vec![];
                                    for (uuid, consumer) in store.iter() {
                                        if let Err(e) = consumer.send_if(buffer.clone(), filter.clone()).await
                                        {
                                            errors.push(format!("Fail to send data to {}, due error: {}", uuid, e));
                                        }
                                    }
                                    if !errors.is_empty() {
                                        tools::logger.err(&errors.join("\n"));
                                    }
                                }
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    format!("ConsumersChannel::SendByFilter: Fail to access to consumers due error: {}", e),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                },
                            },
                            ConsumersChannel::SendTo((uuid, buffer)) => match store.write() {
                                Ok(mut store) => {
                                    if let Some(consumer) = store.get_mut(&uuid) {
                                        if let Err(e) = consumer.send(buffer).await {
                                            tools::logger.err(&format!("Fail to send buffer for consumer {} due error {}", uuid, e));
                                        }
                                    } else {
                                        tools::logger.err(&format!("ConsumersChannel::SendTo: Fail to find consumer {}", uuid));
                                    }
                                },
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    format!("ConsumersChannel::SendTo: Fail to access to consumers due error: {}", e),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                },
                            },
                            ConsumersChannel::Assign((uuid, key, overwrite)) => match store.write() {
                                Ok(mut store) => {
                                    if let Some(consumer) = store.get_mut(&uuid) {
                                        consumer.assign(key, overwrite);
                                    } else {
                                        tools::logger.err(&format!("ConsumersChannel::Assign: Fail to find consumer {}", uuid));
                                    }
                                },
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    format!("ConsumersChannel::Assign: Fail to access to consumers due error: {}", e),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                },
                            },
                            ConsumersChannel::Chunk((uuid, buffer)) => match store.write() {
                                Ok(mut consumers) => {
                                    tools::logger.debug(&format!("New message has been received; uuid: {}; length: {}", uuid, buffer.len()));
                                    if let Some(consumer) = consumers.get_mut(&uuid) {
                                        if let Err(e) = consumer.chunk(&buffer) {
                                            if let Err(e) = feedback.send(ProducerEvents::Reading(
                                                tools::logger.err(&format!("Fail to read connection buffer due error: {}", e))
                                            )).await {
                                                tools::logger.err(&format!("{}", e));
                                            }
                                        }
                                        while let Some((message, header)) = consumer.next() {
                                            match message {
                                                    Protocol::AvailableMessages::Identification(message) => if let Protocol::Identification::AvailableMessages::SelfKey(request) = message {
                                                        let uuid = consumer.key(request, true);
                                                        tools::logger.debug(&format!("{}:: identification is done", uuid));
                                                        if let Err(e) = match (Protocol::Identification::SelfKeyResponse { uuid: uuid.clone() }).pack(header.sequence, Some(uuid.to_string())) {
                                                            Ok(buffer) => if let Err(e) = consumer.send(buffer).await {
                                                                Err(e)
                                                            } else {
                                                                tools::logger.debug(&format!("{}:: identification response has been sent", uuid));
                                                                Ok(())
                                                            },
                                                            Err(e) => Err(e),
                                                        } {
                                                            if let Err(e) = feedback.send(ProducerEvents::ConnectionError(format!("Fail to response for Identification due error: {:?}", e).to_owned())).await {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    message => if !consumer.assigned() {
                                                        if let Err(e) = feedback.send(ProducerEvents::NotAssignedConsumer(tools::logger.err(&format!("Consumer ({}) didn't apply Identification", consumer.get_uuid()).to_owned()))).await {
                                                            tools::logger.err(&format!("{}", e));
                                                        }
                                                        // TODO: Consumer should be disconnected or some feedback should be to consumer
                                                        // it might be some option of producer like NonAssignedStratagy
                                                    } else {
                                                        match message {
                                                            Protocol::AvailableMessages::UserLogin(Protocol::UserLogin::AvailableMessages::Request(request)) => {
                                                                tools::logger.debug(&format!("Protocol::AvailableMessages::UserLogin::Request {:?}", request));
                                                                match UserLogin.write() {
                                                                    Ok(UserLogin) => {
                                                                        use UserLoginObserver::Observer;
                                                                        if let Err(e) = UserLogin.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            header.sequence,
                                                                            request,
                                                                            &broadcast,
                                                                        ).await {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit UserLogin due error: {:?}", e).to_owned())).await {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to UserLogin due error: {}", e).to_owned())).await {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            Protocol::AvailableMessages::Users(Protocol::Users::AvailableMessages::Request(request)) => {
                                                                tools::logger.debug(&format!("Protocol::AvailableMessages::Users::Request {:?}", request));
                                                                match Users.write() {
                                                                    Ok(Users) => {
                                                                        use UsersObserver::Observer;
                                                                        if let Err(e) = Users.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            header.sequence,
                                                                            request,
                                                                            &broadcast,
                                                                        ).await {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit Protocol::Users::Request due error: {:?}", e).to_owned())).await {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to Users due error: {}", e).to_owned())).await {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            Protocol::AvailableMessages::Message(Protocol::Message::AvailableMessages::Request(request)) => {
                                                                tools::logger.debug(&format!("Protocol::AvailableMessages::Message::Request {:?}", request));
                                                                match Message.write() {
                                                                    Ok(Message) => {
                                                                        use MessageObserver::Observer;
                                                                        if let Err(e) = Message.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            header.sequence,
                                                                            request,
                                                                            &broadcast,
                                                                        ).await {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit Message due error: {:?}", e).to_owned())).await {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to Message due error: {}", e).to_owned())).await {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            Protocol::AvailableMessages::Messages(Protocol::Messages::AvailableMessages::Request(request)) => {
                                                                tools::logger.debug(&format!("Protocol::AvailableMessages::Messages::Request {:?}", request));
                                                                match Messages.write() {
                                                                    Ok(Messages) => {
                                                                        use MessagesObserver::Observer;
                                                                        if let Err(e) = Messages.emit(
                                                                            consumer.get_cx(),
                                                                            ucx.clone(),
                                                                            header.sequence,
                                                                            request,
                                                                            &broadcast,
                                                                        ).await {
                                                                            if let Err(e) = feedback.send(ProducerEvents::EmitError(format!("Fail to emit Messages due error: {:?}", e).to_owned())).await {
                                                                                tools::logger.err(&format!("{}", e));
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(format!("Fail to access to Messages due error: {}", e).to_owned())).await {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                            },
                                                        }
                                                    },
                                                };
                                        }
                                    } else {
                                        tools::logger.err(&format!("Fail to find consumer uuid: {}", uuid));
                                    }
                                },
                                Err(e) => if let Err(e) = feedback.send(ProducerEvents::InternalError(
                                    tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                                )).await {
                                    tools::logger.err(&format!("{}", e));
                                }
                            },
                        }
                        Err(e) => {
                            
                        }
                    }
                }
                Ok::<(), String>(())
            }) {
                // TODO: Error message
            }
        });
        Ok(Channel {
            events: rx_feedback,
            consumers: channel_consumers
        })
    }

    fn connected(uuid: Uuid, ucx: UCX) -> Result<(), String> {
        Err(format!("Handler for event conntected isn't implemented."))
    }

    fn disconnected(uuid: Uuid, ucx: UCX) -> Result<EventDisconnectedBroadcasting, String> {
        Err(format!("Handler for event conntected isn't implemented."))
    }

}
