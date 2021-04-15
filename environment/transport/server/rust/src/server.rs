use super::{
    tools,
    handshake::{
        Handshake as HandshakeInterface
    },
    connection::{
        Connection
    },
    channel::{
        Messages
    }
};
use fiber:: {
    server::{
        interface::Interface,
        events::Events,
        errors::Errors,
    },
    logger::Logger,
};
use std::sync::{
    Arc,
    RwLock,
};
use async_std::{
    prelude::*,
    net::{
        TcpListener,
        TcpStream
    }
};
use async_channel::{
    self,
    Sender,
    Receiver,
};
use async_tungstenite::{
    accept_hdr_async,
    WebSocketStream,
    tungstenite::{
        handshake::server::{
            Request,
            Response,
        },
        protocol::frame::coding::CloseCode
    }
};
use futures::{
    executor,
};
use std::{
    collections::{
        HashMap
    }
};

use uuid::Uuid;
use std::thread::spawn;

pub struct Handshake;

impl HandshakeInterface for Handshake {

}

pub struct Server {
    addr: String,
    connections: Arc<RwLock<HashMap<Uuid, Connection>>>,
}

impl Interface for Server {

    fn listen(&'static mut self, events: Sender<Events>, messages: Receiver<(Vec<u8>, Option<Uuid>)>) -> Result<(), String> {
        let addr: String = self.addr.clone();
        let connections = self.connections.clone();
        spawn(move || {
            if let Err(e) = executor::block_on(async {
                let listener = match TcpListener::bind(addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        return Err(Errors::Create(format!("Fail to start server due error {}", e)));
                    }
                };
                let mut incoming = listener.incoming();
                while let Some(stream) = incoming.next().await {
                    match stream {
                        Ok(stream) => {
                            match self.add(events.clone(), stream).await {
                                Ok(uuid) => if let Err(e) = events.send(Events::Connected(uuid)).await {
                                    tools::logger.err(&format!("Fail to send Events::Connected due error: {}", e));
                                },
                                Err(e) => if let Err(e) = events.send(Events::Error(None, format!("{:?}", e).to_string())).await {
                                    tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                                }
                            }
                        },
                        Err(e) => {
                            tools::logger.err(&format!("Fail to get stream from incoming due error: {}", e));
                        }
                    }
                }
                Ok::<(), Errors>(())
            }) {
                tools::logger.err(&format!("Error during running TcpListener: {:?}", e));
            }
        });
        spawn(move || {
            if let Err(e) = executor::block_on(async {
                loop {
                    // if shutdown_rx_channel.try_recv().is_ok() {
                        // We don't care about reasons here
                    //    break;
                    //};
                    match messages.recv().await {
                        Ok((buffer, uuid)) => match connections.write() {
                            Ok(mut connections) => {
                                let len = buffer.len();
                                if let Some(uuid) = uuid {
                                    if let Some(connection) = connections.get_mut(&uuid) {
                                        if let Err(e) = connection.send(buffer).await {
                                            tools::logger.err(&format!("{}:: fail to send buffer ({} bytes) due error: {}", uuid, len, e));
                                        } else {
                                            tools::logger.debug(&format!("{}:: has been sent {} bytes", uuid, len));
                                        }
                                    } else {
                                        tools::logger.warn(&format!("Fail to find connection {} to send buffer ({} bytes) outside", uuid, len));
                                    }
                                } else {
                                    for (uuid, connection) in connections.iter_mut() {
                                        if let Err(e) = connection.send(buffer.clone()).await {
                                            tools::logger.err(&format!("Fail to send buffer to {} due error: {}", uuid, e));
                                        };
                                    }
                                }
                            },
                            Err(e) => { tools::logger.err(&format!("Fail to extract connections to send buffer due error: {}", e)); },
                        },
                        Err(e) => {
                            tools::logger.warn(&format!("Messages channel empty and closed: {}", e));
                            break;
                        }
                    };
                }
                Ok::<(), String>(())
            }) {
                tools::logger.err(&format!("Fail to start messages channel due error: {:?}", e));
            }
        });
        Ok(())
    }

}

impl Server {

    async fn accept(&self, stream: TcpStream) -> Result<WebSocketStream<TcpStream>, String> {
        match accept_hdr_async(stream, |req: &Request, response: Response|{
            Handshake::accept(req, response)
        }).await {
            Ok(ws) => Ok(ws),
            Err(e) => Err(format!("Fail to accept stream due error: {:?}", e)),
        }
    }

    async fn add(&mut self, events: Sender<Events>, stream: TcpStream) -> Result<Uuid, String> {
        match self.accept(stream).await {
            Ok(ws) => {
                let conn = Connection::new(ws);
                let uuid = conn.get_uuid();
                match self.connections.write() {
                    Ok(mut connections) => {
                        // Register
                        let conn = connections.entry(uuid).or_insert(conn);
                        let (tx_channel, rx_channel): (
                            Sender<Messages>,
                            Receiver<Messages>,
                        ) = async_channel::unbounded();
                        // Listen
                        match conn.attach(tx_channel).await {
                            Ok(_) => {
                                match self.messages(events, rx_channel, uuid).await {
                                    Ok(_) => {
                                        tools::logger.debug(&format!("Active connections: {}", connections.len()));
                                        Ok(uuid)
                                    },
                                    Err(e) => Err(tools::logger.err(&format!("Fail to start listening messages of client {} due error: {}", uuid, e))),
                                }
                            }
                            Err(e) => {
                                tools::logger.err(&format!("{}:: error on attaching {}", uuid, e));
                                if conn.close().await.is_err() {
                                    tools::logger.err(&format!("{}:: fail close connection", uuid));
                                }
                                connections.remove(&uuid);
                                Err(tools::logger.err(&format!("Fail to start listening client {} due error: {}", uuid, e)))
                            }
                        }
                    },
                    Err(e) => Err(tools::logger.err(&format!("Fail get connections due error: {}", e))),
                }
            },
            Err(e) => Err(tools::logger.err(&format!("Fail accept connection due error: {}", e)))
        }
    }

    async fn messages(&self, events: Sender<Events>, messages: Receiver<Messages>, uuid: Uuid) -> Result<(), String> {
        let connections = self.connections.clone();
        spawn(move || {
            if let Err(e) = executor::block_on(async {
                let mut disconnected: Option<(Uuid, Option<CloseCode>)> = None;
                loop {
                    let close = |uuid: Uuid, code: Option<CloseCode>| {
                        Self::close(connections.clone(), uuid, code)
                    };
                    match messages.recv().await {
                        Ok(msg) => match msg {
                            Messages::Binary { uuid, buffer } => if let Err(e) = events.send(Events::Received(uuid, buffer)).await {
                                tools::logger.err(&format!("Fail to send Events::Received due error: {}", e));
                            } else {
                                tools::logger.debug(&format!("{}:: [Messages::Binary] event is gotten", uuid));
                            },
                            Messages::Error { uuid, error } => if let Err(e) = events.send(Events::Error(
                                Some(uuid),
                                format!("{:?}", error).to_string(),
                            )).await {
                                tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                            } else {
                                tools::logger.debug(&format!("{}:: [Messages::Error] event is gotten", uuid));
                            },
                            Messages::Disconnect { uuid, code } => {
                                disconnected = Some((uuid, code));
                                if let Err(e) = events.send(Events::Disconnected(uuid)).await {
                                    tools::logger.err(&format!("{}:: Fail to send Events::Disconnected due error: {}", uuid, e));
                                } else {
                                    tools::logger.debug(&format!("{}:: [Messages::Disconnect] event is gotten", uuid));
                                }
                                if close(uuid, code).await.is_err() {
                                    tools::logger.err(&format!("{}:: connection isn't closed", uuid));
                                }
                            },
                        },
                        Err(e) => {
                            if let Some((uuid, code)) = disconnected {
                                tools::logger.debug(&format!("{}:: closing receiver thread. Code: {:?}", uuid, code));
                            } else {
                                tools::logger.err(&format!("Fail to receive connection message due error: {}", e));
                                if close(uuid, None).await.is_err() {
                                    tools::logger.err(&format!("{}:: connection isn't closed", uuid));
                                }
                            }
                            break;
                        }
                    }
                }
                Ok::<(), String>(())
            }) {
                tools::logger.err(&format!("Fail to start listen messages of connection {} message due error: {}", uuid, e));
            }
        });
        Ok(())
    }

    async fn close(connections: Arc<RwLock<HashMap<Uuid, Connection>>>, uuid: Uuid, code: Option<CloseCode>) -> Result<(), String> {
        match connections.write() {
            Ok(mut connections) => {
                if let Some(mut connection) = connections.remove(&uuid) {
                    if if let Some(code) = code {
                        code != CloseCode::Away
                    } else {
                        true
                    } {
                        if let Err(e) = connection.close().await {
                            return Err(tools::logger.err(&format!("{}:: Fail to close connection due error: {}", uuid, e)));
                        } else {
                            tools::logger.debug(&format!("{}:: connection is closed", uuid));
                        }
                    }
                } else {
                    return Err(tools::logger.warn(&format!("{}:: Fail to find connection to close it", uuid)));
                }
                tools::logger.debug(&format!("Active connections: {}", connections.len()));
                Ok(())
            },
            Err(e) => Err(tools::logger.err(&format!("{}:: Fail to close connection. No access to connections: {}", uuid, e))),
        }
    }

}