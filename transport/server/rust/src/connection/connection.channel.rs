use uuid::Uuid;
use tungstenite::protocol::CloseFrame;

pub enum Error {
    Parsing(String),
    ReadSocket(String),
    Channel(String),
}

pub enum Messages {
    Error { uuid: Uuid, error: Error },
    Disconnect { uuid: Uuid, frame: Option<CloseFrame<'static>> },
    Binary { uuid: Uuid, buffer: Vec<u8> },
    Text { uuid: Uuid, text: String }
}