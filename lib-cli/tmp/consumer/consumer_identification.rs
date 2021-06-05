
use super::{
    tools,
    Protocol
};
use fiber::logger::Logger;
use std::{
    cmp::{
        Eq,
        PartialEq
    },
    sync::Arc,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Equal,
    NotEqual,
}
pub type FilterCallback = dyn Fn(
        Uuid,
        Option<Protocol::Identification::SelfKey>,
        Option<Protocol::Identification::AssignedKey>,
    ) -> bool
    + Send
    + Sync;

#[derive(Clone)]
pub struct Filter {
    pub uuid: Option<(Uuid, Condition)>,
    pub assign: Option<bool>,
    pub filter: Option<Arc<Box<FilterCallback>>>,
}

#[derive(Debug, Clone)]
pub struct Identification {
    uuid: Uuid,
    key: Option<Protocol::Identification::SelfKey>,
    assigned: Option<Protocol::Identification::AssignedKey>,
}

impl Identification {
    pub fn new(uuid: Uuid) -> Self {
        Identification {
            uuid,
            key: None,
            assigned: None,
        }
    }

    pub fn key(&mut self, key: Protocol::Identification::SelfKey, overwrite: bool) {
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

    pub fn assign(&mut self, assigned: Protocol::Identification::AssignedKey, overwrite: bool) {
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

    pub fn filter(&self, filter: Filter) -> bool {
        if let Some((uuid, condition)) = filter.uuid {
            return match condition {
                Condition::Equal => uuid == self.uuid,
                Condition::NotEqual => uuid != self.uuid,
            };
        }
        if let Some(assign) = filter.assign {
            return assign == self.assigned();
        }
        if let Some(filter) = filter.filter {
            return filter(self.uuid, self.key.clone(), self.assigned.clone());
        }
        false
    }

    pub fn assigned(&self) -> bool {
        if self.assigned.is_none() {
            tools::logger.warn("Client doesn't have producer identification");
        }
        self.key.is_some()
    }
}
