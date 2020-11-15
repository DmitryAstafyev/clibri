use fiber::{ decode, encode, storage };
use storage::{ Storage };
use decode::*;
use encode::*;

#[derive(Debug, Clone)]
pub struct Ping {
    pub uuid: String,
}

impl StructDecode for Ping {

    fn get_id() -> u32 { 2 }

    fn defaults() -> Self {
        Ping { uuid: String::from("") }
    }

    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.uuid = match String::decode(&mut storage, 1) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }

}

impl StructEncode for Ping {

    fn get_id(&self) -> u32 { 2 }

    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.encode(1) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        Ok(buffer)
    }

}