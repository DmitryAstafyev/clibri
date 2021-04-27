use super::{
    channel::{Control, Messages, Error as ChannelError},
    tools,
};
use fiber::{
    logger::Logger,
    server::{errors::Errors, events::Events, interface::Interface},
};
use futures::{ SinkExt, StreamExt};
use tokio::{
    net::{
        TcpStream
    },
    sync::mpsc::{
        unbounded_channel,
        UnboundedReceiver,
        UnboundedSender
    },
    task::{
        spawn,
    },
    select,
};
use tokio_tungstenite::{
    tungstenite::{
        error::{
            Error,
            ProtocolError,
        },
        protocol::CloseFrame,
        protocol::Message
    },
    WebSocketStream,
};
use uuid::Uuid;

enum State {
    DisconnectByClient(CloseFrame<'static>),
    DisconnectByClientWithError(String),
    DisconnectByServer,
    Error(ChannelError),
}

pub struct Connection {
    uuid: Uuid,
    // socket: Arc<RwLock<WebSocketStream<TcpStream>>>,
    // control: Option<Sender<Control>>,
}

impl Connection {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }

    pub async fn attach(
        &mut self,
        mut ws: WebSocketStream<TcpStream>,
        events: UnboundedSender<Events>,
        messages: UnboundedSender<Messages>,
    ) -> Result<UnboundedSender<Control>, String> {
        let (tx_control, mut rx_control): (UnboundedSender<Control>, UnboundedReceiver<Control>) =
            unbounded_channel();
        let uuid = self.uuid;
        let mut state: Option<State> = None;
        let incomes_task_events = events.clone();
        let send_event = move |event: Events| {
            if let Err(e) = incomes_task_events.send(event) {
                tools::logger.warn(&format!("Cannot send event. Error: {}", e));
            }
        };
        let send_message = move |msg: Messages| {
            match messages.send(msg) {
                Ok(_) => {},
                Err(e) => {
                    if let Err(e) = events.send(Events::ConnectionError(
                        Some(uuid),
                        Errors::FailSendBack(
                            tools::logger.warn(&format!("{}:: Fail to send data back to server. Error: {}", uuid, e))
                        ),
                    )) {
                        tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                    }
                },
            };
        };
        spawn(async move {
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
                                match e {
                                    Error::Protocol(ref e) if e == &ProtocolError::ResetWithoutClosingHandshake => {
                                        state = Some(State::DisconnectByClientWithError(tools::logger.debug(&format!("{}:: Client disconnected without closing handshake", uuid))));
                                    },
                                    _ => {
                                        let e = tools::logger.warn(&format!("{}:: Cannot get message. Error: {:?}", uuid, e));
                                        send_event(Events::ConnectionError(
                                            Some(uuid),
                                            Errors::InvalidMessage(e.clone()),
                                        ));
                                        state = Some(State::Error(ChannelError::ReadSocket(e)));
                                    },
                                };
                                break;
                            }
                        };
                        match msg {
                            Message::Text(_) => {
                                tools::logger.warn(&format!("{}:: has been gotten not binnary data", uuid));
                                send_event(Events::ConnectionError(
                                    Some(uuid.clone()),
                                    Errors::NonBinaryData,
                                ));
                                continue;
                            },
                            Message::Binary(buffer) => {
                                tools::logger.verb(&format!("{}:: binary data {:?}", uuid, buffer));
                                send_message(Messages::Binary {
                                    uuid,
                                    buffer,
                                });
                            },
                            Message::Ping(_) | Message::Pong(_) => {
                                tools::logger.warn(&format!("{}:: Ping / Pong", uuid));
                            },
                            Message::Close(close_frame) => {
                                if let Some(frame) = close_frame {
                                    state = Some(State::DisconnectByClient(frame));
                                    break;
                                }
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
                                    state = Some(State::Error(ChannelError::WriteSocket(tools::logger.err(&format!("{}:: Cannot send data to client. Error: {}", uuid, e)))));
                                    break;
                                }
                            },
                            Control::Disconnect => {
                                state = Some(State::DisconnectByServer);
                                break;
                            },
                        }
                    } 
                };
            }
            tools::logger.debug(&format!("{}:: exit from socket listening loop.", uuid));
            if let Some(state) = state {
                match state {
                    State::DisconnectByServer => {
                        send_message(Messages::Disconnect { uuid, code: None });
                    },
                    State::DisconnectByClient(frame) => {
                        send_message(Messages::Disconnect { uuid, code: Some(frame.code) });
                    },
                    State::DisconnectByClientWithError(e) => {
                        send_message(Messages::Disconnect { uuid, code: None });
                    },
                    State::Error(error) => {
                        send_message(Messages::Error { uuid, error });
                    },
                };
            }
            match ws.close(None).await {
                Ok(()) => {},
                Err(e) => {
                    match e {
                        Error::AlreadyClosed | Error::ConnectionClosed => {
                            tools::logger.debug(&format!("{}:: connection is already closed", uuid));
                        },
                        _ => {
                            send_event(Events::ConnectionError(
                                Some(uuid.clone()),
                                Errors::CannotClose(tools::logger.err(&format!("{}:: fail to close connection", uuid))),
                            ));
                        }
                    }
                }
            };
        });
        Ok(tx_control)
    }

}
