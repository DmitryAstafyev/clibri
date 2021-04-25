use super::{
    channel::{Control, Messages},
    connection::Connection,
    handshake::Handshake as HandshakeInterface,
    tools,
};
use fiber::{
    logger::Logger,
    server::{errors::Errors, events::Events, interface::Interface},
};
use futures::{executor, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::{
    io::{
        AsyncRead,
        AsyncWrite, //AsyncReadExt,
                    //AsyncWriteExt
    },
    net::{TcpListener, TcpStream},
    sync::mpsc::{unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    task::{spawn, JoinHandle},
    join,
    runtime::{Runtime},
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        handshake::server::{Request, Response},
        protocol::frame::coding::CloseCode,
    },
    WebSocketStream,
};
use uuid::Uuid;

pub struct Handshake;

impl HandshakeInterface for Handshake {}

pub struct Server {
    addr: String,
    controlls: Arc<RwLock<HashMap<Uuid, UnboundedSender<Control>>>>,
}

impl Server {

    pub fn new(addr: String) -> Self {
        Self { addr, controlls: Arc::new(RwLock::new(HashMap::new()))}
    }
}

impl Interface for Server {

    fn listen(
        &mut self,
        events: UnboundedSender<Events>,
        mut sending: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
    ) -> Result<(), String> {
        let rt  = match Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(tools::logger.err(&format!("Fail to create runtime executor. Error: {}", e)))
            },
        };
        rt.block_on(async move {
            tools::logger.verb("Runtime is created");
            let addr: String = self.addr.clone();
            let events_cl = events.clone();
            let send_event = move |event: Events| {
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let (tx_tcp_stream, mut rx_tcp_stream): (
                UnboundedSender<TcpStream>,
                UnboundedReceiver<TcpStream>,
            ) = unbounded_channel();
            let streams_task: JoinHandle<Result<(), Errors>> = spawn(async move {
                tools::logger.verb("[task: streams]:: started");
                let listener = match TcpListener::bind(addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        return Err(Errors::Create(format!(
                            "Fail to start server due error {}",
                            e
                        )));
                    }
                };
                send_event(Events::Ready);
                loop {
                    let stream = match listener.accept().await {
                        Ok((stream, _addr)) => {
                            tools::logger.warn(&format!("Getting request to connect from: {}", _addr));
                            // TODO: middleware to confirm acception
                            stream
                        }
                        Err(e) => {
                            send_event(Events::ServerError(Errors::AcceptStream(
                                tools::logger.warn(&format!("Cannot accept connection. Error: {}", e)),
                            )));
                            continue;
                        }
                    };
                    if let Err(e) = tx_tcp_stream.send(stream) {
                        send_event(Events::ServerError(Errors::AcceptStream(
                            tools::logger.warn(&format!("Cannot share stream. Error: {}", e)),
                        )));
                        break;
                    }
                }
                Ok(())
            });
            let events_cl = events.clone();
            let send_event = move |event: Events| {
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let controlls = self.controlls.clone();
            let connection_events = events.clone();
            let (tx_messages, mut rx_messages): (
                UnboundedSender<Messages>,
                UnboundedReceiver<Messages>,
            ) = unbounded_channel();
            let accepting_task: JoinHandle<Result<(), Errors>> = spawn(async move {
                tools::logger.verb("[task: accepting]:: started");
                while let Some(stream) = rx_tcp_stream.recv().await {
                    tools::logger.debug("New stream has been gotten");
                    let ws = match accept_hdr_async(stream, |req: &Request, response: Response| {
                        Handshake::accept(req, response)
                    })
                    .await
                    {
                        Ok(ws) => ws,
                        Err(e) => {
                            tools::logger.warn(&format!("Fail to accept stream due error: {:?}", e));
                            continue;
                        }
                    };
                    tools::logger.debug("Connection has been accepted");
                    let uuid = Uuid::new_v4();
                    let control = match Connection::new(uuid).attach(ws, connection_events.clone(), tx_messages.clone()).await {
                        Ok(control) => control,
                        Err(e) => {
                            send_event(Events::ServerError(Errors::CreateWS(
                                tools::logger
                                    .warn(&format!("Cannot create ws connection. Error: {}", e)),
                            )));
                            continue;
                        }
                    };
                    match controlls.write() {
                        Ok(mut controlls) => {
                            controlls.entry(uuid).or_insert(control);
                            tools::logger.debug("Controll of connection has been added");
                            send_event(Events::Connected(uuid.clone()));
                        },
                        Err(e) => {
                            send_event(Events::ServerError(Errors::CreateWS(
                                tools::logger.err(&format!("Fail get controlls due error: {}", e)),
                            )));
                            continue;
                        }
                    };
                }
                Ok(())
            });
            let controlls = self.controlls.clone();
            let events_cl = events.clone();
            let send_event = move |event: Events| {
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let messages_task: JoinHandle<Result<(), Errors>> = spawn(async move {
                tools::logger.verb("[task: messages]:: started");
                while let Some(msg) = rx_messages.recv().await {
                    match msg {
                        Messages::Binary { uuid, buffer } => send_event(Events::Received(uuid, buffer)),
                        Messages::Disconnect { uuid, code } => {
                            match controlls.write() {
                                Ok(mut controlls) => {
                                    if let Some(_control) = controlls.remove(&uuid) {
                                        tools::logger.debug(&format!("{}:: Channel of connection has been removed", uuid));
                                        send_event(Events::Disconnected(uuid.clone()));
                                    } else {
                                        tools::logger.err(&format!("{}:: Fail to find channel of connection to remove it", uuid));
                                    }
                                },
                                Err(e) => send_event(Events::Error(Some(uuid), tools::logger.err(&format!("{}:: Cannot get access to controllers. Error: {}", uuid, e)))),
                            };
                        },
                        Messages::Error { uuid, error } => send_event(Events::Error(Some(uuid), format!("{:?}", error).to_string()))
                    }
                }
                Ok(())
            });
            let controlls = self.controlls.clone();
            let events_cl = events.clone();
            let send_event = move |event: Events| {
                if let Err(e) = events_cl.send(event) {
                    tools::logger.warn(&format!("Cannot send event. Error: {}", e));
                }
            };
            let sender_task: JoinHandle<Result<(), Errors>> = spawn(async move {
                tools::logger.verb("[task: sender]:: started");
                while let Some((buffer, uuid)) = sending.recv().await {
                    match controlls.write() {
                        Ok(mut controlls) => {
                            if let Some(uuid) = uuid {
                                if let Some(control) = controlls.get_mut(&uuid) {
                                    if let Err(e) = control.send(Control::Send(buffer)) {
                                        send_event(Events::Error(Some(uuid), tools::logger.err(&format!("{}:: Fail to close connection due error: {}", uuid, e))))
                                    }
                                }
                            } else {
                                for (uuid, control) in controlls.iter_mut() {
                                    if let Err(e) = control.send(Control::Send(buffer.clone())) {
                                        send_event(Events::Error(Some(*uuid), tools::logger.err(&format!("{}:: Fail to close connection due error: {}", uuid, e))))
                                    }
                                }
                            }               
                        },
                        Err(e) => {
                            send_event(Events::Error(None, tools::logger.err(&format!("Cannot get access to controllers. Error: {}", e))));
                            break;
                        },
                    };
                }
                Ok(())
            });


            join!(
                streams_task,
                accepting_task,
                messages_task,
                sender_task
            );
        });
        Ok(())
    }
}
