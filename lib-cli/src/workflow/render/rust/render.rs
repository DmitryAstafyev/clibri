#[path = "./render.request.rs"]
pub mod render_request;

#[path = "./render.event.rs"]
pub mod render_event;

#[path = "./render.identification.rs"]
pub mod render_identification;

#[path = "./render.lib.rs"]
pub mod render_lib;

use super::{
    ImplementationRender,
    helpers,
    workflow,
    workflow::{
        store::{
            Store as WorkflowStore
        }
    },
    Protocol,
};
use render_request::{ RenderRequest };
use render_event::{ RenderEvent };
use render_identification::{ RenderIdentification };
use render_lib::{ RenderLib };
use std::{
    path::{
        Path,
        PathBuf,
    }
};

pub struct RustRender {
    signature: u16,
}

impl RustRender {
}

impl ImplementationRender for RustRender {
    fn new(signature: u16) -> Self {
        RustRender {
            signature,
        }
    }

    fn render(&self, base: &Path, store: &WorkflowStore, protocol: &Protocol) -> Result<String, String> {
        for request in &store.requests {
            let render = RenderRequest::new();
            render.render(base, &request)?;
        }
        for event in &store.events {
            let render = RenderEvent::new();
            render.render(base, &event)?;
        }
        (RenderIdentification::new()).render(base, store.get_config()?, protocol)?;
        (RenderLib::new()).render(base, store, protocol)?;
        Ok(String::new())
    }
}
