#[path = "../beacons/mod.rs"]
pub mod beacons;
#[path = "./beacons/mod.rs"]
pub mod beacons_callers;
pub mod consumer;
pub mod emitters;
#[path = "../events/mod.rs"]
pub mod events;
pub mod handlers;
pub mod hub;
pub mod protocol;
#[path = "../responses/mod.rs"]
pub mod responses;

use super::context;
use clibri::{env::logs, server};
use consumer::{identification, Consumer};
use context::Context;
use hub::Hub;
use log::{debug, error, trace, warn};
use protocol::PackingStruct;
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    join, select,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub mod hash {
    pub const PROTOCOL: &str = "2FE9D6137375F6B74B81143B6CA65EEAE6124B6C03C78937C4583DF0B0EF757A";
    pub const WORKFLOW: &str = "94B50A767677D225DE6FDAA072CC1D25F951F69FBC8F42D8D63BBBD2678AF3A8";
}

#[derive(Error, Debug)]
pub enum ProducerError<E: server::Error> {
    #[error("server error: `{0}`")]
    ServerError(E),
    #[error("consumer connection error: `{0}`")]
    ConnectionError(String),
    #[error("consumer error: `{0}`")]
    ConsumerError(String),
    #[error("Not confirmed connection")]
    NotConfirmedConnection,
    #[error("event emitter error: `{0}`")]
    EventEmitterError(emitters::EmitterError),
    #[error("beacon emitter error: `{0}`")]
    BeaconEmitterError(beacons_callers::EmitterError),
    #[error("responsing error: `{0}`")]
    ResponsingError(String),
    #[error("channel access error: `{0}`")]
    ChannelError(String),
    #[error("consumer doesn't have a key; uuid: `{0}`")]
    NoConsumerKey(Uuid),
    #[error("consumer wasn't assigned; uuid: `{0}`")]
    NoAssignedConsumer(Uuid),
    #[error("fail to add consumer into storage; uuid: `{0}`")]
    FailToAddConsumer(Uuid),
    #[error("protocol error: {1}; uuid: `{0}`")]
    Protocol(Uuid, String),
}
use std::marker::PhantomData;

#[allow(dead_code)]
pub mod producer {
    use super::*;

    enum Event {
        EventA(protocol::EventA),
        EventB(protocol::EventB),
        EventsEventA(protocol::Events::EventA),
        EventsEventB(protocol::Events::EventB),
        EventsSubEventA(protocol::Events::Sub::EventA),
        TriggerBeaconsEmitter(protocol::TriggerBeaconsEmitter),
        FinishConsumerTest(protocol::FinishConsumerTest),
    }

    enum MergedChannel<E: server::Error> {
        ServerEvents(server::Events<E>),
        Event(Event),
        Filter(hub::filter::Request),
    }

    #[derive(Clone, Debug)]
    pub struct Events {
        tx_events: UnboundedSender<Event>,
    }

    impl Events {
        pub async fn eventa(&self, event: protocol::EventA) -> Result<(), String> {
            self.tx_events
                .send(Event::EventA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn eventb(&self, event: protocol::EventB) -> Result<(), String> {
            self.tx_events
                .send(Event::EventB(event))
                .map_err(|e| e.to_string())
        }
        pub async fn events_eventa(&self, event: protocol::Events::EventA) -> Result<(), String> {
            self.tx_events
                .send(Event::EventsEventA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn events_eventb(&self, event: protocol::Events::EventB) -> Result<(), String> {
            self.tx_events
                .send(Event::EventsEventB(event))
                .map_err(|e| e.to_string())
        }
        pub async fn events_sub_eventa(
            &self,
            event: protocol::Events::Sub::EventA,
        ) -> Result<(), String> {
            self.tx_events
                .send(Event::EventsSubEventA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn triggerbeaconsemitter(
            &self,
            event: protocol::TriggerBeaconsEmitter,
        ) -> Result<(), String> {
            self.tx_events
                .send(Event::TriggerBeaconsEmitter(event))
                .map_err(|e| e.to_string())
        }
        pub async fn finishconsumertest(
            &self,
            event: protocol::FinishConsumerTest,
        ) -> Result<(), String> {
            self.tx_events
                .send(Event::FinishConsumerTest(event))
                .map_err(|e| e.to_string())
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ProducerIdentificationStrategy {
        // Put warning into logs
        Log,
        // Emit error (would not stop producer)
        EmitError,
        // Disconnect consumer
        Disconnect,
        // Disconnect consumer and emit error
        EmitErrorAndDisconnect,
        // Ignore if consumer doesn't have producer identification
        Ignore,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ConsumerErrorHandelingStrategy {
        // Emit error (would not stop producer)
        EmitError,
        // Disconnect consumer
        Disconnect,
        // Disconnect consumer and emit error
        EmitErrorAndDisconnect,
        // Put warning into logs
        Log,
    }

    #[derive(Debug, Clone)]
    pub struct Options {
        pub producer_indentification_strategy: ProducerIdentificationStrategy,
        pub consumer_error_handeling_strategy: ConsumerErrorHandelingStrategy,
        pub hub_len: usize,
    }

    impl Default for Options {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Options {
        pub fn new() -> Self {
            Options {
                producer_indentification_strategy: ProducerIdentificationStrategy::Log,
                consumer_error_handeling_strategy:
                    ConsumerErrorHandelingStrategy::EmitErrorAndDisconnect,
                hub_len: 1000,
            }
        }
        pub fn producer_indentification_strategy(
            &mut self,
            value: ProducerIdentificationStrategy,
        ) -> &mut Self {
            self.producer_indentification_strategy = value;
            self
        }
        pub fn consumer_error_handeling_strategy(
            &mut self,
            value: ConsumerErrorHandelingStrategy,
        ) -> &mut Self {
            self.consumer_error_handeling_strategy = value;
            self
        }
    }

    #[derive(Clone, Debug)]
    pub struct Control<E, C>
    where
        E: server::Error,
        C: server::Control<E> + Send + Clone,
    {
        server_control: C,
        shutdown: CancellationToken,
        tx_shutdown: Sender<oneshot::Sender<()>>,
        phantom: PhantomData<E>,
        pub events: Events,
    }

    impl<E, C> Control<E, C>
    where
        E: server::Error,
        C: server::Control<E> + Send + Clone,
    {
        pub async fn shutdown(&self) -> Result<(), ProducerError<E>> {
            let (tx_resolver, rx_resolver): (oneshot::Sender<()>, oneshot::Receiver<()>) =
                oneshot::channel();
            self.tx_shutdown.send(tx_resolver).await.map_err(|_| {
                ProducerError::ChannelError(String::from("Fail to send shutdown command"))
            })?;
            rx_resolver
                .await
                .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
            Ok(())
        }

        pub fn get_shutdown_token(&self) -> CancellationToken {
            self.shutdown.clone()
        }

        pub async fn disconnect(&self, uuid: Uuid) -> Result<(), ProducerError<E>> {
            self.server_control
                .disconnect(uuid)
                .await
                .map_err(ProducerError::ServerError)
        }

        pub async fn send(
            &self,
            buffer: Vec<u8>,
            uuid: Option<Uuid>,
        ) -> Result<(), ProducerError<E>> {
            self.server_control
                .send(buffer, uuid)
                .await
                .map_err(ProducerError::ServerError)
        }

        pub async fn broadcast(
            &self,
            uuids: Vec<Uuid>,
            buffer: Vec<u8>,
        ) -> Result<(), ProducerError<E>> {
            for uuid in uuids.iter() {
                if let Err(err) = self.server_control.send(buffer.clone(), Some(*uuid)).await {
                    warn!(
                        target: logs::targets::PRODUCER,
                        "fail to send data to consumer {}: {}", uuid, err
                    );
                }
            }
            Ok(())
        }
    }

    async fn listener<E: server::Error, C: server::Control<E> + 'static + Send + Sync>(
        context: Context,
        mut rx_server_events: UnboundedReceiver<server::Events<E>>,
        mut rx_events: UnboundedReceiver<Event>,
        control: Control<E, C>,
        options: &Options,
    ) -> (Control<E, C>, Context, Option<ProducerError<E>>) {
        let (tx_filter, mut rx_filter): (
            UnboundedSender<hub::filter::Request>,
            UnboundedReceiver<hub::filter::Request>,
        ) = unbounded_channel();
        let mut hubs: HashMap<Uuid, Hub<E, C>> = HashMap::new();
        let mut map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let cancel = control.shutdown.child_token();
        let result = async {
            while let Some(event) = select! {
                mut msg = rx_server_events.recv() => {
                    msg.take().map(MergedChannel::ServerEvents)
                },
                mut msg = rx_events.recv() => {
                    msg.take().map(MergedChannel::Event)
                },
                mut msg = rx_filter.recv() => {
                    msg.take().map(MergedChannel::Filter)
                }
                _ = cancel.cancelled() => None
            } {
                match event {
                    MergedChannel::ServerEvents(event) => {
                        let redirection: Option<(Uuid, server::Events<E>, bool)> = match event {
                            server::Events::Ready => {
                                emitters::ready::emit(&context, &control)
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?;
                                None
                            }
                            server::Events::Shutdown => {
                                debug!(
                                    target: logs::targets::PRODUCER,
                                    "server down event has been received"
                                );
                                break;
                            }
                            server::Events::ServerError(err) => {
                                emitters::error::emit::<E, C>(
                                    ProducerError::ServerError(err),
                                    None,
                                    &context,
                                    None,
                                    &control,
                                )
                                .await
                                .map_err(ProducerError::EventEmitterError)?;
                                None
                            }
                            server::Events::Connected(uuid) => {
                                let mut vacant: Option<&Hub<E, C>> = None;
                                let mut prev = options.hub_len;
                                for (hub_uuid, hub) in &hubs {
                                    let len = hub.len().await?;
                                    if len < options.hub_len && len < prev {
                                        vacant = Some(hub);
                                        prev = len;
                                    }
                                }
                                if let Some(hub) = vacant {
                                    hub.income(server::Events::Connected(uuid))?;
                                    if let Some(map) = map.get_mut(&hub.uuid()) {
                                        map.push(uuid);
                                    }
                                } else {
                                    let hub =
                                        Hub::new(control.clone(), tx_filter.clone(), context.clone(), options.clone());
                                    map.insert(hub.uuid(), vec![uuid]);
                                    hub.income(server::Events::Connected(uuid))?;
                                    hubs.insert(hub.uuid(), hub);
                                }
                                None
                            }
                            server::Events::Disconnected(uuid) => {
                                let mut removed: bool = false;
                                for (hub_uuid, uuids) in map.iter_mut() {
                                    if let Some(pos) = uuids.iter().position(|u| u == &uuid) {
                                        removed = true;
                                        uuids.remove(pos);
                                        break;
                                    }
                                }
                                if !removed {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "Fail to removed connection {}; it wasn't found", uuid
                                    );
                                    emitters::error::emit::<E, C>(
                                        ProducerError::ConnectionError(format!(
                                            "Fail to removed connection {}; it wasn't found", uuid
                                        )),
                                        Some(uuid),
                                        &context,
                                        None,
                                        &control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?;
                                }
                                Some((uuid, server::Events::Disconnected(uuid), true))
                            }
                            server::Events::Received(uuid, buffer) => {
                                Some((uuid, server::Events::Received(uuid, buffer), false))
                            }
                            server::Events::Error(uuid, err) => {
                                if let Some(uuid) = uuid {
                                    Some((uuid, server::Events::Error(Some(uuid), err), false))
                                } else {
                                    emitters::error::emit::<E, C>(
                                        ProducerError::ConsumerError(err),
                                        uuid,
                                        &context,
                                        None,
                                        &control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?;
                                    None
                                }
                            }
                            server::Events::ConnectionError(uuid, err) => {
                                if let Some(uuid) = uuid {
                                    Some((uuid, server::Events::ConnectionError(Some(uuid), err), false))
                                } else {
                                    emitters::error::emit::<E, C>(
                                        ProducerError::ConnectionError(err.to_string()),
                                        uuid,
                                        &context,
                                        None,
                                        &control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?;
                                    None
                                }
                            }
                        };
                        if let Some((uuid, msg, disconnection)) = redirection {
                            let mut redirected: bool = false;
                            for (hub_uuid, consumers) in &map {
                                if consumers.iter().any(|u| u == &uuid) {
                                    hubs[hub_uuid].income(msg)?;
                                    redirected = true;
                                    break;
                                }
                            }
                            if !redirected && !disconnection {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "Fail to find connected {}; server message wouldn't be redirected.", uuid
                                );
                                emitters::error::emit::<E, C>(
                                    ProducerError::ConnectionError(format!(
                                        "Fail to find connected {}",
                                        uuid
                                    )),
                                    Some(uuid),
                                    &context,
                                    None,
                                    &control,
                                )
                                .await
                                .map_err(ProducerError::EventEmitterError)?;
                            }
                        }
                    }
                    MergedChannel::Filter(request) => {
                        let mut filtered: Vec<Uuid> = vec![];
                        match request {
                            hub::filter::Request::All(tx_response) => {
                                for (hub_uuid, uuids) in &map {
                                    filtered = [filtered, uuids.to_vec()].concat();
                                }
                                if tx_response.send(filtered).is_err() {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "Fail to response to Request::All; ", 
                                    );
                                }
                            },
                            hub::filter::Request::Except(uuids, tx_response) => {
                                for (hub_uuid, uuids) in &map {
                                    filtered = [filtered, uuids.to_vec()].concat();
                                }
                                if tx_response.send(filtered.iter().filter(|u| {
                                    !uuids.iter().any(|uuid| *u == uuid)
                                }).cloned().collect()).is_err() {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "Fail to response to Request::Except;",
                                    );
                                }
                            }
                            hub::filter::Request::Filter(cb, tx_response) => {}
                        }
                    }
                    MergedChannel::Event(event) => {
                        let filter = hub::filter::Filter::new(tx_filter.clone());
                        match event {
                            Event::EventA(event) => {
                                if let Err(err) = emitters::eventa::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call eventa handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::EventB(event) => {
                                if let Err(err) = emitters::eventb::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call eventb handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::EventsEventA(event) => {
                                if let Err(err) = emitters::events_eventa::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call events_eventa handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::EventsEventB(event) => {
                                if let Err(err) = emitters::events_eventb::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call events_eventb handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::EventsSubEventA(event) => {
                                if let Err(err) = emitters::events_sub_eventa::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call events_sub_eventa handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::TriggerBeaconsEmitter(event) => {
                                if let Err(err) = emitters::triggerbeaconsemitter::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call triggerbeaconsemitter handler; error: {:?}", err,
                                    );
                                }
                            }
                            Event::FinishConsumerTest(event) => {
                                if let Err(err) = emitters::finishconsumertest::emit::<E, C>(
                                    event, filter, &context, &control,
                                )
                                .await
                                {
                                    warn!(
                                        target: logs::targets::PRODUCER,
                                        "fail call finishconsumertest handler; error: {:?}", err,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Ok::<(), ProducerError<E>>(())
        }
        .await;
        (
            control,
            context,
            if let Err(err) = result {
                Some(err)
            } else {
                None
            },
        )
    }

    pub async fn run<S, C, E>(
        mut server: S,
        options: Options,
        context: Context,
    ) -> Result<(), ProducerError<E>>
    where
        S: server::Impl<E, C> + 'static,
        E: server::Error,
        C: server::Control<E> + 'static + Send + Sync,
    {
        let (tx_events, rx_events): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
            unbounded_channel();
        let (tx_shutdown, mut rx_shutdown): (
            Sender<oneshot::Sender<()>>,
            Receiver<oneshot::Sender<()>>,
        ) = channel(2);
        let events = Events { tx_events };
        let shutdown = CancellationToken::new();
        let server_control = server.control();
        let control = Control {
            server_control: server.control(),
            shutdown,
            tx_shutdown,
            phantom: PhantomData,
            events,
        };
        let rx_server_events = server.observer().map_err(ProducerError::ServerError)?;
        let cancel = control.shutdown.clone();
        let ((server, server_res), (control, context, listener_err), shutdown_response) = join!(
            async {
                debug!(target: logs::targets::PRODUCER, "server: started");
                let result = select! {
                    res = server.listen() => res.map_err(ProducerError::ServerError),
                    _ = cancel.cancelled() => Ok(())
                };
                debug!(target: logs::targets::PRODUCER, "server: finished");
                cancel.cancel();
                (server, result)
            },
            async {
                debug!(target: logs::targets::PRODUCER, "listener: started");
                let result =
                    listener(context, rx_server_events, rx_events, control, &options).await;
                debug!(target: logs::targets::PRODUCER, "listener: finished");
                cancel.cancel();
                result
            },
            async {
                debug!(
                    target: logs::targets::PRODUCER,
                    "shutdown listener: started"
                );
                let result = select! {
                    tx_response = rx_shutdown.recv() => tx_response,
                    _ = cancel.cancelled() => None,
                };
                debug!(
                    target: logs::targets::PRODUCER,
                    "shutdown listener: finished"
                );
                cancel.cancel();
                result
            }
        );
        if let Some(tx_response) = shutdown_response {
            debug!(
                target: logs::targets::PRODUCER,
                "shutdown has been requested"
            );
            server_control
                .shutdown()
                .await
                .map_err(ProducerError::ServerError)?;
            tx_response.send(()).map_err(|_| {
                ProducerError::ChannelError(String::from("Fail to send shutdown response"))
            })?;
            drop(server);
            debug!(
                target: logs::targets::PRODUCER,
                "shutdown response has been sent"
            );
        }
        emitters::shutdown::emit(&context, &control)
            .await
            .map_err(ProducerError::EventEmitterError)?;
        if server_res.is_err() {
            server_res
        } else if let Some(err) = listener_err {
            Err(err)
        } else {
            Ok(())
        }
    }
}
