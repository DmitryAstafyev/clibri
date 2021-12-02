use super::{
    helpers, helpers::render as tools, workflow::beacon::Broadcast, workflow::event::Event,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_BROADCAST: &str = r#"use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::[[event]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let [[broadcast_vars]] =
        events::[[event_mod]]::emit::<E, C>(event, filter, context, control)
            .await
            .map_err(EmitterError::Emitting)?;
[[broadcasts_processing]]
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}"#;
    pub const MODULE_WITHOUT_BROADCAST: &str = r#"use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    event: protocol::[[event]],
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::[[event_mod]]::emit::<E, C>(event, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
}"#;
    pub const DEFAULT_MODULE_WITH_BROADCAST: &str = r#"use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let [[broadcast_vars]] =
        events::[[event_mod]]::emit::<E, C>(identification, filter, context, control)
            .await
            .map_err(EmitterError::Emitting)?;
[[broadcasts_processing]]
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}"#;
    pub const DEFAULT_MODULE_WITHOUT_BROADCAST: &str = r#"use super::{
    broadcast, events, identification, producer::Control, protocol, unbound_pack, Context,
    EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::[[event_mod]]::emit::<E, C>(identification, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
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
        let mut output: String = if event.is_default() {
            if event.broadcasts.is_empty() {
                templates::DEFAULT_MODULE_WITHOUT_BROADCAST.to_owned()
            } else {
                templates::DEFAULT_MODULE_WITH_BROADCAST.to_owned()
            }
        } else if event.broadcasts.is_empty() {
            templates::MODULE_WITHOUT_BROADCAST.to_owned()
        } else {
            templates::MODULE_WITH_BROADCAST.to_owned()
        };
        output = output.replace("[[event]]", &tools::into_rust_path(&event.get_reference()?));
        output = output.replace(
            "[[event_mod]]",
            &event.get_reference()?.to_lowercase().replace(".", "_"),
        );
        if !event.broadcasts.is_empty() {
            output = output.replace("[[event]]", &tools::into_rust_path(&event.get_reference()?));
            output = output.replace(
                "[[event_mod]]",
                &event.get_reference()?.to_lowercase().replace(".", "_"),
            );
            let mut processing = String::new();
            let mut vars = String::new();
            for (pos, broadcast) in event.broadcasts.iter().enumerate() {
                let var_name = self.get_broadcast_var_name(broadcast);
                if broadcast.optional {
                    processing = format!(
                        r#"{}if let Some(mut broadcast_message) = {}.take() {{
    broadcasting.push((
        broadcast_message.0,
        unbound_pack(&0, &mut broadcast_message.1)?,
    ));
}}{}"#,
                        processing,
                        var_name,
                        if pos == event.broadcasts.len() - 1 {
                            ""
                        } else {
                            "\n"
                        }
                    );
                } else {
                    processing = format!(
                        r#"{}broadcasting.push((
    {}.0,
    unbound_pack(&0, &mut {}.1)?,
));{}"#,
                        processing,
                        var_name,
                        var_name,
                        if pos == event.broadcasts.len() - 1 {
                            ""
                        } else {
                            "\n"
                        }
                    );
                }
                vars = format!(
                    "{}{}mut {}",
                    vars,
                    if pos == 0 { "" } else { ", " },
                    var_name
                );
            }
            output = output.replace(
                "[[broadcasts_processing]]",
                &tools::inject_tabs(1, processing),
            );
            if event.broadcasts.len() > 1 {
                vars = format!("({})", vars);
            }
            output = output.replace("[[broadcast_vars]]", &vars);
        }
        helpers::fs::write(dest, output, true)
    }

    fn get_broadcast_var_name(&self, broadcast: &Broadcast) -> String {
        format!("broadcast_{}", broadcast.reference.replace(".", "_")).to_lowercase()
    }

    fn get_dest_file(&self, base: &Path, event: &Event) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("emitters");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(format!(
            "{}.rs",
            event.get_reference()?.to_lowercase().replace(".", "_")
        )))
    }
}
