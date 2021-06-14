use super::{
    workflow::{
        store::{
            Store as WorkflowStore
        },
    },
    Protocol,
};
use super::{ ImplementationRender };
use std::{
    path::{
        Path,
    }
};

pub struct RustRender {
}

impl RustRender {
}

impl ImplementationRender for RustRender {
    fn new() -> Self {
        RustRender {}
    }

    fn render(&self, _base: &Path, _store: &WorkflowStore, _protocol: &Protocol) -> Result<String, String> {
        Ok(String::new())
    }
}
