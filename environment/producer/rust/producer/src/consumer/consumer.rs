use super::consumer_identification::{Filter, Identification};
use super::{tools, ConsumersChannel, Protocol};
use fiber::logger::Logger;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;
pub struct Cx {
    uuid: Uuid,
    consumers: UnboundedSender<ConsumersChannel>,
}

impl Cx {
    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        if let Err(e) = self
            .consumers
            .send(ConsumersChannel::SendTo((self.uuid.clone(), buffer)))
        {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    pub fn send_to(&self, buffer: Vec<u8>, filter: Filter) -> Result<(), String> {
        if let Err(e) = self
            .consumers
            .send(ConsumersChannel::SendByFilter((filter, buffer)))
        {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub fn assign(
        &self,
        assigned: Protocol::Identification::AssignedKey,
        overwrite: bool,
    ) -> Result<(), String> {
        if let Err(e) = self.consumers.send(ConsumersChannel::Assign((
            self.uuid.clone(),
            assigned,
            overwrite,
        ))) {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
}

pub struct Consumer {
    uuid: Uuid,
    buffer: Protocol::Buffer<Protocol::AvailableMessages>,
    identification: Identification,
    cx: Cx,
    sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
}

impl Consumer {
    pub fn new(
        uuid: Uuid,
        consumers: UnboundedSender<ConsumersChannel>,
        sender: UnboundedSender<(Vec<u8>, Option<Uuid>)>,
    ) -> Self {
        Consumer {
            uuid,
            buffer: Protocol::Buffer::new(),
            identification: Identification::new(uuid.clone()),
            cx: Cx { uuid, consumers },
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

    pub fn send(&self, buffer: Vec<u8>) -> Result<(), String> {
        let len = buffer.len();
        if let Err(e) = self.sender.send((buffer, Some(self.uuid))) {
            Err(tools::logger.err(&format!(
                "{}:: Fail to send buffer {} bytes, due error: {}",
                self.get_uuid(),
                len,
                e
            )))
        } else {
            tools::logger.debug(&format!(
                "{}:: Has been sent a buffer {} bytes",
                self.get_uuid(),
                len
            ));
            Ok(())
        }
    }

    pub fn send_if(&self, buffer: Vec<u8>, filter: Filter) -> Result<bool, String> {
        if self.identification.filter(filter.clone()) {
            if let Err(e) = self.send(buffer) {
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
