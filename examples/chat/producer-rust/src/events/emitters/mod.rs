pub mod connected;
pub mod disconnected;
pub mod error;

use crate::{producer::Control, protocol};
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

pub fn broadcast(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &Control,
) -> Result<(), EmitterError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .map_err(|e| EmitterError::Processing(e.to_string()))?;
    Ok(())
}