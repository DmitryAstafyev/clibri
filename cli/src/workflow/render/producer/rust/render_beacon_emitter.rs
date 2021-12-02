use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{
    beacons, identification, pack, producer::Control, protocol, Context, EmitterError,
    ProducerError,
};
use clibri::server;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::[[beacon]],
    sequence: u32,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    beacons::[[beacon_mod]]::emit::<E, C>(identification, beacon, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)?;
    let mut response = protocol::InternalServiceGroup::BeaconConfirmation { error: None };
    let buffer = pack(&sequence, &identification.uuid(), &mut response)?;
    control
        .send(buffer, Some(identification.uuid()))
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Emitting(e.to_string()))?;
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
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[beacon]]", &beacon.reference.replace(".", "::"));
        output = output.replace(
            "[[beacon_mod]]",
            &beacon.reference.to_lowercase().replace(".", "_"),
        );
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
        Ok(dest.join(format!(
            "{}.rs",
            beacon.reference.to_lowercase().replace(".", "_")
        )))
    }
}
