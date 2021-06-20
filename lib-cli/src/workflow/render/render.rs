#[path = "./producer/render.rs"]
pub mod producer;

#[path = "./consumer/render.rs"]
pub mod consumer;

#[path = "./uml/render.rs"]
pub mod uml;

use super::{
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
    render::{
        Target,
        Render as ProtocolRender,
        rust::{
            RustRender as ProtocolRustRender,
        },
        typescript::{
            TypescriptRender as ProtocolTypescriptRender,
        }
    },
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

pub trait ImplementationRender<T: ProtocolRender> {

    fn new() -> Self;
    fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        protocol: &mut Protocol,
        protocol_render: T,
    ) -> Result<String, String>;

}

pub fn render(
    mut consumer_dest: Option<PathBuf>,
    mut producer_dest: Option<PathBuf>,
    store: WorkflowStore,
    protocol: &mut Protocol,
) -> Result<(), String> {
    let consumer_outs = &(store.get_config()?.consumer);
    let producer_outs = &(store.get_config()?.producer);
    if consumer_dest.is_none() && !consumer_outs.is_empty() {
        return Err(String::from("Destination folder for consumer isn't defined. Use --consumer-dest (or -cd) to define it."))
    }
    if producer_dest.is_none() && !producer_outs.is_empty() {
        return Err(String::from("Destination folder for producer isn't defined. Use --producer-dest (or -pd) to define it."))
    }
    if let Some(consumer_dest) = consumer_dest.take() {
        mkdir(&consumer_dest)?;
        for out in consumer_outs {
            let mut dest = consumer_dest.clone();
            match out {
                Target::Rust => {
                    if consumer_outs.len() > 1 {
                        dest = dest.join("rust");
                        mkdir(&dest)?;
                    }
                    (ConsumerRustRender::new()).render(&dest, &store, protocol, ProtocolRustRender::new(true, 0))?;
                },
                Target::TypeScript => {
                    if consumer_outs.len() > 1 {
                        dest = dest.join("typescript");
                        mkdir(&dest)?;
                    }
                    (ConsumerTypescriptRender::new()).render(&dest, &store, protocol, ProtocolTypescriptRender::new(true, 0))?;
                },
            }
        }
    }
    if let Some(producer_dest) = producer_dest.take() {
        mkdir(&producer_dest)?;
        for out in producer_outs {
            let mut dest = producer_dest.clone();
            match out {
                Target::Rust => {
                    if producer_outs.len() > 1 {
                        dest = dest.join("rust");
                        mkdir(&dest)?;
                    }
                    (ProducerRustRender::new()).render(&dest, &store, protocol, ProtocolRustRender::new(true, 0))?;
                },
                Target::TypeScript => {
                    if producer_outs.len() > 1 {
                        dest = dest.join("typescript");
                        mkdir(&dest)?;
                    }
                    (ProducerTypescriptRender::new()).render(&dest, &store, protocol, ProtocolTypescriptRender::new(true, 0))?;
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