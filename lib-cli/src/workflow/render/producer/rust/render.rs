pub mod render_beacon_emitter;
pub mod render_beacon_emitters_mod;
pub mod render_beacon_impl;
pub mod render_beacon_impl_mod;
pub mod render_event_emitter;
pub mod render_event_emitters_mod;
pub mod render_event_impl;
pub mod render_event_impl_mod;
pub mod render_identification;
pub mod render_mod;
pub mod render_protocol;
pub mod render_request_handler;
pub mod render_request_handlers_mod;
pub mod render_request_response;
pub mod render_request_responses_mod;
pub mod render_static;

use super::{
    helpers, workflow, workflow::store::Store as WorkflowStore, ImplementationRender, Protocol,
    ProtocolRender, ProtocolRustRender,
};
use std::path::Path;

pub struct RustRender {}

impl RustRender {}

impl ImplementationRender<ProtocolRustRender> for RustRender {
    fn new() -> Self {
        RustRender {}
    }

    fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        protocol: &mut Protocol,
        protocol_render: ProtocolRustRender,
    ) -> Result<String, String> {
        for request in &store.requests {
            (render_request_response::Render::new()).render(base, request)?;
            (render_request_handler::Render::new()).render(base, request)?;
        }
        (render_request_responses_mod::Render::new()).render(base, &store.requests)?;
        (render_request_handlers_mod::Render::new()).render(base, &store.requests)?;
        for event in &store.events {
            (render_event_impl::Render::new()).render(base, event)?;
            (render_event_emitter::Render::new()).render(base, event)?;
        }
        for beacon in &store.beacons {
            (render_beacon_impl::Render::new()).render(base, beacon)?;
            (render_beacon_emitter::Render::new()).render(base, beacon)?;
        }
        (render_beacon_emitters_mod::Render::new()).render(base, &store.beacons)?;
        (render_beacon_impl_mod::Render::new()).render(base, &store.beacons)?;
        (render_event_impl_mod::Render::new()).render(base, &store.events)?;
        (render_event_emitters_mod::Render::new()).render(base, &store.events)?;
        (render_static::Render::new()).render(base, &store.events)?;
        (render_protocol::Render::new()).render(base, protocol, &protocol_render)?;
        (render_identification::Render::new()).render(base, store, &protocol)?;
        (render_mod::Render::new()).render(base, store, &protocol)?;
        Ok(String::new())
    }
}
