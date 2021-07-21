use thiserror::Error;

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
}
