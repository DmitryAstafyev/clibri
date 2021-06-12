#[path = "./producer/render.rs"]
pub mod producer;

#[path = "./consumer/render.rs"]
pub mod consumer;

use super::{
    stop,
    helpers,
    workflow,
    workflow::{
        store::{
            Store as WorkflowStore
        }
    },
    protocol::store::{
        Store as Protocol
    },
    render::Target,
};
use producer::{
    rust::{ RustRender as ProducerRustRender },
    typescript::{ TypescriptRender as ProducerTypescriptRender },
};
use consumer::{
    rust::{ RustRender as ConsumerRustRender },
    typescript::{ TypescriptRender as ConsumerTypescriptRender },
};

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

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
    mut consumer_dest: Option<PathBuf>,
    mut producer_dest: Option<PathBuf>,
    store: WorkflowStore,
    protocol: &Protocol,
) -> Result<(), String> {
    if let Some(consumer_dest) = consumer_dest.take() {
        mkdir(&consumer_dest)?;
        let outs = &(store.get_config()?.consumer);
        for out in outs {
            let mut dest = consumer_dest.clone();
            match out {
                Target::Rust => {
                    if outs.len() > 1 {
                        dest = dest.join("rust");
                        mkdir(&dest)?;
                    }
                    (ConsumerRustRender::new(0)).render(&dest, &store, protocol)?;
                },
                Target::TypeScript => {
                    if outs.len() > 1 {
                        dest = dest.join("typescript");
                        mkdir(&dest)?;
                    }
                    (ConsumerTypescriptRender::new(0)).render(&dest, &store, protocol)?;
                },
            }
        }
    }
    if let Some(producer_dest) = producer_dest.take() {
        mkdir(&producer_dest)?;
        let outs = &(store.get_config()?.producer);
        for out in outs {
            let mut dest = producer_dest.clone();
            match out {
                Target::Rust => {
                    if outs.len() > 1 {
                        dest = dest.join("rust");
                        mkdir(&dest)?;
                    }
                    (ProducerRustRender::new(0)).render(&dest, &store, protocol)?;
                },
                Target::TypeScript => {
                    if outs.len() > 1 {
                        dest = dest.join("typescript");
                        mkdir(&dest)?;
                    }
                    (ProducerTypescriptRender::new(0)).render(&dest, &store, protocol)?;
                },
            }
        }
    }
    Ok(())
}

fn mkdir(dest: &Path) -> Result<(), String> {
    if !dest.exists() {
        if let Err(e) = fs::create_dir(&dest) {
            return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
        }
    }
    Ok(())
}