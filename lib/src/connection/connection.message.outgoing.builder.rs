use std::convert::TryFrom;
use std::time::{ SystemTime, UNIX_EPOCH };

pub trait MessageHeader {
    fn get_msg_id(&self) -> u32;
    fn get_payload_limit(&self) -> u64;
}

pub trait Message {

    type Msg: serde::Serialize;

    fn new(msg: Self::Msg) -> Self;
    fn get_msg_id(&self) -> u32;
    fn get_payload_limit(&self) -> u64;
    fn get_msg(&self) -> Self::Msg;

    fn get_header(&mut self, len: usize) -> Result<Vec<u8>, String> {
        match u32::try_from(len) {
            Ok(len) => {
                if len as u64 > self.get_payload_limit() {
                    return Err(format!("Outgoing message ID:{} ::Attempt to send payload (size: {} bytes) with limit {} bytes", self.get_msg_id(), len, self.get_payload_limit()));
                }
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => {
                        let mut buf: Vec<u8> = vec!();
                        buf.append(&mut self.get_msg_id().to_le_bytes().to_vec());
                        buf.append(&mut duration.as_secs().to_le_bytes().to_vec());
                        buf.append(&mut len.to_le_bytes().to_vec());
                        Ok(buf)
                    },
                    Err(e) => Err(e.to_string()),
                }

            },
            Err(e) => Err(e.to_string()),
        }
    }

    fn buffer(&mut self) -> Result<Vec<u8>, String> {
        match serde_json::to_string(&self.get_msg()) {
            Ok(str) => {
                let bytes = str.as_bytes();
                match self.get_header(bytes.len()) {
                    Ok(mut header) => {
                        let mut buf: Vec<u8> = vec!();
                        buf.append(&mut header);
                        buf.append(&mut bytes.to_vec());
                        Ok(buf)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

}