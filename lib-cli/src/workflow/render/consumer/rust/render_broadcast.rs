use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(
    event: protocol::[[request]],
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("{} isn't implemented yet");
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

    pub fn render(&self, base: &Path, broadcast: &Broadcast) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, broadcast)?;
        if dest.exists() {
            println!("[SKIP]: {}", dest.to_string_lossy());
            return Ok(());
        }
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[request]]", &(broadcast.reference).replace(".", "::"));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, broadcast: &Broadcast) -> Result<PathBuf, String> {
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
        Ok(dest.join(format!(
            "{}.rs",
            broadcast.reference.replace(".", "_").to_lowercase()
        )))
    }
}
