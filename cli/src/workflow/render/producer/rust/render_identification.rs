use super::{
    helpers, helpers::render as tools, workflow::config::Config, workflow::store::Store, Protocol,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"#![allow(dead_code)]
use super::{producer, protocol, Consumer};
use clibri::env::logs;
use log::debug;
use std::collections::HashMap;
use tokio::sync::mpsc::{error::SendError, UnboundedSender};
use uuid::Uuid;

pub struct Filter<'c> {
    consumers: &'c HashMap<Uuid, Consumer>,
}

impl<'c> Filter<'c> {
    pub fn new(consumers: &'c HashMap<Uuid, Consumer>) -> Self {
        Self { consumers }
    }

    pub fn exclude(&self, uuids: Vec<Uuid>) -> Vec<Uuid> {
        self.consumers
            .keys()
            .filter(|uuid| !uuids.iter().any(|tuuid| &tuuid == uuid))
            .cloned()
            .collect::<Vec<Uuid>>()
    }

    pub fn except(&self, uuid: &Uuid) -> Vec<Uuid> {
        self.consumers
            .keys()
            .filter(|tuuid| *tuuid != uuid)
            .cloned()
            .collect::<Vec<Uuid>>()
    }

    pub fn all(&self) -> Vec<Uuid> {
        self.consumers.keys().cloned().collect()
    }

    pub fn filter<F>(&self, cb: F) -> Vec<Uuid>
    where
        F: Fn(&Identification) -> bool,
    {
        self.consumers
            .values()
            .filter(|consumer| cb(consumer.get_identification()))
            .map(|ident| ident.uuid)
            .collect::<Vec<Uuid>>()
    }
}

#[derive(Debug)]
pub enum IdentificationChannel {
    Key(Uuid, protocol::StructA, bool),
    Assigned(Uuid, protocol::StructC, bool),
}

#[derive(Debug, Clone)]
pub struct Identification {
    uuid: Uuid,
    producer_indentification_strategy: producer::ProducerIdentificationStrategy,
    discredited: bool,
    tx_ident_change: UnboundedSender<IdentificationChannel>,
    key: Option<protocol::StructA>,
    assigned: Option<protocol::StructC>,
}

impl Identification {
    pub fn new(
        uuid: Uuid,
        options: &producer::Options,
        tx_ident_change: UnboundedSender<IdentificationChannel>,
    ) -> Self {
        Identification {
            uuid,
            producer_indentification_strategy: options.producer_indentification_strategy.clone(),
            discredited: false,
            key: None,
            assigned: None,
            tx_ident_change,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn set_key(
        &self,
        key: protocol::[[self_key]],
        overwrite: bool,
    ) -> Result<(), SendError<IdentificationChannel>> {
        self.tx_ident_change
            .send(IdentificationChannel::Key(self.uuid(), key, overwrite))
    }

    pub fn set_assign(
        &self,
        key: protocol::[[assign_key]],
        overwrite: bool,
    ) -> Result<(), SendError<IdentificationChannel>> {
        self.tx_ident_change
            .send(IdentificationChannel::Assigned(self.uuid(), key, overwrite))
    }

    pub fn key(&mut self, key: protocol::[[self_key]], overwrite: bool) {
        if overwrite || self.key.is_none() {
            self.key = Some(key);
        } else if let Some(existing) = &mut self.key {
[[self_key_overwrite]]
        }
    }

    pub fn assign(&mut self, key: protocol::[[assign_key]], overwrite: bool) {
        if overwrite || self.assigned.is_none() {
            self.assigned = Some(key);
        } else if let Some(existing) = &mut self.assigned {
[[assign_key_overwrite]]
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
}"#;
    pub const KEY_OPT_FIELD_CHECK: &str = r#"if let Some([[field]]) = key.[[field]] {
    existing.[[field]] = Some([[field]]);
}"#;
    pub const KEY_FIELD_CHECK: &str = "existing.[[field]] = key.[[field]];";
}

pub struct Render {}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, base: &Path, store: &Store, protocol: &Protocol) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = templates::MODULE.to_owned();
        let config = store.get_config()?;
        output = output.replace(
            "[[assign_key]]",
            &tools::into_rust_path(&store.get_config()?.get_assigned()?),
        );
        output = output.replace(
            "[[self_key]]",
            &tools::into_rust_path(&store.get_config()?.get_self()?),
        );
        output = output.replace(
            "[[self_key_overwrite]]",
            &tools::inject_tabs(3, self.self_key_overwrite(config, protocol)?),
        );
        output = output.replace(
            "[[assign_key_overwrite]]",
            &tools::inject_tabs(3, self.assign_key_overwrite(config, protocol)?),
        );
        helpers::fs::write(dest, output, true)
    }

    fn self_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_self()?) {
            let mut output: String = String::new();
            for (pos, field) in strct.fields.iter().enumerate() {
                output = format!(
                    "{}{}",
                    output,
                    (if field.optional {
                        templates::KEY_OPT_FIELD_CHECK
                    } else {
                        templates::KEY_FIELD_CHECK
                    })
                    .replace("[[field]]", &field.name,)
                );
                if pos < strct.fields.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
    }

    fn assign_key_overwrite(&self, config: &Config, protocol: &Protocol) -> Result<String, String> {
        if let Some(strct) = protocol.get_struct_by_str_path(0, &config.get_assigned()?) {
            let mut output: String = String::new();
            for (pos, field) in strct.fields.iter().enumerate() {
                output = format!(
                    "{}{}",
                    output,
                    (if field.optional {
                        templates::KEY_OPT_FIELD_CHECK
                    } else {
                        templates::KEY_FIELD_CHECK
                    })
                    .replace("[[field]]", &field.name,)
                );
                if pos < strct.fields.len() - 1 {
                    output = format!("{}\n", output);
                }
            }
            Ok(output)
        } else {
            Err(format!("Fail to find key {}", config.get_self()?))
        }
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("consumer");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("identification.rs"))
    }
}
