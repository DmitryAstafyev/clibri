use super::{
    helpers, helpers::render as tools, workflow::beacon::Broadcast, workflow::event::Event,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{
    beacons, identification, producer::Control, protocol, Context,
    EmitterError,
};
use fiber::server;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: protocol::[[beacon]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    beacons::[[beacon_mod]]::emit::<E, C>(identification, beacon, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)?
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
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[beacon]]", &beacon.as_struct_path());
        output = output.replace("[[beacon_mod]]", &beacon.as_mod_name());
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, beacon: &Broadcast) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("beacons");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(beacon.as_filename()))
    }
}
