use super::helpers;
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

    pub fn render(&self, base: &Path) -> Result<(), String> {
        let target = self.get_target_file(base)?;
        //helpers::fs::write(dest, output, true)
        Ok(())
    }

    fn get_target_file(&self, base: &Path) -> Result<PathBuf, String> {
        let mut current = base.clone();
        while let Some(parent) = current.parent() {
            let target = current.join("Cargo.toml");
            if target.exists() {
                return Ok(target);
            } else {
                current = parent;
            }
        }
        Err(format!(
            "Cannot find Cargo.toml. Checked all nested starting from {}",
            base.to_string_lossy()
        ))
    }
}
