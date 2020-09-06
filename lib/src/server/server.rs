use super::{ connection, session, connection_channel, protocol, session_context, controller, Request, Response, ErrorResponse };
use session_context::{ SessionContext };
use controller::{ Controller };
use session::{ Session };
use std::time::{ Duration };
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
pub struct Server<T: Send + Sync + Clone + 'static> {
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    sessions: Arc<RwLock<HashMap<String, Box<dyn Session<T> + Send + Sync + 'static>>>>,
    controller: Arc<RwLock<dyn Controller + Send + Sync + 'static>>,
}

impl<T: Send + Sync + Clone + 'static> Server<T> {

    pub fn new(mut con: impl Controller + 'static) -> Self {
        Server {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            controller: Arc::new(RwLock::new(con)),
        }
    }

    pub fn add( &mut self,
                stream: TcpStream,
                mut session: impl Session<T> + 'static,
                protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
                exceptions: Option<impl Fn(session::Error) -> () + Send + Sync + 'static>) -> () {
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
                        let (tx_channel, rx_channel): (Sender<connection_channel::Messages<T>>, Receiver<connection_channel::Messages<T>>) = mpsc::channel();
                        // Listen
                        match conn.listen(tx_channel, protocol) {
                            Ok(_) => {
                                match self.sessions.write() {
                                    Ok(mut sessions) => {
                                        session.connected(cx.clone());
                                        sessions.entry(conn.get_uuid()).or_insert(Box::new(session));
                                        self.redirect(rx_channel, cx.clone(), exceptions);
                                    },
                                    Err(_e) => {
                                        warn!("Cannot get access to session after connection was done.");
                                        if let Some(cb) = exceptions {
                                            cb(session::Error::Session("Cannot get access to session after connection was done.".to_string()));
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                session.error(session::Error::Connection(e.clone()), Some(cx.clone()));
                                warn!("Client {} error: {}", uuid, e);
                            },
                        }
                    },
                    Err(e) => {
                        session.error(session::Error::Connection(e.to_string().clone()), Some(cx.clone()));
                        error!("Fail get connections due error: {}", e);
                    }
                }
            },
            Err(e) => {
                session.error(session::Error::Socket(e.to_string().clone()), None);
                error!("Fail accept connection due error: {}", e);
            },
        }
    }

    fn redirect(
        &self,
        rx_channel: Receiver<connection_channel::Messages<T>>,
        cx: SessionContext,
        exceptions: Option<impl Fn(session::Error) -> () + Send + Sync + 'static>
    ) {
        let sessions = self.sessions.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            let session_access_err = |e: Option<std::sync::PoisonError<RwLockWriteGuard<HashMap<String, Box<dyn Session<T> + Send + Sync>>>>>| {
                match e {
                    Some(e) => {
                        error!("Fail to get sessions object due error: {}", e);
                        if let Some(cb) = exceptions {
                            cb(session::Error::Session(e.to_string()));
                        }
                    }
                    None => {
                        error!("Fail to find target session");
                        if let Some(cb) = exceptions {
                            cb(session::Error::Session("Fail to find target session".to_string()));
                        }
                    }
                }
            };
            loop {
                match rx_channel.try_recv() {
                    Ok(msg) => {
                        match msg {
                            connection_channel::Messages::Message { uuid, msg } => {
                                match sessions.write() {
                                    Ok(mut sessions) => {
                                        if let Some(sess) = sessions.get_mut(&uuid.to_string()) {
                                            match sess.message(msg, cx.clone()) {
                                                Ok(_) => {},
                                                Err(e) => {
                                                    warn!("Message handler returns error: {}. Session/connection would be dropped", e);
                                                    break;
                                                }
                                            }
                                        } else {
                                            session_access_err(None);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        session_access_err(Some(e));
                                        break;
                                    }
                                }
                            },
                            connection_channel::Messages::Text { uuid, text } => {
                                match sessions.write() {
                                    Ok(mut sessions) => {
                                        if let Some(sess) = sessions.get_mut(&uuid.to_string()) {
                                            match sess.text(text, cx.clone()) {
                                                Ok(_) => {},
                                                Err(e) => {
                                                    warn!("Message handler returns error: {}. Session/connection would be dropped", e);
                                                    break;
                                                }
                                            }
                                        } else {
                                            session_access_err(None);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        session_access_err(Some(e));
                                        break;
                                    }
                                }
                            },
                            connection_channel::Messages::Error { uuid, error } => {
                                match sessions.write() {
                                    Ok(mut sessions) => {
                                        if let Some(sess) = sessions.get_mut(&uuid.to_string()) {
                                            let err: session::Error = match error {
                                                connection_channel::Error::Parsing(err_msg) => session::Error::Parsing(err_msg),
                                                connection_channel::Error::ReadSocket(err_msg) => session::Error::ReadSocket(err_msg),
                                                connection_channel::Error::Channel(err_msg) => session::Error::Channel(err_msg),
                                            };
                                            sess.error(err, Some(cx.clone()));
                                        } else {
                                            session_access_err(None);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        session_access_err(Some(e));
                                        break;
                                    }
                                }
                            },
                            connection_channel::Messages::Disconnect { uuid, frame } => {
                                match sessions.write() {
                                    Ok(mut sessions) => {
                                        if let Some(sess) = sessions.get_mut(&uuid.to_string()) {
                                            sess.disconnect(cx.clone(), frame);
                                        } else {
                                            session_access_err(None);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        session_access_err(Some(e));
                                        break;
                                    }
                                }
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

    fn accept(&mut self, stream: TcpStream) -> Result<WebSocket<TcpStream>, String> {
        match stream.set_nonblocking(true) {
            Ok(_) => {
                debug!("Stream is switched to nonblocking mode");
                match accept_hdr(stream, |req: &Request, mut response: Response| {
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
                        warn!("Connection handshake was failed due error: {}", e);
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