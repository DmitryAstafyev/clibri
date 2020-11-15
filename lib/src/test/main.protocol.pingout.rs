use super::{ encode };
use encode::*;

#[derive(Debug, Clone)]
pub struct PingOut {
    pub uuid: String,
}

impl StructEncode for PingOut {

    fn get_id(&self) -> u32 {
        2
    }

    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.encode(1) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        Ok(buffer)
    }

}