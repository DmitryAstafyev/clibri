#[path = "./render.request.rs"]
pub mod render_request;

#[path = "./render.consumer.rs"]
pub mod render_consumer;

#[path = "./render.interfaces.request.rs"]
pub mod render_interfaces_request;

#[path = "./render.options.rs"]
pub mod render_options;

use super::{
    workflow,
    workflow::{
        store::{
            Store as WorkflowStore
        },
    },
    ImplementationRender,
    helpers,
    Protocol,
};
use render_request::{ RenderRequest };
use render_consumer::{ RenderConsumer };
use render_interfaces_request::{ RenderInterfacesRequest };
use render_options::{ RenderOptions };
use std::{
    path::{
        Path,
    }
};

pub struct TypescriptRender {
}

impl TypescriptRender {
}

impl ImplementationRender for TypescriptRender {
    fn new() -> Self {
        TypescriptRender {
        }
    }

    fn render(&self, base: &Path, store: &WorkflowStore, _protocol: &Protocol) -> Result<String, String> {
        for request in &store.requests {
            (RenderRequest::new()).render(base, &request)?;
        }
        (RenderConsumer::new()).render(base, store)?;
        (RenderInterfacesRequest::new()).render(base)?;
        (RenderOptions::new()).render(base)?;
        Ok(String::new())
    }
}
