use super::{
    channel::{self},
    channel::{
        Messages
    },
    tools
};
use async_tungstenite::{
    WebSocketStream,
    tungstenite::{
        protocol::{
            Message
        },
        protocol::{
            CloseFrame
        },
        error::{
            Error
        }
    }
};
use async_std::{
    prelude::*,
    net::{
        TcpStream
    }
};
use std::sync::{
    Arc,
    RwLock,
};
use async_channel::{
    Sender,
};
use fiber:: {
    logger::Logger,
};
use uuid::Uuid;
use std::io::{self};
use futures::{
    SinkExt
};

pub struct Connection {
    uuid: Uuid,
    socket: Arc<RwLock<WebSocketStream<TcpStream>>>,
}

impl Connection {

    pub fn new(socket: WebSocketStream<TcpStream>) -> Self {
        Connection {
            uuid: Uuid::new_v4(),
            socket: Arc::new(RwLock::new(socket))
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub async fn attach (
        &mut self,
        channel: Sender<Messages>,
    ) -> Result<(), String> {
        let socket = self.socket.clone();
        let uuid = self.uuid.clone();
        tools::logger.debug(&format!("{}:: start listening client", uuid));
        match socket.write() {
            Ok(mut socket) => {
                let mut connection_error: Option<channel::Error> = None;
                let mut disconnect_frame: Option<CloseFrame> = None;
                while let Some(msg) = socket.next().await {
                    match msg {
                        Ok(msg) => {
                            if msg.is_binary() {
                                tools::logger.verb(&format!("{}:: binary data {:?}", uuid, msg));
                            }
                            match msg {
                                Message::Binary(buffer) => {
                                    match channel.send(Messages::Binary {
                                        uuid,
                                        buffer,
                                    }).await {
                                        Ok(_) => {},
                                        Err(e) => {
                                            tools::logger.err(&format!("{}:: fail to send data to session due error: {}", uuid, e));
                                            connection_error = Some(channel::Error::Channel(format!("{}", e)));
                                        },
                                    };
                                },
                                Message::Close(close_frame) => {
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
                            Error::Io(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                                // No need to do something. There are just no data to read
                                // ????
                            },
                            err => {
                                connection_error = Some(channel::Error::ReadSocket(err.to_string()));
                                tools::logger.err(&format!("{}:: fail read message due error: {}", uuid, err));
                                break;
                            }
                        }
                    }
                }
                tools::logger.debug(&format!("{}:: exit from socket listening loop.", uuid));
                if let Some(error) = connection_error {
                    match channel.send(Messages::Error { uuid, error }).await {
                        Ok(_) => tools::logger.debug(&format!("{}:: client would be disconnected", uuid)),
                        Err(e) => tools::logger.err(&format!("{}:: fail to notify server about disconnecting due error: {}", uuid, e)),
                    };
                }
                let code = if let Some(f) = disconnect_frame {
                    Some(f.code)
                } else {
                    None
                };
                match channel.send(Messages::Disconnect { uuid, code }).await {
                    Ok(_) => tools::logger.debug(&format!("{}:: client would be disconnected", uuid)),
                    Err(e) => tools::logger.err(&format!("{}:: fail to notify server about disconnecting due error: {}", uuid, e)),
                };
                tools::logger.debug(&format!("{}:: closing socket thread", uuid));
            },
            Err(e) => {
                tools::logger.warn(&format!("{}:: probably socket is busy; cannot get access due error: {}", uuid, e));
            }
        };
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn send(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let socket = self.socket.clone();
        tools::logger.debug(&format!("{}:: try to get access to socket", self.uuid));
        let result = match socket.write() {
            Ok(mut socket) => {
                tools::logger.debug(&format!("{}:: access to socket has been gotten", self.uuid));
                match socket.send(Message::from(buffer)).await {
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

    pub async fn close(&mut self) -> Result<(), String> {
        match self.socket.write() {
            Ok(mut socket) => {
                tools::logger.debug(&format!("{}:: would close connection", self.uuid));
                match socket.close(None).await {
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