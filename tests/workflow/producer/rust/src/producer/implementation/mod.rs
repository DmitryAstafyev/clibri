#[path = "../beacons/mod.rs"]
pub mod beacons;
#[path = "./beacons/mod.rs"]
pub mod beacons_callers;
pub mod consumer;
pub mod emitters;
#[path = "../events/mod.rs"]
pub mod events;
pub mod handlers;
pub mod protocol;
#[path = "../responses/mod.rs"]
pub mod responses;

use super::context;
use clibri::{env::logs, server};
use consumer::{identification, Consumer};
use context::Context;
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

    async fn add_connection<E: server::Error, C: server::Control<E> + Send + Clone>(
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
        control
            .send(
                (protocol::InternalServiceGroup::ConnectConfirmationBeacon {})
                    .pack(0, Some(uuid.to_string()))
                    .map_err(|e| ProducerError::Protocol(uuid, e))?,
                Some(uuid),
            )
            .await?;
        Ok(())
    }

    async fn remove_connection<E: server::Error, C: server::Control<E> + Send + Clone>(
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

    async fn disconnect<E: server::Error, C: server::Control<E> + Send + Clone>(
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

    async fn responsing_err<E: server::Error, C: server::Control<E> + Send + Clone>(
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

    async fn process_received_data<E: server::Error, C: server::Control<E> + Send + Clone>(
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
                }
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
                                    control,
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
                            }
                            protocol::AvailableMessages::StructC(request) => {
                                if let Err(err) = handlers::structc::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::StructD(request) => {
                                if let Err(err) = handlers::structd::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::StructF(request) => {
                                if let Err(err) = handlers::structf::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::StructEmpty(request) => {
                                if let Err(err) = handlers::structempty::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::GroupA(
                                protocol::GroupA::AvailableMessages::StructA(request),
                            ) => {
                                if let Err(err) = handlers::groupa_structa::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::GroupA(
                                protocol::GroupA::AvailableMessages::StructB(request),
                            ) => {
                                if let Err(err) = handlers::groupa_structb::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::GroupB(
                                protocol::GroupB::AvailableMessages::GroupC(
                                    protocol::GroupB::GroupC::AvailableMessages::StructA(request),
                                ),
                            ) => {
                                if let Err(err) = handlers::groupb_groupc_structa::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::GroupB(
                                protocol::GroupB::AvailableMessages::StructA(request),
                            ) => {
                                if let Err(err) = handlers::groupb_structa::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::GroupB(
                                protocol::GroupB::AvailableMessages::GroupC(
                                    protocol::GroupB::GroupC::AvailableMessages::StructB(request),
                                ),
                            ) => {
                                if let Err(err) = handlers::groupb_groupc_structb::process::<E, C>(
                                    client.get_mut_identification(),
                                    &filter,
                                    &mut context,
                                    request,
                                    header.sequence,
                                    control,
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
                            }
                            protocol::AvailableMessages::BeaconA(beacon) => {
                                if let Err(err) = beacons_callers::beacona::emit::<E, C>(
                                    client.get_mut_identification(),
                                    beacon,
                                    header.sequence,
                                    &filter,
                                    &mut context,
                                    control,
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
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            }
                            protocol::AvailableMessages::Beacons(
                                protocol::Beacons::AvailableMessages::BeaconA(beacon),
                            ) => {
                                if let Err(err) = beacons_callers::beacons_beacona::emit::<E, C>(
                                    client.get_mut_identification(),
                                    beacon,
                                    header.sequence,
                                    &filter,
                                    &mut context,
                                    control,
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
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            }
                            protocol::AvailableMessages::Beacons(
                                protocol::Beacons::AvailableMessages::BeaconB(beacon),
                            ) => {
                                if let Err(err) = beacons_callers::beacons_beaconb::emit::<E, C>(
                                    client.get_mut_identification(),
                                    beacon,
                                    header.sequence,
                                    &filter,
                                    &mut context,
                                    control,
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
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            }
                            protocol::AvailableMessages::Beacons(
                                protocol::Beacons::AvailableMessages::Sub(
                                    protocol::Beacons::Sub::AvailableMessages::BeaconA(beacon),
                                ),
                            ) => {
                                if let Err(err) =
                                    beacons_callers::beacons_sub_beacona::emit::<E, C>(
                                        client.get_mut_identification(),
                                        beacon,
                                        header.sequence,
                                        &filter,
                                        &mut context,
                                        control,
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
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
                                }
                            }
                            protocol::AvailableMessages::Beacons(
                                protocol::Beacons::AvailableMessages::ShutdownServer(beacon),
                            ) => {
                                if let Err(err) =
                                    beacons_callers::beacons_shutdownserver::emit::<E, C>(
                                        client.get_mut_identification(),
                                        beacon,
                                        header.sequence,
                                        &filter,
                                        &mut context,
                                        control,
                                    )
                                    .await
                                {
                                    error!(
                                        target: logs::targets::PRODUCER,
                                        "handeling beacon BeaconsShutdownServer error: {}", err
                                    );
                                    emitters::error::emit::<E, C>(
                                        ProducerError::BeaconEmitterError(err),
                                        Some(uuid),
                                        &mut context,
                                        Some(client.get_mut_identification()),
                                        control,
                                    )
                                    .await
                                    .map_err(ProducerError::EventEmitterError)?
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

    async fn listener<E: server::Error, C: server::Control<E> + Send + Clone>(
        mut context: Context,
        mut rx_server_events: UnboundedReceiver<server::Events<E>>,
        mut rx_events: UnboundedReceiver<Event>,
        control: Control<E, C>,
        options: &Options,
    ) -> (Control<E, C>, Context, Option<ProducerError<E>>) {
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
                        server::Events::ConnectionError(uuid, err) => {
                            emitters::error::emit::<E, C>(
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
                            .map_err(ProducerError::EventEmitterError)?
                        }
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
                        let filter = identification::Filter::new(&consumers).await;
                        match event {
                            Event::EventA(event) => {
                                if let Err(err) = emitters::eventa::emit::<E, C>(
                                    event,
                                    &filter,
                                    &mut context,
                                    &control,
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
                                    event,
                                    &filter,
                                    &mut context,
                                    &control,
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
                                    event,
                                    &filter,
                                    &mut context,
                                    &control,
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
                                    event,
                                    &filter,
                                    &mut context,
                                    &control,
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
                                    event,
                                    &filter,
                                    &mut context,
                                    &control,
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
                            }
                            Event::FinishConsumerTest(event) => {
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
        C: server::Control<E> + Send + Sync + Clone,
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
        let ((server, server_res), (control, mut context, listener_err), shutdown_response) = join!(
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
        emitters::shutdown::emit(&mut context, &control)
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
