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
    net::{
        TcpListener,
        TcpStream
    },
    prelude::*,
    task::{
        JoinHandle,
        spawn
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
use futures::{StreamExt, executor, join};
use std::{
    collections::{
        HashMap
    },
/*
    sync::{
        mpsc::{
            channel,
            Receiver,
            Sender,
        }
    }
    thread,
*/
};

use uuid::Uuid;

pub struct Handshake;

impl HandshakeInterface for Handshake {

}
#[derive(Clone)]
pub enum Heartbeat {
    Stop,
    Error(String),
}

pub struct Server {
    addr: String,
    // connections: Arc<RwLock<HashMap<Uuid, Connection>>>,
    // heartbeat: (Sender<Heartbeat>, Receiver<Heartbeat>),
}

impl Interface for Server {

    fn listen(&mut self, events: Sender<Events>, messages: Receiver<(Vec<u8>, Option<Uuid>)>) -> Result<(), String> {
        let addr: String = self.addr.clone();
        let connections = self.connections.clone();
        let heartbeat = self.heartbeat.1.clone();
        let (tx_shutdown,   rx_shutdown):   (Sender<Heartbeat>, Receiver<Heartbeat>)    = async_channel::unbounded();
        let (tx_connection, rx_connection): (Sender<TcpStream>, Receiver<TcpStream>)    = async_channel::unbounded();
        let (tx_receiver,   rx_receiver):   (Sender<Messages>,  Receiver<Messages>)     = async_channel::unbounded();
        // Thread: Incomes
        // Catch income connections and send into channel stream
        let events_it = events.clone();
        let incomes_thread: JoinHandle<Result<(), Errors>> = spawn(async move {
            let listener = match TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    return Err(Errors::Create(format!("Fail to start server due error {}", e)));
                }
            };
            listener.incoming().for_each_concurrent(None,|stream| async {
                match stream {
                    Ok(stream) => {
                        if let Err(e) = tx_connection.send(stream).await {
                            if let Err(e) = events_it.send(Events::Error(
                                None,
                                format!("{:?}", e).to_string(),
                            )).await {
                                tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                            }
                        }
                    },
                    Err(e) => {
                        tools::logger.err(&format!("Fail to get stream from incoming due error: {}", e));
                    }
                }
            }).await;
            Ok(())
        });

        let events_at = events.clone();
        let accepting_thread: JoinHandle<Result<(), Errors>> = spawn(async move {
            while let Ok(stream) = rx_connection.recv().await {
                tools::logger.debug("New stream has been gotten");
                let ws = match accept_hdr_async(stream, |req: &Request, response: Response|{
                    Handshake::accept(req, response)
                }).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        tools::logger.warn(&format!("Fail to accept stream due error: {:?}", e));
                        continue;
                    },
                };
                tools::logger.debug("Connection has been accepted");
                let uuid = Uuid::new_v4();
                // Register
                match connections.write() {
                    Ok(mut connections) => connections.entry(uuid).or_insert_with(|| Connection::new(ws, uuid)),
                    Err(e) => {
                        tools::logger.err(&format!("Fail get connections due error: {}", e));
                        // if let Err(e) = events_at.send(Events::Error(None, tools::logger.err(&format!("Fail get connections due error: {}", e)))).await {
                        //     tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                        // }
                        continue;
                    },
                };
                match connections.write() {
                    Ok(mut connections) => {
                        if let Some(connection) = connections.get_mut(&uuid) {
                            connection.get_uuid();
                        }
                    },
                    Err(e) => {
                        ;
                    },
                };
                if let Err(e) = events_at.send(Events::Connected(uuid)).await {
                    tools::logger.err(&format!("Fail to send Events::Connected due error: {}", e));
                    // TODO: disconnect as soon as workflow is broken
                } else {
                    tools::logger.debug(&format!("{} connected", uuid));
                }
        /*
                if let Err(e) = self.add(events_at.clone(), tx_receiver.clone(), stream).await {
                    if let Err(e) = events_at.send(Events::Error(None, format!("{:?}", e).to_string())).await {
                        tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                    }
                    return Err::<(), Errors>(Errors::AddStream(tools::logger.err(&format!("Fail to add connection due error: {}", e))));
                } */
            }
            Ok::<(), Errors>(())
        });
        /* 
        let connections_loop = async move {
            while let Ok(stream) = rx_connection.recv().await {
                tools::logger.debug("New stream has been gotten");
                if let Err(e) = self.add(evns.clone(), tx_receiver.clone(), stream).await {
                    if let Err(e) = evns.send(Events::Error(None, format!("{:?}", e).to_string())).await {
                        tools::logger.err(&format!("Fail to send Events::Error due error: {}", e));
                    }
                    return Err::<(), String>(tools::logger.err(&format!("Fail to add connection due error: {}", e)));
                }
                tools::logger.err(&format!("EXIT ! YEAH"));
            }
            Ok::<(), String>(())
        };
        */
        match executor::block_on(async {
            join!(incomes_thread);
            Ok::<(), String>(())
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
        /*
        spawn(async move {
            loop {
                // if shutdown_rx_channel.try_recv().is_ok() {
                    // We don't care about reasons here
                //    break;
                //};
                tools::logger.debug("Starting listen outgoing messages channel");
                match messages.recv().await {
                    Ok((buffer, uuid)) => match connections.write() {
                        Ok(mut connections) => {
                            let len = buffer.len();
                            tools::logger.debug(&format!("{:?}:: wants to send {} bytes", uuid, len));
                            if let Some(uuid) = uuid {
                                println!("HEHE 2");
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
                                println!("HEHE 1");
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
        });
        let (tx_receiver, rx_receiver): (
            Sender<Messages>,
            Receiver<Messages>,
        ) = async_channel::unbounded();
        if let Err(e) = self.receiver(evns.clone(), rx_receiver) {
            return Err(tools::logger.err(&format!("Fail to listen messages of connection due error: {}", e)));
        }
        let heartbeat_loop = async move {
            loop {
                if let Ok(reason) = heartbeat.recv().await {
                    if let Err(e) = tx_shutdown.send(reason.clone()).await {
                        tools::logger.err(&format!("Fail to send shutdown signal due error: {}", e));
                    }
                    match reason {
                        Heartbeat::Stop => {
                            return Ok::<(), String>(());
                        }
                        Heartbeat::Error(e) => {
                            return Err::<(), String>(e);
                        }
                    }
                };
            }
        };
        
        match executor::block_on(async {
            let result = join!(heartbeat_loop, connections_loop);
            if let Err(e) = result.0 {
                return Err::<(), String>(e);
            }
            if let Err(e) = result.1 {
                return Err::<(), String>(e);
            }
            Ok::<(), String>(())
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
        */
    }

}

impl Server {

    pub fn new(addr: String) -> Self {
        Self {
            addr,
            connections: Arc::new(RwLock::new(HashMap::new())),
            heartbeat: async_channel::unbounded()
        }
    }
/*
    async fn accept(&self, stream: TcpStream) -> Result<WebSocketStream<TcpStream>, String> {
        match accept_hdr_async(stream, |req: &Request, response: Response|{
            Handshake::accept(req, response)
        }).await {
            Ok(ws) => Ok(ws),
            Err(e) => Err(format!("Fail to accept stream due error: {:?}", e)),
        }
    }

    async fn add(&mut self, events: Sender<Events>, receiver: Sender<Messages>, stream: TcpStream) -> Result<(), String> {
        match self.accept(stream).await {
            Ok(ws) => {
                tools::logger.debug("Connection has been accepted");
                let conn = Connection::new(ws);
                let uuid = conn.get_uuid();
                match self.connections.write() {
                    Ok(mut connections) => {
                        // Register
                        let conn = connections.entry(uuid).or_insert(conn);
                        if let Err(e) = events.send(Events::Connected(uuid)).await {
                            return Err(tools::logger.err(&format!("Fail to send Events::Connected due error: {}", e)));
                        } else {
                            tools::logger.debug(&format!("{} connected", uuid));
                        }
                        conn.attach(receiver).await;
                        Ok(())
                    },
                    Err(e) => Err(tools::logger.err(&format!("Fail get connections due error: {}", e))),
                }
            },
            Err(e) => Err(tools::logger.err(&format!("Fail accept connection due error: {}", e)))
        }
    }
*/
/*
    fn receiver(&self, events: Sender<Events>, messages: Receiver<Messages>) -> Result<(), String> {
        let connections = self.connections.clone();
        tools::logger.debug(&format!("out message listener attached"));
        std::thread::spawn(move || {
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
                                // TODO: Close all
                                /*
                                if close(uuid, None).await.is_err() {
                                    tools::logger.err(&format!("{}:: connection isn't closed", uuid));
                                }
                                */
                            }
                            break;
                        }
                    }
                }
                Ok::<(), String>(())
            }) {
                // Error
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
*/
}