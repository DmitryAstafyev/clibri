use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum Error {
	#[error("creating server error: `{0}`")]
	Create(String),
	#[error("accepting stream error: `{0}`")]
	AcceptStream(String),
	#[error("opening WS error: `{0}`")]
	CreateWS(String),
	#[error("fail to close connection: `{0}`")]
	CloseConnection(String),
	#[error("expecting binary data only")]
	NonBinaryData,
	#[error("has been gotten invalid message: `{0}`")]
	InvalidMessage(String),
	#[error("error on channel: `{0}`")]
	Channel(String),
	#[error("observer has been taken already")]
	ObserverAlreadyTaken,
	#[error("fail to take sender")]
	FailTakeSender,
	#[error("fail to take control")]
	FailTakeControl,
	#[error("fail to join task")]
	JoinError(JoinError),
}
