// use super::protocol::enums::Enum;
// use super::protocol::fields::Field;
// use super::protocol::groups::Group;
// use super::protocol::store::Store;
// use super::protocol::structs::Struct;
// use super::protocol::types::PrimitiveTypes;
mod render_event_emitter;
mod render_event_impl;
mod render_request_handler;
mod render_request_response;

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
        for event in &store.events {
            (render_event_impl::Render::new()).render(base, event)?;
            (render_event_emitter::Render::new()).render(base, event)?;
        }
        Ok(String::new())
    }
}
