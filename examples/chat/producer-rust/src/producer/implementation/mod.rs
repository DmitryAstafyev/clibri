pub mod consumer;
pub mod emitters;
#[path = "../events/mod.rs"]
pub mod events;
pub mod handlers;
pub mod protocol;
#[path = "../responses/mod.rs"]
pub mod responses;

use super::context;
use consumer::{identification, Consumer};
use context::Context;
use fiber::{env::logs, server, server::Impl};
use log::{debug, error, trace, warn};
use protocol::PackingStruct;
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    select,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub type BroadcastSender = UnboundedSender<(Vec<Uuid>, Vec<u8>)>;
pub type BroadcastReceiver = UnboundedReceiver<(Vec<Uuid>, Vec<u8>)>;

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
    #[error("responsing error: `{0}`")]
    ResponsingError(String),
    #[error("not assigned consumer access: `{0}`")]
    AssignedError(String),
    #[error("channel access error: `{0}`")]
    ChannelError(String),
    #[error("consumer doesn't have a key; uuid: `{0}`")]
    NoConsumerKey(Uuid),
    #[error("consumer wasn't assigned; uuid: `{0}`")]
    NoAssignedConsumer(Uuid),
    #[error("consumer already marked as disconnected; uuid: `{0}`")]
    AlreadyDisconnected(Uuid),
}

pub mod producer {
    use super::*;

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
    pub struct Control {
        tx_server_control: UnboundedSender<server::Control>,
        shutdown: CancellationToken,
        tx_consumer_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    }

    impl Control {
        pub async fn shutdown<E: std::error::Error>(&self) -> Result<(), ProducerError<E>> {
            let (tx_shutdown_confirmation, rx_shutdown_confirmation): (
                oneshot::Sender<Option<E>>,
                oneshot::Receiver<Option<E>>,
            ) = oneshot::channel();
            self.tx_server_control
                .send(server::Control::Shutdown)
                .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
            // TODO: Wait for response
            Ok(())
        }

        pub fn disconnect<E: std::error::Error>(&self, uuid: Uuid) -> Result<(), ProducerError<E>> {
            self.tx_server_control
                .send(server::Control::Disconnect(uuid))
                .map_err(|e| ProducerError::ChannelError(e.to_string()))?;
            Ok(())
        }

        pub fn send<E: std::error::Error>(
            &self,
            buffer: Vec<u8>,
            uuid: Option<Uuid>,
        ) -> Result<(), ProducerError<E>> {
            self.tx_consumer_sender
                .send((buffer, uuid))
                .map_err(|e| ProducerError::ChannelError(e.to_string()))
        }

        pub fn broadcast<E: std::error::Error>(
            &self,
            uuids: Vec<Uuid>,
            buffer: Vec<u8>,
        ) -> Result<(), ProducerError<E>> {
            for uuid in uuids.iter() {
                if let Err(err) = self.tx_consumer_sender.send((buffer.clone(), Some(*uuid))) {
                    warn!(
                        target: logs::targets::PRODUCER,
                        "fail to send data to consumer {}: {}", uuid, err
                    );
                }
            }
            Ok(())
        }
    }

    async fn add_connection<E: std::error::Error>(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "new client connection: {}", uuid,
        );
        consumers.insert(uuid, Consumer::new(uuid, options));
        debug!(
            target: logs::targets::PRODUCER,
            "new connection accepted: {}", uuid,
        );
        if let Err(err) = emitters::connected::emit::<E>(
            uuid,
            context,
            identification::Filter::new(consumers).await,
            control,
        )
        .await
        {
            warn!(
                target: logs::targets::PRODUCER,
                "fail call connected handler for {}; error: {:?}", uuid, err,
            );
        }
        Ok(())
    }

    async fn remove_connection<E: std::error::Error>(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "client disconnected: {}", uuid,
        );
        if consumers.remove(&uuid).is_none() {
            warn!(
                target: logs::targets::PRODUCER,
                "cannot find a client {}; guess it was disconnected already", uuid,
            );
        } else {
            if let Err(err) = emitters::disconnected::emit::<E>(
                uuid,
                context,
                identification::Filter::new(consumers).await,
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
        }
        Ok(())
    }

    async fn disconnect<E: std::error::Error>(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        mut context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError<E>> {
        if let Some(consumer) = consumers.get_mut(&uuid) {
            if consumer.get_identification().is_discredited() {
                return Ok(());
            }
            consumer.get_identification().discredited();
            control.disconnect::<E>(uuid)?;
        } else {
            emitters::error::emit::<E>(
                ProducerError::AlreadyDisconnected(uuid),
                Some(uuid),
                &mut context,
                control,
            )
            .await
            .map_err(ProducerError::EventEmitterError)?;
        }
        Ok(())
    }

    async fn responsing_err<E: std::error::Error>(
        err: String,
        uuid: Uuid,
        mut context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError<E>> {
        warn!(target: logs::targets::PRODUCER, "{}:: {}", uuid, err);
        emitters::error::emit::<E>(
            ProducerError::ResponsingError(err),
            Some(uuid),
            &mut context,
            control,
        )
        .await
        .map_err(ProducerError::EventEmitterError)
    }

    async fn process_received_data<E: std::error::Error>(
        uuid: Uuid,
        buffer: Vec<u8>,
        consumers: &mut HashMap<Uuid, Consumer>,
        mut context: &mut Context,
        control: &Control,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        trace!(
            target: logs::targets::PRODUCER,
            "new chunk of data from {} has been gotten",
            uuid,
        );
        let mut messages: consumer::ConsumerMessages = vec![];
        let mut assigned: bool = false;
        let mut has_key: bool = false;
        if let Some(consumer) = consumers.get_mut(&uuid) {
            if consumer.get_identification().is_discredited() {
                // Consumer is discredited do nothing with it
            } else if let Err(err) = consumer.chunk(&buffer) {
                responsing_err(
                    format!("fail to read chunk of data; error: {}", err),
                    uuid,
                    &mut context,
                    control,
                )
                .await?
            } else {
                messages = consumer.get_messages();
                assigned = consumer.get_identification().assigned();
                has_key = consumer.get_identification().has_key();
            }
        } else {
            responsing_err(
                String::from("fail to find consumer; message wouldn't be processed"),
                uuid,
                &mut context,
                control,
            )
            .await?
        }
        if messages.is_empty() {
            return Ok(());
        }
        for (message, header) in messages.iter() {
            match message {
                protocol::AvailableMessages::Identification(
                    protocol::Identification::AvailableMessages::SelfKey(request),
                ) => {
                    trace!(
                        target: logs::targets::PRODUCER,
                        "consumer {} requested identification",
                        uuid,
                    );
                    if let Some(consumer) = consumers.get_mut(&uuid) {
                        let assigned_uuid = consumer.key(request, true);
                        if let Err(err) = match (protocol::Identification::SelfKeyResponse {
                            uuid: assigned_uuid.clone(),
                        })
                        .pack(header.sequence, Some(assigned_uuid.clone()))
                        {
                            Ok(buffer) => {
                                if let Err(err) = control.send::<E>(buffer, Some(uuid)) {
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
                            )
                            .await?
                        }
                    } else {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "fail to get consumer {}; requested identification is failed", uuid,
                        );
                    }
                }
                message => {
                    if !has_key {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but it doesn't have a key", uuid
                        );
                        disconnect(uuid, consumers, &mut context, control).await?;
                        emitters::error::emit::<E>(
                            ProducerError::NoConsumerKey(uuid),
                            Some(uuid),
                            &mut context,
                            control,
                        )
                        .await
                        .map_err(ProducerError::EventEmitterError)?;
                    } else if !assigned {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but it isn't assigned", uuid
                        );
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::Disconnect
                            || options.producer_indentification_strategy
                                == ProducerIdentificationStrategy::EmitErrorAndDisconnect
                        {
                            disconnect(uuid, consumers, &mut context, control).await?;
                        }
                        if options.producer_indentification_strategy
                            == ProducerIdentificationStrategy::EmitError
                            || options.producer_indentification_strategy
                                == ProducerIdentificationStrategy::EmitErrorAndDisconnect
                        {
                            emitters::error::emit::<E>(
                                ProducerError::NoAssignedConsumer(uuid),
                                Some(uuid),
                                &mut context,
                                control,
                            )
                            .await
                            .map_err(ProducerError::EventEmitterError)?;
                        }
                    } else {
                        match message {
                            protocol::AvailableMessages::UserLogin(
                                protocol::UserLogin::AvailableMessages::Request(request),
                            ) => {
                                if let Err(err) = handlers::user_login::process::<E>(
                                    &mut context,
                                    // TODO: we should not create each time new filter. Filter should be created just in case:
                                    // - consumer connected
                                    // - consumer disconnected
                                    // - consumer assigned
                                    // in all other cases we can clone filter or even send as &mut
                                    identification::Filter::new(consumers).await,
                                    uuid,
                                    request,
                                    header.sequence,
                                    &control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process user_login: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::Messages(
                                protocol::Messages::AvailableMessages::Request(request),
                            ) => {
                                if let Err(err) = handlers::messages::process::<E>(
                                    &mut context,
                                    identification::Filter::new(consumers).await,
                                    uuid,
                                    request,
                                    header.sequence,
                                    &control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process messages: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::Users(
                                protocol::Users::AvailableMessages::Request(request),
                            ) => {
                                if let Err(err) = handlers::users::process::<E>(
                                    &mut context,
                                    identification::Filter::new(consumers).await,
                                    uuid,
                                    request,
                                    header.sequence,
                                    &control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process users: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::Message(
                                protocol::Message::AvailableMessages::Request(request),
                            ) => {
                                if let Err(err) = handlers::message::process::<E>(
                                    &mut context,
                                    identification::Filter::new(consumers).await,
                                    uuid,
                                    request,
                                    header.sequence,
                                    &control,
                                )
                                .await
                                {
                                    responsing_err(
                                        format!("fail to process message: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                    )
                                    .await?
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn listener<E: std::error::Error>(
        mut context: Context,
        mut rx_server_events: UnboundedReceiver<server::Events<E>>,
        control: Control,
        options: &Options,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "listener of server's events is started"
        );
        let mut consumers: HashMap<Uuid, Consumer> = HashMap::new();
        while let Some(event) = rx_server_events.recv().await {
            match event {
                server::Events::Ready => {}
                server::Events::Shutdown => {}
                server::Events::Connected(uuid) => {
                    add_connection(uuid, &mut consumers, &mut context, &control, options).await?
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
                server::Events::Error(uuid, err) => emitters::error::emit::<E>(
                    ProducerError::ConsumerError(err),
                    uuid,
                    &mut context,
                    &control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?,
                server::Events::ConnectionError(uuid, err) => emitters::error::emit::<E>(
                    ProducerError::ConnectionError(err.to_string()),
                    uuid,
                    &mut context,
                    &control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?,
                server::Events::ServerError(err) => emitters::error::emit(
                    ProducerError::ServerError(err),
                    None,
                    &mut context,
                    &control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?,
            }
        }
        debug!(
            target: logs::targets::PRODUCER,
            "listener of server's events is finished"
        );
        Ok(())
    }

    pub async fn run<S, E>(
        mut server: S,
        options: Options,
        context: Context,
    ) -> Result<(), ProducerError<E>>
    where
        S: server::Impl<E>,
        E: std::error::Error,
    {
        let rx_server_events = server.observer().map_err(ProducerError::ServerError)?;
        let tx_consumer_sender = server.sender();
        let shutdown = CancellationToken::new();
        let control = Control {
            tx_server_control: server.control(),
            tx_consumer_sender: tx_consumer_sender.clone(),
            shutdown,
        };
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
                control,
                &options,
            ) => res
        }
    }
}
