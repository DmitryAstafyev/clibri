use super::{helpers, helpers::render as tools, workflow::store::Store};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"#![allow(dead_code)]
pub mod identification;

use super::{producer, protocol};
use log::error;
use thiserror::Error;
use tokio::sync::mpsc::{error::SendError, UnboundedSender};
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
    confirmed: bool,
}

impl Consumer {
    pub fn new(
        uuid: Uuid,
        options: &producer::Options,
        tx_ident_change: UnboundedSender<identification::IdentificationChannel>,
    ) -> Self {
        Self {
            uuid,
            buffer: protocol::Buffer::new(),
            identification: identification::Identification::new(uuid, options, tx_ident_change),
            hash_accepted: false,
            confirmed: false,
        }
    }

    pub fn accept_hash(&mut self) {
        self.hash_accepted = true;
    }

    pub fn is_hash_accepted(&self) -> bool {
        self.hash_accepted
    }


    pub fn is_confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_identification(&self) -> &identification::Identification {
        &self.identification
    }

    pub fn get_mut_identification(&mut self) -> &mut identification::Identification {
        &mut self.identification
    }

    pub fn key(&mut self, key: &protocol::[[self_key]], overwrite: bool) -> String {
        self.identification.key(key.clone(), overwrite);
        self.uuid.to_string()
    }

    pub fn assign(&mut self, key: protocol::[[assign_key]], overwrite: bool) {
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
}"#;
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

    pub fn render(&self, base: &Path, store: &Store) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = templates::MODULE.to_owned();
        output = output.replace(
            "[[assign_key]]",
            &tools::into_rust_path(&store.get_config()?.get_assigned()?),
        );
        output = output.replace(
            "[[self_key]]",
            &tools::into_rust_path(&store.get_config()?.get_self()?),
        );
        helpers::fs::write(dest, output, true)
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
        Ok(dest.join("mod.rs"))
    }
}
