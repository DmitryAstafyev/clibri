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

#[path = "./render.broadcast.rs"]
pub mod render_broadcast;

#[path = "./render.producer.rs"]
pub mod render_producer;

#[path = "./render.app.rs"]
pub mod render_app;

#[path = "./render.protocol.rs"]
pub mod render_protocol;

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
    ProtocolRender,
    ProtocolRustRender,
};
use render_request::{ RenderRequest };
use render_event::{ RenderEvent };
use render_identification::{ RenderIdentification };
use render_producer::{ RenderProducer };
use render_consumer::{ RenderConsumer };
use render_event_connected::{ RenderEventConnected };
use render_broadcast::{ RenderBroadcast };
use render_event_disconnected::{ RenderEventDisconnected };
use render_app::{ RenderApp };
use render_protocol::{ RenderProtocol };
use std::{
    path::{
        Path,
    }
};

pub struct RustRender {
}

impl RustRender {
}

impl ImplementationRender<ProtocolRustRender> for RustRender {
    fn new() -> Self {
        RustRender {
        }
    }

    fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        protocol: &mut Protocol,
        protocol_render: ProtocolRustRender,
    ) -> Result<String, String> {
        for request in &store.requests {
            (RenderRequest::new()).render(base, &request)?;
        }
        for event in &store.events {
            (RenderEvent::new()).render(base, &event)?;
        }
        (RenderEventConnected::new()).render(base, &store.broadcast)?;
        (RenderEventDisconnected::new()).render(base, &store.broadcast)?;
        (RenderBroadcast::new()).render(base, &store.broadcast)?;
        (RenderIdentification::new()).render(base, store.get_config()?, protocol)?;
        (RenderConsumer::new()).render(base, store.get_config()?, protocol)?;
        (RenderProducer::new()).render(base, store, protocol)?;
        (RenderApp::new()).render(base, store, protocol)?;
        (RenderProtocol::new()).render(base, protocol, &protocol_render)?;
        Ok(String::new())
    }
}
