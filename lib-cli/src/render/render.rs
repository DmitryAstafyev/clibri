use super::*;
use super::parser::store::Store;

#[path = "./render.rust.rs"]
pub mod rust;

pub enum ERender {
    Rust,
    TypeScript,
}

pub trait Render {

    fn render(&self, store: Store) -> String;

}
