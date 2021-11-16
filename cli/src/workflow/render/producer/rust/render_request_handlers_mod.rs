use super::{helpers, workflow::request::Request};
use std::{
    fs,
    path::{Path, PathBuf},
};

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

    pub fn render(&self, base: &Path, requests: &[Request]) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = String::from(
            r#"[[mods]]
use super::*;
use clibri::server;
use protocol::PackingStruct;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("packing error: `{0}`")]
    Packing(String),
}

pub fn pack(
    sequence: &u32,
    uuid: &Uuid,
    msg: &mut dyn PackingStruct,
) -> Result<Vec<u8>, HandlerError> {
    msg.pack(*sequence, Some(uuid.to_string()))
        .map_err(HandlerError::Packing)
}

pub async fn broadcast<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    broadcasting: &mut (Vec<Uuid>, Vec<u8>),
    control: &producer::Control<E, C>,
) -> Result<(), HandlerError> {
    control
        .broadcast(broadcasting.0.clone(), broadcasting.1.clone())
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    Ok(())
}"#,
        );
        let mut mods = String::new();
        for request in requests.iter() {
            mods = format!(
                "{}pub mod {};\n",
                mods,
                request.get_request()?.to_lowercase().replace(".", "_")
            );
        }
        output = output.replace("[[mods]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("handlers");
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
