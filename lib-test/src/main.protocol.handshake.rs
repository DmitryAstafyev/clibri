use serde::{Deserialize, Serialize};
use fiber::{msg_income_extractor};
use msg_income_extractor::Extractor;

pub const PAYLOAD_LIMIT: u32 = 1000;

pub const ID: u32 = 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandshakeStruct {}

pub struct Handshake {}

impl Extractor<'_> for Handshake {
    type Msg = HandshakeStruct;
}