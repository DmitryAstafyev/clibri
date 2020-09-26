use super::{ protocol };
use bytes::{Buf};
use std::io::Cursor;
use std::str;
use log::{error, warn, trace};
use uuid::Uuid;
use std::convert::TryFrom;

const MSG_HEADER_LEN: usize =   4 + // {u32} message ID
                                4 + // {u32} payload size
                                8;  // {u64} timestamp

#[derive(Debug, Clone)]
pub struct MessageHeader {
    id: u32,
    len: u32,
    ts: u64,
    len_usize: usize,
}

pub enum ReadError {
    Header(String),
    Parsing(String),
}

pub struct Processor<T: Send + Sync + Clone + 'static>{
    uuid: Uuid,
    buffer: Vec<u8>,
    queue: Vec<IncomeMessage<T>>,
}

#[derive(Clone)]
pub struct IncomeMessage<T: Send + Sync + Clone + 'static> {
    pub header: MessageHeader,
    pub msg: T,
}

impl<T: Send + Sync + Clone + 'static> Processor<T> {

    fn get_header(
        &self,
        buf: &[u8],
        protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
    ) -> Result<MessageHeader, String> {
        let mut header = Cursor::new(buf);
        // Get message id
        let id: u32 = header.get_u32_le();
        // Get timestamp
        let ts: u64 = header.get_u64_le();
        // Get length of payload and payload
        let len: u32 = header.get_u32_le();
        // Check payload limit and message ID
        let limit: u32 = match protocol.get_payload_limit(id) {
            Ok(lim) => lim,
            Err(e) => {
                return Err(format!("Fail to detect payload limit for message id '{}' due error: {}", id, e));
            },
        };
        if limit < len {
            return Err(format!("Message id '{}' has payload size {}, but limit is {}", id, len, limit));
        }
        match usize::try_from(len) {
            Ok(len_usize) => Ok(MessageHeader { id, ts, len, len_usize }),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_message(
        &self,
        header: &MessageHeader,
        buf: &[u8],
        protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
    ) -> Result<T, String> {
        match str::from_utf8(buf) {
            Ok(payload) => {
                trace!("{}:: has been gotten message ID {}, payload len: {}, payload: {}", self.uuid, header.id, header.len, payload);
                match protocol.get_msg(header.id, payload) {
                    Ok(msg) => Ok(msg),
                    Err(e) => Err(format!("Fail get message ID={} due error: {}", header.id, e))
                }
            },
            Err(e) => {
                Err(format!("Fail to get string due error: {:?}", e))
            },
        }
    }

    pub fn new(uuid: Uuid) -> Self {
        Processor {
            uuid,
            buffer: vec!(),
            queue: vec!(),
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn read(
        &mut self,
        buf: &Vec<u8>,
        protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
    ) -> Result<(), ReadError> {
        // Add data into buffer
        self.buffer.append(&mut buf.clone());
        if self.buffer.len() < MSG_HEADER_LEN {
            return Ok(());
        }
        // Get header
        match Self::get_header(&self, &self.buffer[0..MSG_HEADER_LEN], protocol.clone()) {
            Ok(header) => {
                trace!("{}:: reading... len: {}, buffer: {}", self.uuid, header.len, self.buffer.len());
                if header.len_usize > (self.buffer.len() - MSG_HEADER_LEN) {
                    return Ok(());
                }
                // Get payload buffer
                let mut payload = vec![0; header.len_usize];
                payload.copy_from_slice(&self.buffer[MSG_HEADER_LEN..(MSG_HEADER_LEN + header.len_usize)]);
                // Remove message body from stored buffer
                self.buffer = self.buffer.drain((MSG_HEADER_LEN + header.len_usize)..).collect();
                // Parse message
                match Self::get_message(self, &header, &payload, protocol.clone()) {
                    Ok(msg) => {
                        self.queue.push(IncomeMessage {
                            header,
                            msg,
                        });
                        if !self.buffer.is_empty() {
                            self.read(&vec!(), protocol)
                        } else {
                            Ok(())
                        }
                    },
                    Err(e) => {
                        error!("{}:: fail parse message due error: {}", self.uuid, e);
                        Err(ReadError::Parsing(e))
                    },
                }
            },
            Err(e) => {
                warn!("{}:: fail get header due error: {}", self.uuid, e);
                Err(ReadError::Header(e))
            },
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<IncomeMessage<T>> {
        if self.queue.is_empty() {
            return None;
        }
        let message = Some(self.queue[0].clone());
        if self.queue.len() > 1 {
            self.queue = self.queue.drain(1..).collect();
        } else {
            self.queue.clear();
        }
        message
    }

}
