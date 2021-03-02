use super::{connection, connection_channel, connection_context, ErrorResponse, Request, Response};
use connection::Connection;
use connection_context::ConnectionContext;
use fiber::server::events::ServerEvents;
use fiber::server::server::Server as ServerTrait;

use log::{debug, error, warn};
use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use tungstenite::accept_hdr;
use tungstenite::protocol::WebSocket;
use uuid::Uuid;

#[derive(Clone)]
pub enum ServerHeartbeat {
    Stop,
    Error(String),
}

// #[derive(Copy, Clone)]
pub struct Server {
    addr: String,
    connections: Arc<RwLock<HashMap<Uuid, Connection>>>,
    handshake: Option<
        Arc<
            RwLock<
                dyn (Fn(&Request, Response) -> Result<Response, ErrorResponse>)
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    heartbeat: (Sender<ServerHeartbeat>, Receiver<ServerHeartbeat>),
}

impl ServerTrait for Server {

    fn listen(&mut self, events: Sender<ServerEvents>, messages: Receiver<(Vec<u8>, Option<Uuid>)>) -> Result<(), String> {
        let (tx_channel, rx_channel): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();
        let addr: String = self.addr.clone();
        let events_sp = events.clone();
        spawn(move || {
            let listener = TcpListener::bind(addr).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        if let Err(e) = tx_channel.send(stream) {
                            if let Err(e) = events_sp.send(ServerEvents::Error(
                                None,
                                format!("{:?}", e).to_string(),
                            )) {
                                error!("Fail to send ServerEvents::Error due error: {}", e);
                            }
                        }
                    }
                    Err(e) => if let Err(e) = events_sp.send(ServerEvents::Error(None, format!("{:?}", e).to_string())) {
                        error!("Fail to send ServerEvents::Error due error: {}", e);
                    },
                }
            }
        });
        let connections = self.connections.clone();
        let (shutdown_tx_channel, shutdown_rx_channel): (Sender<ServerHeartbeat>, Receiver<ServerHeartbeat>) = mpsc::channel();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                if shutdown_rx_channel.try_recv().is_ok() {
                    // We don't care about reasons here
                    break;
                };
                match messages.try_recv() {
                    Ok((buffer, uuid)) => match connections.write() {
                        Ok(mut connections) => {
                            if let Some(uuid) = uuid {
                                if let Some(connection) = connections.get_mut(&uuid) {
                                    if let Err(e) = connection.send(buffer) {
                                        error!("Fail to send buffer to {} due error: {}", uuid, e);
                                    }
                                } else {
                                    warn!("Fail to find connection {}", uuid);
                                }
                            } else {
                                for (uuid, connection) in connections.iter_mut() {
                                    if let Err(e) = connection.send(buffer.clone()) {
                                        error!("Fail to send buffer to {} due error: {}", uuid, e);
                                    };
                                }
                            }
                        },
                        Err(e) => error!("Fail to extract connections to send buffer due error: {}", e),
                    },
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                };
            }
        });
        let timeout = Duration::from_millis(50);
        loop {
            if let Ok(reason) = self.heartbeat.1.try_recv() {
                if let Err(e) = shutdown_tx_channel.send(reason.clone()) {
                    error!("Fail to send shutdown signal due error: {}", e);
                }
                match reason {
                    ServerHeartbeat::Stop => {
                        return Ok(());
                    }
                    ServerHeartbeat::Error(e) => {
                        return Err(e);
                    }
                }
            };
            match rx_channel.try_recv() {
                Ok(stream) => match self.add(stream, events.clone()) {
                    Ok(uuid) => if let Err(e) = events.send(ServerEvents::Connected(uuid)) {
                        error!("Fail to send ServerEvents::Connected due error: {}", e);
                    },
                    Err(e) => if let Err(e) = events.send(ServerEvents::Error(None, format!("{:?}", e).to_string())) {
                        error!("Fail to send ServerEvents::Error due error: {}", e);
                    },
                },
                Err(_) => {
                    // No needs logs here;
                    thread::sleep(timeout);
                }
            }
        }
    }
}

impl Server {
    #[allow(unused_mut)]
    pub fn new(addr: String) -> Self {
        Server {
            addr,
            connections: Arc::new(RwLock::new(HashMap::new())),
            handshake: None,
            heartbeat: mpsc::channel(),
        }
    }

    pub fn handshake<H>(&mut self, handler: H) -> Result<(), String>
    where
        H: (Fn(&Request, Response) -> Result<Response, ErrorResponse>) + Send + Sync + 'static,
    {
        if self.handshake.is_some() {
            Err(String::from("Handshake handler is already defined"))
        } else {
            self.handshake = Some(Arc::new(RwLock::new(handler)));
            Ok(())
        }
    }

    pub fn add(&mut self, stream: TcpStream, events: Sender<ServerEvents>) -> Result<Uuid, String> {
        match self.accept(stream) {
            Ok(socket) => {
                let mut conn = connection::Connection::new(socket);
                let mut cx = ConnectionContext {
                    uuid: conn.get_uuid(),
                    connections: self.connections.clone(),
                };
                match self.connections.write() {
                    Ok(mut connections) => {
                        // Register
                        let uuid = conn.get_uuid();
                        let conn = connections.entry(uuid).or_insert(conn);
                        let (tx_channel, rx_channel): (
                            Sender<connection_channel::Messages>,
                            Receiver<connection_channel::Messages>,
                        ) = mpsc::channel();
                        // Listen
                        match conn.listen(tx_channel) {
                            Ok(_) => {
                                self.redirect(events, rx_channel, cx.clone());
                                Ok(cx.get_uuid())
                            }
                            Err(e) => {
                                warn!("Client {} error: {}", uuid, e);
                                Err(format!(
                                    "Fail start listening client {} due error: {}",
                                    uuid, e
                                ))
                            }
                        }
                    }
                    Err(e) => {
                        error!("Fail get connections due error: {}", e);
                        Err(format!("Fail get connections due error: {}", e))
                    }
                }
            }
            Err(e) => {
                error!("Fail accept connection due error: {}", e);
                Err(format!("Fail accept connection due error: {}", e))
            }
        }
    }

    fn redirect(&self, events: Sender<ServerEvents>, rx_channel: Receiver<connection_channel::Messages>, _cx: ConnectionContext) {
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match rx_channel.try_recv() {
                    Ok(msg) => match msg {
                        connection_channel::Messages::Binary { uuid, buffer } => if let Err(e) =
                        events.send(ServerEvents::Received(uuid, buffer))
                        {
                            error!(
                                "Fail to send ServerEvents::Received due error: {}",
                                e
                            );
                        },
                        connection_channel::Messages::Error { uuid, error } => if let Err(e) = events.send(ServerEvents::Error(
                            Some(uuid),
                            format!("{:?}", error).to_string(),
                        )) {
                            error!("Fail to send ServerEvents::Error due error: {}", e);
                        },
                        connection_channel::Messages::Disconnect { uuid, frame: _ } => if let Err(e) = events.send(ServerEvents::Disconnected(uuid)) {
                            error!(
                                "Fail to send ServerEvents::Disconnected due error: {}",
                                e
                            );
                        },
                    },
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                }
            }
            // TODO: remove session / connection
        });
    }

    #[allow(unused_mut)]
    fn accept(&mut self, stream: TcpStream) -> Result<WebSocket<TcpStream>, String> {
        let mut handshake_handler = if let Some(h) = self.handshake.clone() {
            h
        } else {
            return Err(String::from("No handler for handshake"));
        };
        match stream.set_nonblocking(true) {
            Ok(_) => {
                debug!("Stream is switched to nonblocking mode");
                match accept_hdr(stream, |req: &Request, mut response: Response| {
                    debug!("Connection is accepted. Calling controller accept-callback");
                    match handshake_handler.write() {
                        Ok(mut handshake_handler) => match handshake_handler(req, response) {
                            Ok(response) => Ok(response),
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(ErrorResponse::new(Some(e.to_string()))),
                    }
                }) {
                    Ok(socket) => Ok(socket),
                    Err(e) => {
                        warn!(
                            "(accept_hdr) Connection handshake was failed due error: {}",
                            e
                        );
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                warn!("Fail to set stream into nonblocking mode due error: {}", e);
                Err(e.to_string())
            }
        }
    }
}
