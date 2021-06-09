#[path = "./render.request.rs"]
pub mod render_request;

#[path = "./render.event.rs"]
pub mod render_event;

#[path = "./render.identification.rs"]
pub mod render_identification;

#[path = "./render.consumer.rs"]
pub mod render_consumer;

#[path = "./render.event.connected.rs"]
pub mod render_event_connected;

#[path = "./render.event.disconnected.rs"]
pub mod render_event_disconnected;

#[path = "./render.producer.rs"]
pub mod render_producer;

#[path = "./render.app.rs"]
pub mod render_app;

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
use render_producer::{ RenderProducer };
use render_consumer::{ RenderConsumer };
use render_event_connected::{ RenderEventConnected };
use render_event_disconnected::{ RenderEventDisconnected };
use render_app::{ RenderApp };
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
            (RenderRequest::new()).render(base, &request)?;
        }
        for event in &store.events {
            (RenderEvent::new()).render(base, &event)?;
        }
        (RenderEventConnected::new()).render(base)?;
        (RenderEventDisconnected::new()).render(base)?;
        (RenderIdentification::new()).render(base, store.get_config()?, protocol)?;
        (RenderConsumer::new()).render(base, store.get_config()?, protocol)?;
        (RenderProducer::new()).render(base, store, protocol)?;
        (RenderApp::new()).render(base, store, protocol)?;
        Ok(String::new())
    }
}
