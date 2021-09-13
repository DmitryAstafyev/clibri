pub mod message;
pub mod messages;
pub mod user_login;
pub mod users;

use crate::{producer::Control, protocol};
use protocol::PackingStruct;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("packing error: `{0}`")]
    Packing(String),
    #[error("response: `{0}`")]
    Response(String),
}

pub fn pack(
    sequence: &u32,
    uuid: &Uuid,
    msg: &mut dyn PackingStruct,
) -> Result<Vec<u8>, HandlerError> {
    msg.pack(*sequence, Some(uuid.to_string()))
        .map_err(HandlerError::Packing)
}

pub fn broadcast(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &Control,
) -> Result<(), HandlerError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .map_err(|e| HandlerError::Processing(e.to_string()))?;
    Ok(())
}
