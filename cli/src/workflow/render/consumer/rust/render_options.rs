use super::{helpers, helpers::render as tools, WorkflowStore};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::protocol;

#[derive(Debug, Clone)]
pub struct Options {
    pub reconnection: ReconnectionStrategy,
    pub key: protocol::[[self_key]],
}
impl Options {
    pub fn defualt(key: protocol::[[self_key]]) -> Self {
        Options {
            reconnection: ReconnectionStrategy::Reconnect(2000),
            key,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReconnectionStrategy {
    Reconnect(u64),
    DoNotReconnect,
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

    pub fn render(&self, base: &Path, store: &WorkflowStore) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace(
            "[[self_key]]",
            &tools::into_rust_path(&store.get_config()?.get_self()?),
        );
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("consumer");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("options.rs"))
    }
}
