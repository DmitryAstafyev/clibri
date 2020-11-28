use super::*;

use super::parser::fields::Field;
use super::parser::groups::Group;
use super::parser::store::Store;
use super::parser::structs::Struct;

#[path = "./render.rust.rs"]
pub mod rust;

pub trait Render {

    fn render(&self, store: Store) -> String {
        let mut body = String::new();
        for strct in &store.structs {
            if strct.parent == 0 {
                body = format!("{}{}\n", body, self.structs(strct, &mut store.clone(), 0)).to_string();
            }
        }
        for group in &store.groups {
            if group.parent == 0 {
                body = format!("{}{}\n", body, self.groups(group, &mut store.clone(), 0)).to_string();
            }
        }
        body
    }

    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String;

    fn structs(&self, strct: &Struct, store: &mut Store, level: u8) -> String;

    //fn fields(&self, field: &Field) -> String;

    fn spaces(&self, level: u8) -> String {
        "    ".repeat(level as usize)
    }
}
