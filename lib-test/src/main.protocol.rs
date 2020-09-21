use fiber:: { protocol, msg_income_extractor };
use msg_income_extractor::Extractor;

#[path = "./main.protocol.ping.rs"]
mod ping;

#[derive(Debug, Clone)]
pub enum Messages {
    Ping(ping::PingStruct),
}

#[derive(Debug, Clone)]
pub struct Protocol {

}

impl protocol::Protocol<Messages> for Protocol {

    fn get_msg(&self, id: u32, payload: &str) -> Result<Messages, String> {
        match id {
            ping::ID => {
                match ping::Ping::new(payload) {
                    Ok(msg) => Ok(Messages::Ping(msg)),
                    Err(e) => Err(format!("Fail to parse \"Ping\" message due error: {}", e)),
                }
            }
            _ => Err(format!("Invalid id \"{:?}\"", id))
        }
    }

    fn get_payload_limit(&self, id: u32) -> Result<u32, String> {
        match id {
            ping::ID => {
                Ok(ping::PAYLOAD_LIMIT)
            }
            _ => Err(format!("Invalid id \"{:?}\"", id))
        }
    }
}