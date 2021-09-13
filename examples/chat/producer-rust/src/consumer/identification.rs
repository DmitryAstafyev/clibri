use super::{protocol, Consumer};
use fiber::env::logs;
use log::warn;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Filter {
    pub consumers: HashMap<Uuid, Identification>,
}

impl Filter {
    pub async fn new(consumers: &mut HashMap<Uuid, Consumer>) -> Self {
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
    pub uuid: Uuid,
    key: Option<protocol::Identification::SelfKey>,
    assigned: Option<protocol::Identification::AssignedKey>,
}

impl Identification {
    pub fn new(uuid: Uuid) -> Self {
        Identification {
            uuid,
            key: None,
            assigned: None,
        }
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

    pub fn assign(&mut self, assigned: protocol::Identification::AssignedKey, overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(assigned);
        } else if let Some(existing) = &mut self.assigned {
            if let Some(uuid) = assigned.uuid {
                existing.uuid = Some(uuid);
            }
            if let Some(auth) = assigned.auth {
                existing.auth = Some(auth);
            }
        }
    }

    pub fn assigned(&self) -> bool {
        if self.assigned.is_none() {
            warn!(
                target: logs::targets::PRODUCER,
                "Client doesn't have producer identification"
            );
        }
        self.key.is_some()
    }
}
