use clibri::client;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ConsumerError<E: client::Error> {
    #[error("fail to read income buffer: `{0}`")]
    BufferError(String),
    #[error("unknown message: `{0}`")]
    UnknownMessage(String),
    #[error("unexpected response: `{0}`")]
    UnexpectedResponse(String),
    #[error("protocol error: `{0}`")]
    Protocol(String),
    #[error("error during broadcast handeling: `{0}`")]
    Broadcast(String),
    #[error("unknown broadcast message: `{0}`")]
    UnknownBroadcast(String),
    #[error("error on handshake: `{0}`")]
    Handshake(String),
    #[error("error on hash check: `{0}`")]
    HashCheck(String),
    #[error("fail to accept message as pending: `{0}`")]
    Pending(String),
    #[error("API channel error: `{0}`")]
    APIChannel(String),
    #[error("Client channel error: `{0}`")]
    ClientChannel(String),
    #[error("fail to get response")]
    GettingResponse,
    #[error("fail to parse uuid")]
    Uuid,
    #[error("Invalid sequence: `{0}`")]
    Sequence(String),
    #[error("client error: `{0}`")]
    Client(E),
    #[error("No client available")]
    NoClient,
    #[error("Timeout")]
    Timeout,
}
