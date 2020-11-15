use fiber:: { protocol, storage, decode };
use storage::{ Storage };
use decode::{ StructDecode };

#[path = "./main.protocol.ping.rs"]
mod ping;

#[derive(Debug, Clone)]
pub enum Messages {
    Ping(ping::Ping),
}

#[derive(Debug, Clone)]
pub struct Protocol {

}

impl protocol::Protocol<Messages> for Protocol {

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