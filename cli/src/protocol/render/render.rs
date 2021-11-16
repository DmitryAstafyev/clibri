pub mod rust;
pub mod typescript;

use super::protocol::store::Store;
use super::*;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Rust,
    TypeScript,
}

pub trait Render {
    fn new(embedded: bool, signature: u16) -> Self;
    fn render(&self, store: &mut Store, dest: &Path) -> Result<(), String>;
}
