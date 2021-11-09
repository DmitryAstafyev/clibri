pub mod api;
pub mod controller;
pub mod error;
pub mod options;

use super::{broadcasts, events, protocol, Context};
use api::{Api, Channel};
use controller::Consumer;
use error::ConsumerError;
use fiber::{client, env, env::logs};
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
    pub const PROTOCOL: &str = "F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE";
    pub const WORKFLOW: &str = "19C9064E89D04F2ECE3CA473265296B295FEC7C62F7548D8C51A1A1E631149C6";
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

pub enum Emitter<E: client::Error> {
    Connected,
    Disconnected,
    Shutdown(Option<ConsumerError<E>>),
    Error(ConsumerError<E>),    
    EventsUserConnected(protocol::Events::UserConnected),
    EventsMessage(protocol::Events::Message),
    EventsUserDisconnected(protocol::Events::UserDisconnected),
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

pub async fn connect<C, E, Ctrl>(
    client: C,
    context: Context,
    options: Options,
) -> Result<ConsumerGetter<E>, ConsumerError<E>>
where
    C: 'static + client::Impl<E, Ctrl>,
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
                Ok((client, mut context, rx_consumer_getter, shutdown)) => {
                    if let ReconnectionStrategy::Reconnect(timeout) = options.reconnection {
                        debug!(
                            target: logs::targets::CONSUMER,
                            "reconnection in {} ms", timeout
                        );
                        if events::event_reconnect::handler(timeout, &mut context).await {
                            sleep(Duration::from_millis(timeout)).await;
                            Some((client, context, rx_consumer_getter))
                        } else {
                            debug!(target: logs::targets::CONSUMER, "refuse to reconnect");
                            shutdown.cancel();
                            None
                        }
                    } else {
                        debug!(target: logs::targets::CONSUMER, "reconnection is disabled");
                        shutdown.cancel();
                        None
                    }
                }
                Err((shutdown, err)) => {
                    debug!(
                        target: logs::targets::CONSUMER,
                        "listener has been finished with error: {}", err
                    );
                    shutdown.cancel();
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
        CancellationToken,
    ),
    (CancellationToken, ConsumerError<E>),
>
where
    C: 'static + client::Impl<E, Ctrl>,
    E: client::Error,
    Ctrl: 'static + client::Control<E> + Send + Sync + Clone,
{
    let (tx_client_api, rx_client_api): (UnboundedSender<Channel>, UnboundedReceiver<Channel>) =
        unbounded_channel();
    let (tx_auth, rx_auth): (Sender<Auth<E>>, Receiver<Auth<E>>) = channel(2);
    let (tx_consumer_event, rx_consumer_event): (Sender<Emitter<E>>, Receiver<Emitter<E>>) =
        channel(3);
    let api = Api::new(tx_client_api, tx_auth.clone());
    let consumer = Consumer::new(api.clone());
    let shutdown_token = api.get_shutdown_token();
    let shutdown_token_out = api.get_shutdown_token();
    let client_control = client.control();
    let rx_client_events = client
        .observer()
        .map_err(|e| (shutdown_token.clone(), ConsumerError::Client(e)))?;
    let getter_task_canceler = CancellationToken::new();
    let getter_task_canceler_caller = getter_task_canceler.clone();
    let getter_consumer = consumer.clone();
    let emitter_consumer = consumer.clone();
    debug!(target: logs::targets::CONSUMER, "listener task is started");
    let (rx_consumer_getter, context, result) = join!(
        async move {
            debug!(target: logs::targets::CONSUMER, "getter subtask is started");
            select! {
                _ = async {
                    while let Some(tx_response) = rx_consumer_getter.recv().await {
                        if tx_response.send(getter_consumer.clone()).is_err() {
                            warn!(
                                target: logs::targets::CONSUMER,
                                "fail to send consumer instance"
                            );
                        }
                    }
                } => {},
                _ = getter_task_canceler.cancelled() => {}
            };
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
            debug!(
                target: logs::targets::CONSUMER,
                "listener subtask is started"
            );
            let result: Result<Option<C>, ConsumerError<E>> = select! {
                res = client_messages_task::<E>(rx_client_events, tx_consumer_event.clone(), rx_auth, tx_auth.clone(), api.clone(), options) => {
                    if let Err(err) = res {
                        Err(err)
                    } else {
                        Ok(None)
                    }
                },
                res = client_api_task::<E, Ctrl>(client_control.clone(), api.clone(), tx_auth, rx_client_api) => {
                    if let Err(err) = res {
                        Err(err)
                    } else {
                        Ok(None)
                    }
                },
                res = client.connect() => {
                    if let Err(err) = res {
                        if let Err(err) = tx_consumer_event.send(Emitter::Error(ConsumerError::Client(err))).await {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to trigger Emitter::Error; error: {}", err
                            );
                        }
                    }
                    if let Err(err) = tx_consumer_event.send(Emitter::Disconnected).await {
                        error!(
                            target: logs::targets::CONSUMER,
                            "fail emit disconnected because: {}", err
                        );
                    }
                    Ok(Some(client))
                },
                _ = shutdown_token.cancelled() => {
                    if let Err(err) = client_control.shutdown().await {
                        error!(
                            target: logs::targets::CONSUMER,
                            "client is shutdown with error: {}", err
                        );
                    }
                    Ok(None)
                },
            };
            let error: Option<ConsumerError<E>> = if let Err(err) = result.as_ref() {
                Some(err.clone())
            } else {
                None
            };
            if let Err(err) = tx_consumer_event.send(Emitter::Shutdown(error)).await {
                warn!(
                    target: logs::targets::CONSUMER,
                    "fail to trigger Emitter::Shutdown; error: {}", err
                );
            }
            getter_task_canceler_caller.cancel();
            debug!(
                target: logs::targets::CONSUMER,
                "listener subtask is finished"
            );
            result
        }
    );
    debug!(target: logs::targets::CONSUMER, "listener task is finished");
    match result {
        Ok(client) => {
            if let Some(client) = client {
                Ok((client, context, rx_consumer_getter, shutdown_token_out))
            } else {
                Err((shutdown_token_out, ConsumerError::NoClient))
            }
        }
        Err(err) => Err((shutdown_token_out, err)),
    }
}

async fn consumer_emitter_task<E>(
    mut rx_consumer_event: Receiver<Emitter<E>>,
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
                events::event_error::handler::<E>(err, &mut context, consumer.clone()).await;
            }
            Emitter::Connected => {
                events::event_connected::handler(&mut context, consumer.clone()).await;
            }
            Emitter::Disconnected => {
                events::event_disconnected::handler(&mut context, consumer.clone()).await;
            }
            Emitter::Shutdown(err) => {
                events::event_shutdown::handler(err, &mut context, consumer.clone()).await;
            }            
            Emitter::EventsUserConnected(msg) => {
                broadcasts::events_userconnected::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::EventsMessage(msg) => {
                broadcasts::events_message::handler(msg, &mut context, consumer.clone()).await;
            },
            Emitter::EventsUserDisconnected(msg) => {
                broadcasts::events_userdisconnected::handler(msg, &mut context, consumer.clone()).await;
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
    api: Api<E>,
    tx_auth: Sender<Auth<E>>,
    mut rx_client_api: UnboundedReceiver<Channel>,
) -> Result<(), ConsumerError<E>>
where
    E: client::Error,
    Ctrl: client::Control<E> + Send + Sync + Clone,
{
    trace!(target: logs::targets::CONSUMER, "client_api_task: started");
    let mut pending: HashMap<u32, oneshot::Sender<protocol::AvailableMessages>> = HashMap::new();
    let mut sequence: u32 = 10;
    let shutdown = api.get_shutdown_token();
    select! {
        _ = async move {
            while let Some(command) = rx_client_api.recv().await {
                match command {
                    Channel::Send(buffer) => {
                        if let Err(err) = client.send(client::Message::Binary(buffer)).await {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to send data: {}", err
                            );
                            api.shutdown();
                        }
                    }
                    Channel::Request((sequence, buffer, tx_response)) => {
                        if pending.contains_key(&sequence) {
                            // Error
                            error!(
                                target: logs::targets::CONSUMER,
                                "sequence #{} already has pending sender behind", sequence
                            );
                        }
                        pending.insert(sequence, tx_response);
                        if let Err(err) = client.send(client::Message::Binary(buffer)).await {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to send data: {}", err
                            );
                            api.shutdown();
                        }
                    }
                    Channel::AcceptIncome((sequence, msg, tx_response)) => {
                        let accepted = if let Some(response) = pending.remove(&sequence) {
                            if let Err(err) = response.send(msg) {
                                error!(
                                    target: logs::targets::CONSUMER,
                                    "fail to send pending response: {:?}", err
                                );
                                api.shutdown();
                            }
                            true
                        } else {
                            false
                        };
                        if let Err(err) = tx_response.send(accepted) {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to send Channel::AcceptIncome response: {:?}", err
                            );
                            api.shutdown();
                        }
                    }
                    Channel::Uuid(tx_response) => {
                        if let Err(err) = tx_auth.send(Auth::GetUuid(tx_response)).await {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to send Channel::Uuid response: {:?}", err
                            );
                            api.shutdown();
                        }
                    }
                    Channel::Sequence(tx_response) => {
                        sequence += 1;
                        if sequence >= u32::MAX - 1 {
                            sequence = 10;
                        }
                        if let Err(err) = tx_response.send(sequence) {
                            error!(
                                target: logs::targets::CONSUMER,
                                "fail to send Channel::Sequence response: {:?}", err
                            );
                            api.shutdown();
                        }
                    }
                }
            }
        } => {},
        _ = shutdown.cancelled() => {},
    };
    trace!(target: logs::targets::CONSUMER, "client_api_task: finished");
    Ok(())
}

async fn emit_error<E>(err: ConsumerError<E>, tx_consumer_event: &Sender<Emitter<E>>)
where
    E: client::Error,
{
    warn!(target: logs::targets::CONSUMER, "{}", err);
    if let Err(err) = tx_consumer_event.send(Emitter::Error(err)).await {
        error!(
            target: logs::targets::CONSUMER,
            "fail emit error because: {}", err
        );
    }
}

async fn client_messages_task<E>(
    mut rx_client_event: UnboundedReceiver<client::Event<E>>,
    tx_consumer_event: Sender<Emitter<E>>,
    mut rx_auth: Receiver<Auth<E>>,
    tx_auth: Sender<Auth<E>>,
    api: Api<E>,
    options: Options,
) -> Result<(), ConsumerError<E>>
where
    E: client::Error,
{
    trace!(
        target: logs::targets::CONSUMER,
        "client_messages_task: started"
    );
    let mut buffer = protocol::Buffer::new();
    let mut uuid: Option<Uuid> = None;
    let shutdown = api.get_shutdown_token();
    let result = select! {
        res = async move {
            while let Some(msg) = select! {
                msg = rx_client_event.recv() => msg.map(MergedClientChannel::Client),
                msg = rx_auth.recv() => msg.map(MergedClientChannel::Auth),
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
                                                        emit_error::<E>(
                                                                ConsumerError::Pending(format!("Fail to handle broadcast Events::Connect; error: {}", err)),
                                                                &tx_consumer_event
                                                            ).await;
                                                    }
                                                };
                                                match msg.msg {                                                    
                                                    protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::UserConnected(msg)) => {
                                                        if let Err(err) = tx_consumer_event.send(Emitter::EventsUserConnected(msg)).await {
                                                            return Err(ConsumerError::APIChannel(err.to_string()));
                                                        }
                                                    },
                                                    protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::Message(msg)) => {
                                                        if let Err(err) = tx_consumer_event.send(Emitter::EventsMessage(msg)).await {
                                                            return Err(ConsumerError::APIChannel(err.to_string()));
                                                        }
                                                    },
                                                    protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::UserDisconnected(msg)) => {
                                                        if let Err(err) = tx_consumer_event.send(Emitter::EventsUserDisconnected(msg)).await {
                                                            return Err(ConsumerError::APIChannel(err.to_string()));
                                                        }
                                                    },
                                                    _ => {
                                                        emit_error::<E>(
                                                            ConsumerError::UnknownMessage(format!("header: {:?}", msg.header)),
                                                            &tx_consumer_event
                                                        ).await;
                                                    }
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            emit_error::<E>(
                                                ConsumerError::BufferError(format!("{:?}", err)),
                                                &tx_consumer_event,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                smth => {
                                    emit_error::<E>(
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
                                        .await
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
                                if let Err(err) = tx_consumer_event.send(Emitter::Disconnected).await {
                                    error!(
                                        target: logs::targets::CONSUMER,
                                        "fail emit disconnected because: {}", err
                                    );
                                }
                            }
                            client::Event::Error(err) => {
                                emit_error::<E>(ConsumerError::Client(err), &tx_consumer_event).await;
                            }
                        }
                    }
                    MergedClientChannel::Auth(msg) => match msg {
                        Auth::SetUuid(result) => match result {
                            Ok(assigned_uuid) => {
                                uuid =
                                    Some(Uuid::from_str(&assigned_uuid).map_err(|_| ConsumerError::Uuid)?);
                                if let Err(err) = tx_consumer_event.send(Emitter::Connected).await {
                                    error!(
                                        target: logs::targets::CONSUMER,
                                        "fail emit connected because: {}", err
                                    );
                                }
                            }
                            Err(err) => {
                                emit_error::<E>(err, &tx_consumer_event).await;
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
        } => res,
        _ = shutdown.cancelled() => { Ok(()) }
    };
    trace!(
        target: logs::targets::CONSUMER,
        "client_messages_task: finished"
    );
    result
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