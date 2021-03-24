use super::Protocol;
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EFilterMatchCondition {
    PartialEqual,
    Equal,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub key: Option<Protocol::Identification::SelfKey>,
    pub assigned: Option<Protocol::Identification::AssignedKey>,
    pub condition: EFilterMatchCondition,
}

#[derive(Debug, Clone)]
pub struct Identification {
    key: Option<Protocol::Identification::SelfKey>,
    assigned: Option<Protocol::Identification::AssignedKey>,
}

impl Identification {
    pub fn new() -> Self {
        Identification {
            key: None,
            assigned: None,
        }
    }

    pub fn key(&mut self, key: Protocol::Identification::SelfKey) {
        self.key = Some(key);
    }

    pub fn assign(&mut self, assigned: Protocol::Identification::AssignedKey) {
        self.assigned = Some(assigned);
    }

    pub fn filter(
        &self,
        filter: Filter,
    ) -> bool {
        let key_match = if let Some(key) = filter.key {
            if let Some(o_key) = self.key.as_ref() {
                match filter.condition {
                    EFilterMatchCondition::Equal => {
                        if o_key.id == key.id
                            && o_key.location == key.location
                            && o_key.uuid == key.uuid
                        {
                            true
                        } else {
                            false
                        }
                    }
                    EFilterMatchCondition::PartialEqual => {
                        if o_key.id == key.id
                            || o_key.location == key.location
                            || o_key.uuid == key.uuid
                        {
                            true
                        } else {
                            false
                        }
                    }
                }
            } else {
                false
            }
        } else {
            true
        };
        let assigned_match = if let Some(assigned) = filter.assigned {
            if let Some(o_assigned) = self.assigned.as_ref() {
                match filter.condition {
                    EFilterMatchCondition::Equal => {
                        if o_assigned.auth == assigned.auth
                            && o_assigned.uuid == assigned.uuid
                        {
                            true
                        } else {
                            false
                        }
                    }
                    EFilterMatchCondition::PartialEqual => {
                        if o_assigned.auth == assigned.auth
                            || o_assigned.uuid == assigned.uuid
                        {
                            true
                        } else {
                            false
                        }
                    }
                }
            } else {
                false
            }
        } else {
            true
        };
        key_match && assigned_match
    }

    pub fn assigned(&self) -> bool {
        self.key.is_some() && self.assigned.is_some()
    }
}
