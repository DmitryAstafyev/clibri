use super::{helpers, helpers::render as tools, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{identification, producer::Control, protocol, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::[[beacon]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    println!("Handler for protocol::[[beacon]] isn't implemented");
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

    pub fn render(&self, base: &Path, beacon: &Broadcast) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, beacon)?;
        if dest.exists() {
            println!("[SKIP]: {}", dest.to_string_lossy());
            return Ok(());
        }
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[beacon]]", &tools::into_rust_path(&beacon.reference));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, beacon: &Broadcast) -> Result<PathBuf, String> {
        let dest = base.join("beacons");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(format!(
            "{}.rs",
            beacon.reference.to_lowercase().replace(".", "_")
        )))
    }
}
