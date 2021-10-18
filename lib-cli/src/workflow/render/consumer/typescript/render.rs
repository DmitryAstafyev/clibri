#[path = "./render.request.rs"]
pub mod render_request;

#[path = "./render.consumer.rs"]
pub mod render_consumer;

#[path = "./render.interfaces.request.rs"]
pub mod render_interfaces_request;

#[path = "./render.options.rs"]
pub mod render_options;

#[path = "./render.protocol.rs"]
pub mod render_protocol;

use super::{
    helpers, workflow, workflow::store::Store as WorkflowStore, ImplementationRender, Protocol,
    ProtocolRender, ProtocolTypescriptRender,
};
use render_consumer::RenderConsumer;
use render_interfaces_request::RenderInterfacesRequest;
use render_options::RenderOptions;
use render_protocol::RenderProtocol;
use render_request::RenderRequest;
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
            (RenderRequest::new()).render(base, &request)?;
        }
        (RenderConsumer::new()).render(base, store, &protocol)?;
        (RenderInterfacesRequest::new()).render(base)?;
        (RenderOptions::new()).render(base)?;
        (RenderProtocol::new()).render(base, protocol, &protocol_render)?;
        Ok(String::new())
    }
}
