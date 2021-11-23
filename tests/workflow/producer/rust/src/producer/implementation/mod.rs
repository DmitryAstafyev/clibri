
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
use clibri::{env::logs, server};
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
    pub const PROTOCOL: &str = "CF4FB13658612FE64ACBFDAD2D42DED0D59ABB9A899EFB15099CB02896B8A646";
    pub const WORKFLOW: &str = "C055B0290F4EF09272E955952F9FF390DEFF5BCB55749B73D8D4CE82ACEE4A13";
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
        StructA(protocol::StructA),
        StructB(protocol::StructB),
        GroupBStructA(protocol::GroupB::StructA),
        GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
        GroupDStructP(protocol::GroupD::StructP),
        TriggerBeaconsEmitter(protocol::TriggerBeaconsEmitter),
        FinishConsumerTest(protocol::FinishConsumerTest),
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
        pub async fn structa(
            &self,
            event: protocol::StructA,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::StructA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn structb(
            &self,
            event: protocol::StructB,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::StructB(event))
                .map_err(|e| e.to_string())
        }
        pub async fn groupb_structa(
            &self,
            event: protocol::GroupB::StructA,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::GroupBStructA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn groupb_groupc_structa(
            &self,
            event: protocol::GroupB::GroupC::StructA,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::GroupBGroupCStructA(event))
                .map_err(|e| e.to_string())
        }
        pub async fn groupd_structp(
            &self,
            event: protocol::GroupD::StructP,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::GroupDStructP(event))
                .map_err(|e| e.to_string())
        }
        pub async fn triggerbeaconsemitter(
            &self,
            event: protocol::TriggerBeaconsEmitter,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::TriggerBeaconsEmitter(event))
                .map_err(|e| e.to_string())
        }
        pub async fn finishconsumertest(
            &self,
            event: protocol::FinishConsumerTest,
        ) -> Result<(), String> {
            self.tx_unbounded_events
                .send(UnboundedEventsList::FinishConsumerTest(event))
                .map_err(|e| e.to_string())
        }
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
        pub events: UnboundedEvents,
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
                protocol::AvailableMessages::StructA(request) if !client.is_hash_accepted() => {
                    trace!(
                        target: logs::targets::PRODUCER,
                        "consumer {} requested identification",
                        uuid,
                    );
                    let assigned_uuid = client.key(request, true);
                    if let Err(err) = match (protocol::InternalServiceGroup::SelfKeyResponse {
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
                            protocol::AvailableMessages::StructA(request) => {
                                if let Err(err) = handlers::structa::process::<E, C>(
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
                                        format!("fail to process structa: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::StructC(request) => {
                                if let Err(err) = handlers::structc::process::<E, C>(
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
                                        format!("fail to process structc: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::StructD(request) => {
                                if let Err(err) = handlers::structd::process::<E, C>(
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
                                        format!("fail to process structd: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::StructF(request) => {
                                if let Err(err) = handlers::structf::process::<E, C>(
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
                                        format!("fail to process structf: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::StructEmpty(request) => {
                                if let Err(err) = handlers::structempty::process::<E, C>(
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
                                        format!("fail to process structempty: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructA(request)) => {
                                if let Err(err) = handlers::groupa_structa::process::<E, C>(
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
                                        format!("fail to process groupa_structa: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructB(request)) => {
                                if let Err(err) = handlers::groupa_structb::process::<E, C>(
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
                                        format!("fail to process groupa_structb: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructA(request))) => {
                                if let Err(err) = handlers::groupb_groupc_structa::process::<E, C>(
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
                                        format!("fail to process groupb_groupc_structa: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::StructA(request)) => {
                                if let Err(err) = handlers::groupb_structa::process::<E, C>(
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
                                        format!("fail to process groupb_structa: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructB(request))) => {
                                if let Err(err) = handlers::groupb_groupc_structb::process::<E, C>(
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
                                        format!("fail to process groupb_groupc_structb: {}", err),
                                        uuid,
                                        &mut context,
                                        control,
                                        options,
                                        &mut Some(client),
                                    )
                                    .await?
                                }
                            },
                            protocol::AvailableMessages::BeaconA(beacon) => {
                                if let Err(err) = beacons_callers::beacona::emit::<E, C>(
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
                                        "handeling beacon BeaconA error: {}", err
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
                            },
                            protocol::AvailableMessages::Beacons(protocol::Beacons::AvailableMessages::BeaconA(beacon)) => {
                                if let Err(err) = beacons_callers::beacons_beacona::emit::<E, C>(
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
                                        "handeling beacon BeaconsBeaconA error: {}", err
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
                            },
                            protocol::AvailableMessages::Beacons(protocol::Beacons::AvailableMessages::BeaconB(beacon)) => {
                                if let Err(err) = beacons_callers::beacons_beaconb::emit::<E, C>(
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
                                        "handeling beacon BeaconsBeaconB error: {}", err
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
                            },
                            protocol::AvailableMessages::Beacons(protocol::Beacons::AvailableMessages::Sub(protocol::Beacons::Sub::AvailableMessages::BeaconA(beacon))) => {
                                if let Err(err) = beacons_callers::beacons_sub_beacona::emit::<E, C>(
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
                                        "handeling beacon BeaconsSubBeaconA error: {}", err
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
                            },
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
                        UnboundedEventsList::StructA(event) => {
                            if let Err(err) = emitters::structa::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call structa handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::StructB(event) => {
                            if let Err(err) = emitters::structb::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call structb handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::GroupBStructA(event) => {
                            if let Err(err) = emitters::groupb_structa::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call groupb_structa handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::GroupBGroupCStructA(event) => {
                            if let Err(err) = emitters::groupb_groupc_structa::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call groupb_groupc_structa handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::GroupDStructP(event) => {
                            if let Err(err) = emitters::groupd_structp::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call groupd_structp handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::TriggerBeaconsEmitter(event) => {
                            if let Err(err) = emitters::triggerbeaconsemitter::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call triggerbeaconsemitter handler; error: {:?}", err,
                                );
                            }
                        },
                        UnboundedEventsList::FinishConsumerTest(event) => {
                            if let Err(err) = emitters::finishconsumertest::emit::<E, C>(
                                event,
                                &filter,
                                &mut context,
                                &control,
                            )
                            .await
                            {
                                warn!(
                                    target: logs::targets::PRODUCER,
                                    "fail call finishconsumertest handler; error: {:?}", err,
                                );
                            }
                        },
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
            events: unbounded_events.clone(),
        };
        task::spawn(async move {
            let shutdown = CancellationToken::new();
            let control = Control {
                server_control: server.control(),
                shutdown,
                phantom: PhantomData,
                events: unbounded_events,
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
}