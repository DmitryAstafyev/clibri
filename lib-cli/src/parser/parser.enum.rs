
use super::{ PrimitiveTypes };

#[derive(Debug)]
pub struct EnumItem {
    pub name: String,
    pub types: Option<PrimitiveTypes::ETypes>,
    pub ref_type_id: Option<usize>,
    pub str_value: Option<String>,
}

#[derive(Debug)]
pub struct Enum {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub variants: Vec<EnumItem>,
    current: Option<EnumItem>,
}

impl Enum {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Enum {
            id,
            parent,
            name,
            variants: vec![],
            current: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        if let Some(mut current) = self.current.take() {
            current.name = name;
            if current.ref_type_id.is_some() {
                self.variants.push(current);
                self.current = None;
            } else {
                self.current = Some(current);
            }
        } else {
            panic!("Cannot set name of enum item, because enum item wasn't opened");
        }
    }

    pub fn set_type(&mut self, types: PrimitiveTypes::ETypes) {
        if self.current.is_some() {
            panic!("Attempt to add new enum item, while previous isn't closed");
        }
        self.current = Some(EnumItem{
            str_value: Some(String::new()),
            types: Some(types),
            ref_type_id: None,
            name: String::new(),
        });
    }

    pub fn set_type_ref(&mut self, ref_type_id: usize) {
        if self.current.is_some() {
            panic!("Attempt to add new enum item, while previous isn't closed");
        }
        self.current = Some(EnumItem{
            str_value: None,
            types: None,
            ref_type_id: Some(ref_type_id),
            name: String::new(),
        });
    }

    pub fn set_simple(&mut self, value: &str) {
        self.set_type(PrimitiveTypes::ETypes::Estr);
        self.set_name(value.to_string());
        self.set_inital_value(Some(value.to_string()));
    }

    pub fn set_inital_value(&mut self, value: Option<String>) {
        if let Some(mut current) = self.current.take() {
            current.str_value = value;
            self.variants.push(current);
            self.current = None;
        } else {
            panic!("Cannot set name of enum item, because enum item wasn't opened");
        }
    }

}