#![allow(dead_code)]
use super::{producer, protocol, Consumer};
use clibri::env::logs;
use log::debug;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Filter {
    pub uuids: Vec<Uuid>,
}

impl Filter {
    pub async fn new(consumers: &HashMap<Uuid, Consumer>) -> Self {
        Self {
            uuids: consumers.keys().cloned().collect(),
        }
    }

    // pub fn filter<F>(&self, cb: F) -> Vec<Uuid>
    // where
    //     F: Fn(&Identification) -> bool,
    // {
    //     self.uuids
    //         .values()
    //         .cloned()
    //         .collect::<Vec<Identification>>()
    //         .iter()
    //         .filter(|ident| cb(ident))
    //         .map(|ident| ident.uuid)
    //         .collect::<Vec<Uuid>>()
    // }

    pub fn exclude(&self, uuids: Vec<Uuid>) -> Vec<Uuid> {
        self.uuids
            .iter()
            .filter(|uuid| !uuids.iter().any(|tuuid| &tuuid == uuid))
            .cloned()
            .collect::<Vec<Uuid>>()
    }

    pub fn except(&self, uuid: &Uuid) -> Vec<Uuid> {
        self.uuids
            .iter()
            .filter(|tuuid| *tuuid != uuid)
            .cloned()
            .collect::<Vec<Uuid>>()
    }

    pub fn all(&self) -> Vec<Uuid> {
        self.uuids.to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct Identification {
    uuid: Uuid,
    producer_indentification_strategy: producer::ProducerIdentificationStrategy,
    discredited: bool,
    key: Option<protocol::StructA>,
    assigned: Option<protocol::StructC>,
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

    pub fn key(&mut self, key: protocol::StructA, overwrite: bool) {
        if overwrite || self.key.is_none() {
            self.key = Some(key);
        } else if let Some(existing) = &mut self.key {
            existing.field_str = key.field_str;
            existing.field_str_empty = key.field_str_empty;
            existing.field_u8 = key.field_u8;
            existing.field_u16 = key.field_u16;
            existing.field_u32 = key.field_u32;
            existing.field_u64 = key.field_u64;
            existing.field_i8 = key.field_i8;
            existing.field_i16 = key.field_i16;
            existing.field_i32 = key.field_i32;
            existing.field_i64 = key.field_i64;
            existing.field_f32 = key.field_f32;
            existing.field_f64 = key.field_f64;
            existing.field_bool = key.field_bool;
        }
    }

    pub fn assign(&mut self, key: protocol::StructC, overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(key);
        } else if let Some(existing) = &mut self.assigned {
            if let Some(field_str) = key.field_str {
                existing.field_str = Some(field_str);
            }
            if let Some(field_u8) = key.field_u8 {
                existing.field_u8 = Some(field_u8);
            }
            if let Some(field_u16) = key.field_u16 {
                existing.field_u16 = Some(field_u16);
            }
            if let Some(field_u32) = key.field_u32 {
                existing.field_u32 = Some(field_u32);
            }
            if let Some(field_u64) = key.field_u64 {
                existing.field_u64 = Some(field_u64);
            }
            if let Some(field_i8) = key.field_i8 {
                existing.field_i8 = Some(field_i8);
            }
            if let Some(field_i16) = key.field_i16 {
                existing.field_i16 = Some(field_i16);
            }
            if let Some(field_i32) = key.field_i32 {
                existing.field_i32 = Some(field_i32);
            }
            if let Some(field_i64) = key.field_i64 {
                existing.field_i64 = Some(field_i64);
            }
            if let Some(field_f32) = key.field_f32 {
                existing.field_f32 = Some(field_f32);
            }
            if let Some(field_f64) = key.field_f64 {
                existing.field_f64 = Some(field_f64);
            }
            if let Some(field_bool) = key.field_bool {
                existing.field_bool = Some(field_bool);
            }
        }
    }

    pub fn assigned(&self) -> bool {
        if self.assigned.is_none() {
            match self.producer_indentification_strategy {
                producer::ProducerIdentificationStrategy::Ignore => true,
                producer::ProducerIdentificationStrategy::Log => {
                    debug!(
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