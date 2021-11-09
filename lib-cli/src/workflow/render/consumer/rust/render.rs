#[path = "./render_broadcast.rs"]
pub mod render_broadcast;

#[path = "./render_broadcast_mod.rs"]
pub mod render_broadcast_mod;

#[path = "./render_static.rs"]
pub mod render_static;

#[path = "./render_controller.rs"]
pub mod render_controller;

#[path = "./render_consumer.rs"]
pub mod render_consumer;

#[path = "./render_protocol.rs"]
pub mod render_protocol;

use super::{
    helpers, workflow, workflow::beacon::Broadcast, workflow::store::Store as WorkflowStore,
    ImplementationRender, Protocol, ProtocolRender, ProtocolRustRender,
};
use std::path::Path;

pub struct RustRender {}

impl RustRender {
    fn get_all_broadcasts(&self, store: &WorkflowStore) -> Vec<Broadcast> {
        let mut broadcasts: Vec<Broadcast> = vec![];
        for request in &store.requests {
            for action in &request.actions {
                for broadcast in &action.broadcast {
                    if broadcasts
                        .iter()
                        .any(|i| i.reference == broadcast.reference)
                    {
                        continue;
                    } else {
                        broadcasts.push(broadcast.clone());
                    }
                }
            }
        }
        for event in &store.events {
            for broadcast in &event.broadcasts {
                if broadcasts
                    .iter()
                    .any(|i| i.reference == broadcast.reference)
                {
                    continue;
                } else {
                    broadcasts.push(broadcast.clone());
                }
            }
        }
        broadcasts
    }
}

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
        let broadcasts = self.get_all_broadcasts(store);
        for broadcast in &broadcasts {
            (render_broadcast::Render::new()).render(base, &broadcast)?;
        }
        (render_broadcast_mod::Render::new()).render(base, &broadcasts)?;
        (render_controller::Render::new()).render(base, store)?;
        (render_consumer::Render::new()).render(base, store, protocol, &broadcasts)?;
        (render_protocol::Render::new()).render(base, protocol, &protocol_render)?;
        (render_static::Render::new()).render(base)?;

        Ok(String::new())
    }
}
