use super::{ buffer, connection_channel, protocol };
use std::time::{ Duration, Instant };
use log::{ error, warn, debug, trace };
use std::net::{ TcpStream };
use tungstenite::protocol::Message as ProtocolMessage;
use tungstenite::protocol::{WebSocket, CloseFrame};
use uuid::Uuid;
use std::thread::spawn;
use std::sync::mpsc::{ Sender };
use std::sync::{ Arc, RwLock };
use std::thread;
use std::io::{self};
//use buffer::msg::Outgoing::disconnect_force::{DisconnectForce, DisconnectForceStruct};
//use buffer::msg::Outgoing::{Message};

pub struct Connection {
    pub uuid: Uuid,
    pub heartbeat: Instant,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub alias: Option<String>,
    socket: Arc<RwLock<WebSocket<TcpStream>>>,
}

impl Connection {

    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Connection {
            uuid: uuid.clone(),
            heartbeat: Instant::now(),
            lat: None,
            lon: None,
            alias: None,
            socket: Arc::new(RwLock::new(socket)),
        }
    }

    pub fn get_uuid(&mut self) -> String {
        self.uuid.clone().to_string()
    }

    pub fn listen<T: Send + Sync + Clone + 'static>(
        &mut self,
        tx_channel: Sender<connection_channel::Messages<T>>,
        protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
    ) -> Result<(), String> {
        let socket = self.socket.clone();
        let mut buffer: buffer::Processor<T> = buffer::Processor::new(self.uuid);
        let uuid = self.uuid;
        let channel = tx_channel.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            let mut connection_error: Option<connection_channel::Error> = None;
            let mut disconnect_frame: Option<CloseFrame> = None;
            loop {
                match socket.write() {
                    Ok(mut socket) => {
                        if !socket.can_read() {
                            break;
                        }
                        match socket.read_message() {
                            Ok(msg) => {
                                if msg.is_binary() {
                                    trace!("{}:: binary data {:?}", uuid, msg);
                                }
                                match msg {
                                    ProtocolMessage::Binary(buf) => {
                                        match buffer.read(&buf, protocol.clone()) {
                                            Ok(_) => loop {
                                                match buffer.next() {
                                                    Some(msg) => {
                                                        match channel.send(connection_channel::Messages::Message {
                                                            uuid: uuid.clone(),
                                                            msg: msg.msg,
                                                        }) {
                                                            Ok(_) => break,
                                                            Err(e) => {
                                                                error!("{}:: fail to send data to session due error: {}", uuid, e);
                                                                connection_error = Some(connection_channel::Error::Channel(format!("{}", e).to_string()));
                                                                break;
                                                            },
                                                        };
                                                    },
                                                    None => break
                                                }
                                            },
                                            Err(e) => {
                                                let error: String;
                                                match e {
                                                    buffer::ReadError::Header(err) => {
                                                        error = err;
                                                        warn!("{}:: fail to parse message header due error: {}. Connection will be dropped.", uuid, error);
                                                    },
                                                    buffer::ReadError::Parsing(err) => {
                                                        error = err;
                                                        warn!("{}:: fail to parse message payload due error: {}. Connection will be dropped.", uuid, error);
                                                    },
                                                };
                                                /*
                                                TODO: should be move out here
                                                match DisconnectForce::new(DisconnectForceStruct {
                                                    reason: error.clone(),
                                                }) {
                                                    Ok(buf) => {
                                                        // send message
                                                        match socket.write_message(ProtocolMessage::from(buf)) {
                                                            Ok(_) => debug!("{}:: connection would be dropped", uuid),
                                                            Err(e) => warn!("{}:: fail to send message due error: {}", uuid, e)
                                                        }
                                                    },
                                                    Err(e) => warn!("{}:: fail get message DisconnectForce due error: {}", uuid, e)
                                                };
                                                */
                                                connection_error = Some(connection_channel::Error::Parsing(error));
                                                break;
                                            }
                                        }
                                    },
                                    ProtocolMessage::Text(text) => {
                                        match channel.send(connection_channel::Messages::Text {
                                            uuid: uuid.clone(),
                                            text: text,
                                        }) {
                                            Ok(_) => break,
                                            Err(e) => {
                                                error!("{}:: fail to send data to session due error: {}", uuid, e);
                                                connection_error = Some(connection_channel::Error::Channel(format!("{}", e).to_string()));
                                                break;
                                            },
                                        };
                                    },
                                    /*
                                    ProtocolMessage::Ping(buf: Vec<u8>) => {

                                    },
                                    ProtocolMessage::Pong(buf: Vec<u8>) => {

                                    },*/
                                    ProtocolMessage::Close(close_frame) => {
                                        if let Some(frame) = close_frame {
                                            disconnect_frame = Some(frame);
                                        }
                                    },
                                    _ => { 
                                        error!("{}:: expected only binary data", uuid);
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
                                    error!("{}:: fail read message due error: {}", uuid, err);
                                    break;
                                }
                            }
                        }
                    },
                    Err(e) => warn!("{}:: probably socket is busy; cannot get access due error: {}", uuid, e)
                }
                // Thread should sleep a bit to let "send" method work.
                thread::sleep(timeout);
            }
            if let Some(error) = connection_error {
                match channel.send(connection_channel::Messages::Error { uuid: uuid, error: error }) {
                    Ok(_) => debug!("{}:: client would be disconnected", uuid),
                    Err(e) => error!("{}:: fail to notify server about disconnecting due error: {}", uuid, e),
                };
            }
            match channel.send(connection_channel::Messages::Disconnect { uuid: uuid, frame: disconnect_frame }) {
                Ok(_) => debug!("{}:: client would be disconnected", uuid),
                Err(e) => error!("{}:: fail to notify server about disconnecting due error: {}", uuid, e),
            };
        });
        Ok(())
    }

    #[allow(dead_code)]
    pub fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let socket = self.socket.clone();
        debug!("{}:: try to get access to socket", self.uuid);
        match socket.write() {
            Ok(mut socket) => {
                debug!("{}:: access to socket has been gotten", self.uuid);
                return match socket.write_message(ProtocolMessage::from(buffer)) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}:: fail to send message due error: {}", self.uuid, e)),
                };
            },
            Err(e) => {
                error!("{}:: probably socket is busy; cannot get access due error: {}", self.uuid, e);   
                return Err(format!("{}:: probably socket is busy; cannot get access due error: {}", self.uuid, e));
            }
        };
    }

}