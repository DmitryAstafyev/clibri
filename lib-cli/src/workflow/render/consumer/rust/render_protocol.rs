use super::{Protocol, ProtocolRender, ProtocolRustRender};
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

    pub fn render(
        &self,
        base: &Path,
        protocol: &mut Protocol,
        protocol_render: &ProtocolRustRender,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        protocol_render.render(protocol, &dest)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("protocol");
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
