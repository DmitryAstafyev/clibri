
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
pub mod scope;

use super::context;
use consumer::{identification, Consumer};
use context::Context;
use clibri::{env::logs, server};
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
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub mod hash {
    pub const PROTOCOL: &str = "F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE";
    pub const WORKFLOW: &str = "1FB095BCD76804F2CD8195635CA425F05ADE78599369564AD30A3123A8BA3259";
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

    type ShutdownSender = Sender<Option<oneshot::Sender<()>>>;
    type ShutdownReceiver = Receiver<Option<oneshot::Sender<()>>>;

    enum Event {
        ServerEventsUserKickOff(protocol::ServerEvents::UserKickOff),
        ServerEventsUserAlert(protocol::ServerEvents::UserAlert),
    }

    enum MergedChannel<E: server::Error> {
        ServerEvents(server::Events<E>),
        Event(Event),
        Identification(identification::IdentificationChannel),
        Confirmation(Uuid),
    }

    #[derive(Clone, Debug)]
    pub struct Events {
        tx_events: UnboundedSender<Event>,
    }

    impl Events {
        pub async fn serverevents_userkickoff(
            &self,
            event: protocol::ServerEvents::UserKickOff,
        ) -> Result<(), String> {
            self.tx_events
                .send(Event::ServerEventsUserKickOff(event))
                .map_err(|e| e.to_string())
        }
        pub async fn serverevents_useralert(
            &self,
            event: protocol::ServerEvents::UserAlert,
        ) -> Result<(), String> {
            self.tx_events
                .send(Event::ServerEventsUserAlert(event))
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
        E: server::Error,
        C: server::Control<E>,
    {
        server_control: C,
        shutdown: CancellationToken,
        tx_shutdown: ShutdownSender,
        phantom: PhantomData<E>,
        pub events: Events,
    }

    impl<E, C> Control<E, C>
    where
        E: server::Error,
        C: server::Control<E>,
    {
        pub async fn shutdown(&self, wait: bool) -> Result<(), ProducerError<E>> {
            if wait {
                let (tx_resolver, rx_resolver): (oneshot::Sender<()>, oneshot::Receiver<()>) =
                    oneshot::channel();
                self.tx_shutdown
                    .send(Some(tx_resolver))
                    .await
                    .map_err(|_| {
                        ProducerError::ChannelError(String::from("Fail to send shutdown command"))
                    })?;
                rx_resolver
                    .await
                    .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
            } else {
                self.tx_shutdown.send(None).await.map_err(|_| {
                    ProducerError::ChannelError(String::from("Fail to send shutdown command"))
                })?;
            }
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

    async fn add_connection<E: server::Error, C: server::Control<E>>(
        uuid: Uuid,
        consumers: &'_ mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control<E, C>,
        options: &Options,
        tx_ident_change: UnboundedSender<identification::IdentificationChannel>,
        tx_confirm: UnboundedSender<Uuid>,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "new consumer connection: {}", uuid,
        );
        if consumers.contains_key(&uuid) {
            return Err(ProducerError::FailToAddConsumer(uuid));
        }
        let filter = identification::Filter::new(consumers);
        let consumer = Consumer::new(uuid, options, tx_ident_change);
        debug!(
            target: logs::targets::PRODUCER,
            "new connection accepted: {}", uuid,
        );
        if let Err(err) = emitters::connected::emit::<E, C>(
            consumer.get_identification(),
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
        consumers.insert(uuid, consumer);
        tx_confirm
            .send(uuid)
            .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
        Ok(())
    }

    async fn remove_connection<E: server::Error, C: server::Control<E>>(
        uuid: Uuid,
        consumers: &'_ mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "consumer disconnected: {}", uuid,
        );
        if let Some(consumer) = consumers.remove(&uuid) {
            let filter = identification::Filter::new(consumers);
            if let Err(err) = emitters::disconnected::emit::<E, C>(
                consumer.get_identification(),
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
                "consumer {} has been disconnected", uuid,
            );
        } else {
            warn!(
                target: logs::targets::PRODUCER,
                "cannot find a consumer {}; guess it was disconnected already", uuid,
            );
        }
        Ok(())
    }

    async fn disconnect<E: server::Error, C: server::Control<E>>(
        uuid: Uuid,
        consumer: &mut Consumer,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        if consumer.get_identification().is_discredited() {
            return Ok(());
        }
        consumer.get_mut_identification().discredited();
        control.disconnect(uuid).await?;
        Ok(())
    }

    async fn responsing_err<E: server::Error, C: server::Control<E>>(
        err: String,
        uuid: Uuid,
        context: &mut Context,
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
                warn!(
                    target: logs::targets::PRODUCER,
                    "{}:: consumer would be disconnected because of error: {}", uuid, err
                );
                disconnect(uuid, consumer, control).await?;
            } else {
                warn!(
                    target: logs::targets::PRODUCER,
                    "{}:: consumer isn't found; it wasn't connected", uuid
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
                context,
                consumer
                    .as_deref_mut()
                    .map(|consumer| consumer.get_identification()),
                control,
            )
            .await
            .map_err(ProducerError::EventEmitterError)?;
        }
        Ok(())
    }

    async fn process_received_data<E: server::Error, C: server::Control<E>>(
        uuid: Uuid,
        buffer: Vec<u8>,
        consumers: &'_ mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control<E, C>,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        trace!(
            target: logs::targets::PRODUCER,
            "new chunk of data from {} has been gotten",
            uuid,
        );
        let (messages, assigned, has_key) = if let Some(consumer) = consumers.get_mut(&uuid) {
            if !consumer.is_confirmed() {
                disconnect(uuid, consumer, control).await?;
                emitters::error::emit::<E, C>(
                    ProducerError::NotConfirmedConnection,
                    Some(uuid),
                    context,
                    Some(consumer.get_identification()),
                    control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?;
                return Ok(());
            }
            if consumer.get_identification().is_discredited() {
                // Consumer is discredited do nothing with it
                return Ok(());
            } else if let Err(err) = consumer.chunk(&buffer) {
                responsing_err(
                    format!("fail to read chunk of data; error: {}", err),
                    uuid,
                    context,
                    control,
                    options,
                    &mut Some(consumer),
                )
                .await?;
                return Ok(());
            } else {
                (
                    consumer.get_messages(),
                    consumer.get_identification().assigned(),
                    consumer.get_identification().has_key(),
                )
            }
        } else {
            responsing_err(
                String::from("fail to find consumer; message wouldn't be processed"),
                uuid,
                context,
                control,
                options,
                &mut None,
            )
            .await?;
            return Ok(());
        };
        if messages.is_empty() {
            return Ok(());
        }
        for (message, header) in messages.iter() {
            let consumer = if let Some(consumer) = consumers.get_mut(&uuid) {
                consumer
            } else {
                return Ok(());
            };
            match message {
                protocol::AvailableMessages::Identification(protocol::Identification::AvailableMessages::SelfKey(request)) if !consumer.is_hash_accepted() => {
                    trace!(
                        target: logs::targets::PRODUCER,
                        "consumer {} requested identification",
                        uuid,
                    );
                    let consumer = if let Some(consumer) = consumers.get_mut(&uuid) {
                        consumer
                    } else {
                        return Ok(());
                    };
                    let assigned_uuid = consumer.key(request, true);
                    if let Err(err) = match (protocol::InternalServiceGroup::SelfKeyResponse {
                        uuid: assigned_uuid.clone(),
                    })
                    .pack(header.sequence, Some(assigned_uuid.clone()))
                    {
                        Ok(buffer) => {
                            if let Err(err) = control.send(buffer, Some(uuid)).await {
                                Err(err.to_string())
                            } else {
                                debug!(
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
                            context,
                            control,
                            options,
                            &mut Some(consumer),
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
                        consumer.accept_hash();
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
                                debug!(
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
                            context,
                            control,
                            options,
                            &mut Some(consumer),
                        )
                        .await?
                    }
                    if !valid {
                        disconnect(uuid, consumer, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            context,
                            Some(consumer.get_identification()),
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    }
                },
                message => {
                    if !consumer.is_hash_accepted() {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but hash of consumer invalid", uuid
                        );
                        disconnect(uuid, consumer, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            context,
                            Some(consumer.get_identification()),
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    } else if !has_key {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but it doesn't have a key", uuid
                        );
                        disconnect(uuid, consumer, control).await?;
                        emitters::error::emit::<E, C>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            context,
                            Some(consumer.get_identification()),
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
                            disconnect(uuid, consumer, control).await?;
                        }
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::EmitError
                            || options.producer_indentification_strategy
                                == ProducerIdentificationStrategy::EmitErrorAndDisconnect
                        {
                            emitters::error::emit::<E, C>(
                                ProducerError::NoAssignedConsumer(uuid),
                                Some(uuid),
                                context,
                                Some(consumer.get_identification()),
                                control,
                            )
                            .await
                            .map_err(ProducerError::EventEmitterError)?;
                        }
                    } else {
                        let filter = identification::Filter::new(consumers);
                        let consumer = if let Some(consumer) = consumers.get(&uuid) {
                            consumer
                        } else {
                            return Ok(());
                        };
                        match message {
                            protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(request)) => {
                                if let Err(err) = handlers::userlogin_request::process::<E, C>(
                                    consumer.get_identification(),
                                    &filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process userlogin_request: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::Users(protocol::Users::AvailableMessages::Request(request)) => {
                                if let Err(err) = handlers::users_request::process::<E, C>(
                                    consumer.get_identification(),
                                    &filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process users_request: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Request(request)) => {
                                if let Err(err) = handlers::message_request::process::<E, C>(
                                    consumer.get_identification(),
                                    &filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process message_request: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::Messages(protocol::Messages::AvailableMessages::Request(request)) => {
                                if let Err(err) = handlers::messages_request::process::<E, C>(
                                    consumer.get_identification(),
                                    &filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process messages_request: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::Beacons(protocol::Beacons::AvailableMessages::LikeUser(beacon)) => {
                                if let Err(err) = beacons_callers::beacons_likeuser::emit::<E, C>(
                                    consumer.get_identification(),
                                    beacon,
                                    header.sequence,
                                    &filter,
                                    context,
                                    control,
                                )
                                .await
                                {
                                    error!(
                                        target: logs::targets::PRODUCER,
                                        "handeling beacon BeaconsLikeUser error: {}", err
                                    );
                                    emitters::error::emit::<E, C>(
                                        ProducerError::BeaconEmitterError(err),
                                        Some(uuid),
                                        context,
                                        Some(consumer.get_identification()),
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            },
                            protocol::AvailableMessages::Beacons(protocol::Beacons::AvailableMessages::LikeMessage(beacon)) => {
                                if let Err(err) = beacons_callers::beacons_likemessage::emit::<E, C>(
                                    consumer.get_identification(),
                                    beacon,
                                    header.sequence,
                                    &filter,
                                    context,
                                    control,
                                )
                                .await
                                {
                                    error!(
                                        target: logs::targets::PRODUCER,
                                        "handeling beacon BeaconsLikeMessage error: {}", err
                                    );
                                    emitters::error::emit::<E, C>(
                                        ProducerError::BeaconEmitterError(err),
                                        Some(uuid),
                                        context,
                                        Some(consumer.get_identification()),
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn listener<E: server::Error, C: server::Control<E>>(
        mut context: Context,
        mut rx_server_events: UnboundedReceiver<server::Events<E>>,
        mut rx_events: UnboundedReceiver<Event>,
        control: Control<E, C>,
        options: &Options,
    ) -> (Control<E, C>, Context, Result<(), ProducerError<E>>) {
        let (tx_ident_change, mut rx_ident_change): (
            UnboundedSender<identification::IdentificationChannel>,
            UnboundedReceiver<identification::IdentificationChannel>,
        ) = unbounded_channel();
        let (tx_confirm, mut rx_confirm): (UnboundedSender<Uuid>, UnboundedReceiver<Uuid>) =
            unbounded_channel();
        let mut consumers: HashMap<Uuid, Consumer> = HashMap::new();
        let cancel = control.shutdown.child_token();
        let result = async {
            while let Some(event) = select! {
                mut msg = rx_server_events.recv() => {
                    msg.take().map(MergedChannel::ServerEvents)
                },
                mut msg = rx_events.recv() => {
                    msg.take().map(MergedChannel::Event)
                },
                mut msg = rx_ident_change.recv() => {
                    msg.take().map(MergedChannel::Identification)
                },
                mut msg = rx_confirm.recv() => {
                    msg.take().map(MergedChannel::Confirmation)
                },
                _ = cancel.cancelled() => None
            } {
                match event {
                    MergedChannel::ServerEvents(event) => match event {
                        server::Events::Ready => emitters::ready::emit(&mut context, &control)
                            .await
                            .map_err(ProducerError::EventEmitterError)?,
                        server::Events::Shutdown => {
                            debug!(
                                target: logs::targets::PRODUCER,
                                "server down event has been received"
                            );
                            break;
                        }
                        server::Events::Connected(uuid) => {
                            add_connection(
                                uuid,
                                &mut consumers,
                                &mut context,
                                &control,
                                options,
                                tx_ident_change.clone(),
                                tx_confirm.clone(),
                            )
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
                                    .get(uuid)
                                    .map(|consumer| consumer.get_identification())
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
                                    .get(uuid)
                                    .map(|consumer| consumer.get_identification())
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
                    MergedChannel::Event(event) => {
                        let filter = identification::Filter::new(&consumers);
                        match event {
                            Event::ServerEventsUserKickOff(event) => {
                            if let Err(err) = emitters::serverevents_userkickoff::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call serverevents_userkickoff handler; error: {:?}", err,
                                );
                            }
                        },
                        Event::ServerEventsUserAlert(event) => {
                            if let Err(err) = emitters::serverevents_useralert::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call serverevents_useralert handler; error: {:?}", err,
                                );
                            }
                        },
                        }
                    }
                    MergedChannel::Identification(msg) => match msg {
                        identification::IdentificationChannel::Key(uuid, key, overwrite) => {
                            if let Some(consumer) = consumers.get_mut(&uuid) {
                                consumer.get_mut_identification().key(key, overwrite);
                            } else {
                                emitters::error::emit::<E, C>(
                                    ProducerError::ConsumerError(format!(
                                        "Fail to find consumer {} to change self-key",
                                        uuid
                                    )),
                                    Some(uuid),
                                    &mut context,
                                    None,
                                    &control,
                                )
                                .await
                                .map_err(ProducerError::EventEmitterError)?;
                            }
                        }
                        identification::IdentificationChannel::Assigned(uuid, key, overwrite) => {
                            if let Some(consumer) = consumers.get_mut(&uuid) {
                                consumer.get_mut_identification().assign(key, overwrite);
                            } else {
                                emitters::error::emit::<E, C>(
                                    ProducerError::ConsumerError(format!(
                                        "Fail to find consumer {} to change assigned-key",
                                        uuid
                                    )),
                                    Some(uuid),
                                    &mut context,
                                    None,
                                    &control,
                                )
                                .await
                                .map_err(ProducerError::EventEmitterError)?;
                            }
                        }
                    },
                    MergedChannel::Confirmation(uuid) => {
                        if let Some(consumer) = consumers.get_mut(&uuid) {
                            consumer.confirm();
                            control
                                .send(
                                    (protocol::InternalServiceGroup::ConnectConfirmationBeacon {})
                                        .pack(0, Some(uuid.to_string()))
                                        .map_err(|e| ProducerError::Protocol(uuid, e))?,
                                    Some(uuid),
                                )
                                .await?;
                        } else {
                            emitters::error::emit::<E, C>(
                                ProducerError::ConsumerError(format!(
                                    "Fail to find consumer {} to confirm connection",
                                    uuid
                                )),
                                Some(uuid),
                                &mut context,
                                None,
                                &control,
                            )
                            .await
                            .map_err(ProducerError::EventEmitterError)?;
                        }
                    }
                }
            }
            Ok::<(), ProducerError<E>>(())
        }
        .await;
        (control, context, result)
    }

    pub async fn run<S, C, E>(
        mut server: S,
        options: Options,
        context: Context,
    ) -> Result<(), ProducerError<E>>
    where
        S: server::Impl<E, C>,
        E: server::Error,
        C: server::Control<E>,
    {
        let (tx_events, rx_events): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
            unbounded_channel();
        let (tx_shutdown, mut rx_shutdown): (ShutdownSender, ShutdownReceiver) = channel(2);
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
        let (
            (server, server_res),
            (control, mut context, listener_res),
            (shutdown_res, shutdown_response),
        ) = join!(
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
                let shutdown_response = select! {
                    tx_response = rx_shutdown.recv() => tx_response,
                    _ = cancel.cancelled() => None,
                };
                let shutdown_res = server_control
                    .shutdown()
                    .await
                    .map_err(ProducerError::ServerError);
                debug!(
                    target: logs::targets::PRODUCER,
                    "shutdown listener: finished"
                );
                cancel.cancel();
                (shutdown_res, shutdown_response)
            }
        );
        if let Some(tx_response) = shutdown_response {
            debug!(
                target: logs::targets::PRODUCER,
                "shutdown has been requested"
            );
            if let Some(tx_response) = tx_response {
                tx_response.send(()).map_err(|_| {
                    ProducerError::ChannelError(String::from("Fail to send shutdown response"))
                })?;
            }
            drop(server);
            debug!(
                target: logs::targets::PRODUCER,
                "shutdown response has been sent"
            );
        }
        emitters::shutdown::emit(&mut context, &control)
            .await
            .map_err(ProducerError::EventEmitterError)?;
        if server_res.is_err() {
            server_res
        } else if listener_res.is_err() {
            listener_res
        } else {
            Ok(())
        }
    }
}