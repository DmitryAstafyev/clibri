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
use super::{ ImplementationRender, stop };
use regex::Regex;
use std::include_str;
use std::{
    path::{
        Path,
        PathBuf,
    }
};

pub struct RustRender {
    signature: u16,
}

impl RustRender {
}

impl ImplementationRender for RustRender {
    fn new(signature: u16) -> Self {
        RustRender {
            signature,
        }
    }

    fn render(&self, base: &Path, store: &WorkflowStore, protocol: &Protocol) -> Result<String, String> {
        Ok(String::new())
    }
}
