use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        broadcast::{
            Broadcast
        }
    },
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

mod templates {
    pub const MODULE: &str = r#"
use super::{
    Protocol,
};
pub enum Broadcast {[[broadcast]]
}
"#;
}

pub struct RenderBroadcast {
}

impl Default for RenderBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBroadcast {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        broadcasts: &Vec<Broadcast>,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = String::new();
        for broadcast in broadcasts {
            output = format!("{}\n{}(Protocol::{}),",
                output,
                broadcast.reference.replace(".", ""),
                broadcast.reference.replace(".", "::"),
            );
        }
        output = templates::MODULE.replace("[[broadcast]]", &tools::inject_tabs(1, output));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("broadcast");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("broadcast.rs"))
    }

}

