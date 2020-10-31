#[derive(Debug)]
pub enum EnumValue {
    StringValue(String),
    NumericValue(usize),
}

#[derive(Debug)]
pub struct EnumItem {
    pub name: String,
    pub value: EnumValue,
}

#[derive(Debug)]
pub struct Enum {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub variants: Vec<String>,
}

impl Enum {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Enum {
            id,
            parent,
            name,
            variants: vec![],
        }
    }

    pub fn add(&mut self, val: String) {
        if self.variants.iter().any(|i| i.trim() == val.trim()) {
            panic!("Enum item \"{}\" is already defined", val);
        }
        self.variants.push(val);
    }

}