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
    pub const MODULE: &str = 
r#"use super::{
    Filter,
    Broadcast,
    Protocol::PackingStruct,
};
use uuid::Uuid;

#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        uuid: Uuid,
        ucx: UCX,
        broadcast: &dyn Fn(Filter, Broadcast) -> Result<(), String>,
    ) -> () {
        panic!("hanlder method for Connected isn't implemented");
    }
}

#[derive(Clone)]
pub struct ObserverEvent {}

impl ObserverEvent {
    pub fn new() -> Self {
        ObserverEvent {}
    }

    pub fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        uuid: Uuid,
        ucx: UCX,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> () {
        Self::handler(uuid, ucx, &(|filter: Filter, message: Broadcast| {
            broadcast(filter, match message {[[messages]],
            })
        }));
    }
}

impl Observer for ObserverEvent {}
"#;
}

pub struct RenderEventDisconnected {
}

impl Default for RenderEventDisconnected {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEventDisconnected {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        broadcasts: &Vec<Broadcast>,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut messages: String = String::new();
        for broadcast in broadcasts {
            messages = format!("{}\nBroadcast::{}(mut msg) => msg.pack(0, None)?", messages, broadcast.reference.replace(".", ""));
        }
        let output = templates::MODULE.replace("[[messages]]", &tools::inject_tabs(4, messages));
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("events");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("default_event_disconnected.rs"))
    }

}

