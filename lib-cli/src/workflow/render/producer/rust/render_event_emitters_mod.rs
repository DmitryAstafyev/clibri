use super::{helpers, workflow::event::Event};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"[[mods]]pub mod connected;
pub mod disconnected;
pub mod error;
pub mod ready;
pub mod shutdown;

use super::*;
use fiber::server;
use protocol::PackingStruct;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum EmitterError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("emitting error: `{0}`")]
    Emitting(String),
    #[error("packing error: `{0}`")]
    Packing(String),
}

pub fn pack(
    sequence: &u32,
    uuid: &Uuid,
    msg: &mut dyn PackingStruct,
) -> Result<Vec<u8>, EmitterError> {
    msg.pack(*sequence, Some(uuid.to_string()))
        .map_err(EmitterError::Packing)
}

pub fn unbound_pack(sequence: &u32, msg: &mut dyn PackingStruct) -> Result<Vec<u8>, EmitterError> {
    msg.pack(*sequence, None).map_err(EmitterError::Packing)
}

pub async fn broadcast<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &producer::Control<E, C>,
) -> Result<(), EmitterError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Processing(e.to_string()))?;
    Ok(())
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

    pub fn render(&self, base: &Path, events: &Vec<Event>) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut mods = String::new();
        for event in events.iter() {
            if !event.is_default() {
                mods = format!("{}pub mod {};\n", mods, event.as_mod_name()?);
            }
        }
        let output = templates::MODULE.to_owned().replace("[[mods]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("emitters");
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
