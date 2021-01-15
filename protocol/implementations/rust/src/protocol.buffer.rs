use super::packing;
use packing::Header;

#[derive(Debug)]
pub enum ReadError {
    Header(String),
    Parsing(String),
    Signature(String),
}

#[derive(Clone)]
pub struct IncomeMessage<T: Clone> {
    pub header: Header,
    pub msg: T,
}

pub trait DecodeBuffer<T> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<T, String>;
    fn get_signature(&self) -> u16;
}

pub struct Buffer<T: Clone> {
    buffer: Vec<u8>,
    queue: Vec<IncomeMessage<T>>,
}
#[allow(clippy::len_without_is_empty)]
impl<T: Clone> Buffer<T> where Self: DecodeBuffer<T> {

    fn get_message(&self, header: &Header, buf: &[u8]) -> Result<T, ReadError> {
        if self.get_signature() != header.signature {
            Err(ReadError::Signature(format!("Signature dismatch; expectation: {}; message: {}", self.get_signature(), header.signature)))
        } else {
            match self.get_msg(header.id, buf) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(ReadError::Parsing(format!(
                    "Fail get message id={}, signature={} due error: {}",
                    header.id, header.signature, e
                ))),
            }
        }
    }

    pub fn new() -> Self {
        Buffer {
            buffer: vec![],
            queue: vec![],
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn chunk(&mut self, buf: &Vec<u8>) -> Result<(), ReadError> {
        // Add data into buffer
        self.buffer.append(&mut buf.clone());
        if !packing::has_header(&self.buffer) {
            return Ok(());
        }
        // Get header
        let header: Header = match packing::get_header(&self.buffer) {
            Ok(v) => v,
            Err(e) => {
                return Err(ReadError::Header(e));
            }
        };
        if !packing::has_body(&self.buffer, &header) {
            return Ok(());
        }
        let (body, rest) = match packing::get_body(&self.buffer, &header) {
            Ok(v) => v,
            Err(e) => {
                return Err(ReadError::Parsing(e));
            }
        };
        self.buffer = rest;
        match Self::get_message(self, &header, &body) {
            Ok(msg) => {
                self.queue.push(IncomeMessage { header, msg });
                if !self.buffer.is_empty() {
                    self.chunk(&vec![])
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e)
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

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }

}
