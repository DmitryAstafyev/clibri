use serde::{Deserialize, Serialize};
use fiber::{msg_outgoing_builder};
use msg_outgoing_builder::Message;

pub const PAYLOAD_LIMIT: u64 = 1000;

pub const ID: u32 = 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingOutStruct {}

pub struct PingOut {}

impl Message for PingOut {
    
    type Msg = PingOutStruct;

    fn get_msg_id(&self) -> u32 {
        ID
    }

    fn get_payload_limit(&self) -> u64 {
        PAYLOAD_LIMIT
    }

    fn get_msg(&self) -> PingOutStruct {
        PingOutStruct {}
    }
}