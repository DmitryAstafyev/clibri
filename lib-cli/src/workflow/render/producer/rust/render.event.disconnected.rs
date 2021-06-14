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
r#"use super::{
    Filter,
    Broadcast,
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
        broadcast: &dyn Fn(Filter, Broadcast) -> Result<(), String>,
    ) -> () {
        Self::handler(uuid, ucx, broadcast);
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
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        helpers::fs::write(dest, templates::MODULE.to_owned(), true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("events");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        Ok(dest.join("event_disconnected.rs"))
    }

}

