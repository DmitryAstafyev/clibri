use super::Protocol;
use std::cmp::{ PartialEq, Eq };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EFilterMatchCondition {
    PartialEqual,
    Equal,
}

#[derive(Debug, Clone)]
pub struct Identification {
    key: Option<Protocol::Identification::SelfKey>,
    assigned: Option<Protocol::Identification::AssignedKey>,
}

impl Identification {

    pub fn new() -> Self {
        Identification { key: None, assigned: None }
    }

    pub fn key(&mut self, key: Protocol::Identification::SelfKey) {
        self.key = Some(key);
    }

    pub fn assign(&mut self, assigned: Protocol::Identification::AssignedKey) {
        self.assigned = Some(assigned);
    }

    pub fn filter(&self, key: Option<Protocol::Identification::SelfKey>, condition: EFilterMatchCondition) -> bool {
        let key = if let Some(key) = key {
            key
        } else {
            return false;
        };
        if let Some(o_key) = self.key.as_ref() {
            if let Some(assigned) = self.assigned.as_ref() {
                match condition {
                    EFilterMatchCondition::Equal => {
                        if o_key.id == key.id && 
                           o_key.location == key.location &&
                           o_key.uuid == key.uuid {
                            true
                        } else {
                            false
                        }
                    },
                    EFilterMatchCondition::PartialEqual => {
                        if o_key.id == key.id ||
                           o_key.location == key.location ||
                           o_key.uuid == key.uuid {
                            true
                        } else {
                            false
                        }
                    },
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn assigned(&self) -> bool {
        self.key.is_some() && self.assigned.is_some()
    }

}