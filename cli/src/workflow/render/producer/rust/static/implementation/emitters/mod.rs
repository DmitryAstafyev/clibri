pub mod connected;
pub mod disconnected;
pub mod error;
pub mod user_kickoff;

use super::*;
use clibri::server;
use protocol::PackingStruct;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum EmitterError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("emitting error: `{0}`")]
    Emitting(String),
    #[error("packing error: `{0}`")]
    Packing(String),
}

pub fn pack(
    sequence: &u32,
    uuid: &Uuid,
    msg: &mut dyn PackingStruct,
) -> Result<Vec<u8>, EmitterError> {
    msg.pack(*sequence, Some(uuid.to_string()))
        .map_err(EmitterError::Packing)
}

pub fn unbound_pack(sequence: &u32, msg: &mut dyn PackingStruct) -> Result<Vec<u8>, EmitterError> {
    msg.pack(*sequence, None).map_err(EmitterError::Packing)
}

pub async fn broadcast<E: server::Error, C: server::Control<E> + Send + Clone>(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &producer::Control<E, C>,
) -> Result<(), EmitterError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Processing(e.to_string()))?;
    Ok(())
}
