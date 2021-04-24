use uuid::Uuid;
use tokio_tungstenite::{
    tungstenite::{
        protocol::{
            frame::{
                coding::{
                    CloseCode
                }
            }
        },
    }
};

#[derive(Debug, Clone)]
pub enum Error {
    Parsing(String),
    ReadSocket(String),
    WriteSocket(String),
    Channel(String),
}

#[derive(Debug, Clone)]
pub enum Messages {
    Error { uuid: Uuid, error: Error },
    Disconnect { uuid: Uuid, code: Option<CloseCode> },
    Binary { uuid: Uuid, buffer: Vec<u8> },
}

#[derive(Debug, Clone)]
pub enum Control {
    Send(Vec<u8>),
    Disconnect,
}
