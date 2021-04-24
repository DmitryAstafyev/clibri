use super::consumer_identification::{Identification, Filter};
use super::{ Protocol, ConsumersChannel, tools };
use fiber::logger::{ Logger };
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use async_channel::{
    Sender,
};
use futures::{
    executor,
    Future,
};
pub struct Cx {
    uuid: Uuid,
    consumers: Arc<Mutex<Sender<ConsumersChannel>>>,
}

impl Cx {
    pub async fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        match self.consumers.lock() {
            Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::SendTo((self.uuid.clone(), buffer))).await {
                Err(e.to_string())
            } else {
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn send_to(
        &self,
        buffer: Vec<u8>,
        filter: Filter,
    ) -> Result<(), String> {
        match self.consumers.lock() {
            Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::SendByFilter((filter, buffer))).await {
                Err(e.to_string())
            } else {
                Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub async fn assign(&self, assigned: Protocol::Identification::AssignedKey, overwrite: bool) -> Result<(), String> {
        match self.consumers.lock() {
            Ok(consumers) => if let Err(e) = consumers.send(ConsumersChannel::Assign((self.uuid.clone(), assigned, overwrite))).await {
                Err(e.to_string())
            } else {
                Ok(())
            },
            Err(e) => Err(e.to_string()),
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
    pub fn new(uuid: Uuid, consumers: Arc<Mutex<Sender<ConsumersChannel>>>, sender: Arc<Mutex<Sender<(Vec<u8>, Option<Uuid>)>>>) -> Self {
        Consumer {
            uuid,
            buffer: Protocol::Buffer::new(),
            identification: Identification::new(uuid.clone()),
            cx: Cx {
                uuid,
                consumers,
            },
            sender,
        }
    }

    pub fn chunk(&mut self, buffer: &Vec<u8>) -> Result<(), String> {
        if let Err(e) = self.buffer.chunk(buffer, Some(self.uuid.to_string())) {
            Err(format!("{:?}", e))
        } else {
            Ok(())
        }
    }

    pub fn next(&mut self) -> Option<(Protocol::AvailableMessages, Protocol::PackageHeader)> {
        if let Some(msg) = self.buffer.next() {
            Some((msg.msg, msg.header))
        } else {
            None
        }
    }

    pub async fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        let len = buffer.len();
        match self.sender.lock() {
            Ok(sender) => if let Err(e) = sender.send((buffer, Some(self.uuid))).await {
                    Err(tools::logger.err(&format!("{}:: Fail to send buffer {} bytes, due error: {}", self.get_uuid(), len, e)))
                } else {
                    tools::logger.debug(&format!("{}:: Has been sent a buffer {} bytes", self.get_uuid(), len));
                    Ok(())
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn send_if(
        &self,
        buffer: Vec<u8>,
        filter: Filter,
    ) -> Result<bool, String> {
        if self.identification.filter(filter.clone()) {
            if let Err(e) = self.send(buffer).await {
                Err(e)
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub fn get_cx(&mut self) -> &Cx {
        &self.cx
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn key(&mut self, key: Protocol::Identification::SelfKey, overwrite: bool) -> String {
        self.identification.key(key, overwrite);
        self.uuid.to_string()
    }

    pub fn assign(&mut self, key: Protocol::Identification::AssignedKey, overwrite: bool) {
        self.identification.assign(key, overwrite);
    }

    pub fn assigned(&self) -> bool {
        self.identification.assigned()
    }
}
