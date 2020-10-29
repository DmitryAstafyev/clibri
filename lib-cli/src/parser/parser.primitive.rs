use super::{ PrimitiveTypes };

#[derive(Debug)]
pub struct PrimitiveField {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub kind: String,
}

impl PrimitiveField {

    pub fn new(id: usize, parent: usize, kind: String) -> Self {
        PrimitiveField {
            id,
            parent,
            name: String::new(),
            kind,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_type(&mut self, kind: PrimitiveTypes::ETypes) {
        if let Some(primitive) = PrimitiveTypes::get_entity_as_string(kind) {
            self.kind = primitive;
        } else {
            panic!("Unknown type");
        }
    }

}