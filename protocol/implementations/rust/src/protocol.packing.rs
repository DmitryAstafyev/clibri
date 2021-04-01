use super::encode::{ StructEncode, EnumEncode };
use super::{ sizes };
use bytes::{ Buf };
use std::io::Cursor;
use std::convert::TryFrom;
use std::time::{ SystemTime, UNIX_EPOCH };

// injectable
const MSG_HEADER_LEN: usize =   sizes::U32_LEN + // {u32} message ID
                                sizes::U16_LEN + // {u16} signature
                                sizes::U32_LEN + // {u32} sequence
                                sizes::U64_LEN + // {u64} body size
                                sizes::U64_LEN;  // {u64} timestamp

#[derive(Debug, Clone)]
pub struct PackageHeader {
    pub id: u32,
    pub signature: u16,
    pub sequence: u32,
    pub len: u64,
    pub ts: u64,
    pub len_usize: usize,
}

pub fn has_buffer_header(buf: &[u8]) -> bool {
    buf.len() >= MSG_HEADER_LEN
}

pub fn get_header_from_buffer(buf: &[u8]) -> Result<PackageHeader, String> {
    let mut header = Cursor::new(buf);
    if buf.len() < MSG_HEADER_LEN {
        return Err(format!("Cannot extract header of package because size of header {} bytes, but size of buffer {} bytes.", MSG_HEADER_LEN, buf.len()));
    }
    // Get message id
    let id: u32 = header.get_u32_le();
    // Get signature
    let signature: u16 = header.get_u16_le();
    // Get sequence
    let sequence: u32 = header.get_u32_le();
    // Get timestamp
    let ts: u64 = header.get_u64_le();
    // Get length of payload and payload
    let len: u64 = header.get_u64_le();
    let len_usize = match usize::try_from(len) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };
    Ok(PackageHeader { id, signature, sequence, ts, len, len_usize })
}

pub fn has_buffer_body(buf: &[u8], header: &PackageHeader) -> bool {
    buf.len() >= header.len_usize + MSG_HEADER_LEN
}

pub fn get_body_from_buffer(buf: &[u8], header: &PackageHeader) -> Result<(Vec<u8>, Vec<u8>), String> {
    if buf.len() < header.len_usize + MSG_HEADER_LEN {
        return Err(format!("Cannot extract body of package because size in header {} bytes, but size of buffer {} bytes.", header.len, buf.len() - MSG_HEADER_LEN));
    }
    // Get body
    let mut body = vec![0; header.len_usize];
    body.copy_from_slice(&buf[MSG_HEADER_LEN..(MSG_HEADER_LEN + header.len_usize)]);
    let mut rest = vec![0; buf.len() - MSG_HEADER_LEN - header.len_usize];
    rest.copy_from_slice(&buf[(MSG_HEADER_LEN + header.len_usize)..]);
    Ok((body, rest))
}

pub fn pack<T>(mut msg: T, sequence: u32) -> Result<Vec<u8>, String> where T: StructEncode {
    match msg.abduct() {
        Ok(buffer) => pack_buffer(msg.get_id(), msg.get_signature(), sequence, buffer),
        Err(e) => Err(e),
    }
}

pub fn pack_buffer(msg_id: u32, signature: u16, sequence: u32, msg_buf: Vec<u8>) -> Result<Vec<u8>, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let mut buf: Vec<u8> = vec!();
            buf.append(&mut msg_id.to_le_bytes().to_vec());
            buf.append(&mut signature.to_le_bytes().to_vec());
            buf.append(&mut sequence.to_le_bytes().to_vec());
            buf.append(&mut duration.as_secs().to_le_bytes().to_vec());
            buf.append(&mut (msg_buf.len() as u64).to_le_bytes().to_vec());
            buf.append(&mut msg_buf.to_vec());
            Ok(buf)
        },
        Err(e) => Err(e.to_string()),
    }
}

pub trait PackingStruct: StructEncode {

    fn pack(&mut self, sequence: u32) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf),
            Err(e) => Err(e),
        }
    }

}

pub trait PackingEnum: EnumEncode {

    fn pack(&mut self, sequence: u32) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf),
            Err(e) => Err(e),
        }
    }

}
