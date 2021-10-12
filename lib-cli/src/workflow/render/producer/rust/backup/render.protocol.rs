use super::{
    ProtocolRender,
    ProtocolRustRender,
    Protocol,
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

pub struct RenderProtocol {
}

impl Default for RenderProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderProtocol {
    
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
        let dest = base.join("protocol");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("protocol.rs"))
    }

}

