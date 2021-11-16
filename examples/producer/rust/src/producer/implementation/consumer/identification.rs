#![allow(dead_code)]
use super::{producer, protocol, Consumer};
use clibri::env::logs;
use log::warn;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Filter {
    pub consumers: HashMap<Uuid, Identification>,
}

impl Filter {
    pub async fn new(consumers: &HashMap<Uuid, Consumer>) -> Self {
        let mut identifications: HashMap<Uuid, Identification> = HashMap::new();
        for consumer in consumers.values() {
            identifications.insert(consumer.get_uuid(), consumer.get_identification());
        }
        Self {
            consumers: identifications,
        }
    }

    pub fn filter<F>(&self, cb: F) -> Vec<Uuid>
    where
        F: Fn(&Identification) -> bool,
    {
        self.consumers
            .values()
            .cloned()
            .collect::<Vec<Identification>>()
            .iter()
            .filter(|ident| cb(&ident))
            .map(|ident| ident.uuid)
            .collect::<Vec<Uuid>>()
    }

    pub fn exclude(&self, uuids: Vec<Uuid>) -> Vec<Uuid> {
        self.consumers
            .values()
            .cloned()
            .collect::<Vec<Identification>>()
            .iter()
            .filter(|ident| uuids.iter().find(|uuid| &ident.uuid == *uuid).is_none())
            .map(|ident| ident.uuid)
            .collect::<Vec<Uuid>>()
    }

    pub fn except(&self, uuid: Uuid) -> Vec<Uuid> {
        self.consumers
            .values()
            .cloned()
            .collect::<Vec<Identification>>()
            .iter()
            .filter(|ident| ident.uuid != uuid)
            .map(|ident| ident.uuid)
            .collect::<Vec<Uuid>>()
    }
}

#[derive(Debug, Clone)]
pub struct Identification {
    uuid: Uuid,
    producer_indentification_strategy: producer::ProducerIdentificationStrategy,
    discredited: bool,
    key: Option<protocol::Identification::SelfKey>,
    assigned: Option<protocol::Identification::AssignedKey>,
}

impl Identification {
    pub fn new(uuid: Uuid, options: &producer::Options) -> Self {
        Identification {
            uuid,
            producer_indentification_strategy: options.producer_indentification_strategy.clone(),
            discredited: false,
            key: None,
            assigned: None,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn key(&mut self, key: protocol::Identification::SelfKey, overwrite: bool) {
        if overwrite || self.key.is_none() {
            self.key = Some(key);
        } else if let Some(existing) = &mut self.key {
            if let Some(uuid) = key.uuid {
                existing.uuid = Some(uuid);
            }
            if let Some(id) = key.id {
                existing.id = Some(id);
            }
            if let Some(location) = key.location {
                existing.location = Some(location);
            }
        }
    }

    pub fn assign(&mut self, key: protocol::Identification::AssignedKey, overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(key);
        } else if let Some(existing) = &mut self.assigned {
            if let Some(uuid) = key.uuid {
                existing.uuid = Some(uuid);
            }
            if let Some(auth) = key.auth {
                existing.auth = Some(auth);
            }
        }
    }

    pub fn assigned(&self) -> bool {
        if self.assigned.is_none() {
            match self.producer_indentification_strategy {
                producer::ProducerIdentificationStrategy::Ignore => true,
                producer::ProducerIdentificationStrategy::Log => {
                    warn!(
                        target: logs::targets::PRODUCER,
                        "{}:: client doesn't have producer identification", self.uuid
                    );
                    true
                }
                _ => false,
            }
        } else {
            true
        }
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn discredited(&mut self) {
        self.discredited = true;
    }

    pub fn is_discredited(&self) -> bool {
        self.discredited
    }
}