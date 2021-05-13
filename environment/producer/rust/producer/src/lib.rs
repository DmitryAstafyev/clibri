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

#[allow(non_snake_case)]
#[path = "./events/event.KickOff.rs"]
pub mod KickOffEvent;


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
    pub tx_event_kickoff: UnboundedSender<KickOffEvent::Event>,
}

impl Events {
    pub fn new(
        tx_event_kickoff: UnboundedSender<KickOffEvent::Event>
    ) -> Self {
        Events {
            tx_event_kickoff,
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

    pub fn assign(&self, uuid: Uuid, assigned: Protocol::Identification::AssignedKey, overwrite: bool) -> Result<(), SendError<ConsumersChannel>> {
        self.consumers.send(ConsumersChannel::Assign((uuid, assigned, overwrite)))
    }

    pub fn send(&self, filter: Filter, buffer: Vec<u8>) -> Result<(), SendError<ConsumersChannel>> {
        self.consumers.send(ConsumersChannel::SendByFilter((filter, buffer)))
    }

    pub fn disconnect(filter: Filter) -> Result<(), String> {
        Ok(())
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
        tx_sender,
        tx_producer_events,
        ucx.clone(),
        rx_consumers_task_sd,
    );
    let (tx_event_kickoff, rx_event_kickoff): (
        UnboundedSender<KickOffEvent::Event>,
        UnboundedReceiver<KickOffEvent::Event>,
    ) = unbounded_channel();
    let events = Events::new(
        tx_event_kickoff
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
        rx_event_kickoff,
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
    tx_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    tx_producer_events: UnboundedSender<ProducerEvents>,
    ucx: UCX,
    rx_shutdown: Receiver<()>,
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
                                match Connected.write() {
                                    Ok(Connected) => {
                                        Connected.emit(
                                            uuid.clone(),
                                            ucx.clone(),
                                            &broadcast,
                                        );
                                    }
                                    Err(e) => if let Err(e) = tx_producer_events.send(
                                        ProducerEvents::Error(ProducerError::InternalError(
                                            format!("Fail to access to Connected event handler due error: {}", e).to_owned()
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
                                match Disconnected.write() {
                                    Ok(Disconnected) => {
                                        Disconnected.emit(
                                            uuid.clone(),
                                            ucx.clone(),
                                            &broadcast,
                                        );
                                    }
                                    Err(e) => if let Err(e) = tx_producer_events.send(
                                        ProducerEvents::Error(ProducerError::InternalError(
                                            format!("Fail to access to Connected event handler due error: {}", e).to_owned()
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
                                                    Protocol::AvailableMessages::UserLogin(Protocol::UserLogin::AvailableMessages::Request(request)) => {
                                                        tools::logger.debug(&format!("Protocol::AvailableMessages::UserLogin::Request {:?}", request));
                                                        match UserLogin.write() {
                                                            Ok(UserLogin) => {
                                                                if let Err(e) = UserLogin.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    header.sequence,
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = tx_producer_events.send(
                                                                        ProducerEvents::Error(
                                                                            ProducerError::EmitError(format!("Fail to emit UserLogin due error: {:?}", e).to_owned())
                                                                        )
                                                                    ) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = tx_producer_events.send(
                                                                ProducerEvents::Error(ProducerError::InternalError(
                                                                    format!("Fail to access to UserLogin due error: {}", e).to_owned()
                                                                ))
                                                            ) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    Protocol::AvailableMessages::Users(Protocol::Users::AvailableMessages::Request(request)) => {
                                                        tools::logger.debug(&format!("Protocol::AvailableMessages::Users::Request {:?}", request));
                                                        match Users.write() {
                                                            Ok(Users) => {
                                                                if let Err(e) = Users.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    header.sequence,
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = tx_producer_events.send(
                                                                        ProducerEvents::Error(ProducerError::EmitError(
                                                                            format!("Fail to emit Protocol::Users::Request due error: {:?}", e).to_owned()
                                                                        ))
                                                                    ) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = tx_producer_events.send(
                                                                ProducerEvents::Error(ProducerError::InternalError(
                                                                    format!("Fail to access to Users due error: {}", e).to_owned()
                                                                ))
                                                            ) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    Protocol::AvailableMessages::Message(Protocol::Message::AvailableMessages::Request(request)) => {
                                                        tools::logger.debug(&format!("Protocol::AvailableMessages::Message::Request {:?}", request));
                                                        match Message.write() {
                                                            Ok(Message) => {
                                                                if let Err(e) = Message.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    header.sequence,
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = tx_producer_events.send(
                                                                        ProducerEvents::Error(
                                                                            ProducerError::EmitError(format!("Fail to emit Message due error: {:?}", e).to_owned())
                                                                        )
                                                                    ) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = tx_producer_events.send(
                                                                ProducerEvents::Error(
                                                                    ProducerError::InternalError(format!("Fail to access to Message due error: {}", e).to_owned())
                                                                )
                                                            ) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
                                                    Protocol::AvailableMessages::Messages(Protocol::Messages::AvailableMessages::Request(request)) => {
                                                        tools::logger.debug(&format!("Protocol::AvailableMessages::Messages::Request {:?}", request));
                                                        match Messages.write() {
                                                            Ok(Messages) => {
                                                                if let Err(e) = Messages.emit(
                                                                    consumer.get_cx(),
                                                                    ucx.clone(),
                                                                    header.sequence,
                                                                    request,
                                                                    &broadcast,
                                                                ) {
                                                                    if let Err(e) = tx_producer_events.send(ProducerEvents::Error(ProducerError::EmitError(format!("Fail to emit Messages due error: {:?}", e).to_owned()))) {
                                                                        tools::logger.err(&format!("{}", e));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => if let Err(e) = tx_producer_events.send(ProducerEvents::Error(ProducerError::InternalError(format!("Fail to access to Messages due error: {}", e).to_owned()))) {
                                                                tools::logger.err(&format!("{}", e));
                                                            }
                                                        }
                                                    },
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
    mut control: Control,
    rx_event_kickoff: UnboundedReceiver<KickOffEvent::Event>,
    rx_shutdown: Receiver<()>,
) -> JoinHandle<Result<(), String>> {
    spawn(async move {
        tools::logger.debug("[task: events]:: started");
        join!(
            KickOffEvent::ObserverEvent::listen(ucx.clone(), control.clone(), rx_event_kickoff),
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
