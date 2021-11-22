pub mod beacona;
pub mod beacons_beacona;
pub mod beacons_beaconb;
pub mod beacons_sub_beacona;


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