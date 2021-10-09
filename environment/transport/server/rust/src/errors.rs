use thiserror::Error;

#[derive(Error, Debug, Clone)]
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
	#[error("fail to take api channel")]
	FailTakeAPI,
	#[error("error on body parsing: `{0}`")]
	BodyParsing(String),
	#[error("server error: `{0}`")]
	HttpServer(String),
	#[error("error while distributing: `{0}`")]
	Distributing(String),
	#[error("fail to parse string to SocketAddr: `{0}`")]
	SocketAddr(String),
}
