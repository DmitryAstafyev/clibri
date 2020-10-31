use super::{ PrimitiveTypes };

#[derive(Debug)]
pub enum EReferenceToType {
    Struct,
    Enum,
}

#[derive(Debug)]
pub struct Field {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub kind: String,
    pub ref_type: Option<EReferenceToType>,
    pub ref_type_id: Option<usize>,
    pub repeated: bool,
}

impl Field {

    pub fn new(id: usize, parent: usize, kind: String) -> Self {
        Field {
            id,
            parent,
            name: String::new(),
            kind,
            ref_type: None,
            ref_type_id: None,
            repeated: false,
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

    pub fn set_type_ref(&mut self, ref_type: EReferenceToType, ref_type_id: usize) {
        self.ref_type = Some(ref_type);
        self.ref_type_id = Some(ref_type_id);
    }

    pub fn set_as_repeated(&mut self) {
        self.repeated = true;
    }

}