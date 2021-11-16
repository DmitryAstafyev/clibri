pub mod userlogin_request;
pub mod users_request;
pub mod message_request;
pub mod messages_request;

use super::*;
use clibri::server;
use protocol::PackingStruct;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("packing error: `{0}`")]
    Packing(String),
}

pub fn pack(
    sequence: &u32,
    uuid: &Uuid,
    msg: &mut dyn PackingStruct,
) -> Result<Vec<u8>, HandlerError> {
    msg.pack(*sequence, Some(uuid.to_string()))
        .map_err(HandlerError::Packing)
}

pub async fn broadcast<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &producer::Control<E, C>,
) -> Result<(), HandlerError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    Ok(())
}