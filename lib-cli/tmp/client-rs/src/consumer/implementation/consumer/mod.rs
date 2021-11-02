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
    select,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::spawn,
};
use uuid::Uuid;

pub mod hash {
    pub const PROTOCOL: &str = "F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE";
    pub const WORKFLOW: &str = "497F08C6B69D62FB7B05CB1FC27CD9BF5D516578D9D845C3C5D1FDD0A5097672";
}

#[derive(Debug)]
pub enum Auth<E: std::error::Error> {
    SetUuid(Result<String, ConsumerError<E>>),
    GetUuid(oneshot::Sender<Option<Uuid>>),
}

#[derive(Debug)]
pub enum MergedClientChannel<E: std::error::Error> {
    Client(client::Event<E>),
    Auth(Auth<E>),
}

pub async fn connect<C, E, Ctrl>(
    mut client: C,
    context: Context,
    options: Options,
) -> Result<Consumer<E>, ConsumerError<E>>
where
    C: 'static + client::Impl<E, Ctrl>,
    E: 'static + std::error::Error + Sync + Send + Clone,
    Ctrl: 'static + client::Control<E> + Send + Sync + Clone,
{
    env::logs::init();
    debug!(target: logs::targets::CONSUMER, "attempt to connect");
    let client_control = client.control();
    let rx_client_events = client.observer().map_err(ConsumerError::Client)?;
    debug!(target: logs::targets::CONSUMER, "connected");
    let (tx_client_api, rx_client_api): (UnboundedSender<Channel>, UnboundedReceiver<Channel>) =
        unbounded_channel();
    let (tx_auth, rx_auth): (Sender<Auth<E>>, Receiver<Auth<E>>) = channel(20);
    let api = Api::new(tx_client_api, tx_auth.clone());
    let consumer = Consumer::new(api.clone());
    let shutdown = api.get_shutdown_token();
    let consumer_messages_task = consumer.clone();
    spawn(async move {
        debug!(target: logs::targets::CONSUMER, "main thread is started");
        select! {
            res = client_messages_task::<E>(rx_client_events, rx_auth, tx_auth.clone(), api.clone(), consumer_messages_task, context, options) => {
            },
            res = client_api_task::<E, Ctrl>(client_control, api.clone(), tx_auth, rx_client_api) => {
            },
            res = client.connect() => {
            },
            _ = shutdown.cancelled() => {
            },
        };
        debug!(target: logs::targets::CONSUMER, "main thread is finished");
    });
    Ok(consumer)
}

async fn client_api_task<E, Ctrl>(
    client: Ctrl,
    api: Api<E>,
    tx_auth: Sender<Auth<E>>,
    mut rx_client_api: UnboundedReceiver<Channel>,
) where
    E: 'static + std::error::Error + Sync + Send + Clone,
    Ctrl: client::Control<E> + Send + Sync + Clone,
{
    let mut pending: HashMap<u32, oneshot::Sender<protocol::AvailableMessages>> = HashMap::new();
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
                println!(">>>>>>>>>>>>>>>>>>>>>>> 1");
                if let Err(err) = tx_auth.send(Auth::GetUuid(tx_response)).await {
                    error!(
                        target: logs::targets::CONSUMER,
                        "fail to send Channel::Uuid response: {:?}", err
                    );
                    api.shutdown();
                } else {
                    println!(">>>>>>>>>>>>>>>>>>>>>>> 1 OK");
                }
            }
        }
    }
}

async fn emit_error<E>(err: ConsumerError<E>, context: &mut Context, consumer: &mut Consumer<E>)
where
    E: 'static + std::error::Error + Sync + Send + Clone,
{
    warn!(target: logs::targets::CONSUMER, "{}", err);
    events::event_error::handler::<E>(err, context, consumer).await;
}

async fn client_messages_task<E>(
    mut rx_client_event: UnboundedReceiver<client::Event<E>>,
    mut rx_auth: Receiver<Auth<E>>,
    tx_auth: Sender<Auth<E>>,
    api: Api<E>,
    mut consumer: Consumer<E>,
    mut context: Context,
    options: Options,
) -> Result<(), ConsumerError<E>>
where
    E: 'static + std::error::Error + Sync + Send + Clone,
{
    trace!(
        target: logs::targets::CONSUMER,
        "client_messages_task: started"
    );
    let mut buffer = protocol::Buffer::new();
    let mut uuid: Option<Uuid> = None;
    while let Some(msg) = select! {
        msg = rx_client_event.recv() => msg.map(MergedClientChannel::Client),
        msg = rx_auth.recv() => msg.map(MergedClientChannel::Auth),
    } {
        println!(">>>>>>>>>>>>>>>>>>>>> 2: {:?}", msg);
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
                                                        &mut context,
                                                        &mut consumer
                                                    ).await;
                                            }
                                        };
                                        match msg.msg {
                                                protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::Message(msg)) => {
                                                    if let Err(err) = broadcasts::events_message::handler(msg, &mut context, &mut consumer).await {
                                                        emit_error::<E>(
                                                            ConsumerError::Broadcast(format!("Fail to handle broadcast Events::Message; error: {}", err)),
                                                            &mut context,
                                                            &mut consumer
                                                        ).await;
                                                    }
                                                },
                                                protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::UserConnected(msg)) => {
                                                    if let Err(err) = broadcasts::events_connected::handler(msg, &mut context, &mut consumer).await {
                                                        emit_error::<E>(
                                                            ConsumerError::Broadcast(format!("Fail to handle broadcast Events::Connect; error: {}", err)),
                                                            &mut context,
                                                            &mut consumer
                                                        ).await;
                                                    }
                                                },
                                                protocol::AvailableMessages::Events(protocol::Events::AvailableMessages::UserDisconnected(msg)) => {
                                                    if let Err(err) = broadcasts::events_disconnected::handler(msg, &mut context, &mut consumer).await {
                                                        emit_error::<E>(
                                                            ConsumerError::Broadcast(format!("Fail to handle broadcast Events::Disconnect; error: {}", err)),
                                                            &mut context,
                                                            &mut consumer
                                                        ).await;
                                                    }
                                                },
                                                _ => {
                                                    emit_error::<E>(
                                                        ConsumerError::UnknownMessage(format!("header: {:?}", msg.header)),
                                                        &mut context,
                                                        &mut consumer
                                                    ).await;
                                                }
                                            }
                                    }
                                }
                                Err(err) => {
                                    emit_error::<E>(
                                        ConsumerError::BufferError(format!("{:?}", err)),
                                        &mut context,
                                        &mut consumer,
                                    )
                                    .await;
                                }
                            }
                        }
                        smth => {
                            emit_error::<E>(
                                ConsumerError::UnknownMessage(format!("{:?}", smth)),
                                &mut context,
                                &mut consumer,
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
                        events::event_disconnected::handler(&mut context, &mut consumer).await;
                        // TODO: Emit disconnected. Stratagy: reconnect or wait. With event send maybe trigger to reconnect (based on stratagy)
                    }
                    client::Event::Error(err) => {
                        emit_error::<E>(ConsumerError::Client(err), &mut context, &mut consumer)
                            .await;
                    }
                }
            }
            MergedClientChannel::Auth(msg) => match msg {
                Auth::SetUuid(result) => match result {
                    Ok(assigned_uuid) => {
                        uuid =
                            Some(Uuid::from_str(&assigned_uuid).map_err(|_| ConsumerError::Uuid)?);
                        events::event_connected::handler(&mut context, &mut consumer).await;/// THIS IS LOCK US
                    }
                    Err(err) => {
                        emit_error::<E>(err, &mut context, &mut consumer).await;
                    }
                },
                Auth::GetUuid(tx_response) => {
                    tx_response.send(uuid).map_err(|_| {
                        ConsumerError::APIChannel(String::from("Fail to response on uuid request"))
                    })?;
                }
            },
        }
        println!(">>>>>>>>>>>>>>>>>>>>> 2-BACK");
    }
    trace!(
        target: logs::targets::CONSUMER,
        "client_messages_task: finished"
    );
    Ok(())
}

async fn auth<E>(api: Api<E>, options: Options) -> Result<String, ConsumerError<E>>
where
    E: 'static + std::error::Error + Sync + Send + Clone,
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
