use super::{helpers, workflow::beacon::Broadcast, workflow::event::Event};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_BROADCAST: &str = r#"use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

[[broadcast_types]]
#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::[[event]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<([[broadcast_refs]]), String> {
    panic!("Handler for protocol::[[event]] isn't implemented");
}"#;
    pub const MODULE_WITHOUT_BROADCAST: &str = r#"use super::{identification, producer::Control, protocol, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::[[event]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    protocol::[[event]]
}"#;
    pub const DEFAULT_MODULE: &str = r#"use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

[[broadcast_types]]
#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<([[broadcast_refs]]), String> {
    panic!("Handler for protocol::[[event]] isn't implemented");
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

    pub fn render(&self, base: &Path, event: &Event) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, event)?;
        if dest.exists() {
            println!("[SKIP]: {}", dest.to_string_lossy());
            return Ok(());
        }
        let output: String = if event.broadcasts.is_empty() {
            let mut out = templates::MODULE_WITHOUT_BROADCAST.to_owned();
            out = out.replace("[[event]]", &self.into_rust_path(&event.get_reference()?));
            out
        } else {
            let mut out = if self.is_default(event)? {
                templates::DEFAULT_MODULE.to_owned()
            } else {
                templates::MODULE_WITH_BROADCAST.to_owned()
            };
            out = out.replace("[[event]]", &self.into_rust_path(&event.get_reference()?));
            let mut types = String::new();
            let mut refs = String::new();
            for (pos, broadcast) in event.broadcasts.iter().enumerate() {
                let type_name = self.get_broadcast_type_name(broadcast);
                if broadcast.optional {
                    types = format!(
                        "{}type {} = Option<(Vec<Uuid>, protocol::{})>;\n",
                        types,
                        type_name,
                        self.into_rust_path(&broadcast.reference),
                    );
                } else {
                    types = format!(
                        "{}type {} = (Vec<Uuid>, protocol::{});\n",
                        types,
                        type_name,
                        self.into_rust_path(&broadcast.reference),
                    );
                }
                refs = format!("{}{}{}", refs, if pos == 0 { "" } else { ", " }, type_name);
            }
            out = out.replace("[[broadcast_types]]", &types);
            out = out.replace("[[broadcast_refs]]", &refs);
            out
        };
        helpers::fs::write(dest, output, true)
    }

    fn is_default(&self, event: &Event) -> Result<bool, String> {
        if event.get_reference()? == "connected" || event.get_reference()? == "disconnected" {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }

    fn get_broadcast_type_name(&self, broadcast: &Broadcast) -> String {
        format!("Broadcast{}", broadcast.reference.replace(".", ""))
    }

    fn get_dest_file(&self, base: &Path, event: &Event) -> Result<PathBuf, String> {
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
        Ok(dest.join(event.as_filename()?))
    }
}
