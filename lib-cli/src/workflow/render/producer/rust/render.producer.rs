use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        store::{
            Store as WorkflowStore
        }
    },
    Protocol,
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

mod templates {
    pub const MODULE: &str = r#"
#[path = "./traits/observer.rs"]
pub mod observer;

#[path = "./protocol/protocol.rs"]
pub mod protocol;

#[path = "./consumer/consumer.rs"]
pub mod consumer;

#[path = "./consumer/consumer_identification.rs"]
pub mod consumer_identification;

[[requests_declaration]]

#[path = "./events/deafault_event_connected.rs"]
pub mod default_connected_event;

#[path = "./events/deafault_event_disconnected.rs"]
pub mod default_disconnected_event;

[[events_declaration]]

use super::tools;
use consumer::Consumer;
use consumer_identification::Filter;
use fiber::{
    logger::Logger,
    server::{
        control::Control as ServerControl,
        events::Events as ServerEvents,
        interface::Interface
    }
};
use protocol as Protocol;
use std::collections::HashMap;
use std::sync::{
    Arc,
    RwLock
};
use std::pin::Pin;
use tokio::{
    select,
    join,
    sync::mpsc::{
        unbounded_channel,
        UnboundedReceiver,
        UnboundedSender,
        error::SendError
    },
    sync::oneshot::{
        channel,
        Receiver,
        Sender
    },
    task::{
        spawn,
        JoinHandle
    },
    runtime::Runtime,
};
use uuid::Uuid;
use futures::Future;
use Protocol::PackingStruct;

#[derive(Debug)]
pub enum RequestObserverErrors {
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    BeforeResponseActionFail(String),
    ErrorOnEventsEmit(String),
    GettingConclusionError(String),
    AfterConclusionError(String),
    BroadcastingError(String),
}

pub enum ProducerError {
    InternalError(String),
    EmitError(String),
    EventError(String),
    EventChannelError(String),
    EventListenError(String),
    ConnectionError(String),
    BroadcastingError(String),
    ServerError(String),
    NotAssignedConsumer(String),
    Reading(String),
}

pub enum ProducerEvents {
    Shutdown,
    ServerReady,
    ConsumerConnected(Uuid),
    ConsumerDisconnected(Uuid),
    Error(ProducerError),
}

#[derive(Clone)]
pub enum ConsumersChannel {
    Add(Uuid),
    Remove(Uuid),
    SendByFilter((Filter, Vec<u8>)),
    SendTo((Uuid, Vec<u8>)),
    Assign((Uuid, [[indentification_assigned_key]], bool)),
    Chunk((Uuid, Vec<u8>)),
    Disconnect(Filter),
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

#[allow(non_snake_case)]
trait ProducerEventsHolderTrait {

    fn ConsumerConnected(_uuid: Uuid) {
        tools::logger.debug("[producer event: ConsumerConnected] doesn't have handler");
    }

    fn ConsumerDisconnected(_uuid: Uuid) {
        tools::logger.debug("[producer event: ConsumerDisconnected] doesn't have handler");
    }

    fn Shutdown() {
        tools::logger.debug("[producer event: Shutdown] doesn't have handler");
    }

    fn ServerReady() {
        tools::logger.debug("[producer event: ServerReady] doesn't have handler");
    }

    fn Error(_err: ProducerError) {
        tools::logger.debug("[producer event: InternalError] doesn't have handler");
    }

}

pub struct ProducerEventsHolder;

impl ProducerEventsHolderTrait for ProducerEventsHolder {}

#[derive(Clone)]
pub struct Events {
[[events_struct_declaration]],
}

impl Events {
    pub fn new(
[[events_struct_args]]
    ) -> Self {
        Events {
[[events_struct_props]]
        }
    }
}

#[derive(Clone)]
pub struct Control {
    server_control: UnboundedSender<ServerControl>,
    consumers: UnboundedSender<ConsumersChannel>,
    pub events: Events,
}

impl Control {

    pub fn new(
        server_control: UnboundedSender<ServerControl>,
        consumers: UnboundedSender<ConsumersChannel>,
        events: Events,
    ) -> Self {
        Control {
            server_control,
            consumers,
            events,
        }
    }

    pub fn shutdown(&self) -> Result<(), SendError<ServerControl>> {
        self.server_control.send(ServerControl::Shutdown)
    }

    pub fn assign(&self, uuid: Uuid, assigned: [[indentification_assigned_key]], overwrite: bool) -> Result<(), SendError<ConsumersChannel>> {
        self.consumers.send(ConsumersChannel::Assign((uuid, assigned, overwrite)))
    }

    pub fn send(&self, filter: Filter, buffer: Vec<u8>) -> Result<(), SendError<ConsumersChannel>> {
        self.consumers.send(ConsumersChannel::SendByFilter((filter, buffer)))
    }

    pub fn disconnect(&self, filter: Filter) -> Result<(), SendError<ConsumersChannel>> {
        self.consumers.send(ConsumersChannel::Disconnect(filter))
    }

}

pub enum StartError {
    Runtime()
}

pub fn init_and_start<
    S: 'static + Interface + Sync + Send,
    UCX: 'static + Sync + Send + Clone,
>(
    server: S,
    ucx: UCX,
    control: Option<std::sync::mpsc::Sender<Control>>,
) -> Result<(), std::io::Error> {
    let rt  = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            return Err(e);
        },
    };
    rt.block_on(async move {
        let (thread, controller) = init(server, ucx);
        if let Some(sender) = control {
            if let Err(e) = sender.send(controller) {
                panic!("Cannot send control. Error: {}", e);
            }
        }
        thread.await;
    });
    Ok(())
}

pub fn init<
    S: 'static + Interface + Sync + Send,
    UCX: 'static + Sync + Send + Clone,
>(
    mut server: S,
    ucx: UCX,
) -> (Pin<Box<dyn Future<Output = ()>>>, Control) {
    let (tx_server_events, rx_server_events): (
        UnboundedSender<ServerEvents>,
        UnboundedReceiver<ServerEvents>) =
        unbounded_channel();
    let (tx_sender, rx_sender): (
        UnboundedSender<(Vec<u8>, Option<Uuid>)>,
        UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) = unbounded_channel();
    let (tx_producer_events, rx_producer_events): (
        UnboundedSender<ProducerEvents>,
        UnboundedReceiver<ProducerEvents>,
    ) = unbounded_channel();
    let (tx_consumers, rx_consumers): (
        UnboundedSender<ConsumersChannel>,
        UnboundedReceiver<ConsumersChannel>,
    ) = unbounded_channel();
    let (tx_server_control, rx_server_control): (
        UnboundedSender<ServerControl>,
        UnboundedReceiver<ServerControl>,
    ) = unbounded_channel();
    let (tx_server_listener_task_sd, rx_server_listener_task_sd): (
        Sender<()>,
        Receiver<()>,
    ) = channel();
    let server_listener_task: JoinHandle<Result<(), String>> = spawn_server_listener(
        rx_server_events,
        tx_consumers.clone(),
        tx_producer_events.clone(),
        rx_server_listener_task_sd,
    );
    let (tx_producer_events_holder_task_sd, rx_producer_events_holder_task_sd): (
        Sender<()>,
        Receiver<()>,
    ) = channel();
    let producer_events_holder_task: JoinHandle<Result<(), String>> = spawn_producer_events_holder(
        rx_producer_events,
        rx_producer_events_holder_task_sd,
    );
    let (tx_consumers_task_sd, rx_consumers_task_sd): (
        Sender<()>,
        Receiver<()>,
    ) = channel();
    let consumers_task: JoinHandle<Result<(), String>> = spawn_consumers(
        tx_consumers.clone(),
        rx_consumers,
        tx_server_control.clone(),
        tx_sender,
        tx_producer_events,
        ucx.clone(),
        rx_consumers_task_sd,
    );
[[events_channels]]
    let events = Events::new(
[[events_senders]]
    );
    let control = Control::new(
        tx_server_control,
        tx_consumers,
        events,
    );
    let (tx_events_task_sd, rx_events_task_sd): (
        Sender<()>,
        Receiver<()>,
    ) = channel();
    let events_task: JoinHandle<Result<(), String>> = spawn_events(
        ucx,
        control.clone(),
[[events_receivers]],
        rx_events_task_sd,
    );
    let task = async move {
        tools::logger.debug("[task: main]:: started");
        select! {
            _ = server_listener_task => {
                tools::logger.debug("[task: server listener]:: finished in chain");
            },
            _ = server.listen(tx_server_events, rx_sender, Some(rx_server_control)) => {
                tools::logger.debug("[task: server]:: finished in chain");
            },
            _ = producer_events_holder_task => {
                tools::logger.debug("[task: producer events holder]:: finished in chain");
            },
            _ = consumers_task => {
                tools::logger.debug("[task: consumers task]:: finished in chain");
            },
            _ = events_task => {
                tools::logger.debug("[task: events task]:: finished in chain");
            },
        };
        for task in (vec![
            Some(("server listener", tx_server_listener_task_sd)),
            Some(("events holder", tx_producer_events_holder_task_sd)),
            Some(("consumers", tx_consumers_task_sd)),
            Some(("events", tx_events_task_sd)),
        ]).iter_mut() {
            if let Some(task) = task.take() {
                if let Err(_e) = task.1.send(()) {
                    tools::logger.debug(&format!("Fail send finish signal to task: {}", task.0));
                }
            }
        }
        tools::logger.debug("[task: main]:: finished");
    };
    (Box::pin(task), control)
}

fn spawn_server_listener(
    mut rx_server_events: UnboundedReceiver<ServerEvents>,
    tx_consumers: UnboundedSender<ConsumersChannel>,
    tx_producer_events: UnboundedSender<ProducerEvents>,
    rx_shutdown: Receiver<()>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.debug("[task: server listener]:: started");
        let (_tx_streams_task_sd, rx_streams_task_sd): (
            Sender<()>,
            Receiver<()>,
        ) = channel();
        select! {
            _ = async {
                tools::logger.debug("[task: server listener]:: started");
                while let Some(event) = rx_server_events.recv().await {
                    let consumers = tx_consumers.clone();
                    let producer_events = tx_producer_events.clone();
                    match event {
                        ServerEvents::Ready => {

                        },
                        ServerEvents::Connected(uuid) => if let Err(e) = consumers.send(ConsumersChannel::Add(uuid)) {
                            if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        ServerEvents::Disconnected(uuid) => if let Err(e) = consumers.send(ConsumersChannel::Remove(uuid)) {
                            if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        ServerEvents::Received(uuid, buffer) => if let Err(e) = consumers.send(ConsumersChannel::Chunk((uuid, buffer))) {
                            if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        ServerEvents::Error(uuid, e) => if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::ConnectionError(
                            tools::logger.err(&format!("Error {:?}: {}", uuid, e)),
                        ))) {
                            tools::logger.err(&format!("{}", e));
                        },
                        ServerEvents::ConnectionError(uuid, e) => if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::ConnectionError(
                            tools::logger.err(&format!("ConnectionError {:?}: {:?}", uuid, e)),
                        ))) {
                            tools::logger.err(&format!("{}", e));
                        },
                        ServerEvents::ServerError(e) => {
                            if let Err(e) = producer_events.send(ProducerEvents::Error(ProducerError::ConnectionError(
                                tools::logger.err(&format!("ServerError {:?}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                            break;
                        },
                        ServerEvents::Shutdown => {
                            // TODO
                        }
                    }
                }
            } => {
                tools::logger.debug("[task: server listener]:: natural finish");
            },
            _ = rx_streams_task_sd => {
                tools::logger.debug("[task: server listener]:: no more server events");
            },
            _ = rx_shutdown => {
                tools::logger.debug("[task: server listener]:: shutdown called");
            }
        };
        tools::logger.debug("[task: server listener]:: finished");
        Ok(())
    })
}

fn spawn_producer_events_holder(
    mut rx_producer_events: UnboundedReceiver<ProducerEvents>,
    rx_shutdown: Receiver<()>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.debug("[task: producer events holder]:: started");
        select! {
            _ = async {
                while let Some(event) = rx_producer_events.recv().await {
                    match event {
                        ProducerEvents::ConsumerConnected(uuid) => ProducerEventsHolder::ConsumerConnected(uuid),
                        ProducerEvents::ConsumerDisconnected(uuid) => ProducerEventsHolder::ConsumerDisconnected(uuid),
                        ProducerEvents::ServerReady => ProducerEventsHolder::ServerReady(),
                        ProducerEvents::Shutdown => ProducerEventsHolder::Shutdown(),
                        ProducerEvents::Error(e) => ProducerEventsHolder::Error(e),
                    }
                };
            } => {
                tools::logger.debug("[task: producer events holder]:: natural finish");
            },
            _ = rx_shutdown => {
                tools::logger.debug("[task: producer events holder]:: shutdown called");
            }
        }
        tools::logger.debug("[task: producer events holder]:: finished");
        Ok(())
    })
}

#[allow(non_snake_case)]
fn spawn_consumers<
    UCX: 'static + Sync + Send + Clone,
>(
    tx_consumers: UnboundedSender<ConsumersChannel>,
    mut rx_consumers: UnboundedReceiver<ConsumersChannel>,
    tx_server_control: UnboundedSender<ServerControl>,
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_producer_events: UnboundedSender<ProducerEvents>,
    ucx: UCX,
    rx_shutdown: Receiver<()>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.verb("[task: consumers]:: started");
        let store: Arc<RwLock<HashMap<Uuid, Consumer>>> = Arc::new(RwLock::new(HashMap::new()));
[[requests_definitions]]
        let arc_connected = Arc::new(RwLock::new(default_connected_event::ObserverEvent::new()));
        let arc_disconnected = Arc::new(RwLock::new(default_disconnected_event::ObserverEvent::new()));
        select! {
            _ = async {
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
                                if let Err(e) = tx_producer_events.send(ProducerEvents::ConsumerConnected(uuid)) {
                                    tools::logger.err(&format!("{}", e));
                                }
                                match arc_connected.write() {
                                    Ok(connected) => {
                                        connected.emit(
                                            uuid,
                                            ucx.clone(),
                                            &broadcast,
                                        );
                                    }
                                    Err(e) => if let Err(e) = tx_producer_events.send(
                                        ProducerEvents::Error(ProducerError::InternalError(
                                            format!("Fail to access to connected event handler due error: {}", e).to_owned()
                                        ))
                                    ) {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                            }
                            Err(e) => if let Err(e) = tx_producer_events.send(
                                ProducerEvents::Error(ProducerError::InternalError(
                                    format!("ConsumersChannel::Add: Fail to access to consumers due error: {}", e)
                                ))
                            ) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        ConsumersChannel::Remove(uuid) => match store.write() {
                            Ok(mut store) => {
                                store.remove(&uuid);
                                if let Err(e) = tx_producer_events.send(ProducerEvents::ConsumerDisconnected(uuid)) {
                                    tools::logger.err(&format!("{}", e));
                                }
                                tools::logger.debug(&format!("Consumer uuid: {} disconnected and destroyed", uuid));
                                match arc_disconnected.write() {
                                    Ok(disconnected) => {
                                        disconnected.emit(
                                            uuid,
                                            ucx.clone(),
                                            &broadcast,
                                        );
                                    }
                                    Err(e) => if let Err(e) = tx_producer_events.send(
                                        ProducerEvents::Error(ProducerError::InternalError(
                                            format!("Fail to access to connected event handler due error: {}", e).to_owned()
                                        ))
                                    ) {
                                        tools::logger.err(&format!("{}", e));
                                    }
                                }
                            },
                            Err(e) => if let Err(e) = tx_producer_events.send(
                                ProducerEvents::Error(ProducerError::InternalError(
                                    format!("ConsumersChannel::Remove: Fail to access to consumers due error: {}", e)
                                ))
                            ) {
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
                            Err(e) => if let Err(e) = tx_producer_events.send(
                                ProducerEvents::Error(ProducerError::InternalError(
                                    format!("ConsumersChannel::SendByFilter: Fail to access to consumers due error: {}", e)
                                ))
                            ) {
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
                            Err(e) => if let Err(e) = tx_producer_events.send(
                                ProducerEvents::Error(ProducerError::InternalError(
                                    format!("ConsumersChannel::SendTo: Fail to access to consumers due error: {}", e)
                                ))
                            ) {
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
                            Err(e) => if let Err(e) = tx_producer_events.send(
                                ProducerEvents::Error(ProducerError::InternalError(
                                    format!("ConsumersChannel::Assign: Fail to access to consumers due error: {}", e)
                                ))
                            ) {
                                tools::logger.err(&format!("{}", e));
                            },
                        },
                        ConsumersChannel::Chunk((uuid, buffer)) => match store.write() {
                            Ok(mut consumers) => {
                                tools::logger.debug(&format!("New message has been received; uuid: {}; length: {}", uuid, buffer.len()));
                                if let Some(consumer) = consumers.get_mut(&uuid) {
                                    if let Err(e) = consumer.chunk(&buffer) {
                                        if let Err(e) = tx_producer_events.send(ProducerEvents::Error(ProducerError::Reading(
                                            tools::logger.err(&format!("Fail to read connection buffer due error: {}", e))
                                        ))) {
                                            tools::logger.err(&format!("{}", e));
                                        }
                                    }
                                    while let Some((message, header)) = consumer.next() {
                                        match message {
                                            [[indentification_self_enum_ref]] => {
                                                let uuid = consumer.key(request, true);
                                                tools::logger.debug(&format!("{}:: identification is done", uuid));
                                                if let Err(e) = match ([[indentification_self_response]] { uuid: uuid.clone() }).pack(header.sequence, Some(uuid.to_string())) {
                                                    Ok(buffer) => if let Err(e) = consumer.send(buffer) {
                                                        Err(e)
                                                    } else {
                                                        tools::logger.debug(&format!("{}:: identification response has been sent", uuid));
                                                        Ok(())
                                                    },
                                                    Err(e) => Err(e),
                                                } {
                                                    if let Err(e) = tx_producer_events.send(
                                                        ProducerEvents::Error(ProducerError::ConnectionError(
                                                            format!("Fail to response for Identification due error: {:?}", e).to_owned()
                                                        ))
                                                    ) {
                                                        tools::logger.err(&format!("{}", e));
                                                    }
                                                }
                                            },
                                            message => if !consumer.assigned() {
                                                if let Err(e) = tx_producer_events.send(
                                                    ProducerEvents::Error(ProducerError::NotAssignedConsumer(
                                                        tools::logger.err(&format!("Consumer ({}) didn't apply Identification", consumer.get_uuid()).to_owned())
                                                    ))
                                                ) {
                                                    tools::logger.err(&format!("{}", e));
                                                }
                                                // TODO: Consumer should be disconnected or some tx_producer_events should be to consumer
                                                // it might be some option of producer like NonAssignedStratagy
                                            } else {
                                                match message {
[[requests_emitters]]
                                                    _ => {
                                                    },
                                                }
                                            },
                                        }
                                    }
                                } else {
                                    tools::logger.err(&format!("Fail to find consumer uuid: {}", uuid));
                                }
                            },
                            Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::Error(ProducerError::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Chunk: Fail to access to consumers due error: {}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                        },
                        ConsumersChannel::Disconnect(filter) => match store.read() {
                            Ok(consumers) => {
                                let mut errors: Vec<String> = vec![];
                                for (_uuid, consumer) in consumers.iter() {
                                    if consumer.is_filtered(filter.clone()) {
                                        if let Err(err) = tx_server_control.send(ServerControl::Disconnect(consumer.get_uuid())) {
                                            errors.push(format!("Fail to Disconnect {}, due error: {}", consumer.get_uuid(), err));
                                        }
                                    }
                                }
                                if !errors.is_empty() {
                                    tools::logger.err(&errors.join("\n"));
                                }
                            },
                            Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::Error(ProducerError::InternalError(
                                tools::logger.err(&format!("ConsumersChannel::Disconnect: Fail to access to consumers due error: {}", e)),
                            ))) {
                                tools::logger.err(&format!("{}", e));
                            }
                        }
                    }
                }
            } => {
                tools::logger.debug("[task: consumers]:: natural finish");
            },
            _ = rx_shutdown => {
                tools::logger.debug("[task: consumers]:: shutdown called");
            },
        }
        tools::logger.verb("[task: consumers]:: finished");
        Ok(())
    })
}

#[allow(non_snake_case)]
fn spawn_events<
    UCX: 'static + Sync + Send + Clone,
>(
    ucx: UCX,
    control: Control,
[[events_receiver_declaration]],
    rx_shutdown: Receiver<()>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.debug("[task: events]:: started");
        join!(
[[events_tasks]],
            rx_shutdown,
        );
        tools::logger.debug("[task: events]:: finished");
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
        Err(String::from("Handler for event conntected isn't implemented."))
    }

    fn disconnected(uuid: Uuid, ucx: UCX) -> Result<EventDisconnectedBroadcasting, String> {
        Err(String::from("Handler for event conntected isn't implemented."))
    }

}

pub struct Producer {

}

impl<UCX: 'static + Sync + Send + Clone> ProducerTrait<UCX> for Producer {

}    
"#;
    pub const REQUEST_DEC: &str = 
r#"#[path = "./observers/[[filename]]"]
pub mod [[module_name]];"#;
    pub const EVENT_DEC: &str =
r#"#[path = "[[filename]]"]
pub mod [[module_name]];"#;
    pub const EVENT_STRUCT_DEC: &str =
"pub [[module_name]]: UnboundedSender<[[module_name]]::Event>";
    pub const EVENT_ARG_DEC: &str =
"[[module_name]]: UnboundedSender<[[module_name]]::Event>";
    pub const EVENT_CHANNEL: &str =
r#"let (tx_[[module_name]], rx_[[module_name]]): (
    UnboundedSender<[[module_name]]::Event>,
    UnboundedReceiver<[[module_name]]::Event>,
) = unbounded_channel();"#;
    pub const EVENT_RECEIVER_DEC: &str =
"rx_[[module_name]]: UnboundedReceiver<[[module_name]]::Event>";
    pub const EVENT_TASK: &str = 
"[[module_name]]::ObserverEvent::listen(ucx.clone(), control.clone(), rx_[[module_name]])";
    pub const REQUEST_DEF: &str = 
"let arc_[[module_name]] = Arc::new(RwLock::new([[module_name]]::ObserverRequest::new()));";
    pub const REQUEST_EMITTER: &str =
r#"[[message_enum_ref]] => {
    tools::logger.debug(&format!("[[debug_ref]] {:?}", request));
    match arc_[[emitter]].write() {
        Ok([[emitter]]) => {
            if let Err(e) = [[emitter]].emit(
                consumer.get_cx(),
                ucx.clone(),
                header.sequence,
                request,
                &broadcast,
            ) {
                if let Err(e) = tx_producer_events.send(
                    ProducerEvents::Error(
                        ProducerError::EmitError(format!("Fail to emit [[debug_ref]] due error: {:?}", e).to_owned())
                    )
                ) {
                    tools::logger.err(&format!("{}", e));
                }
            }
        }
        Err(e) => if let Err(e) = tx_producer_events.send(
            ProducerEvents::Error(ProducerError::InternalError(
                format!("Fail to access to [[debug_ref]] due error: {}", e).to_owned()
            ))
        ) {
            tools::logger.err(&format!("{}", e));
        }
    }
},"#;
}

pub struct RenderProducer {
}

impl Default for RenderProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderProducer {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        _protocol: &Protocol,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[requests_declaration]]", &self.requests_declarations(store)?);
        output = output.replace("[[requests_definitions]]", &tools::inject_tabs(2, self.requests_definitions(store)?));
        output = output.replace("[[requests_emitters]]", &tools::inject_tabs(13, self.requests_emitters(store)?));
        output = output.replace("[[indentification_self_enum_ref]]", &self.indentification_self_enum_ref(store)?);
        output = output.replace("[[indentification_self_response]]", &format!("Protocol::{}", self.into_rust_path(&store.get_config()?.self_key_response)));
        output = output.replace("[[indentification_assigned_key]]", &format!("Protocol::{}", self.into_rust_path(&store.get_config()?.get_assigned()?)));
        output = output.replace("[[events_declaration]]", &self.events_declarations(store)?);
        output = output.replace("[[events_struct_declaration]]", &tools::inject_tabs(1, self.events_struct_declarations(store)?));
        output = output.replace("[[events_struct_args]]", &tools::inject_tabs(2, self.events_struct_args(store)?));
        output = output.replace("[[events_struct_props]]", &tools::inject_tabs(3, self.events_struct_props(store)?));
        output = output.replace("[[events_channels]]", &tools::inject_tabs(1, self.events_channels(store)?));
        output = output.replace("[[events_senders]]", &tools::inject_tabs(2, self.events_senders(store)?));
        output = output.replace("[[events_receivers]]", &tools::inject_tabs(2, self.events_receivers(store)?));
        output = output.replace("[[events_receiver_declaration]]", &tools::inject_tabs(1, self.events_receiver_declaration(store)?));
        output = output.replace("[[events_tasks]]", &tools::inject_tabs(3, self.events_tasks(store)?));
        helpers::fs::write(dest, output, true)
    }

    fn requests_declarations(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::REQUEST_DEC
                    .replace("[[filename]]", &request.as_filename()?)
                    .replace("[[module_name]]", &request.as_mod_name()?)
            );
            if pos < store.requests.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn requests_definitions(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::REQUEST_DEF
                    .replace("[[module_name]]", &request.as_mod_name()?)
            );
            if pos < store.requests.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn requests_emitters(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            let mut emitter: String = String::from(templates::REQUEST_EMITTER);
            let parts: Vec<String> = request.get_request()?.split('.').collect::<Vec<&str>>().iter().map(|v| String::from(*v)).collect();
            let enum_ref: String = if parts.len() == 1 {
                format!("Protocol::AvailableMessages::{}(Protocol::{}(request))", parts[0], parts[0])
            } else {
                //Protocol::AvailableMessages::UserLogin(Protocol::UserLogin::AvailableMessages::Request(Protocol::UserLogin::Request::AvailableMessages::Request(request))) => {
                //Protocol::AvailableMessages::UserLogin(Protocol::UserLogin::AvailableMessages::Request(request))
                let mut chain: String = String::from("");
                for (pos, part) in parts.iter().enumerate() {
                    let mut step: String = String::from("Protocol");
                    for n in 0..pos {
                        step = format!("{}::{}", step, parts[n]);
                    }
                    step = format!("{}::AvailableMessages::{}(", step, part);
                    chain = format!("{}{}", chain, step);
                }
                format!("{}request{}", chain, ")".repeat(parts.len()))
            };
            emitter = emitter.replace("[[emitter]]", &request.as_mod_name()?);
            emitter = emitter.replace("[[message_enum_ref]]", &enum_ref);
            emitter = emitter.replace("[[debug_ref]]", &request.get_request()?);
            output = format!("{}{}",
                output,
                emitter,
            );
            if pos < store.requests.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn indentification_self_enum_ref(&self, store: &WorkflowStore) -> Result<String, String> {
        let parts: Vec<String> = store.get_config()?.get_self()?.split('.').collect::<Vec<&str>>().iter().map(|v| String::from(*v)).collect();
        if parts.len() == 1 {
            Ok(format!("Protocol::AvailableMessages::{}(Protocol::{}(request))", parts[0], parts[0]))
        } else {
            let mut chain: String = String::from("");
            for (pos, part) in parts.iter().enumerate() {
                let mut step: String = String::from("Protocol");
                for n in 0..pos {
                    step = format!("{}::{}", step, parts[n]);
                }
                step = format!("{}::AvailableMessages::{}(", step, part);
                chain = format!("{}{}", chain, step);
            }
            Ok(format!("{}request{}", chain, ")".repeat(parts.len())))
        }
    }

    fn events_declarations(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_DEC
                    .replace("[[filename]]", &event.as_filename()?)
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_struct_declarations(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_STRUCT_DEC 
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_struct_args(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_ARG_DEC 
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_struct_props(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                event.as_mod_name()?,
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_channels(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_CHANNEL 
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_senders(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}tx_{}",
                output,
                event.as_mod_name()?,
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events_receivers(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}rx_{}",
                output,
                event.as_mod_name()?,
            );
            if pos < store.events.len() - 1 {
                output = format!("{},\n", output);
            }
        }
        Ok(output)
    }

    fn events_receiver_declaration(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_RECEIVER_DEC 
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{},\n", output);
            }
        }
        Ok(output)
    }

    fn events_tasks(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_TASK 
                    .replace("[[module_name]]", &event.as_mod_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{},\n", output);
            }
        }
        Ok(output)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        if !base.exists() {
            if let Err(e) = fs::create_dir(&base) {
                return Err(format!("Fail to create dest folder {}. Error: {}", base.to_string_lossy(), e));
            }
        }
        Ok(base.join("producer.rs"))
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }



}

