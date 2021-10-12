use super::{
    helpers, helpers::render as tools, workflow::broadcast::Broadcast, workflow::event::Event,
};
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

    pub fn render(&self, base: &Path, events: &Vec<Event>) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = String::from("use super::*;\n");
        for event in events.iter() {
            output = format!("{}pub mod {};\n", output, event.as_mod_name()?);
        }
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("events");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
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
