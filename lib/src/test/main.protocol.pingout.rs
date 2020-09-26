use serde::{Deserialize, Serialize};
use super::{msg_outgoing_builder};
use msg_outgoing_builder::Message;

pub const PAYLOAD_LIMIT: u64 = 1000;

pub const ID: u32 = 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingOutStruct {
    pub uuid: String,
}

pub struct PingOut {
    _msg: PingOutStruct,
}

impl Message for PingOut {
    
    type Msg = PingOutStruct;

    fn new(msg: Self::Msg) -> Self {
        PingOut {
            _msg: msg,
        }
    }

    fn get_msg_id(&self) -> u32 {
        ID
    }

    fn get_payload_limit(&self) -> u64 {
        PAYLOAD_LIMIT
    }

    fn get_msg(&self) -> Self::Msg {
        self._msg.clone()
    }

}