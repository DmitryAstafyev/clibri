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

#[allow(non_snake_case)]
#[path = "./events/event.Connected.rs"]
pub mod ConnectedEvent;

#[allow(non_snake_case)]
#[path = "./events/event.Disconnected.rs"]
pub mod DisconnectedEvent;


use super::tools;
use consumer::Consumer;
use consumer_identification::Filter;
use fiber::{
    logger::Logger,
    server::{
        control::Control as ServerControl,
        events::Events,
        interface::Interface
    }
};
use protocol as Protocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::pin::Pin;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    sync::oneshot::{channel, Receiver, Sender},
    task::{spawn, JoinHandle},
    runtime::Runtime,
};
use uuid::Uuid;
use futures::Future;
use Protocol::PackingStruct;

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

#[allow(non_snake_case)]
pub struct EventDisconnectedBroadcasting {
    pub UserDisconnected: (Filter, Protocol::Events::UserDisconnected),
    pub Message: Option<(Filter, Protocol::Events::Message)>,
}

pub fn broadcasting(
    consumers: UnboundedSender<ConsumersChannel>,
    filter: Filter,
    buffer: Vec<u8>,
) -> Result<(), String> {
    if let Err(e) = consumers.send(ConsumersChannel::SendByFilter((filter, buffer))) {
        Err(tools::logger.err(&format!("Fail to get access consumers channel due error: {}", e)))
    } else {
        Ok(())
    }
}

pub struct Controlaa<UCX>
where
    UCX: 'static + Sync + Send + Clone,
{
    pub events: UnboundedReceiver<ProducerEvents<UCX>>,
    pub server: Pin<Box<dyn Future<Output = ()>>>,
    consumers: UnboundedSender<ConsumersChannel>,
}

// impl<UCX: 'static + Sync + Send + Clone> Channel<UCX> {

//     pub fn send(&self, filter: Filter, buffer: Vec<u8>) -> Result<(), String> {
//         Err(String::from(""))
//         /*
//         match self.consumers.lock() {
//             Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::SendByFilter((filter, buffer))) {
//                 Err(e.to_string())
//             } else {
//                 Ok(())
//             },
//             Err(e) => Err(e.to_string()),
//         }
//         */
//     }

//     pub fn assign(&self, uuid: Uuid, assigned: Protocol::Identification::AssignedKey, overwrite: bool) -> Result<(), String> {
//         Err(String::from(""))
//         /*
//         match self.consumers.lock() {
//             Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Assign((uuid, assigned, overwrite))) {
//                 Err(e.to_string())
//             } else {
//                 Ok(())
//             },
//             Err(e) => Err(e.to_string()),
//         }
//         */
//     }

//     pub fn drop(&self, uuid: Uuid) -> Result<(), String> {
//         // TODO: drop connection of consumer
//         Ok(())
//     }
// }


pub fn listen<
    S: 'static + Interface + Sync + Send,
    UCX: 'static + Sync + Send + Clone,
>(
    mut server: S,
    ucx: UCX,
) -> (impl Future<Output = ()>, UnboundedReceiver<ProducerEvents<UCX>>, UnboundedSender<ServerControl>) {
    let (tx_server_events, rx_server_events): (
        UnboundedSender<Events>,
        UnboundedReceiver<Events>) =
        unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_producer_events, rx_producer_events): (
        UnboundedSender<ProducerEvents<UCX>>,
        UnboundedReceiver<ProducerEvents<UCX>>,
    ) = unbounded_channel();
    let (tx_consumers, rx_consumers): (
        UnboundedSender<ConsumersChannel>,
        UnboundedReceiver<ConsumersChannel>,
    ) = unbounded_channel();
    let (tx_server_control, rx_server_control): (
        UnboundedSender<ServerControl>,
        UnboundedReceiver<ServerControl>,
    ) = unbounded_channel();
    let server_listener_task: JoinHandle<Result<(), String>> = spawn_server_listener(
        rx_server_events,
        tx_consumers.clone(),
        tx_producer_events.clone(),
    );
    // let server_rt_tast : JoinHandle<Result<(), String>> = spawn_server(
    //     server,
    //     tx_server_events,
    //     rx_sender,
    //     rx_server_control,
    // );
    let consumers_task: JoinHandle<Result<(), String>> = spawn_consumers(
        tx_consumers,
        rx_consumers,
        tx_sender,
        tx_producer_events,
        ucx,
    );
    let task = async move {
        select! {
            _ = server_listener_task => {
                tools::logger.debug("Server listener tasks is finished");
            },
            _ = server.listen(tx_server_events, rx_sender, Some(rx_server_control)) => {
                tools::logger.debug("Server is finished");
            },
            _ = consumers_task => {
                tools::logger.debug("Consumers tasks is finished");
            },
        };
    };
    (task, rx_producer_events, tx_server_control)
}

fn spawn_server_listener<
    UCX: 'static + Sync + Send + Clone,
>(
    mut rx_server_events: UnboundedReceiver<Events>,
    tx_consumers: UnboundedSender<ConsumersChannel>,
    tx_producer_events: UnboundedSender<ProducerEvents<UCX>>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.verb("[task: server listener]:: started");
        let (tx_streams_task_sd, rx_streams_task_sd): (
            Sender<()>,
            Receiver<()>,
        ) = channel();
        select! {
            _ = async {
                tools::logger.verb("[task: server listener]:: started");
                while let Some(event) = rx_server_events.recv().await {
                    let consumers = tx_consumers.clone();
                    let producer_events = tx_producer_events.clone();
                    match event {
                        Events::Ready => {

                        },
                        Events::Connected(uuid) => if let Err(e) = consumers.send(ConsumersChannel::Add(uuid)) {
                            if let Err(e) = producer_events.send(ProducerEvents::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e)),
                            )) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        Events::Disconnected(uuid) => if let Err(e) = consumers.send(ConsumersChannel::Remove(uuid)) {
                            if let Err(e) = producer_events.send(ProducerEvents::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e)),
                            )) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        Events::Received(uuid, buffer) => if let Err(e) = consumers.send(ConsumersChannel::Chunk((uuid, buffer))) {
                            if let Err(e) = producer_events.send(ProducerEvents::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                            )) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        Events::Error(uuid, e) => if let Err(e) = producer_events.send(ProducerEvents::ConnectionError(
                            tools::logger.err(&format!("Error {:?}: {}", uuid, e)),
                        )) {
                            tools::logger.err(&format!("{}", e));
                        },
                        Events::ConnectionError(uuid, e) => if let Err(e) = producer_events.send(ProducerEvents::ConnectionError(
                            tools::logger.err(&format!("ConnectionError {:?}: {:?}", uuid, e)),
                        )) {
                            tools::logger.err(&format!("{}", e));
                        },
                        Events::ServerError(e) => {
                            if let Err(e) = producer_events.send(ProducerEvents::ConnectionError(
                                tools::logger.err(&format!("ServerError {:?}", e)),
                            )) {
                                tools::logger.err(&format!("{}", e));
                            }
                            break;
                        },
                    }
                }
            } => {},
            _ = rx_streams_task_sd => {}
        };
        tools::logger.verb("[task: server listener]:: finished");
        Ok(())
    })
}

fn spawn_server<
    S: 'static + Interface + Sync + Send,
>(
    mut server: S,
    tx_server_events: UnboundedSender<Events>,
    rx_sender: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    rx_server_control: UnboundedReceiver<ServerControl>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async move {
            if let Err(e) = server.listen(tx_server_events, rx_sender, Some(rx_server_control)).await {
                tools::logger.err(&format!("[T] fail to create server: {}", e));
            }
        });
        Ok(())
    })
}

fn spawn_consumers<
    UCX: 'static + Sync + Send + Clone,
>(
    tx_consumers: UnboundedSender<ConsumersChannel>,
    mut rx_consumers: UnboundedReceiver<ConsumersChannel>,
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_producer_events: UnboundedSender<ProducerEvents<UCX>>,
    ucx: UCX,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.verb("[task: consumers]:: started");
        let store: Arc<RwLock<HashMap<Uuid, Consumer>>> = Arc::new(RwLock::new(HashMap::new()));
        let UserLogin = Arc::new(RwLock::new(UserLoginObserver::ObserverRequest::new()));
        let Users = Arc::new(RwLock::new(UsersObserver::ObserverRequest::new()));
        let Message = Arc::new(RwLock::new(MessageObserver::ObserverRequest::new()));
        let Messages = Arc::new(RwLock::new(MessagesObserver::ObserverRequest::new()));
        let Connected = Arc::new(RwLock::new(ConnectedEvent::ObserverEvent::new()));
        let Disconnected = Arc::new(RwLock::new(DisconnectedEvent::ObserverEvent::new()));
        while let Some(event) = rx_consumers.recv().await {
            let broadcast = |filter: Filter, buffer: Vec<u8>| {
                match broadcasting(tx_consumers.clone(), filter, buffer) {
                    Ok(_) => Ok::<(), String>(()),
                    Err(e) => Err::<(), String>(e)
                }
            };
            match event {
                ConsumersChannel::Add(uuid) => match store.write() {
                    Ok(mut store) => {
                        let _consumer = store.entry(uuid).or_insert_with(|| {
                            Consumer::new(
                                uuid,
                                tx_consumers.clone(),
                                tx_sender.clone(),
                            )
                        });
                        tools::logger.debug(&format!("New Consumer added; uuid: {}", uuid));
                        if let Err(e) = tx_producer_events.send(ProducerEvents::Connected((uuid, ucx.clone()))) {
                            tools::logger.err(&format!("{}", e));
                        }
                        match Connected.write() {
                            Ok(Connected) => {
                                use ConnectedEvent::Observer;
                                Connected.emit(
                                    uuid.clone(),
                                    ucx.clone(),
                                    &broadcast,
                                );
                            }
                            Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to Connected event handler due error: {}", e).to_owned())) {
                                tools::logger.err(&format!("{}", e));
                            }
                        }
                    }
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e),
                    )) {
                        tools::logger.err(&format!("{}", e));
                    }
                },
                ConsumersChannel::Remove(uuid) => match store.write() {
                    Ok(mut store) => {
                        store.remove(&uuid);
                        if let Err(e) = tx_producer_events.send(ProducerEvents::Disconnected(uuid)) {
                            tools::logger.err(&format!("{}", e));
                        }
                        tools::logger.debug(&format!("Consumer uuid: {} disconnected and destroyed", uuid));
                        match Disconnected.write() {
                            Ok(Disconnected) => {
                                use DisconnectedEvent::Observer;
                                Disconnected.emit(
                                    uuid.clone(),
                                    ucx.clone(),
                                    &broadcast,
                                );
                            }
                            Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to Connected event handler due error: {}", e).to_owned())) {
                                tools::logger.err(&format!("{}", e));
                            }
                        }
                    },
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e),
                    )) {
                        tools::logger.err(&format!("{}", e));
                    }
                },
                ConsumersChannel::SendByFilter((filter, buffer)) => match store.write() {
                    Ok(store) => {
                        let mut errors: Vec<String> = vec![];
                        for (uuid, consumer) in store.iter() {
                            if let Err(e) = consumer.send_if(buffer.clone(), filter.clone())
                            {
                                errors.push(format!("Fail to send data to {}, due error: {}", uuid, e));
                            }
                        }
                        if !errors.is_empty() {
                            tools::logger.err(&errors.join("\n"));
                        }
                    }
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        format!("ConsumersChannel::SendByFilter: Fail to access to consumers due error: {}", e),
                    )) {
                        tools::logger.err(&format!("{}", e));
                    },
                },
                ConsumersChannel::SendTo((uuid, buffer)) => match store.write() {
                    Ok(mut store) => {
                        if let Some(consumer) = store.get_mut(&uuid) {
                            if let Err(e) = consumer.send(buffer) {
                                tools::logger.err(&format!("Fail to send buffer for consumer {} due error {}", uuid, e));
                            }
                        } else {
                            tools::logger.err(&format!("ConsumersChannel::SendTo: Fail to find consumer {}", uuid));
                        }
                    },
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        format!("ConsumersChannel::SendTo: Fail to access to consumers due error: {}", e),
                    )) {
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
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        format!("ConsumersChannel::Assign: Fail to access to consumers due error: {}", e),
                    )) {
                        tools::logger.err(&format!("{}", e));
                    },
                },
                ConsumersChannel::Chunk((uuid, buffer)) => match store.write() {
                    Ok(mut consumers) => {
                        tools::logger.debug(&format!("New message has been received; uuid: {}; length: {}", uuid, buffer.len()));
                        if let Some(consumer) = consumers.get_mut(&uuid) {
                            if let Err(e) = consumer.chunk(&buffer) {
                                if let Err(e) = tx_producer_events.send(ProducerEvents::Reading(
                                    tools::logger.err(&format!("Fail to read connection buffer due error: {}", e))
                                )) {
                                    tools::logger.err(&format!("{}", e));
                                }
                            }
                            while let Some((message, header)) = consumer.next() {
                                match message {
                                    Protocol::AvailableMessages::Identification(message) => if let Protocol::Identification::AvailableMessages::SelfKey(request) = message {
                                        let uuid = consumer.key(request, true);
                                        tools::logger.debug(&format!("{}:: identification is done", uuid));
                                        if let Err(e) = match (Protocol::Identification::SelfKeyResponse { uuid: uuid.clone() }).pack(header.sequence, Some(uuid.to_string())) {
                                            Ok(buffer) => if let Err(e) = consumer.send(buffer) {
                                                Err(e)
                                            } else {
                                                tools::logger.debug(&format!("{}:: identification response has been sent", uuid));
                                                Ok(())
                                            },
                                            Err(e) => Err(e),
                                        } {
                                            if let Err(e) = tx_producer_events.send(ProducerEvents::ConnectionError(format!("Fail to response for Identification due error: {:?}", e).to_owned())) {
                                                tools::logger.err(&format!("{}", e));
                                            }
                                        }
                                    },
                                    message => if !consumer.assigned() {
                                        if let Err(e) = tx_producer_events.send(ProducerEvents::NotAssignedConsumer(tools::logger.err(&format!("Consumer ({}) didn't apply Identification", consumer.get_uuid()).to_owned()))) {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                        // TODO: Consumer should be disconnected or some tx_producer_events should be to consumer
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
                                                        ) {
                                                            if let Err(e) = tx_producer_events.send(ProducerEvents::EmitError(format!("Fail to emit UserLogin due error: {:?}", e).to_owned())) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to UserLogin due error: {}", e).to_owned())) {
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
                                                        ) {
                                                            if let Err(e) = tx_producer_events.send(ProducerEvents::EmitError(format!("Fail to emit Protocol::Users::Request due error: {:?}", e).to_owned())) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to Users due error: {}", e).to_owned())) {
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
                                                        ) {
                                                            if let Err(e) = tx_producer_events.send(ProducerEvents::EmitError(format!("Fail to emit Message due error: {:?}", e).to_owned())) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to Message due error: {}", e).to_owned())) {
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
                                                        ) {
                                                            if let Err(e) = tx_producer_events.send(ProducerEvents::EmitError(format!("Fail to emit Messages due error: {:?}", e).to_owned())) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(format!("Fail to access to Messages due error: {}", e).to_owned())) {
                                                        tools::logger.err(&format!("{}", e));
                                                    }
                                                }
                                            },
                                            _ => {
                                            },
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        } else {
                            tools::logger.err(&format!("Fail to find consumer uuid: {}", uuid));
                        }
                    },
                    Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::InternalError(
                        tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                    )) {
                        tools::logger.err(&format!("{}", e));
                    }
                },
            }
        }
        tools::logger.verb("[task: consumers]:: finished");
        Ok(())
    })
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
pub trait ProducerTrait<UCX>
where
    UCX: 'static + Sync + Send + Clone,
{

    fn connected(uuid: Uuid, ucx: UCX) -> Result<(), String> {
        Err(format!("Handler for event conntected isn't implemented."))
    }

    fn disconnected(uuid: Uuid, ucx: UCX) -> Result<EventDisconnectedBroadcasting, String> {
        Err(format!("Handler for event conntected isn't implemented."))
    }

}

pub struct Producer {

}

impl<UCX: 'static + Sync + Send + Clone> ProducerTrait<UCX> for Producer {

}
