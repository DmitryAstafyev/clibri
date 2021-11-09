use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"[[mods]]
use super::{context::Context, implementation::Consumer, protocol};"#;
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

    pub fn render(&self, base: &Path, broadcasts: Vec<Broadcast>) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        let mut mods: String = String::new();
        for broadcast in broadcasts {
            mods = format!(
                "{}\npub mod {};",
                mods,
                &(broadcast.reference).replace(".", "_").to_lowercase()
            );
        }
        output = output.replace("[[mods]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("broadcasts");
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
