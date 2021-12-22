pub mod filter;
use super::{
    beacons_callers,
    consumer::{identification, Consumer},
    context::Context,
    emitters, handlers, hash, producer,
    producer::{ConsumerErrorHandelingStrategy, Control, Options, ProducerIdentificationStrategy},
    protocol,
    protocol::PackingStruct,
    ProducerError,
};
use clibri::{env::logs, server};
use filter::FilterCallbackBoxed;
use log::{debug, error, trace, warn};
use std::collections::HashMap;
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

pub enum ConsumerFilterRule {
    All,
    Except(Vec<Uuid>),
}

pub enum Inward {
    Filter(FilterCallbackBoxed, oneshot::Sender<Vec<Uuid>>),
    Consumers(ConsumerFilterRule, oneshot::Sender<Vec<Uuid>>),
    Connections(oneshot::Sender<usize>),
}

enum Event<E: server::Error> {
    Server(server::Events<E>),
    Inward(Inward),
}

#[derive(Clone, Debug)]
pub struct Hub<E: server::Error, C: server::Control<E> + 'static + Send + Sync> {
    tx_hub: UnboundedSender<server::Events<E>>,
    tx_inward: UnboundedSender<Inward>,
    shutdown: CancellationToken,
    control: Option<Control<E, C>>,
    uuid: Uuid,
}

impl<E: server::Error, C: server::Control<E> + 'static + Send + Sync> Hub<E, C> {
    pub fn new(
        control: Control<E, C>,
        tx_filter: UnboundedSender<filter::Request>,
        context: Context,
        options: Options,
    ) -> Self {
        let (tx_hub, rx_hub): (
            UnboundedSender<server::Events<E>>,
            UnboundedReceiver<server::Events<E>>,
        ) = unbounded_channel();
        let (tx_inward, rx_inward): (UnboundedSender<Inward>, UnboundedReceiver<Inward>) =
            unbounded_channel();
        let hub = Hub {
            tx_hub: tx_hub.clone(),
            tx_inward,
            shutdown: CancellationToken::new(),
            control: None,
            uuid: Uuid::new_v4(),
        };
        let shutdown = hub.shutdown.clone();
        task::spawn(async move {
            if let Err(err) = Self::listener(
                rx_hub,
                rx_inward,
                tx_filter,
                control,
                context,
                options,
                shutdown.clone(),
            )
            .await
            {
                error!(
                    target: logs::targets::PRODUCER,
                    "Hub listener finished with error: {}", err
                );
                // TODO: error handler
            }
            shutdown.cancel();
        });
        hub
    }

    pub async fn len(&self) -> Result<usize, ProducerError<E>> {
        let (tx_response, rx_response): (oneshot::Sender<usize>, oneshot::Receiver<usize>) =
            oneshot::channel();
        self.tx_inward
            .send(Inward::Connections(tx_response))
            .map_err(|e| {
                ProducerError::ChannelError(format!("Fail to request len of hub; error: {}", e))
            })?;
        rx_response.await.map_err(|_| {
            ProducerError::ChannelError("Fail to get response for len on hub".to_string())
        })
    }

    pub fn income(&self, msg: server::Events<E>) -> Result<(), ProducerError<E>> {
        self.tx_hub.send(msg).map_err(|e| {
            ProducerError::ChannelError(format!(
                "Fail to send income server message into hub; error: {}",
                e
            ))
        })
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    async fn listener(
        mut rx_hub: UnboundedReceiver<server::Events<E>>,
        mut rx_inward: UnboundedReceiver<Inward>,
        tx_filter: UnboundedSender<filter::Request>,
        control: Control<E, C>,
        context: Context,
        options: Options,
        shutdown: CancellationToken,
    ) -> Result<(), ProducerError<E>> {
        let (tx_identification, rx_identification): (
            UnboundedSender<identification::IdentificationChannel>,
            UnboundedReceiver<identification::IdentificationChannel>,
        ) = unbounded_channel();
        let mut consumers: HashMap<Uuid, Consumer> = HashMap::new();
        while let Some(event) = select! {
            mut msg = rx_hub.recv() => msg.take().map(Event::Server),
            mut msg = rx_inward.recv() => msg.take().map(Event::Inward),
            _ = shutdown.cancelled() => None,
        } {
            match event {
                Event::Server(event) => match event {
                    server::Events::Connected(uuid) => {
                        Self::add(
                            uuid,
                            &mut consumers,
                            tx_filter.clone(),
                            &context,
                            &control,
                            &options,
                            tx_identification.clone(),
                        )
                        .await?;
                    }
                    server::Events::Disconnected(uuid) => {
                        Self::remove(uuid, &mut consumers, tx_filter.clone(), &context, &control)
                            .await?
                    }
                    server::Events::Received(uuid, buffer) => {
                        Self::data(
                            uuid,
                            buffer,
                            &mut consumers,
                            tx_filter.clone(),
                            &context,
                            &control,
                            &options,
                        )
                        .await?
                    }
                    server::Events::Error(uuid, err) => emitters::error::emit::<E, C>(
                        ProducerError::ConsumerError(err),
                        uuid,
                        &context,
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
                        &context,
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
                    _ => {}
                },
                Event::Inward(event) => match event {
                    Inward::Connections(rx_response) => {
                        if rx_response.send(consumers.len()).is_err() {
                            error!(
                                target: logs::targets::PRODUCER,
                                "Cannot response to connections count request"
                            );
                            // TODO: error handler
                        }
                    }
                    Inward::Filter(filter, rx_response) => {
                        let mut uuids: Vec<Uuid> = vec![];
                        for (uuid, consumer) in &consumers {
                            if filter(consumer.get_identification()) {
                                uuids.push(*uuid);
                            }
                        }
                        if rx_response.send(uuids).is_err() {
                            error!(
                                target: logs::targets::PRODUCER,
                                "Cannot response to filter request"
                            );
                            // TODO: error handler
                        }
                    }
                    Inward::Consumers(rule, rx_response) => {
                        let uuids: Vec<Uuid> = match rule {
                            ConsumerFilterRule::All => consumers.keys().cloned().collect(),
                            ConsumerFilterRule::Except(exception) => consumers
                                .keys()
                                .filter(|uuid| !exception.iter().any(|tuuid| &tuuid == uuid))
                                .cloned()
                                .collect::<Vec<Uuid>>(),
                        };
                        if rx_response.send(uuids).is_err() {
                            error!(
                                target: logs::targets::PRODUCER,
                                "Cannot response to filter request"
                            );
                            // TODO: error handler
                        }
                    }
                },
            }
        }
        Ok(())
    }

    async fn add(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        tx_filter: UnboundedSender<filter::Request>,
        context: &Context,
        control: &Control<E, C>,
        options: &Options,
        tx_identification: UnboundedSender<identification::IdentificationChannel>,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "new client connection: {}", uuid,
        );
        if consumers.contains_key(&uuid) {
            return Err(ProducerError::FailToAddConsumer(uuid));
        }
        let mut consumer = Consumer::new(uuid, options, tx_identification);
        debug!(
            target: logs::targets::PRODUCER,
            "new connection accepted: {}", uuid,
        );
        if let Err(err) = emitters::connected::emit::<E, C>(
            consumer.get_identification(),
            filter::Filter::new(tx_filter),
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
        consumer.confirm();
        control
            .send(
                (protocol::InternalServiceGroup::ConnectConfirmationBeacon {})
                    .pack(0, Some(uuid.to_string()))
                    .map_err(|e| ProducerError::Protocol(uuid, e))?,
                Some(uuid),
            )
            .await?;
        consumers.insert(uuid, consumer);
        Ok(())
    }

    async fn remove(
        uuid: Uuid,
        consumers: &mut HashMap<Uuid, Consumer>,
        tx_filter: UnboundedSender<filter::Request>,
        context: &Context,
        control: &Control<E, C>,
    ) -> Result<(), ProducerError<E>> {
        debug!(
            target: logs::targets::PRODUCER,
            "client disconnected: {}", uuid,
        );
        if let Some(client) = consumers.remove(&uuid) {
            if let Err(err) = emitters::disconnected::emit::<E, C>(
                client.get_identification(),
                filter::Filter::new(tx_filter),
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

    async fn disconnect(
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

    async fn err(
        err: String,
        uuid: Uuid,
        context: &Context,
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
                Self::disconnect(uuid, consumer, control).await?;
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
                &context,
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

    async fn data(
        uuid: Uuid,
        buffer: Vec<u8>,
        consumers: &'_ mut HashMap<Uuid, Consumer>,
        tx_filter: UnboundedSender<filter::Request>,
        context: &Context,
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
                Self::disconnect(uuid, consumer, control).await?;
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
                Self::err(
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
            Self::err(
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
                protocol::AvailableMessages::StructA(request) if !consumer.is_hash_accepted() => {
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
                        Self::err(
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
                        Self::err(
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
                        Self::disconnect(uuid, consumer, control).await?;
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
                }
                message => {
                    if !consumer.is_hash_accepted() {
                        warn!(
                            target: logs::targets::PRODUCER,
                            "consumer {} tries to send data, but hash of client invalid", uuid
                        );
                        Self::disconnect(uuid, consumer, control).await?;
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
                        Self::disconnect(uuid, consumer, control).await?;
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
                            Self::disconnect(uuid, consumer, control).await?;
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
                        let filter = filter::Filter::new(tx_filter.clone());
                        let consumer = if let Some(consumer) = consumers.get(&uuid) {
                            consumer
                        } else {
                            return Ok(());
                        };
                        match message {
                            protocol::AvailableMessages::StructA(request) => {
                                if let Err(err) = handlers::structa::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process structa: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::StructC(request) => {
                                if let Err(err) = handlers::structc::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process structc: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::StructD(request) => {
                                if let Err(err) = handlers::structd::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process structd: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::StructF(request) => {
                                if let Err(err) = handlers::structf::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process structf: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::StructEmpty(request) => {
                                if let Err(err) = handlers::structempty::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process structempty: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::GroupA(
                                protocol::GroupA::AvailableMessages::StructA(request),
                            ) => {
                                if let Err(err) = handlers::groupa_structa::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process groupa_structa: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::GroupA(
                                protocol::GroupA::AvailableMessages::StructB(request),
                            ) => {
                                if let Err(err) = handlers::groupa_structb::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process groupa_structb: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
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
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process groupb_groupc_structa: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::GroupB(
                                protocol::GroupB::AvailableMessages::StructA(request),
                            ) => {
                                if let Err(err) = handlers::groupb_structa::process::<E, C>(
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process groupb_structa: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
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
                                    consumer.get_identification(),
                                    filter,
                                    context,
                                    request,
                                    header.sequence,
                                    control,
                                )
                                .await
                                {
                                    Self::err(
                                        format!("fail to process groupb_groupc_structb: {}", err),
                                        uuid,
                                        context,
                                        control,
                                        options,
                                        &mut consumers.get_mut(&uuid),
                                    )
                                    .await?
                                }
                            }
                            protocol::AvailableMessages::BeaconA(beacon) => {
                                if let Err(err) = beacons_callers::beacona::emit::<E, C>(
                                    consumer.get_identification(),
                                    beacon,
                                    header.sequence,
                                    filter,
                                    context,
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
                                        context,
                                        Some(consumer.get_identification()),
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
                                    consumer.get_identification(),
                                    beacon,
                                    header.sequence,
                                    filter,
                                    context,
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
                                        context,
                                        Some(consumer.get_identification()),
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
                                    consumer.get_identification(),
                                    beacon,
                                    header.sequence,
                                    filter,
                                    context,
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
                                        context,
                                        Some(consumer.get_identification()),
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
                                        consumer.get_identification(),
                                        beacon,
                                        header.sequence,
                                        filter,
                                        context,
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
                                        context,
                                        Some(consumer.get_identification()),
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
                                        consumer.get_identification(),
                                        beacon,
                                        header.sequence,
                                        filter,
                                        context,
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
                                        context,
                                        Some(consumer.get_identification()),
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
}
