use super::{ connection, connection_channel, controller, Request, Response, ErrorResponse };
use controller::{ Controller };
use std::{sync::PoisonError, time::{ Duration }};
use std::net::{ TcpStream };
use log::{ error, warn, debug };
use std::collections::{ HashMap };
use connection::{ Connection };
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc;
use std::thread;
use std::thread::spawn;
use std::sync::{ Arc, RwLock, RwLockWriteGuard };
use tungstenite::accept_hdr;
use tungstenite::protocol::WebSocket;

// #[derive(Copy, Clone)]
pub struct Server {
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    controller: Arc<RwLock<dyn Controller + Send + Sync + 'static>>,
}

impl Server {
    
    #[allow(unused_mut)]
    pub fn new(stream: TcpStream, mut con: impl Controller + 'static) -> Self {
        Server {
            connections: Arc::new(RwLock::new(HashMap::new())),
            controller: Arc::new(RwLock::new(con)),
        }
    }

    pub fn add( &mut self,
                stream: TcpStream) -> Result<(), String> {
        match self.accept(stream) {
            Ok(socket) => {
                let mut conn = connection::Connection::new(socket);
                let cx = SessionContext {
                    uuid: conn.get_uuid(),
                    connections: self.connections.clone(),
                };
                match self.connections.write() {
                    Ok(mut connections) => {
                        // Register
                        let uuid = conn.get_uuid();
                        let conn = connections.entry(uuid.clone()).or_insert(conn);
                        let (tx_channel, rx_channel): (Sender<connection_channel::Messages>, Receiver<connection_channel::Messages>) = mpsc::channel();
                        // Listen
                        match conn.listen(tx_channel) {
                            Ok(_) => {
                                self.redirect(rx_channel, cx);
                                Ok(())
                            },
                            Err(e) => {
                                warn!("Client {} error: {}", uuid, e);
                                Err(format!("Fail start listening client {} due error: {}", uuid, e))
                            },
                        }
                    },
                    Err(e) => {
                        error!("Fail get connections due error: {}", e);
                        Err(format!("Fail get connections due error: {}", e))
                    }
                }
            },
            Err(e) => {
                error!("Fail accept connection due error: {}", e);
                Err(format!("Fail accept connection due error: {}", e))
            },
        }
    }

    fn redirect(
        &self,
        rx_channel: Receiver<connection_channel::Messages>,
        cx: SessionContext
    ) {
        let contrl = self.controller.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match rx_channel.try_recv() {
                    Ok(msg) => {
                        match msg {
                            connection_channel::Messages::Binary { uuid, buffer } => {
                                // TODO: send buffer
                            },
                            connection_channel::Messages::Text { uuid, text } => {
                                // TODO: send test
                            },
                            connection_channel::Messages::Error { uuid, error } => {
                                // Report Error
                                /*
                                let err: session::Error = match error {
                                    connection_channel::Error::Parsing(err_msg) => session::Error::Parsing(err_msg),
                                    connection_channel::Error::ReadSocket(err_msg) => session::Error::ReadSocket(err_msg),
                                    connection_channel::Error::Channel(err_msg) => session::Error::Channel(err_msg),
                                };
                                */
                            },
                            connection_channel::Messages::Disconnect { uuid, frame } => {
                                // Report disconnection
                            }
                        }
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
        match stream.set_nonblocking(true) {
            Ok(_) => {
                debug!("Stream is switched to nonblocking mode");
                match accept_hdr(stream, |req: &Request, mut response: Response| {
                    debug!("Connection is accepted. Calling controller accept-callback");
                    match self.controller.write() {
                        Ok(mut controller) => {
                            match controller.handshake(req, response) {
                                Ok(response) => Ok(response),
                                Err(e) => Err(e),
                            }
                        },
                        Err(e) => Err(ErrorResponse::new(Some(e.to_string())))
                    }
                }) {
                    Ok(socket) => Ok(socket),
                    Err(e) => {
                        warn!("(accept_hdr) Connection handshake was failed due error: {}", e);
                        Err(e.to_string())
                    },
                }
            },
            Err(e) => {
                warn!("Fail to set stream into nonblocking mode due error: {}", e);
                Err(e.to_string())
            }
        }
    }

}