// use super::protocol::enums::Enum;
// use super::protocol::fields::Field;
// use super::protocol::groups::Group;
// use super::protocol::store::Store;
// use super::protocol::structs::Struct;
// use super::protocol::types::PrimitiveTypes;
use super::workflow::{
    store::{Store as WorkflowStore}
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

    fn render(&self, base: &Path, store: &WorkflowStore) -> Result<String, String> {
        Ok(String::new())
    }
}
