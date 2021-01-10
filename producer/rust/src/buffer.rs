use super::{ package, protocol };
use uuid::Uuid;
use package::{ Header };

pub enum ReadError {
    Header(String),
    Parsing(String),
}

pub struct Buffer<T: Send + Sync + Clone + 'static>{
    uuid: Uuid,
    buffer: Vec<u8>,
    queue: Vec<IncomeMessage<T>>,
}

#[derive(Clone)]
pub struct IncomeMessage<T: Send + Sync + Clone + 'static> {
    pub header: Header,
    pub msg: T,
}

impl<T: Send + Sync + Clone + 'static> Buffer<T> {

    fn get_message(
        &self,
        header: &Header,
        buf: &[u8],
        protocol: impl protocol::Protocol<T> + Send + Sync + Clone + 'static,
    ) -> Result<T, String> {
        println!("{}:: has been gotten message ID {}, declared len: {}, actual len: {}", self.uuid, header.id, header.len, buf.len());
        match protocol.get_msg(header.id, buf) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("Fail get message ID={} due error: {}", header.id, e))
        }
    }

    pub fn new(uuid: Uuid) -> Self {
        Buffer {
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
        if !package::has_header(&self.buffer) {
            return Ok(());
        }
        // Get header
        let header: Header = match package::get_header(&self.buffer) {
            Ok(v) => v,
            Err(e) => { return Err(ReadError::Header(e)); },
        };
        println!("{}:: reading... len: {}, buffer: {}", self.uuid, header.len, self.buffer.len());
        if !package::has_body(&self.buffer, &header) {
            return Ok(());
        }
        let (body, rest) = match package::get_body(&self.buffer, &header) {
            Ok(v) => v,
            Err(e) => { return Err(ReadError::Parsing(e)); },
        };
        self.buffer = rest;
        match Self::get_message(self, &header, &body, protocol.clone()) {
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
                println!("{}:: fail parse message due error: {}", self.uuid, e);
                Err(ReadError::Parsing(e))
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
