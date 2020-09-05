use std::convert::TryFrom;
use std::time::{ SystemTime, UNIX_EPOCH };

#[derive(Debug, Clone)]
pub enum Messages {
    //DisconnectForce(disconnect_force::DisconnectForceStruct),
    //ClientDisconnected(client_disconnected::ClientDisconnectStruct),
}

pub trait MessageHeader {
    fn get_msg_id(&self) -> u32;
    fn get_payload_limit(&self) -> u64;
}

pub trait Message {

    type Msg: MessageHeader + serde::Serialize;

    fn get_header(msg: &Self::Msg, len: usize) -> Result<Vec<u8>, String> {
        match u32::try_from(len) {
            Ok(len) => {
                if len as u64 > msg.get_payload_limit() {
                    return Err(format!("Outgoing message ID:{} ::Attempt to send payload (size: {} bytes) with limit {} bytes", msg.get_msg_id(), len, msg.get_payload_limit()));
                }
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => {
                        let mut buf: Vec<u8> = vec!();
                        buf.append(&mut msg.get_msg_id().to_be_bytes().to_vec());
                        buf.append(&mut duration.as_secs().to_be_bytes().to_vec());
                        buf.append(&mut len.to_be_bytes().to_vec());
                        return Ok(buf);
                    },
                    Err(e) => Err(e.to_string()),
                }

            },
            Err(e) => Err(e.to_string()),
        }
    }

    fn new(msg: Self::Msg) -> Result<Vec<u8>, String> {
        match serde_json::to_string(&msg) {
            Ok(str) => {
                let bytes = str.as_bytes();
                match Self::get_header(&msg, bytes.len()) {
                    Ok(header) => {
                        let mut buf: Vec<u8> = vec!();
                        buf.append(&mut header.clone());
                        buf.append(&mut bytes.to_vec());
                        return Ok(buf);
                    },
                    Err(e) => Err(e.to_string()),
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

}