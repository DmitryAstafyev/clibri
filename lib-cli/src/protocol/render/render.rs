use super::*;
use super::protocol::store::Store;

#[path = "./render.rust.rs"]
pub mod rust;

#[path = "./render.typescript.rs"]
pub mod typescript;

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Rust,
    TypeScript,
}

pub trait Render {

    fn new(embedded: bool, signature: u16) -> Self;
    fn render(&self, store: &mut Store) -> String;

}
