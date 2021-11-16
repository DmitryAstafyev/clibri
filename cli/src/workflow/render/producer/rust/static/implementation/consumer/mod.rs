#![allow(dead_code)]
pub mod identification;

use super::{producer, protocol};
use log::error;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("fail to read buffer: `{0:?}`")]
    Reading(protocol::ReadError),
}

pub type ConsumerMessages = Vec<(protocol::AvailableMessages, protocol::PackageHeader)>;

pub struct Consumer {
    uuid: Uuid,
    buffer: protocol::Buffer<protocol::AvailableMessages>,
    identification: identification::Identification,
    hash_accepted: bool,
}

impl Consumer {
    pub fn new(uuid: Uuid, options: &producer::Options) -> Self {
        Self {
            uuid,
            buffer: protocol::Buffer::new(),
            identification: identification::Identification::new(uuid, options),
            hash_accepted: false,
        }
    }

    pub fn accept_hash(&mut self) {
        self.hash_accepted = true;
    }

    pub fn is_hash_accepted(&self) -> bool {
        self.hash_accepted
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_identification(&self) -> identification::Identification {
        self.identification.clone()
    }

    pub fn get_mut_identification(&mut self) -> &mut identification::Identification {
        &mut self.identification
    }

    pub fn key(&mut self, key: &protocol::Identification::SelfKey, overwrite: bool) -> String {
        self.identification.key(key.clone(), overwrite);
        self.uuid.to_string()
    }

    pub fn assign(&mut self, key: protocol::Identification::AssignedKey, overwrite: bool) {
        self.identification.assign(key, overwrite);
    }

    #[allow(clippy::ptr_arg)]
    pub fn chunk(&mut self, buffer: &Vec<u8>) -> Result<(), ConsumerError> {
        self.buffer
            .chunk(buffer, Some(self.uuid.to_string()))
            .map_err(ConsumerError::Reading)
    }

    pub fn get_messages(&mut self) -> ConsumerMessages {
        let mut msgs: ConsumerMessages = vec![];
        while let Some(msg) = self.buffer.next() {
            msgs.push((msg.msg, msg.header));
        }
        msgs
    }
}