use super::*;
use super::parser::store::Store;

#[path = "./render.rust.rs"]
pub mod rust;

#[path = "./render.typescript.rs"]
pub mod typescript;

pub enum ERender {
    Rust,
    TypeScript,
}

pub trait Render {

    fn render(&self, store: Store) -> String;

}
