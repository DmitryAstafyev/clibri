use super::Protocol;
use std::cmp::{ PartialEq, Eq };
use Protocol::StructDecode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EFilterMatchCondition {
    PartialEqual,
    Equal,
}

#[derive(Debug, Clone)]
pub struct Identification {
    fp: Protocol::Identification::Key,
    assigned: bool,
}

impl Identification {

    pub fn new() -> Self {
        Identification { fp: Protocol::Identification::Key::defaults(), assigned: false }
    }

    pub fn set(&mut self, fp: Protocol::Identification::Key) {
        self.fp = fp;
        self.assigned = true;
    }

    pub fn filter(&self, request: Protocol::Identification::Key, condition: EFilterMatchCondition) -> bool {
        match condition {
            EFilterMatchCondition::Equal => {
                if self.fp.id == request.id && 
                   self.fp.location == request.location &&
                   self.fp.uuid == request.uuid {
                    true
                } else {
                    false
                }
            },
            EFilterMatchCondition::PartialEqual => {
                if self.fp.id == request.id ||
                   self.fp.location == request.location ||
                   self.fp.uuid == request.uuid {
                    true
                } else {
                    false
                }
            },
        }
    }

    pub fn assigned(&self) -> bool {
        self.assigned
    }

}