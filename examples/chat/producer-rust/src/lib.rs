pub mod consumer;
pub mod context;
#[path = "./events/emitters/mod.rs"]
pub mod emitters;
pub mod events;
#[path = "./responses/handlers/mod.rs"]
pub mod handlers;
pub mod protocol;
pub mod responses;

use consumer::{identification, Consumer};
use context::Context;
use fiber::{
    env::logs,
    server::{control::Control as ServerControl, events::Events, interface::Interface},
};
use fiber_transport_server::{errors::Error as ServerError, server::Server};
use log::{debug, error, trace, warn};
use protocol::PackingStruct;
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub type BroadcastSender = UnboundedSender<(Vec<Uuid>, Vec<u8>)>;
pub type BroadcastReceiver = UnboundedReceiver<(Vec<Uuid>, Vec<u8>)>;

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("server error: `{0}`")]
    ServerError(ServerError),
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
}

pub mod producer {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Control {
        tx_server_control: UnboundedSender<ServerControl>,
        shutdown: CancellationToken,
        tx_consumer_sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    }

    impl Control {
        pub async fn shutdown(&self) -> Result<(), ProducerError> {
            Ok(())
        }

        pub async fn disconnect(&self, uuid: Uuid) -> Result<(), ProducerError> {
            Ok(())
        }

        pub fn send(&self, buffer: Vec<u8>, uuid: Option<Uuid>) -> Result<(), ProducerError> {
            self.tx_consumer_sender
                .send((buffer, uuid))
                .map_err(|e| ProducerError::ChannelError(e.to_string()))
        }

        pub fn broadcast(&self, uuids: Vec<Uuid>, buffer: Vec<u8>) -> Result<(), ProducerError> {
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

    async fn add_connection(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError> {
        debug!(
            target: logs::targets::PRODUCER,
            "new client connection: {}", uuid,
        );
        consumers.insert(uuid, Consumer::new(uuid));
        debug!(
            target: logs::targets::PRODUCER,
            "new connection accepted: {}", uuid,
        );
        if let Err(err) = emitters::connected::emit(
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

    async fn remove_connection(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError> {
        debug!(
            target: logs::targets::PRODUCER,
            "client disconnected: {}", uuid,
        );
        consumers.remove(&uuid);
        if let Err(err) = emitters::disconnected::emit(
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
            "disconnection of client {} is accepted", uuid,
        );
        Ok(())
    }

    async fn responsing_err(
        err: String,
        uuid: Uuid,
        mut context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError> {
        warn!(target: logs::targets::PRODUCER, "{}:: {}", uuid, err);
        emitters::error::emit(
            ProducerError::ResponsingError(err),
            Some(uuid),
            &mut context,
            control,
        )
        .await
        .map_err(ProducerError::EventEmitterError)
    }

    async fn process_received_data(
        uuid: Uuid,
        buffer: Vec<u8>,
        consumers: &mut HashMap<Uuid, Consumer>,
        mut context: &mut Context,
        control: &Control,
    ) -> Result<(), ProducerError> {
        trace!(
            target: logs::targets::PRODUCER,
            "new chunk of data from {} has been gotten",
            uuid,
        );
        let mut messages: consumer::ConsumerMessages = vec![];
        let mut assigned: bool = false;
        if let Some(consumer) = consumers.get_mut(&uuid) {
            if let Err(err) = consumer.chunk(&buffer) {
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
                                if let Err(err) = control.send(buffer, Some(uuid)) {
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
                    if !assigned {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but it isn't assigned", uuid
                        );
                        // TODO: ProducerEvent
                        // TODO: Consumer should be disconnected or some tx_producer_events should be to consumer
                        // it might be some option of producer like NonAssignedStratagy
                    } else {
                        match message {
                            protocol::AvailableMessages::UserLogin(
                                protocol::UserLogin::AvailableMessages::Request(request),
                            ) => {
                                if let Err(err) = handlers::user_login::process(
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
                                if let Err(err) = handlers::messages::process(
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
                                if let Err(err) = handlers::users::process(
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
                                if let Err(err) = handlers::message::process(
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

    async fn listener(
        mut context: Context,
        mut rx_server_events: UnboundedReceiver<Events<ServerError>>,
        control: Control,
    ) -> Result<(), ProducerError> {
        debug!(
            target: logs::targets::PRODUCER,
            "listener of server's events is started"
        );
        let mut consumers: HashMap<Uuid, Consumer> = HashMap::new();
        while let Some(event) = rx_server_events.recv().await {
            match event {
                Events::Ready => {}
                Events::Shutdown => {}
                Events::Connected(uuid) => {
                    add_connection(uuid, &mut consumers, &mut context, &control).await?
                }
                Events::Disconnected(uuid) => {
                    remove_connection(uuid, &mut consumers, &mut context, &control).await?
                }
                Events::Received(uuid, buffer) => {
                    process_received_data(uuid, buffer, &mut consumers, &mut context, &control)
                        .await?
                }
                Events::Error(uuid, err) => emitters::error::emit(
                    ProducerError::ConsumerError(err),
                    uuid,
                    &mut context,
                    &control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?,
                Events::ConnectionError(uuid, err) => emitters::error::emit(
                    ProducerError::ConnectionError(err.to_string()),
                    uuid,
                    &mut context,
                    &control,
                )
                .await
                .map_err(ProducerError::EventEmitterError)?,
                Events::ServerError(err) => emitters::error::emit(
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

    pub async fn run(mut server: Server, context: Context) -> Result<(), ProducerError> {
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
            ) => res
        }
    }
}
