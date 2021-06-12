#[path = "./render.request.rs"]
pub mod render_request;

use super::{
    workflow,
    workflow::{
        store::{
            Store as WorkflowStore
        },
    },
    ImplementationRender,
    stop,
    helpers,
    Protocol,
};
use render_request::{ RenderRequest };

use regex::Regex;
use std::include_str;
use std::{
    path::{
        Path,
        PathBuf,
    }
};

pub struct TypescriptRender {
    signature: u16,
}

impl TypescriptRender {
}

impl ImplementationRender for TypescriptRender {
    fn new(signature: u16) -> Self {
        TypescriptRender {
            signature,
        }
    }

    fn render(&self, base: &Path, store: &WorkflowStore, protocol: &Protocol) -> Result<String, String> {
        for request in &store.requests {
            (RenderRequest::new()).render(base, &request)?;
        }
        Ok(String::new())
    }
}
