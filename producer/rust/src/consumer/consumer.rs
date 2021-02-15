use super::consumer_context::Context;
use super::consumer_identification::{EFilterMatchCondition, Identification};
use super::Protocol;
use fiber::server::context::ConnectionContext;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct Cx<CX>
where
    CX: ConnectionContext + Send + Sync,
{
    own: Arc<RwLock<CX>>,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>,
}

impl<CX> Context for Cx<CX>
where
    CX: ConnectionContext + Send + Sync,
{
    fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        match self.own.write() {
            Ok(mut own) => own.send(buffer),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn send_to(
        &self,
        buffer: Vec<u8>,
        filter: HashMap<String, String>,
        condition: EFilterMatchCondition,
    ) -> Result<(), String> {
        match self.consumers.write() {
            Ok(consumers) => {
                let mut errors: Vec<String> = vec![];
                for (uuid, consumer) in consumers.iter() {
                    if let Err(e) =
                        consumer.send_if(buffer.clone(), filter.clone(), condition.clone())
                    {
                        errors.push(format!("Fail to send data to {}, due error: {}", uuid, e));
                    }
                }
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors.join("\n"))
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }
}

pub struct Consumer<CX>
where
    CX: ConnectionContext + Send + Sync,
{
    uuid: Uuid,
    _buffer: Protocol::Buffer<Protocol::AvailableMessages>,
    identification: Identification,
    cx: Cx<CX>,
}

impl<CX> Consumer<CX>
where
    CX: ConnectionContext + Send + Sync,
{
    pub fn new(own: Arc<RwLock<CX>>, consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            _buffer: Protocol::Buffer::new(),
            identification: Identification::new(),
            cx: Cx { own, consumers },
        }
    }

    pub fn read(&mut self, _buffer: Vec<u8>) -> Result<Protocol::AvailableMessages, String> {
        Err("".to_owned())
    }

    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        self.cx.send(buffer)
    }

    pub fn send_if(
        &self,
        buffer: Vec<u8>,
        filter: HashMap<String, String>,
        condition: EFilterMatchCondition,
    ) -> Result<bool, String> {
        if self.identification.filter(filter, condition) {
            if let Err(e) = self.send(buffer) {
                Err(e)
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub fn get_cx(&mut self) -> &impl Context {
        &self.cx
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}
