#[path = "./render.request.rs"]
pub mod render_request;

use super::{
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
use std::{
    path::{
        Path,
    }
};

pub struct UmlRender {
}

impl UmlRender {
}

impl UmlRender {
    pub fn new() -> Self {
        UmlRender {}
    }

    pub fn render(
        &self,
        dest: &Path,
        store: &WorkflowStore,
        _protocol: &mut Protocol,
    ) -> Result<(), String> {
        let mut output: String = String::new();
        for request in &store.requests {
            output = format!("{}\n{}",
                output,
                RenderRequest::new().render(request)?
            );
        }
        helpers::fs::write(dest.to_path_buf(), output, true)
    }
}
