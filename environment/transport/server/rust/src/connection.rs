use super::{
    channel::{Control, Error as ChannelError, Messages},
    errors::Error,
    server::MonitorEvent,
};
use fiber::{env::logs, server};
use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use tokio::{
    net::TcpStream,
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::spawn,
};
use tokio_tungstenite::{
    tungstenite::{
        error::{Error as TungsteniteError, ProtocolError},
        protocol::CloseFrame,
        protocol::Message,
    },
    WebSocketStream,
};
use uuid::Uuid;

enum State {
    DisconnectByClient(Option<CloseFrame<'static>>),
    DisconnectByClientWithError(String),
    DisconnectByServer,
    Error(ChannelError),
}

pub struct Connection {
    uuid: Uuid,
}

impl Connection {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }

    pub async fn attach(
        &mut self,
        mut ws: WebSocketStream<TcpStream>,
        events: UnboundedSender<server::Events<Error>>,
        messages: UnboundedSender<Messages>,
        monitor: Option<UnboundedSender<(u16, MonitorEvent)>>,
        port: u16,
    ) -> Result<UnboundedSender<Control>, String> {
        let (tx_control, mut rx_control): (UnboundedSender<Control>, UnboundedReceiver<Control>) =
            unbounded_channel();
        let uuid = self.uuid;
        let mut state: Option<State> = None;
        let incomes_task_events = events.clone();
        let send_event = move |event: server::Events<Error>| {
            if let Err(e) = incomes_task_events.send(event) {
                warn!(
                    target: logs::targets::SERVER,
                    "Cannot send event. Error: {}", e
                );
            }
        };
        if let Some(monitor) = monitor.as_ref() {
            monitor
                .send((port, MonitorEvent::Connected))
                .map_err(|_e| String::from("Fail send monitor event - connected"))?;
        }
        let send_message = move |msg: Messages| {
            match messages.send(msg) {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        target: logs::targets::SERVER,
                        "{}:: Fail to send data back to server. Error: {}", uuid, e
                    );
                    if let Err(e) = events.send(server::Events::ConnectionError(
                        Some(uuid),
                        Error::Channel(format!("{}", e)),
                    )) {
                        warn!(
                            target: logs::targets::SERVER,
                            "Cannot send event. Error: {}", e
                        );
                    }
                }
            };
        };
        spawn(async move {
            let mut shutdown_resolver: Option<oneshot::Sender<()>> = None;
            loop {
                select! {
                    msg = ws.next() => {
                        let msg = if let Some(msg) = msg {
                            msg
                        } else {
                            continue;
                        };
                        let msg = match msg {
                            Ok(msg) => msg,
                            Err(e) => {
                                if let TungsteniteError::Protocol(ref e) = e {
                                    if e == &ProtocolError::ResetWithoutClosingHandshake {
                                        debug!(target: logs::targets::SERVER, "{}:: Client disconnected without closing handshake", uuid);
                                        state = Some(State::DisconnectByClientWithError(format!("{}", e)));
                                    }
                                }
                                if state.is_none() {
                                    warn!(target: logs::targets::SERVER, "{}:: Cannot get message. Error: {:?}", uuid, e);
                                    send_event(server::Events::ConnectionError(
                                        Some(uuid),
                                        Error::InvalidMessage(format!("{}", e)),
                                    ));
                                    state = Some(State::Error(ChannelError::ReadSocket(format!("{}", e))));
                                }
                                break;
                            }
                        };
                        match msg {
                            Message::Text(_) => {
                                warn!(target: logs::targets::SERVER, "{}:: has been gotten not binnary data", uuid);
                                send_event(server::Events::ConnectionError(
                                    Some(uuid),
                                    Error::NonBinaryData,
                                ));
                                continue;
                            },
                            Message::Binary(buffer) => {
                                info!(target: logs::targets::SERVER, "{}:: binary data {:?}", uuid, buffer);
                                send_message(Messages::Binary {
                                    uuid,
                                    buffer,
                                });
                            },
                            Message::Ping(_) | Message::Pong(_) => {
                                warn!(target: logs::targets::SERVER, "{}:: Ping / Pong", uuid);
                            },
                            Message::Close(close_frame) => {
                                state = Some(State::DisconnectByClient(close_frame));
                                break;
                            }
                        }
                    },
                    cmd = rx_control.recv() => {
                        let cmd = if let Some(cmd) = cmd {
                            cmd
                        } else {
                            continue;
                        };
                        match cmd {
                            Control::Send(buffer) => {
                                if let Err(e) = ws.send(Message::from(buffer)).await {
                                    error!(target: logs::targets::SERVER, "{}:: Cannot send data to client. Error: {}", uuid, e);
                                    state = Some(State::Error(ChannelError::WriteSocket(format!("{}", e))));
                                    break;
                                }
                            },
                            Control::Disconnect(tx_shutdown_resolver) => {
                                shutdown_resolver = Some(tx_shutdown_resolver);
                                state = Some(State::DisconnectByServer);
                                break;
                            },
                        }
                    }
                };
            }
            debug!(
                target: logs::targets::SERVER,
                "{}:: exit from socket listening loop.", uuid
            );
            if let Some(state) = state {
                match state {
                    State::DisconnectByServer => {
                        send_message(Messages::Disconnect { uuid, code: None });
                    }
                    State::DisconnectByClient(frame) => {
                        send_message(Messages::Disconnect {
                            uuid,
                            code: if let Some(frame) = frame {
                                Some(frame.code)
                            } else {
                                None
                            },
                        });
                    }
                    State::DisconnectByClientWithError(e) => {
                        debug!(
                            target: logs::targets::SERVER,
                            "{}:: client error: {}", uuid, e
                        );
                        send_message(Messages::Disconnect { uuid, code: None });
                    }
                    State::Error(error) => {
                        send_message(Messages::Error { uuid, error });
                    }
                };
            }
            match ws.close(None).await {
                Ok(()) => {}
                Err(e) => match e {
                    TungsteniteError::AlreadyClosed | TungsteniteError::ConnectionClosed => {
                        debug!(
                            target: logs::targets::SERVER,
                            "{}:: connection is already closed", uuid
                        );
                    }
                    _ => {
                        error!(
                            target: logs::targets::SERVER,
                            "{}:: fail to close connection", uuid
                        );
                        send_event(server::Events::ConnectionError(
                            Some(uuid),
                            Error::CloseConnection(format!("{}:: fail to close connection", uuid)),
                        ));
                    }
                },
            };
            drop(ws);
            if let Some(monitor) = monitor.as_ref() {
                if let Err(_err) = monitor.send((port, MonitorEvent::Disconnected)) {
                    error!(
                        target: logs::targets::SERVER,
                        "{}:: Fail send monitor event - disconnected", uuid
                    );
                }
            }
            if let Some(tx_shutdown_resolver) = shutdown_resolver.take() {
                if tx_shutdown_resolver.send(()).is_err() {
                    error!(
                        target: logs::targets::SERVER,
                        "{}:: Fail send disconnect confirmation", uuid
                    );
                }
            }
        });
        Ok(tx_control)
    }
}
