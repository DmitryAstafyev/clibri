pub mod beacons_likeuser;
pub mod beacons_likemessage;


use super::*;
use protocol::PackingStruct;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmitterError {
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