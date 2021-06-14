// use super::protocol::enums::Enum;
// use super::protocol::fields::Field;
// use super::protocol::groups::Group;
// use super::protocol::store::Store;
// use super::protocol::structs::Struct;
// use super::protocol::types::PrimitiveTypes;
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

pub struct TypescriptRender {
}

impl TypescriptRender {
}

impl ImplementationRender for TypescriptRender {
    fn new() -> Self {
        TypescriptRender {
        }
    }

    fn render(&self, _base: &Path, _store: &WorkflowStore, _protocol: &Protocol) -> Result<String, String> {
        Ok(String::new())
    }
}
