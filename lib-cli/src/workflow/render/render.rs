#[path = "./rust/render.rs"]
pub mod rust;

#[path = "./typescript/render.rs"]
pub mod typescript;

use super::{
    stop,
    helpers,
    workflow::self,
    workflow::{
        store::{
            Store as WorkflowStore
        }
    },
    protocol::store::{
        Store as Protocol
    },
};
use rust::{
    RustRender,
};
use std::{
    path::{
        Path,
        PathBuf,
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Rust,
    TypeScript,
}

pub trait ImplementationRender {

    fn new(signature: u16) -> Self;
    fn render(&self, base: &Path, store: &WorkflowStore, protocol: &Protocol) -> Result<String, String>;

}

pub struct ProtocolRefs {
    pub typescript: Option<PathBuf>,
    pub rust: Option<PathBuf>,
}

pub fn render(
    protocol_refs: ProtocolRefs,
    consumer_dest: Option<PathBuf>,
    provider_dest: Option<PathBuf>,
    store: WorkflowStore,
    protocol: &Protocol,
) -> Result<(), String> {
    let render: RustRender = RustRender::new(1);
    render.render(Path::new("/storage/projects/private/fiber/lib-cli/tmp"), &store, protocol)?;
    Ok(())
}