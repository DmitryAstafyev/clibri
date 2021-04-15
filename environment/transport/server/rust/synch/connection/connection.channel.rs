use uuid::Uuid;
use tungstenite::protocol::frame::coding::CloseCode;

#[derive(Debug, Clone)]
pub enum Error {
    Parsing(String),
    ReadSocket(String),
    Channel(String),
}

#[derive(Debug, Clone)]
pub enum Messages {
    Error { uuid: Uuid, error: Error },
    Disconnect { uuid: Uuid, code: Option<CloseCode> },
    Binary { uuid: Uuid, buffer: Vec<u8> },
}