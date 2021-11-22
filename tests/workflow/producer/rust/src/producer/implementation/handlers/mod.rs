pub mod structa;
pub mod structc;
pub mod structd;
pub mod structf;
pub mod structempty;
pub mod groupa_structa;
pub mod groupa_structb;
pub mod groupb_groupc_structa;
pub mod groupb_structa;
pub mod groupb_groupc_structb;

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