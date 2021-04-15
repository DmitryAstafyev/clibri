use super::{ connection_channel, tools };
use fiber::logger::{ Logger };
use std::time::{ Duration, Instant };
use std::net::{ TcpStream };
use tungstenite::protocol::Message as ProtocolMessage;
use tungstenite::protocol::{WebSocket, CloseFrame};
use uuid::Uuid;
use std::thread::spawn;
use std::sync::mpsc::{ Sender };
use std::sync::{ Arc, RwLock };
use std::thread;
use std::io::{self};

pub struct Connection {
    pub uuid: Uuid,
    pub heartbeat: Instant,
    socket: Arc<RwLock<WebSocket<TcpStream>>>,
}

impl Connection {

    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Connection {
            uuid,
            heartbeat: Instant::now(),
            socket: Arc::new(RwLock::new(socket)),
        }
    }

    pub fn get_uuid(&mut self) -> Uuid {
        self.uuid.clone()
    }

    pub fn listen (
        &mut self,
        channel: Sender<connection_channel::Messages>,
    ) -> Result<(), String> {
        let socket = self.socket.clone();
        let uuid = self.uuid;
        spawn(move || {
            let timeout = Duration::from_millis(50);
            let mut connection_error: Option<connection_channel::Error> = None;
            let mut disconnect_frame: Option<CloseFrame> = None;
            tools::logger.debug(&format!("{}:: start listening client", uuid));
            loop {
                match socket.write() {
                    Ok(mut socket) => {
                        if !socket.can_read() {
                            tools::logger.debug(&format!("{}:: cannot read socket. Client is disconnected.", uuid));
                            break;
                        }
                        match socket.read_message() {
                            Ok(msg) => {
                                if msg.is_binary() {
                                    tools::logger.verb(&format!("{}:: binary data {:?}", uuid, msg));
                                }
                                match msg {
                                    ProtocolMessage::Binary(buffer) => {
                                        match channel.send(connection_channel::Messages::Binary {
                                            uuid,
                                            buffer,
                                        }) {
                                            Ok(_) => {},
                                            Err(e) => {
                                                tools::logger.err(&format!("{}:: fail to send data to session due error: {}", uuid, e));
                                                connection_error = Some(connection_channel::Error::Channel(format!("{}", e)));
                                            },
                                        };
                                    },
                                    ProtocolMessage::Close(close_frame) => {
                                        if let Some(frame) = close_frame {
                                            disconnect_frame = Some(frame);
                                        }
                                    },
                                    _ => { 
                                        tools::logger.err(&format!("{}:: expected only binary data", uuid));
                                        // break;
                                    },
                                }
                            },
                            Err(e) => match e {
                                tungstenite::error::Error::Io(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                                    // No need to do something. There are just no data to read
                                },
                                err => {
                                    connection_error = Some(connection_channel::Error::ReadSocket(err.to_string()));
                                    tools::logger.err(&format!("{}:: fail read message due error: {}", uuid, err));
                                    break;
                                }
                            }
                        }
                    },
                    Err(e) => { tools::logger.warn(&format!("{}:: probably socket is busy; cannot get access due error: {}", uuid, e)); }
                }
                // Thread should sleep a bit to let "send" method work.
                thread::sleep(timeout);
            };
            tools::logger.debug(&format!("{}:: exit from socket listening loop.", uuid));
            if let Some(error) = connection_error {
                match channel.send(connection_channel::Messages::Error { uuid, error }) {
                    Ok(_) => tools::logger.debug(&format!("{}:: client would be disconnected", uuid)),
                    Err(e) => tools::logger.err(&format!("{}:: fail to notify server about disconnecting due error: {}", uuid, e)),
                };
            }
            let code = if let Some(f) = disconnect_frame {
                Some(f.code)
            } else {
                None
            };
            match channel.send(connection_channel::Messages::Disconnect { uuid, code }) {
                Ok(_) => tools::logger.debug(&format!("{}:: client would be disconnected", uuid)),
                Err(e) => tools::logger.err(&format!("{}:: fail to notify server about disconnecting due error: {}", uuid, e)),
            };
            tools::logger.debug(&format!("{}:: closing socket thread", uuid));
        });
        Ok(())
    }

    #[allow(dead_code)]
    pub fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let socket = self.socket.clone();
        tools::logger.debug(&format!("{}:: try to get access to socket", self.uuid));
        let result = match socket.write() {
            Ok(mut socket) => {
                tools::logger.debug(&format!("{}:: access to socket has been gotten", self.uuid));
                match socket.write_message(ProtocolMessage::from(buffer)) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}:: fail to send message due error: {}", self.uuid, e)),
                }
            },
            Err(e) => {
                tools::logger.err(&format!("{}:: probably socket is busy; cannot get access due error: {}", self.uuid, e));   
                Err(format!("{}:: probably socket is busy; cannot get access due error: {}", self.uuid, e))
            }
        };
        result
    }

    pub fn close(&mut self) -> Result<(), String> {
        match self.socket.write() {
            Ok(mut socket) => {
                tools::logger.debug(&format!("{}:: would close connection", self.uuid));
                match socket.close(None) {
                    Ok(()) => Ok(()),
                    Err(_) => {
                        tools::logger.err(&format!("{}:: fail to close connection", self.uuid));   
                        Err("Fail to close connection".to_owned())
                    }
                }
            },
            Err(e) => {
                tools::logger.err(&format!("{}:: probably socket is busy; fail to close connection due error: {}", self.uuid, e));   
                Err(format!("{}:: probably socket is busy; cannot fail to close connection due error: {}", self.uuid, e))
            }
        }
    }

}