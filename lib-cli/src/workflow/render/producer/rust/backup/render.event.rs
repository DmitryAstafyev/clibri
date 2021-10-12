use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        event::{
            Event
        }
    }
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
    Control,
    Protocol,
    Protocol::PackingStruct,
    consumer_identification::Filter
};
use tokio::{
    sync::mpsc::{
        UnboundedReceiver,
    }
};
use fiber::env::logs;
use log::{error};

pub enum Broadcast {
[[broadcast]],
}

#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        event: &Protocol::[[event_ref]],
        ucx: UCX,
        control: Control,
    ) -> Option<Vec<(Filter, Broadcast)>> {
        panic!("hanlder method for [[event_ref]] isn't implemented");
    }
}

#[derive(Clone)]
pub struct ObserverEvent {
    
}

impl ObserverEvent {

    pub async fn listen<UCX: 'static + Sync + Send + Clone>(
        ucx: UCX,
        control: Control,
        mut rx_event: UnboundedReceiver<Protocol::[[event_ref]]>,
    ) {
        while let Some(event) = rx_event.recv().await {
            if let Some(mut messages) = Self::handler(
                &event,
                ucx.clone(),
                control.clone()
            ) {
                loop {
                    if messages.is_empty() {
                        break;
                    }
                    let (filter, message) = messages.remove(0);
                    match match message {
[[broadcasts_pack]],
                    } {
                        Ok(buffer) => if let Err(err) = control.send(filter, buffer) {
                            error!(target: logs::targets::PRODUCER, "[event: [[event_ref]]] fail to send message due error: {}", err);
                        },
                        Err(err) => {
                            error!(target: logs::targets::PRODUCER, "[event: [[event_ref]]] fail to get a buffer due error: {}", err);
                        },
                    }
                }
            }
        }
    }
}

impl Observer for ObserverEvent {}

"#;
}

pub struct RenderEvent {
}

impl Default for RenderEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEvent {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        event: &Event,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, event)?;
        let event_ref = self.into_rust_path(&event.get_reference()?);
        let mut broadcasts_enum: String = String::new();
        let mut broadcasts_pack: String = String::new();
        for (pos, broadcast) in event.broadcasts.iter().enumerate() {
            let enum_item = broadcast.to_string().replace(".", "");
            broadcasts_enum = format!("{}{}",
                broadcasts_enum,
                format!("{}(Protocol::{})",
                    enum_item,
                    self.into_rust_path(broadcast),
                ),
            );
            broadcasts_pack = format!("{}Broadcast::{}(mut msg) => msg.pack(0, None)",
                broadcasts_pack,
                enum_item,
            );
            if pos < event.broadcasts.len() - 1 {
                broadcasts_enum = format!("{},\n", broadcasts_enum);
                broadcasts_pack = format!("{},\n", broadcasts_pack);
            }
        }
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[event_ref]]", &event_ref);
        output = output.replace("[[broadcasts_pack]]", &tools::inject_tabs(6, broadcasts_pack));
        output = output.replace("[[broadcast]]", &tools::inject_tabs(1, broadcasts_enum));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, event: &Event) -> Result<PathBuf, String> {
        let dest = base.join("events");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        let event = event.get_reference()?;
        Ok(dest.join(format!("{}.rs", event.to_lowercase().replace(".", "_"))))
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }



}

