use super::{ protocol };
use fiber_protocol_rs::decode::{ StructDecode };
use fiber_protocol_rs::storage::{ Storage };

#[path = "./main.protocol.ping.rs"]
pub mod ping;

#[derive(Debug, Clone)]
pub enum Messages {
    Ping(ping::Ping),
}

#[derive(Debug, Clone)]
pub struct TestProtocol {

}

impl protocol::Protocol<Messages> for TestProtocol {

    fn get_msg(&self, id: u32, buffer: &[u8]) -> Result<Messages, String> {
        let storage = match Storage::new(buffer.to_vec()) {
            Ok(storage) => storage,
            Err(e) => { return Err(e); }
        };
        if id == ping::Ping::get_id() {
            let mut msg: ping::Ping = ping::Ping::defaults();
            match msg.extract(storage) {
                Ok(_) => Ok(Messages::Ping(msg)),
                Err(e) => Err(e)
            }
        } else {
            Err(format!("Invalid id \"{:?}\"", id))
        }
    }

}