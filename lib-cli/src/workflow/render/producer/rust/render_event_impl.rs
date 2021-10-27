use super::{
    helpers, helpers::render as tools, workflow::beacon::Broadcast, workflow::event::Event,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{identification, producer::Control, protocol, Context};
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
        let mut output: String = if self.is_default(event)? {
            templates::DEFAULT_MODULE.to_owned()
        } else {
            templates::MODULE.to_owned()
        };
        output = output.replace("[[event]]", &tools::into_rust_path(&event.get_reference()?));
        let mut types = String::new();
        let mut refs = String::new();
        for (pos, broadcast) in event.broadcasts.iter().enumerate() {
            let type_name = self.get_broadcast_type_name(broadcast);
            if broadcast.optional {
                types = format!(
                    "{}type {} = Option<(Vec<Uuid>, protocol::{})>;\n",
                    types,
                    type_name,
                    tools::into_rust_path(&broadcast.reference),
                );
            } else {
                types = format!(
                    "{}type {} = (Vec<Uuid>, protocol::{});\n",
                    types,
                    type_name,
                    tools::into_rust_path(&broadcast.reference),
                );
            }
            refs = format!("{}{}{}", refs, if pos == 0 { "" } else { ", " }, type_name);
        }
        output = output.replace("[[broadcast_types]]", &types);
        output = output.replace("[[broadcast_refs]]", &refs);
        helpers::fs::write(dest, output, true)
    }

    fn is_default(&self, event: &Event) -> Result<bool, String> {
        if event.get_reference()? == "connected" || event.get_reference()? == "disconnected" {
            Ok(true)
        } else {
            Ok(false)
        }
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
