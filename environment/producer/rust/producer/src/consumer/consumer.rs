use super::consumer_context::Context;
use super::consumer_identification::{EFilterMatchCondition, Identification};
use super::Protocol;
use std::collections::HashMap;
use std::sync::mpsc::{Sender};
use std::sync::{Arc, RwLock, Mutex};
use uuid::Uuid;

pub struct Cx {
    uuid: Uuid,
    consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
}

impl Context for Cx {
    fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        match self.consumers.write() {
            Ok(mut consumers) => {
                if let Some(consumer) = consumers.get_mut(&self.uuid) {
                    if let Err(e) = consumer.send(buffer) {
                        Err(format!("Fail to send buffer for consumer {} due error {}", self.uuid, e))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(format!("Fail to find consumer {}", self.uuid))
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn send_to(
        &self,
        buffer: Vec<u8>,
        filter: Protocol::Identification::Key,
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

pub struct Consumer {
    uuid: Uuid,
    buffer: Protocol::Buffer<Protocol::AvailableMessages>,
    identification: Identification,
    cx: Cx,
    sender: Arc<Mutex<Sender<(Vec<u8>, Option<Uuid>)>>>,
}

impl Consumer {
    pub fn new(consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>, sender: Arc<Mutex<Sender<(Vec<u8>, Option<Uuid>)>>>) -> Self {
        let uuid: Uuid = Uuid::new_v4();
        Consumer {
            uuid,
            buffer: Protocol::Buffer::new(),
            identification: Identification::new(),
            cx: Cx {
                uuid,
                consumers,
            },
            sender,
        }
    }

    pub fn chunk(&mut self, buffer: &Vec<u8>) -> Result<(), String> {
        if let Err(e) = self.buffer.chunk(buffer) {
            Err(format!("{:?}", e))
        } else {
            Ok(())
        }
    }

    pub fn next(&mut self) -> Option<Protocol::AvailableMessages> {
        if let Some(msg) = self.buffer.next() {
            Some(msg.msg)
        } else {
            None
        }
    }

    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        match self.sender.lock() {
            Ok(sender) => if let Err(e) = sender.send((buffer, Some(self.uuid))) {
                    Err(e.to_string())
                } else {
                    Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn send_if(
        &self,
        buffer: Vec<u8>,
        filter: Protocol::Identification::Key,
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
        self.uuid
    }

    pub fn assign(&mut self, key: Protocol::Identification::Key) -> String {
        self.identification.set(key);
        self.uuid.to_string()
    }

    pub fn assigned(&self) -> bool {
        self.identification.assigned()
    }
}
