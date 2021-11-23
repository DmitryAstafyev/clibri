pub mod structa;
pub mod structb;
pub mod groupb_structa;
pub mod groupb_groupc_structa;
pub mod groupd_structp;
pub mod triggerbeaconsemitter;
pub mod finishconsumertest;
pub mod connected;
pub mod disconnected;
pub mod error;
pub mod ready;
pub mod shutdown;

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

pub async fn broadcast<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &producer::Control<E, C>,
) -> Result<(), EmitterError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Processing(e.to_string()))?;
    Ok(())
}