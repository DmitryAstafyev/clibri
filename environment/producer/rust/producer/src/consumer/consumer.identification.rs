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
    fp: Protocol::Identification,
}

impl Identification {

    pub fn new() -> Self {
        Identification { fp: Protocol::Identification::defaults() }
    }

    pub fn set(&mut self, fp: Protocol::Identification) {
        self.fp = fp;
    }

    pub fn filter(&self, request: Protocol::Identification, condition: EFilterMatchCondition) -> bool {
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

}