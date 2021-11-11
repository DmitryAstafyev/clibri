use super::{helpers, helpers::render as tools, workflow::store::Store, Protocol};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"
pub mod consumer;
pub mod emitters;
pub mod handlers;
pub mod protocol;
#[path = "./beacons/mod.rs"]
pub mod beacons_callers;
#[path = "../beacons/mod.rs"]
pub mod beacons;
#[path = "../events/mod.rs"]
pub mod events;
#[path = "../responses/mod.rs"]
pub mod responses;

use super::context;
use consumer::{identification, Consumer};
use context::Context;
use fiber::{env::logs, server};
use log::{debug, error, trace, warn};
use protocol::PackingStruct;
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    join, select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub mod hash {
    pub const PROTOCOL: &str = "[[protocol_hash]]";
    pub const WORKFLOW: &str = "[[workflow_hash]]";
}

#[derive(Error, Debug)]
pub enum ProducerError<E: std::error::Error> {
    #[error("server error: `{0}`")]
    ServerError(E),
    #[error("consumer connection error: `{0}`")]
    ConnectionError(String),
    #[error("consumer error: `{0}`")]
    ConsumerError(String),
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
}
use std::marker::PhantomData;

#[allow(dead_code)]
pub mod producer {
    use super::*;

    pub enum ManageChannel {
        Shutdown(oneshot::Sender<()>),
    }

    enum UnboundedEventsList {
[[events_list]]
    }

    enum MergedChannel<E: std::error::Error> {
        ServerEvents(server::Events<E>),
        UnboundedEventsList(UnboundedEventsList),
    }

    #[derive(Clone, Debug)]
    pub struct UnboundedEvents {
        tx_unbounded_events: UnboundedSender<UnboundedEventsList>,
    }

    impl UnboundedEvents {
[[events_callers]]
    }

    pub struct Manage {
        tx_manage_channel: UnboundedSender<ManageChannel>,
        shutdown_tracker_token: CancellationToken,
        pub events: UnboundedEvents,
    }

    impl Manage {
        pub async fn shutdown<E: std::error::Error>(&self) -> Result<(), ProducerError<E>> {
            let (tx_resolver, rx_resolver): (oneshot::Sender<()>, oneshot::Receiver<()>) =
                oneshot::channel();
            self.tx_manage_channel
                .send(ManageChannel::Shutdown(tx_resolver))
                .map_err(|_| {
                    ProducerError::ChannelError(String::from("Fail to send shutdown command"))
                })?;
            rx_resolver
                .await
                .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
            Ok(())
        }

        pub fn is_down(&self) -> bool {
            self.shutdown_tracker_token.is_cancelled()
        }

        pub fn get_shutdown_tracker(&self) -> CancellationToken {
            self.shutdown_tracker_token.clone()
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
        E: std::error::Error,
        C: server::Control<E> + Send + Clone,
    {
        server_control: C,
        shutdown: CancellationToken,
        phantom: PhantomData<E>,
    }

    impl<E, C> Control<E, C>
    where
        E: std::error::Error,
        C: server::Control<E> + Send + Clone,
    {
        pub async fn shutdown(&self) -> Result<(), ProducerError<E>> {
            self.server_control
                .shutdown()
                .await
                .map_err(ProducerError::ServerError)?;
            self.shutdown.cancel();
            Ok(())
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

    async fn add_connection<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control<E, C>,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "new client connection: {}", uuid,
        );
        if consumers.contains_key(&uuid) {
            return Err(ProducerError::FailToAddConsumer(uuid));
        }
        let filter = identification::Filter::new(consumers).await;
        let mut client = Consumer::new(uuid, options);
        debug!(
            target: logs::targets::PRODUCER,
            "new connection accepted: {}", uuid,
        );
        if let Err(err) = emitters::connected::emit::<E, C>(
            client.get_mut_identification(),
            &filter,
            context,
            control,
        )
        .await
        {
            warn!(
                target: logs::targets::PRODUCER,
                "fail call connected handler for {}; error: {:?}", uuid, err,
            );
        }
        consumers.insert(uuid, client);
        Ok(())
    }

    async fn remove_connection<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "client disconnected: {}", uuid,
        );
        let filter = identification::Filter::new(consumers).await;
        if let Some(mut client) = consumers.remove(&uuid) {
            if let Err(err) = emitters::disconnected::emit::<E, C>(
                client.get_mut_identification(),
                &filter,
                context,
                control,
            )
            .await
            {
                warn!(
                    target: logs::targets::PRODUCER,
                    "fail call connected handler for {}; error: {:?}", uuid, err,
                );
            }
            debug!(
                target: logs::targets::PRODUCER,
                "client {} has been disconnected", uuid,
            );
        } else {
            warn!(
                target: logs::targets::PRODUCER,
                "cannot find a client {}; guess it was disconnected already", uuid,
            );
        }
        Ok(())
    }

    async fn disconnect<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        uuid: Uuid,
        consumer: &mut Consumer,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        if consumer.get_identification().is_discredited() {
            return Ok(());
        }
        consumer.get_identification().discredited();
        control.disconnect(uuid).await?;
        Ok(())
    }

    async fn responsing_err<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        err: String,
        uuid: Uuid,
        mut context: &mut Context,
        control: &Control<E, C>,
        options: &Options,
        consumer: &mut Option<&mut Consumer>,
    ) -> Result<(), ProducerError<E>> {
        if options.consumer_error_handeling_strategy == ConsumerErrorHandelingStrategy::Log {
            warn!(target: logs::targets::PRODUCER, "{}:: {}", uuid, err);
        }
        if options.consumer_error_handeling_strategy == ConsumerErrorHandelingStrategy::Disconnect
            || options.consumer_error_handeling_strategy
                == ConsumerErrorHandelingStrategy::EmitErrorAndDisconnect
        {
            if let Some(consumer) = consumer.as_deref_mut() {
                disconnect(uuid, consumer, control).await?;
            } else {
                error!(
                    target: logs::targets::PRODUCER,
                    "{}:: fail to find consumer to disconnect client", uuid
                );
            }
        }
        if options.consumer_error_handeling_strategy == ConsumerErrorHandelingStrategy::EmitError
            || options.consumer_error_handeling_strategy
                == ConsumerErrorHandelingStrategy::EmitErrorAndDisconnect
        {
            emitters::error::emit::<E, C>(
                ProducerError::ResponsingError(err),
                Some(uuid),
                &mut context,
                consumer
                    .as_deref_mut()
                    .map(|consumer| consumer.get_mut_identification()),
                control,
            )
            .await
            .map_err(ProducerError::EventEmitterError)?;
        }
        Ok(())
    }

    async fn process_received_data<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        uuid: Uuid,
        buffer: Vec<u8>,
        consumers: &mut HashMap<Uuid, Consumer>,
        mut context: &mut Context,
        control: &Control<E, C>,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        trace!(
            target: logs::targets::PRODUCER,
            "new chunk of data from {} has been gotten",
            uuid,
        );
        let filter = identification::Filter::new(consumers).await;
        let mut messages: consumer::ConsumerMessages = vec![];
        let mut assigned: bool = false;
        let mut has_key: bool = false;
        let mut client: Option<&mut Consumer> = if let Some(consumer) = consumers.get_mut(&uuid) {
            if consumer.get_identification().is_discredited() {
                // Consumer is discredited do nothing with it
                None
            } else if let Err(err) = consumer.chunk(&buffer) {
                responsing_err(
                    format!("fail to read chunk of data; error: {}", err),
                    uuid,
                    &mut context,
                    control,
                    options,
                    &mut Some(consumer),
                )
                .await?;
                None
            } else {
                messages = consumer.get_messages();
                assigned = consumer.get_identification().assigned();
                has_key = consumer.get_identification().has_key();
                Some(consumer)
            }
        } else {
            responsing_err(
                String::from("fail to find consumer; message wouldn't be processed"),
                uuid,
                &mut context,
                control,
                options,
                &mut None,
            )
            .await?;
            None
        };
        if messages.is_empty() {
            return Ok(());
        }
        let client = if let Some(client) = client.take() {
            client
        } else {
            return Ok(());
        };
        for (message, header) in messages.iter() {
            match message {
                [[indentification_self_enum_ref]] => {
                    trace!(
                        target: logs::targets::PRODUCER,
                        "consumer {} requested identification",
                        uuid,
                    );
                    let assigned_uuid = client.key(request, true);
                    if let Err(err) = match (protocol::[[indentification_self_response]] {
                        uuid: assigned_uuid.clone(),
                    })
                    .pack(header.sequence, Some(assigned_uuid.clone()))
                    {
                        Ok(buffer) => {
                            if let Err(err) = control.send(buffer, Some(uuid)).await {
                                Err(err.to_string())
                            } else {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "{}:: identification response has been sent", uuid,
                                );
                                Ok(())
                            }
                        }
                        Err(err) => Err(err),
                    } {
                        responsing_err(
                            format!("fail to send identification response: {}", err),
                            uuid,
                            &mut context,
                            control,
                            options,
                            &mut Some(client),
                        )
                        .await?
                    }
                }
                protocol::AvailableMessages::InternalServiceGroup(
                    protocol::InternalServiceGroup::AvailableMessages::HashRequest(request),
                ) => {
                    trace!(
                        target: logs::targets::PRODUCER,
                        "consumer {} requested hash check",
                        uuid,
                    );
                    let valid = if request.protocol != hash::PROTOCOL {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} uses invalid protocol hash ({}); valid protocol hash: {}",
                            uuid,
                            request.protocol,
                            hash::PROTOCOL
                        );
                        false
                    } else if request.workflow != hash::WORKFLOW {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} uses invalid workflow hash ({}); valid workflow hash: {}",
                            uuid,
                            request.workflow,
                            hash::WORKFLOW
                        );
                        false
                    } else {
                        trace!(
                            target: logs::targets::PRODUCER,
                            "consumer {} hash has been accepted",
                            uuid,
                        );
                        client.accept_hash();
                        true
                    };
                    if let Err(err) = match (protocol::InternalServiceGroup::HashResponse {
                        error: if !valid {
                            Some(String::from("Hash is invalid"))
                        } else {
                            None
                        },
                    })
                    .pack(header.sequence, Some(uuid.to_string()))
                    {
                        Ok(buffer) => {
                            if let Err(err) = control.send(buffer, Some(uuid)).await {
                                Err(err.to_string())
                            } else {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "{}:: hash check results has been sent", uuid,
                                );
                                Ok(())
                            }
                        }
                        Err(err) => Err(err),
                    } {
                        responsing_err(
                            format!("fail to send hash check results response: {}", err),
                            uuid,
                            &mut context,
                            control,
                            options,
                            &mut Some(client),
                        )
                        .await?
                    }
                    if !valid {
                        disconnect(uuid, client, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            &mut context,
                            Some(client.get_mut_identification()),
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    }
                },
                message => {
                    if !client.is_hash_accepted() {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but hash of client invalid", uuid
                        );
                        disconnect(uuid, client, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            &mut context,
                            Some(client.get_mut_identification()),
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    } else if !has_key {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but it doesn't have a key", uuid
                        );
                        disconnect(uuid, client, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            &mut context,
                            Some(client.get_mut_identification()),
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    } else if !assigned
                        && options.producer_indentification_strategy
                            != ProducerIdentificationStrategy::Ignore
                    {
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::Log
                        {
                            warn!(
                                target: logs::targets::PRODUCER,
                                "consumer {} tries to send data, but it isn't assigned", uuid
                            );
                        }
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::Disconnect
                            || options.producer_indentification_strategy
                                == ProducerIdentificationStrategy::EmitErrorAndDisconnect
                        {
                            disconnect(uuid, client, control).await?;
                        }
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::EmitError
                            || options.producer_indentification_strategy
                                == ProducerIdentificationStrategy::EmitErrorAndDisconnect
                        {
                            emitters::error::emit::<E, C>(
                                ProducerError::NoAssignedConsumer(uuid),
                                Some(uuid),
                                &mut context,
                                Some(client.get_mut_identification()),
                                control,
                            )
                            .await
                            .map_err(ProducerError::EventEmitterError)?;
                        }
                    } else {
                        match message {
[[requests]]
[[beacons]]
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn listener<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        mut context: Context,
        mut rx_server_events: UnboundedReceiver<server::Events<E>>,
        mut rx_unbounded_events: UnboundedReceiver<UnboundedEventsList>,
        control: Control<E, C>,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "listener of server's events is started"
        );
        let mut consumers: HashMap<Uuid, Consumer> = HashMap::new();
        while let Some(event) = select! {
            mut msg = rx_server_events.recv() => {
                msg.take().map(MergedChannel::ServerEvents)
            },
            mut msg = rx_unbounded_events.recv() => {
                msg.take().map(MergedChannel::UnboundedEventsList)
            }
        } {
            match event {
                MergedChannel::ServerEvents(event) => match event {
                    server::Events::Ready => emitters::ready::emit(&mut context, &control)
                        .await
                        .map_err(ProducerError::EventEmitterError)?,
                    server::Events::Shutdown => emitters::shutdown::emit(&mut context, &control)
                        .await
                        .map_err(ProducerError::EventEmitterError)?,
                    server::Events::Connected(uuid) => {
                        add_connection(uuid, &mut consumers, &mut context, &control, options)
                            .await?
                    }
                    server::Events::Disconnected(uuid) => {
                        remove_connection(uuid, &mut consumers, &mut context, &control).await?
                    }
                    server::Events::Received(uuid, buffer) => {
                        process_received_data(
                            uuid,
                            buffer,
                            &mut consumers,
                            &mut context,
                            &control,
                            options,
                        )
                        .await?
                    }
                    server::Events::Error(uuid, err) => emitters::error::emit::<E, C>(
                        ProducerError::ConsumerError(err),
                        uuid,
                        &mut context,
                        if let Some(uuid) = uuid.as_ref() {
                            consumers
                                .get_mut(uuid)
                                .map(|consumer| consumer.get_mut_identification())
                        } else {
                            None
                        },
                        &control,
                    )
                    .await
                    .map_err(ProducerError::EventEmitterError)?,
                    server::Events::ConnectionError(uuid, err) => emitters::error::emit::<E, C>(
                        ProducerError::ConnectionError(err.to_string()),
                        uuid,
                        &mut context,
                        if let Some(uuid) = uuid.as_ref() {
                            consumers
                                .get_mut(uuid)
                                .map(|consumer| consumer.get_mut_identification())
                        } else {
                            None
                        },
                        &control,
                    )
                    .await
                    .map_err(ProducerError::EventEmitterError)?,
                    server::Events::ServerError(err) => emitters::error::emit::<E, C>(
                        ProducerError::ServerError(err),
                        None,
                        &mut context,
                        None,
                        &control,
                    )
                    .await
                    .map_err(ProducerError::EventEmitterError)?,
                },
                MergedChannel::UnboundedEventsList(event) => {
                    let filter = identification::Filter::new(&consumers).await;
                    match event {
[[events]]
                    }
                }
            }
        }
        debug!(
            target: logs::targets::PRODUCER,
            "listener of server's events is finished"
        );
        Ok(())
    }

    async fn main_task<S, C, E>(
        mut server: S,
        options: Options,
        context: Context,
        control: Control<E, C>,
        rx_unbounded_events: UnboundedReceiver<UnboundedEventsList>,
    ) -> Result<(), ProducerError<E>>
    where
        S: server::Impl<E, C>,
        C: server::Control<E> + Send + Clone,
        E: std::error::Error + Clone,
    {
        let rx_server_events = server.observer().map_err(ProducerError::ServerError)?;
        let cancel = control.shutdown.child_token();
        select! {
            res = async {
                debug!(
                    target: logs::targets::PRODUCER,
                    "starting server"
                );
                server.listen().await.map_err(ProducerError::ServerError)
            } => res,
            res = listener(
                context,
                rx_server_events,
                rx_unbounded_events,
                control,
                &options,
            ) => res,
            _ = cancel.cancelled() => {
                Ok(())
            }
        }
    }

    async fn manage_task<E: std::error::Error, C: server::Control<E> + Send + Clone>(
        mut rx_manage_channel: UnboundedReceiver<ManageChannel>,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        debug!(target: logs::targets::PRODUCER, "manage_task is started");
        if let Some(command) = rx_manage_channel.recv().await {
            match command {
                ManageChannel::Shutdown(tx_resolver) => {
                    control.shutdown().await?;
                    if tx_resolver.send(()).is_err() {
                        error!(
                            target: logs::targets::PRODUCER,
                            "fail to send shutdown confirmation"
                        );
                    }
                }
            }
        }
        debug!(target: logs::targets::PRODUCER, "manage_task is finished");
        Ok(())
    }

    pub async fn run<S, C, E>(
        server: S,
        options: Options,
        context: Context,
    ) -> Result<Manage, ProducerError<E>>
    where
        S: server::Impl<E, C> + 'static,
        E: std::error::Error + Clone + Send + Sync,
        C: server::Control<E> + Send + Sync + Clone,
    {
        let (tx_manage_channel, rx_manage_channel): (
            UnboundedSender<ManageChannel>,
            UnboundedReceiver<ManageChannel>,
        ) = unbounded_channel();
        let (tx_unbounded_events, rx_unbounded_events): (
            UnboundedSender<UnboundedEventsList>,
            UnboundedReceiver<UnboundedEventsList>,
        ) = unbounded_channel();
        let shutdown_tracker_token = CancellationToken::new();
        let unbounded_events = UnboundedEvents {
            tx_unbounded_events,
        };
        let manage = Manage {
            tx_manage_channel,
            shutdown_tracker_token: shutdown_tracker_token.clone(),
            events: unbounded_events,
        };
        task::spawn(async move {
            let shutdown = CancellationToken::new();
            let control = Control {
                server_control: server.control(),
                shutdown,
                phantom: PhantomData,
            };
            let (main_task_res, manage_task_res) = join!(
                main_task(
                    server,
                    options,
                    context,
                    control.clone(),
                    rx_unbounded_events
                ),
                manage_task::<E, C>(rx_manage_channel, &control),
            );
            if let Err(err) = main_task_res.as_ref() {
                error!(
                    target: logs::targets::PRODUCER,
                    "main task is finished with error: {}", err
                );
            }
            if let Err(err) = manage_task_res.as_ref() {
                error!(
                    target: logs::targets::PRODUCER,
                    "manage task is finished with error: {}", err
                );
            }
            shutdown_tracker_token.cancel();
        });
        Ok(manage)
    }
}"#;
    pub const REQUEST: &str = r#"[[ref]] => {
    if let Err(err) = handlers::[[module]]::process::<E, C>(
        client.get_mut_identification(),
        &filter,
        &mut context,
        request,
        header.sequence,
        &control,
    )
    .await
    {
        responsing_err(
            format!("fail to process [[module]]: {}", err),
            uuid,
            &mut context,
            control,
            options,
            &mut Some(client),
        )
        .await?
    }
},"#;
    pub const BEACON: &str = r#"[[ref]] => {
    if let Err(err) = beacons_callers::[[module]]::emit::<E, C>(
        client.get_mut_identification(),
        beacon,
        header.sequence,
        &filter,
        &mut context,
        &control,
    )
    .await
    {
        error!(
            target: logs::targets::PRODUCER,
            "handeling beacon [[struct]] error: {}", err
        );
        emitters::error::emit::<E, C>(
            ProducerError::BeaconEmitterError(err),
            Some(uuid),
            &mut context,
            Some(client.get_mut_identification()),
            &control,
        )
        .await
        .map_err(ProducerError::EventEmitterError)?
    }
},"#;
    pub const EVENT: &str = r#"UnboundedEventsList::[[name]](event) => {
    if let Err(err) = emitters::[[module]]::emit::<E, C>(
        event,
        &filter,
        &mut context,
        &control,
    )
    .await
    {
        warn!(
            target: logs::targets::PRODUCER,
            "fail call [[module]] handler; error: {:?}", err,
        );
    }
},"#;
    pub const EVENT_CALLER: &str = r#"pub async fn [[module]](
    &self,
    event: protocol::[[ref]],
) -> Result<(), String> {
    self.tx_unbounded_events
        .send(UnboundedEventsList::[[name]](event))
        .map_err(|e| e.to_string())
}"#;
}

pub struct Render {}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, base: &Path, store: &Store, protocol: &Protocol) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[requests]]", &self.get_requests(store)?);
        output = output.replace("[[beacons]]", &self.get_beacons(store)?);
        output = output.replace("[[events]]", &self.get_events(store)?);
        output = output.replace("[[events_list]]", &self.get_events_list(store)?);
        output = output.replace("[[events_callers]]", &self.get_events_callers(store)?);
        output = output.replace(
            "[[indentification_self_enum_ref]]",
            &self.indentification_self_enum_ref(store)?,
        );
        output = output.replace(
            "[[indentification_self_response]]",
            &tools::into_rust_path(&store.get_config()?.self_key_response),
        );
        output = output.replace("[[protocol_hash]]", &protocol.get_hash());
        output = output.replace("[[workflow_hash]]", &store.get_hash());
        helpers::fs::write(dest, output, true)
    }

    fn get_requests(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            let mut request_output: String = String::from(templates::REQUEST);
            let parts: Vec<String> = request
                .get_request()?
                .split('.')
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| String::from(*v))
                .collect();
            let enum_ref: String = if parts.len() == 1 {
                format!(
                    "protocol::AvailableMessages::{}(protocol::{}(request))",
                    parts[0], parts[0]
                )
            } else {
                //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(protocol::UserLogin::Request::AvailableMessages::Request(request))) => {
                //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(request))
                let mut chain: String = String::from("");
                for (pos, part) in parts.iter().enumerate() {
                    let mut step: String = String::from("protocol");
                    for part in parts.iter().take(pos) {
                        step = format!("{}::{}", step, part);
                    }
                    step = format!("{}::AvailableMessages::{}(", step, part);
                    chain = format!("{}{}", chain, step);
                }
                format!("{}request{}", chain, ")".repeat(parts.len()))
            };
            request_output = request_output.replace("[[ref]]", &enum_ref);
            request_output = request_output.replace(
                "[[module]]",
                &request.get_request()?.to_lowercase().replace(".", "_"),
            );
            output = format!("{}{}", output, request_output);
            if pos < store.requests.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(tools::inject_tabs(7, output))
    }

    fn get_beacons(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, beacon) in store.beacons.iter().enumerate() {
            let mut beacon_output: String = String::from(templates::BEACON);
            let parts: Vec<String> = beacon
                .reference
                .split('.')
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| String::from(*v))
                .collect();
            let enum_ref: String = if parts.len() == 1 {
                format!(
                    "protocol::AvailableMessages::{}(protocol::{}(beacon))",
                    parts[0], parts[0]
                )
            } else {
                //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(protocol::UserLogin::Request::AvailableMessages::Request(request))) => {
                //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(request))
                let mut chain: String = String::from("");
                for (pos, part) in parts.iter().enumerate() {
                    let mut step: String = String::from("protocol");
                    for part in parts.iter().take(pos) {
                        step = format!("{}::{}", step, part);
                    }
                    step = format!("{}::AvailableMessages::{}(", step, part);
                    chain = format!("{}{}", chain, step);
                }
                format!("{}beacon{}", chain, ")".repeat(parts.len()))
            };
            beacon_output = beacon_output.replace("[[ref]]", &enum_ref);
            beacon_output = beacon_output.replace("[[struct]]", &beacon.reference.replace(".", ""));
            beacon_output = beacon_output.replace(
                "[[module]]",
                &beacon.reference.to_lowercase().replace(".", "_"),
            );
            output = format!("{}{}", output, beacon_output);
            if pos < store.beacons.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(tools::inject_tabs(7, output))
    }

    fn get_events(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            if !event.is_default() {
                let mut event_output: String = String::from(templates::EVENT);
                event_output = event_output.replace(
                    "[[module]]",
                    &event.get_reference()?.to_lowercase().replace(".", "_"),
                );
                event_output =
                    event_output.replace("[[name]]", &event.get_reference()?.replace(".", ""));
                output = format!("{}{}", output, event_output);
                if pos < store.events.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
        }
        Ok(tools::inject_tabs(6, output))
    }

    fn get_events_list(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            if !event.is_default() {
                output = format!(
                    "{}{}(protocol::{}),{}",
                    output,
                    event.get_reference()?.replace(".", ""),
                    event.get_reference()?.replace(".", "::"),
                    if pos < store.events.len() - 1 {
                        "\n"
                    } else {
                        ""
                    }
                );
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_events_callers(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            if !event.is_default() {
                let mut event_output: String = String::from(templates::EVENT_CALLER);
                event_output = event_output.replace(
                    "[[module]]",
                    &event.get_reference()?.to_lowercase().replace(".", "_"),
                );
                event_output =
                    event_output.replace("[[name]]", &event.get_reference()?.replace(".", "::"));
                event_output =
                    event_output.replace("[[ref]]", &event.get_reference()?.replace(".", "::"));
                output = format!("{}{}", output, event_output);
                if pos < store.events.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn indentification_self_enum_ref(&self, store: &Store) -> Result<String, String> {
        let parts: Vec<String> = store
            .get_config()?
            .get_self()?
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| String::from(*v))
            .collect();
        if parts.len() == 1 {
            Ok(format!(
                "protocol::AvailableMessages::{}(protocol::{}(request))",
                parts[0], parts[0]
            ))
        } else {
            let mut chain: String = String::from("");
            for (pos, part) in parts.iter().enumerate() {
                let mut step: String = String::from("protocol");
                for part in parts.iter().take(pos) {
                    step = format!("{}::{}", step, part);
                }
                step = format!("{}::AvailableMessages::{}(", step, part);
                chain = format!("{}{}", chain, step);
            }
            Ok(format!("{}request{}", chain, ")".repeat(parts.len())))
        }
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("mod.rs"))
    }
}
