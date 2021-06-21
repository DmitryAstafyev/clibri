#[path = "./render.request.rs"]
pub mod render_request;
#[path = "./render.event.rs"]
pub mod render_event;
#[path = "./render.broadcasts.rs"]
pub mod render_broadcasts;
use super::{
    helpers,
    helpers::{
        render as tools,
    },
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
use render_broadcasts::{ RenderBroadcasts };
use std::{
    path::{
        Path,
    }
};
mod templates {
    pub const MODULE: &str = 
r#"@startuml

    collections Consumers as Consumers
    control Controller as Controller
    [[content]]
@enduml"#;
}
pub struct PumlRender {
}

impl PumlRender {
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
            output = format!("{}\n{}\n",
                output,
                tools::inject_tabs(1, RenderRequest::new().render(request)?),
            );
        }
        for event in &store.events {
            output = format!("{}\n{}\n",
                output,
                tools::inject_tabs(1, RenderEvent::new().render(event)?),
            );
        }
        output = format!("{}\n{}\n",
            output,
            tools::inject_tabs(1, RenderBroadcasts::new().render(&store.broadcast)?),
        );
        output = templates::MODULE.replace("[[content]]", &output);
        helpers::fs::write(dest.to_path_buf(), output, true)
    }
}
