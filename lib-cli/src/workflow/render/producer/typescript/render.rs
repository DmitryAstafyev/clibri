mod render_beacon_emitter;
mod render_beacon_emitters_mod;
mod render_beacon_impl;
mod render_event_emitter;
mod render_event_emitters_mod;
mod render_event_impl;
mod render_identification;
mod render_mod;
mod render_protocol;
mod render_request_handler;
mod render_request_handlers_mod;
mod render_request_response;
mod render_static;

use super::{
    helpers, workflow, workflow::store::Store as WorkflowStore, ImplementationRender, Protocol,
    ProtocolRender, ProtocolTypescriptRender,
};
use std::path::Path;

pub struct TypescriptRender {}

impl TypescriptRender {}

impl ImplementationRender<ProtocolTypescriptRender> for TypescriptRender {
    fn new() -> Self {
        TypescriptRender {}
    }

    fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        protocol: &mut Protocol,
        protocol_render: ProtocolTypescriptRender,
    ) -> Result<String, String> {
        for request in &store.requests {
            (render_request_response::Render::new()).render(base, request)?;
            (render_request_handler::Render::new()).render(base, request)?;
        }
        (render_request_handlers_mod::Render::new()).render(base, &store.requests)?;
        for event in &store.events {
            (render_event_impl::Render::new()).render(base, event)?;
            (render_event_emitter::Render::new()).render(base, event)?;
        }
        (render_event_emitters_mod::Render::new()).render(base, &store.events)?;
        for beacon in &store.beacons {
            (render_beacon_impl::Render::new()).render(base, beacon)?;
            (render_beacon_emitter::Render::new()).render(base, beacon)?;
        }
        (render_beacon_emitters_mod::Render::new()).render(base, &store.beacons)?;
        (render_protocol::Render::new()).render(base, protocol, &protocol_render)?;
        (render_identification::Render::new()).render(base, store, &protocol)?;
        (render_static::Render::new()).render(base, &store.events)?;
        (render_mod::Render::new()).render(base, store, &protocol)?;
        Ok(String::new())
    }
}
