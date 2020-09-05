use serde::{Deserialize, Serialize};
use fiber::{msg_income_extractor};
use msg_income_extractor::Extractor;

pub const PAYLOAD_LIMIT: u32 = 1000;

pub const ID: u32 = 1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingStruct {
    pub guid: String,
}

pub struct Ping {}

impl Extractor<'_> for Ping {
    type Msg = PingStruct;
}