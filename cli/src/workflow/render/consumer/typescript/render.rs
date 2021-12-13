pub mod render_beacon;
pub mod render_consumer;
pub mod render_interfaces_request;
pub mod render_options;
pub mod render_package;
pub mod render_protocol;
pub mod render_request;
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
            (render_request::Render::new()).render(base, request)?;
        }
        for beacon in &store.beacons {
            (render_beacon::Render::new()).render(base, beacon)?;
        }
        (render_consumer::Render::new()).render(base, store, protocol)?;
        (render_interfaces_request::Render::new()).render(base)?;
        (render_options::Render::new()).render(base)?;
        (render_protocol::Render::new()).render(base, protocol, &protocol_render)?;
        (render_package::Render::new()).render(base)?;
        Ok(String::new())
    }
}
