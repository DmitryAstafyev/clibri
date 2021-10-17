#[path = "./render.beacon.rs"]
pub mod render_beacon;
#[path = "./render.event.rs"]
pub mod render_event;
#[path = "./render.request.rs"]
pub mod render_request;
use super::{
    helpers, helpers::render as tools, workflow, workflow::store::Store as WorkflowStore, Protocol,
};
use render_beacon::RenderBeacons;
use render_event::RenderEvent;
use render_request::RenderRequest;
use std::path::Path;
mod templates {
    pub const MODULE: &str = r#"@startuml

    collections Consumers as Consumers
    control Controller as Controller
    [[content]]
@enduml"#;
}
pub struct PumlRender {}

impl PumlRender {}

impl Default for PumlRender {
    fn default() -> Self {
        Self::new()
    }
}

impl PumlRender {
    pub fn new() -> Self {
        PumlRender {}
    }

    pub fn render(
        &self,
        dest: &Path,
        store: &WorkflowStore,
        _protocol: &mut Protocol,
    ) -> Result<(), String> {
        let mut output: String = String::new();
        for request in &store.requests {
            output = format!(
                "{}\n{}\n",
                output,
                tools::inject_tabs(1, RenderRequest::new().render(request)?),
            );
        }
        for event in &store.events {
            output = format!(
                "{}\n{}\n",
                output,
                tools::inject_tabs(1, RenderEvent::new().render(event)?),
            );
        }
        output = format!(
            "{}\n{}\n",
            output,
            tools::inject_tabs(1, RenderBeacons::new().render(&store.beacons)?),
        );
        output = templates::MODULE.replace("[[content]]", &output);
        helpers::fs::write(dest.to_path_buf(), output, true)
    }
}
