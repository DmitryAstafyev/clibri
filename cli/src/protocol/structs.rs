use super::{stop, Field};

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub fields: Vec<Field>,
    pub path: Vec<String>,
}

impl Struct {
    pub fn new(id: usize, parent: usize, name: String, path: Vec<String>) -> Self {
        Struct {
            id,
            parent,
            name,
            fields: vec![],
            path,
        }
    }

    pub fn add_field(&mut self, mut field: Field) {
        if self.fields.iter().any(|f| f.name == field.name) {
            stop!(
                "Fail to add field \"{}\" into \"{}\" because field with same name already exist",
                field.name,
                self.name
            );
        }
        field.parent = self.id;
        self.fields.push(field);
    }

    pub fn set_fields(&mut self, mut fields: Vec<Field>) {
        for field in fields.iter_mut() {
            field.parent = self.id;
        }
        self.fields = fields;
    }
}
