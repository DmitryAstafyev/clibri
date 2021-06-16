use super::{
    workflow::{
        store::{
            Store as WorkflowStore
        },
    },
    Protocol,
    ProtocolRender,
    ProtocolRustRender,
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

impl ImplementationRender<ProtocolRustRender> for RustRender {
    fn new() -> Self {
        RustRender {}
    }

    fn render(
        &self,
        _base: &Path,
        _store: &WorkflowStore,
        _protocol: &mut Protocol,
        _protocol_render: ProtocolRustRender,
    ) -> Result<String, String> {
        Ok(String::new())
    }
}
