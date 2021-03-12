use uuid::Uuid;
use tungstenite::protocol::CloseFrame;

#[derive(Debug, Clone)]
pub enum Error {
    Parsing(String),
    ReadSocket(String),
    Channel(String),
}

#[derive(Debug, Clone)]
pub enum Messages {
    Error { uuid: Uuid, error: Error },
    Disconnect { uuid: Uuid, frame: Option<CloseFrame<'static>> },
    Binary { uuid: Uuid, buffer: Vec<u8> },
}