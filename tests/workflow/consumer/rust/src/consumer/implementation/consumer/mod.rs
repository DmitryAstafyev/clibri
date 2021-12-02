pub mod api;
pub mod controller;
pub mod error;
pub mod options;

use super::{broadcasts, events, protocol, Context};
use api::{Api, Channel};
use controller::Consumer;
use error::ConsumerError;
use clibri::{client, env, env::logs};
use log::{debug, error, trace, warn};
use options::{Options, ReconnectionStrategy};
use protocol::PackingStruct;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::{
    join, select,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::spawn,
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub mod hash {
    pub const PROTOCOL: &str = "CF4FB13658612FE64ACBFDAD2D42DED0D59ABB9A899EFB15099CB02896B8A646";
    pub const WORKFLOW: &str = "C055B0290F4EF09272E955952F9FF390DEFF5BCB55749B73D8D4CE82ACEE4A13";
}

#[derive(Debug)]
pub enum Auth<E: client::Error> {
    SetUuid(Result<String, ConsumerError<E>>),
    GetUuid(oneshot::Sender<Option<Uuid>>),
}

#[derive(Debug)]
pub enum MergedClientChannel<E: client::Error> {
    Client(client::Event<E>),
    Auth(Auth<E>),
}

#[derive(Debug)]
pub enum Emitter<E: client::Error> {
    Connected,
    Disconnected,
    Shutdown(Option<ConsumerError<E>>),
    Error(ConsumerError<E>),    
    StructD(protocol::StructD),
    StructF(protocol::StructF),
    StructJ(protocol::StructJ),
    GroupBGroupCStructB(protocol::GroupB::GroupC::StructB),
    StructB(protocol::StructB),
    StructC(protocol::StructC),
    StructA(protocol::StructA),
    GroupAStructA(protocol::GroupA::StructA),
    GroupAStructB(protocol::GroupA::StructB),
    GroupBStructA(protocol::GroupB::StructA),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
    TriggerBeacons(protocol::TriggerBeacons),
    FinishConsumerTestBroadcast(protocol::FinishConsumerTestBroadcast),
}

pub struct ConsumerGetter<E: client::Error> {
    tx_consumer_getter: UnboundedSender<oneshot::Sender<Consumer<E>>>,
}

pub type ConsumerGetterChannel<E> = (
    UnboundedSender<oneshot::Sender<Consumer<E>>>,
    UnboundedReceiver<oneshot::Sender<Consumer<E>>>,
);

impl<E: client::Error> ConsumerGetter<E> {
    pub async fn get(&self) -> Result<Consumer<E>, ConsumerError<E>> {
        let (tx_response, rx_response): (
            oneshot::Sender<Consumer<E>>,
            oneshot::Receiver<Consumer<E>>,
        ) = oneshot::channel();
        self.tx_consumer_getter
            .send(tx_response)
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
        rx_response
            .await
            .map_err(|e| ConsumerError::APIChannel(e.to_string()))
    }
}

mod shortcuts {
    use super::*;

    pub async fn emit_event<E>(tx_consumer_event: &UnboundedSender<Emitter<E>>, event: Emitter<E>)
    where
        E: client::Error,
    {
        if let Err(err) = tx_consumer_event.send(event) {
            error!(
                target: logs::targets::CONSUMER,
                "fail to trigger consumer event; error: {}", err
            );
        }
    }

    pub async fn emit_error<E>(
        err: ConsumerError<E>,
        tx_consumer_event: &UnboundedSender<Emitter<E>>,
    ) where
        E: client::Error,
    {
        warn!(target: logs::targets::CONSUMER, "{}", err);
        if let Err(err) = tx_consumer_event.send(Emitter::Error(err)) {
            error!(
                target: logs::targets::CONSUMER,
                "fail emit error because: {}", err
            );
        }
    }
}

pub async fn connect<C, E, Ctrl>(
    client: C,
    context: Context,
    options: Options,
) -> Result<ConsumerGetter<E>, ConsumerError<E>>
where
    C: 'static + client::Impl<E, Ctrl> + Send + Sync,
    E: client::Error,
    Ctrl: 'static + client::Control<E> + Send + Sync + Clone,
{
    env::logs::init();
    let (tx_consumer_getter, rx_consumer_getter): ConsumerGetterChannel<E> = unbounded_channel();
    spawn(async move {
        trace!(target: logs::targets::CONSUMER, "main thread: started");
        let mut holder = Some((client, context, rx_consumer_getter));
        while let Some((client, context, rx_consumer_getter)) = holder.take() {
            holder = match listen(client, context, options.clone(), rx_consumer_getter).await {
                Ok((client, mut context, rx_consumer_getter, shutdowned)) => {
                    if let ReconnectionStrategy::Reconnect(timeout) = options.reconnection {
                        if shutdowned {
                            None
                        } else {
                            debug!(
                                target: logs::targets::CONSUMER,
                                "reconnection in {} ms", timeout
                            );
                            if events::reconnect::handler(timeout, &mut context).await {
                                sleep(Duration::from_millis(timeout)).await;
                                Some((client, context, rx_consumer_getter))
                            } else {
                                debug!(target: logs::targets::CONSUMER, "refuse to reconnect");
                                None
                            }
                        }
                    } else {
                        debug!(target: logs::targets::CONSUMER, "reconnection is disabled");
                        None
                    }
                }
                Err(err) => {
                    debug!(
                        target: logs::targets::CONSUMER,
                        "listener has been finished with error: {}", err
                    );
                    None
                }
            };
        }
        trace!(target: logs::targets::CONSUMER, "main thread: finished");
    });
    Ok(ConsumerGetter { tx_consumer_getter })
}

async fn listen<C, E, Ctrl>(
    mut client: C,
    context: Context,
    options: Options,
    mut rx_consumer_getter: UnboundedReceiver<oneshot::Sender<Consumer<E>>>,
) -> Result<
    (
        C,
        Context,
        UnboundedReceiver<oneshot::Sender<Consumer<E>>>,
        bool,
    ),
    ConsumerError<E>,
>
where
    C: 'static + client::Impl<E, Ctrl>,
    E: client::Error,
    Ctrl: 'static + client::Control<E> + Send + Sync + Clone,
{
    let (tx_client_api, rx_client_api): (UnboundedSender<Channel>, UnboundedReceiver<Channel>) =
        unbounded_channel();
    let (tx_auth, rx_auth): (UnboundedSender<Auth<E>>, UnboundedReceiver<Auth<E>>) =
        unbounded_channel();
    let (tx_consumer_event, rx_consumer_event): (
        UnboundedSender<Emitter<E>>,
        UnboundedReceiver<Emitter<E>>,
    ) = unbounded_channel();
    let (tx_shutdown, mut rx_shutdown): (
        Sender<oneshot::Sender<()>>,
        Receiver<oneshot::Sender<()>>,
    ) = channel(2);
    let api = Api::new(tx_client_api, tx_auth.clone());
    let consumer = Consumer::new(api.clone(), options.request_timeout);
    let client_control = client.control();
    let client_control_shutdown = client.control();
    let rx_client_events = client.observer().map_err(ConsumerError::Client)?;
    let getter_task_canceler = CancellationToken::new();
    let getter_task_canceler_caller = getter_task_canceler.clone();
    let getter_consumer = consumer.clone();
    let emitter_consumer = consumer.clone();
    let cancel = api.get_shutdown_token();
    let cancel_shutdown_getter = api.get_shutdown_token();
    debug!(target: logs::targets::CONSUMER, "listener task is started");
    let (rx_consumer_getter, context, mut shutdown, client) = join!(
        async move {
            debug!(target: logs::targets::CONSUMER, "getter subtask is started");
            while let Some(tx_response) = select! {
                tx_response = rx_consumer_getter.recv() => tx_response,
                _ = getter_task_canceler.cancelled() => None
            } {
                if tx_response.send(getter_consumer.clone()).is_err() {
                    warn!(
                        target: logs::targets::CONSUMER,
                        "fail to send consumer instance"
                    );
                }
            }
            debug!(
                target: logs::targets::CONSUMER,
                "getter subtask is finished"
            );
            rx_consumer_getter
        },
        async move {
            debug!(
                target: logs::targets::CONSUMER,
                "emitter subtask is started"
            );
            let context =
                consumer_emitter_task::<E>(rx_consumer_event, emitter_consumer, context).await;
            debug!(
                target: logs::targets::CONSUMER,
                "emitter subtask is finished"
            );
            context
        },
        async move {
            if let Some(tx_response) = select! {
                tx_response = rx_shutdown.recv() => tx_response,
                _ = cancel_shutdown_getter.cancelled() => None,
            } {
                if let Err(err) = client_control_shutdown.shutdown().await {
                    error!(
                        target: logs::targets::CONSUMER,
                        "client is shutdowned with error: {}", err
                    );
                }
                cancel_shutdown_getter.cancel();
                Some(tx_response)
            } else {
                None
            }
        },
        async move {
            debug!(
                target: logs::targets::CONSUMER,
                "listener subtask is started"
            );
            let tx_auth_message = tx_auth.clone();
            let (client_messages_task_res, client_api_task_res, client_res) = join!(
                async {
                    debug!(
                        target: logs::targets::CONSUMER,
                        "client_messages_task: started"
                    );
                    let result = client_messages_task::<E>(
                        rx_client_events,
                        tx_consumer_event.clone(),
                        rx_auth,
                        tx_auth_message,
                        api.clone(),
                        options,
                        cancel.clone(),
                    )
                    .await;
                    debug!(
                        target: logs::targets::CONSUMER,
                        "client_messages_task: finished"
                    );
                    cancel.cancel();
                    result
                },
                async {
                    debug!(target: logs::targets::CONSUMER, "client_api_task: started");
                    let result = client_api_task::<E, Ctrl>(
                        client_control,
                        tx_auth,
                        rx_client_api,
                        tx_shutdown,
                        cancel.clone(),
                    )
                    .await;
                    debug!(target: logs::targets::CONSUMER, "client_api_task: finished");
                    cancel.cancel();
                    result
                },
                async {
                    debug!(target: logs::targets::CONSUMER, "client connect: started");
                    let result = select! {
                        result = client.connect() => result,
                        _ = cancel.cancelled() => Ok(())
                    };
                    debug!(target: logs::targets::CONSUMER, "client connect: finished");
                    if let Err(err) = result.as_ref() {
                        shortcuts::emit_error(
                            ConsumerError::Client(err.clone()),
                            &tx_consumer_event,
                        )
                        .await;
                    }
                    shortcuts::emit_event(&tx_consumer_event, Emitter::Disconnected).await;
                    cancel.cancel();
                    if let Err(err) = result {
                        Err(ConsumerError::Client(err))
                    } else {
                        Ok(client)
                    }
                }
            );
            shortcuts::emit_event(
                &tx_consumer_event,
                Emitter::Shutdown(if let Err(err) = client_res.as_ref() {
                    Some(err.clone())
                } else if let Err(err) = client_messages_task_res.as_ref() {
                    Some(err.clone())
                } else if let Err(err) = client_api_task_res.as_ref() {
                    Some(err.clone())
                } else {
                    None
                }),
            )
            .await;
            getter_task_canceler_caller.cancel();
            debug!(
                target: logs::targets::CONSUMER,
                "listener subtask is finished"
            );
            if let Err(err) = client_messages_task_res {
                Err(err)
            } else if let Err(err) = client_api_task_res {
                Err(err)
            } else {
                client_res
            }
        }
    );
    debug!(target: logs::targets::CONSUMER, "listener task is finished");
    let shutdowned = if let Some(tx_response) = shutdown.take() {
        if tx_response.send(()).is_err() {
            error!(
                target: logs::targets::CONSUMER,
                "fail to send shutdown response/confirmation"
            );
        }
        true
    } else {
        false
    };
    match client {
        Ok(client) => Ok((client, context, rx_consumer_getter, shutdowned)),
        Err(err) => Err(err),
    }
}

async fn consumer_emitter_task<E>(
    mut rx_consumer_event: UnboundedReceiver<Emitter<E>>,
    consumer: Consumer<E>,
    mut context: Context,
) -> Context
where
    E: client::Error,
{
    trace!(
        target: logs::targets::CONSUMER,
        "consumer_emitter_task: started"
    );
    while let Some(msg) = rx_consumer_event.recv().await {
        match msg {
            Emitter::Error(err) => {
                warn!(target: logs::targets::CONSUMER, "{}", err);
                events::error::handler::<E>(err, &mut context, consumer.clone()).await;
            }
            Emitter::Connected => {
                events::connected::handler(&mut context, consumer.clone()).await;
            }
            Emitter::Disconnected => {
                events::disconnected::handler(&mut context, consumer.clone()).await;
            }
            Emitter::Shutdown(err) => {
                events::shutdown::handler(err, &mut context, consumer.clone()).await;
            }            
            Emitter::StructD(msg) => {
                broadcasts::structd::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::StructF(msg) => {
                broadcasts::structf::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::StructJ(msg) => {
                broadcasts::structj::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::GroupBGroupCStructB(msg) => {
                broadcasts::groupb_groupc_structb::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::StructB(msg) => {
                broadcasts::structb::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::StructC(msg) => {
                broadcasts::structc::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::StructA(msg) => {
                broadcasts::structa::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::GroupAStructA(msg) => {
                broadcasts::groupa_structa::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::GroupAStructB(msg) => {
                broadcasts::groupa_structb::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::GroupBStructA(msg) => {
                broadcasts::groupb_structa::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::GroupBGroupCStructA(msg) => {
                broadcasts::groupb_groupc_structa::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::TriggerBeacons(msg) => {
                broadcasts::triggerbeacons::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::FinishConsumerTestBroadcast(msg) => {
                broadcasts::finishconsumertestbroadcast::handler(msg, &mut context, consumer.clone()).await;
            },
        };
    }
    trace!(
        target: logs::targets::CONSUMER,
        "consumer_emitter_task: finished"
    );
    context
}

async fn client_api_task<E, Ctrl>(
    client: Ctrl,
    tx_auth: UnboundedSender<Auth<E>>,
    mut rx_client_api: UnboundedReceiver<Channel>,
    tx_shutdown: Sender<oneshot::Sender<()>>,
    cancel: CancellationToken,
) -> Result<(), ConsumerError<E>>
where
    E: client::Error,
    Ctrl: client::Control<E> + Send + Sync + Clone,
{
    let mut pending: HashMap<u32, oneshot::Sender<protocol::AvailableMessages>> = HashMap::new();
    let mut sequence: u32 = 10;
    while let Some(command) = select! {
        command = rx_client_api.recv() => command,
        _ = cancel.cancelled() => None
    } {
        match command {
            Channel::Send(buffer) => {
                client
                    .send(client::Message::Binary(buffer))
                    .await
                    .map_err(ConsumerError::Client)?;
            }
            Channel::Request((sequence, buffer, tx_response)) => {
                if pending.contains_key(&sequence) {
                    return Err(ConsumerError::Sequence(sequence.to_string()));
                }
                pending.insert(sequence, tx_response);
                client
                    .send(client::Message::Binary(buffer))
                    .await
                    .map_err(|e| ConsumerError::ClientChannel(e.to_string()))?;
            }
            Channel::AcceptIncome((sequence, msg, tx_response)) => {
                let accepted = if let Some(response) = pending.remove(&sequence) {
                    response.send(msg).map_err(|_| {
                        ConsumerError::APIChannel(format!(
                            "Fail to use pending channel; sequence: {}",
                            sequence
                        ))
                    })?;
                    true
                } else {
                    false
                };
                tx_response.send(accepted).map_err(|_| {
                    ConsumerError::APIChannel(String::from(
                        "Fail to send response for for Channel::AcceptIncome",
                    ))
                })?;
            }
            Channel::Uuid(tx_response) => {
                tx_auth.send(Auth::GetUuid(tx_response)).map_err(|_| {
                    ConsumerError::APIChannel(String::from(
                        "Fail to send response for for Channel::Uuid",
                    ))
                })?;
            }
            Channel::Sequence(tx_response) => {
                sequence += 1;
                if sequence >= u32::MAX - 1 {
                    sequence = 10;
                }
                tx_response.send(sequence).map_err(|_| {
                    ConsumerError::APIChannel(String::from(
                        "Fail to send response for for Channel::Sequence",
                    ))
                })?;
            }
            Channel::Shutdown(tx_response) => {
                tx_shutdown.send(tx_response).await.map_err(|_| {
                    ConsumerError::APIChannel(String::from(
                        "Fail to send request for for Channel::Shutdown",
                    ))
                })?;
            }
        }
    }
    Ok(())
}

async fn client_messages_task<E>(
    mut rx_client_event: UnboundedReceiver<client::Event<E>>,
    tx_consumer_event: UnboundedSender<Emitter<E>>,
    mut rx_auth: UnboundedReceiver<Auth<E>>,
    tx_auth: UnboundedSender<Auth<E>>,
    api: Api<E>,
    options: Options,
    cancel: CancellationToken,
) -> Result<(), ConsumerError<E>>
where
    E: client::Error,
{
    let mut buffer = protocol::Buffer::new();
    let mut uuid: Option<Uuid> = None;
    while let Some(msg) = select! {
        msg = rx_client_event.recv() => msg.map(MergedClientChannel::Client),
        msg = rx_auth.recv() => msg.map(MergedClientChannel::Auth),
        _ = cancel.cancelled() => None,
    } {
        match msg {
            MergedClientChannel::Client(msg) => {
                trace!(target: logs::targets::CONSUMER, "client event: {:?}", msg);
                match msg {
                    client::Event::Message(msg) => match msg {
                        client::Message::Binary(income) => {
                            trace!(
                                target: logs::targets::CONSUMER,
                                "has been received {} bytes",
                                income.len()
                            );
                            match buffer.chunk(&income, None) {
                                Ok(()) => {
                                    while let Some(msg) = buffer.next() {
                                        match api.accept(msg.header.sequence, msg.msg.clone()).await
                                        {
                                            Ok(accepted) => {
                                                if accepted {
                                                    continue;
                                                }
                                            }
                                            Err(err) => {
                                                shortcuts::emit_error::<E>(
                                                        ConsumerError::Pending(format!("Fail to handle broadcast Events::Connect; error: {}", err)),
                                                        &tx_consumer_event
                                                    ).await;
                                            }
                                        };
                                        match msg.msg {                                                    
                                                    protocol::AvailableMessages::StructD(msg) => {
                                                        tx_consumer_event.send(Emitter::StructD(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::StructF(msg) => {
                                                        tx_consumer_event.send(Emitter::StructF(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::StructJ(msg) => {
                                                        tx_consumer_event.send(Emitter::StructJ(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructB(msg))) => {
                                                        tx_consumer_event.send(Emitter::GroupBGroupCStructB(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::StructB(msg) => {
                                                        tx_consumer_event.send(Emitter::StructB(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::StructC(msg) => {
                                                        tx_consumer_event.send(Emitter::StructC(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::StructA(msg) => {
                                                        tx_consumer_event.send(Emitter::StructA(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructA(msg)) => {
                                                        tx_consumer_event.send(Emitter::GroupAStructA(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructB(msg)) => {
                                                        tx_consumer_event.send(Emitter::GroupAStructB(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::StructA(msg)) => {
                                                        tx_consumer_event.send(Emitter::GroupBStructA(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructA(msg))) => {
                                                        tx_consumer_event.send(Emitter::GroupBGroupCStructA(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::TriggerBeacons(msg) => {
                                                        tx_consumer_event.send(Emitter::TriggerBeacons(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                                    protocol::AvailableMessages::FinishConsumerTestBroadcast(msg) => {
                                                        tx_consumer_event.send(Emitter::FinishConsumerTestBroadcast(msg)).map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                                                    },
                                            _ => {
                                                shortcuts::emit_error::<E>(
                                                    ConsumerError::UnknownMessage(format!("header: {:?}", msg.header)),
                                                    &tx_consumer_event
                                                ).await;
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    shortcuts::emit_error::<E>(
                                        ConsumerError::BufferError(format!("{:?}", err)),
                                        &tx_consumer_event,
                                    )
                                    .await;
                                }
                            }
                        }
                        smth => {
                            shortcuts::emit_error::<E>(
                                ConsumerError::UnknownMessage(format!("{:?}", smth)),
                                &tx_consumer_event,
                            )
                            .await;
                        }
                    },
                    client::Event::Connected(_) => {
                        let api_auth = api.clone();
                        let options_auth = options.clone();
                        let tx_auth_response_auth = tx_auth.clone();
                        spawn(async move {
                            if let Err(err) = tx_auth_response_auth
                                .send(Auth::SetUuid(auth(api_auth, options_auth).await))
                            {
                                error!(
                                    target: logs::targets::CONSUMER,
                                    "fail to send response for consumer auth: {}", err
                                );
                            }
                        });
                    }
                    client::Event::Disconnected => {
                        uuid = None;
                        tx_consumer_event
                            .send(Emitter::Disconnected)
                            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                    }
                    client::Event::Error(err) => {
                        shortcuts::emit_error::<E>(ConsumerError::Client(err), &tx_consumer_event)
                            .await;
                    }
                }
            }
            MergedClientChannel::Auth(msg) => match msg {
                Auth::SetUuid(result) => match result {
                    Ok(assigned_uuid) => {
                        uuid =
                            Some(Uuid::from_str(&assigned_uuid).map_err(|_| ConsumerError::Uuid)?);
                        tx_consumer_event
                            .send(Emitter::Connected)
                            .map_err(|e| ConsumerError::APIChannel(e.to_string()))?;
                    }
                    Err(err) => {
                        shortcuts::emit_error::<E>(err.clone(), &tx_consumer_event).await;
                        return Err(err);
                    }
                },
                Auth::GetUuid(tx_response) => {
                    tx_response.send(uuid).map_err(|_| {
                        ConsumerError::APIChannel(String::from("Fail to response on uuid request"))
                    })?;
                }
            },
        }
    }
    Ok(())
}

async fn auth<E>(api: Api<E>, options: Options) -> Result<String, ConsumerError<E>>
where
    E: client::Error,
{
    debug!(
        target: logs::targets::CONSUMER,
        "client is connected; sending self-key"
    );
    let mut key = options.key.clone();
    let response = api
        .request(
            0,
            &key.pack(0, None)
                .map_err(|e| ConsumerError::Protocol(format!("fail to pack key: {}", e)))?,
        )
        .await
        .map_err(|e| {
            error!(
                target: logs::targets::CONSUMER,
                "fail to send self-key request: {}", e
            );
            ConsumerError::Handshake(format!("fail to send self-key request: {}", e))
        })?;
    let response = match response {
        protocol::AvailableMessages::InternalServiceGroup(
            protocol::InternalServiceGroup::AvailableMessages::SelfKeyResponse(response),
        ) => response,
        _ => {
            error!(
                target: logs::targets::CONSUMER,
                "expecting SelfKeyResponse; has been gotten: {:?}", response
            );
            return Err(ConsumerError::Handshake(format!(
                "unknown response: {:?}",
                response
            )));
        }
    };
    let uuid = response.uuid;
    debug!(
        target: logs::targets::CONSUMER,
        "self-key accepted; checking hash"
    );
    let mut hash = protocol::InternalServiceGroup::HashRequest {
        protocol: hash::PROTOCOL.to_string(),
        workflow: hash::WORKFLOW.to_string(),
    };
    let response = api
        .request(
            1,
            &hash
                .pack(1, None)
                .map_err(|e| ConsumerError::Protocol(format!("fail to pack key: {}", e)))?,
        )
        .await
        .map_err(|e| {
            error!(
                target: logs::targets::CONSUMER,
                "fail to send hash request: {}", e
            );
            ConsumerError::Handshake(format!("fail to send hash request: {}", e))
        })?;
    let response = match response {
        protocol::AvailableMessages::InternalServiceGroup(
            protocol::InternalServiceGroup::AvailableMessages::HashResponse(response),
        ) => response,
        _ => {
            error!(
                target: logs::targets::CONSUMER,
                "expecting HashResponse; has been gotten: {:?}", response
            );
            return Err(ConsumerError::HashCheck(format!(
                "unknown response: {:?}",
                response
            )));
        }
    };
    if let Some(err) = response.error {
        error!(
            target: logs::targets::CONSUMER,
            "hash rejected with: {}", err
        );
        return Err(ConsumerError::HashCheck(err));
    }
    debug!(target: logs::targets::CONSUMER, "hash accepted");
    Ok(uuid)
}