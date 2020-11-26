use super::{ Field };

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub fields: Vec<Field>,
}

impl Struct {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Struct {
            id,
            parent,
            name,
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, mut field: Field) {
        if self.fields.iter().any(|f| f.name == field.name) {
            panic!("Fail to add field \"{}\" into \"{}\" because field with same name already exist", field.name, self.name);
        }
        field.parent = self.id;
        self.fields.push(field);
    }

}
