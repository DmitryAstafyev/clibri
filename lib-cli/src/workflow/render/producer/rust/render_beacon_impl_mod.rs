use super::{helpers, helpers::render as tools, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"[[mods]]"#;
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

    pub fn render(&self, base: &Path, beacons: &Vec<Broadcast>) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut mods = String::new();
        for beacon in beacons.iter() {
            mods = format!("{}pub mod {};\n", mods, beacon.as_mod_name());
        }
        let output = templates::MODULE.to_owned().replace("[[mods]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("beacons");
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
