use super::{
    helpers,
};

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

mod templates {
    pub const MODULE: &str =
r#"export enum ERequestState {
    Ready,
    Pending,
    Destroyed,
}"#;
}

pub struct RenderInterfacesRequest {
}

impl Default for RenderInterfacesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderInterfacesRequest {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        helpers::fs::write(dest, templates::MODULE.to_owned(), true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("interfaces");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("request.ts"))
    }

}

