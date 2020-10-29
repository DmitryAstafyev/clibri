use super::{ PrimitiveField, Enum };

#[derive(Debug)]
pub struct Struct {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub fields: Vec<PrimitiveField>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
}

impl Struct {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Struct {
            id,
            parent,
            name,
            fields: vec![],
            structs: vec![],
            enums: vec![],
        }
    }

    pub fn find(&mut self, id: usize) -> Option<&mut Struct> {
        for child in self.structs.iter_mut() {
            if child.id == id {
                return Some(child);
            }
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }

    pub fn add_field(&mut self, mut field: PrimitiveField) {
        if self.fields.iter().any(|f| f.name == field.name) {
            panic!("Fail to add field \"{}\" into \"{}\" because field with same name already exist", field.name, self.name);
        }
        field.parent = self.id;
        self.fields.push(field);
    }

}
