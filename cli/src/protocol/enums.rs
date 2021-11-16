use super::{stop, PrimitiveTypes, Store};

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub name: String,
    pub types: Option<PrimitiveTypes::ETypes>,
    pub ref_type_id: Option<usize>,
    pub ref_type_path: Vec<usize>,
    pub repeated: bool,
    pub type_path: Vec<String>,
}

impl EnumItem {
    pub fn accept_type(&mut self, store: &Store, own_group_id: usize) {
        if self.type_path.is_empty() {
            stop!("Fail to accept field type because no any type references were provided");
        }
        let first = self.type_path[0].clone();
        if self.type_path.len() == 1 && PrimitiveTypes::get_entity(&first).is_some() {
            if let Some(type_ref) = PrimitiveTypes::get_entity(&first) {
                self.types = Some(type_ref);
            } else {
                stop!("Fail to get primitive type {}", first);
            }
        } else {
            let path = if let Some(path) = store.find_by_path(own_group_id, &self.type_path) {
                // Has been found in own group
                path
            } else if let Some(path) = store.find_by_path(0, &self.type_path) {
                // Has been found in root group
                path
            } else {
                stop!("Fail to find type: {}", self.type_path.join("."));
            };
            let (_, type_id) = path[path.len() - 1].clone();
            self.ref_type_id = Some(type_id);
            self.ref_type_path = path[0..path.len() - 1]
                .iter()
                .map(|(_name, id)| *id)
                .collect();
        }
    }

    pub fn add_type_path(&mut self, type_str: &str) {
        self.type_path.push(String::from(type_str));
    }

    pub fn get_path(&self) -> String {
        self.type_path[0..self.type_path.len() - 1].join("::")
    }

    pub fn get_full_name(&self) -> String {
        self.type_path.join("::")
    }
}

#[derive(Debug, Clone)]
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
            self.variants.push(current);
            self.current = None;
        } else {
            stop!("Cannot set name of enum item, because enum item wasn't opened");
        }
    }

    pub fn set_type_ref(&mut self, ref_type_id: usize) {
        if let Some(mut current) = self.current.take() {
            if current.ref_type_path.is_empty() {
                stop!("Attempt to add new enum item, while previous isn't closed");
            } else if current.ref_type_id.is_some() {
                stop!("Type of enum's option is already defined");
            } else {
                current.ref_type_id = Some(ref_type_id);
                self.current = Some(current);
            }
        } else {
            self.current = Some(EnumItem {
                types: None,
                ref_type_id: Some(ref_type_id),
                name: String::new(),
                ref_type_path: vec![],
                repeated: false,
                type_path: vec![],
            });
        }
    }

    pub fn set_as_repeated(&mut self) {
        if let Some(mut current) = self.current.take() {
            current.repeated = true;
            self.current = Some(current);
        } else {
            stop!("Cannot set repeated flag of enum item, because enum item wasn't opened");
        }
    }

    pub fn set_simple(&mut self, value: &str) {
        if self.current.is_some() {
            stop!("Attempt to add new enum item, while previous isn't closed");
        }
        self.current = Some(EnumItem {
            types: Some(PrimitiveTypes::ETypes::Estr),
            ref_type_id: None,
            name: String::new(),
            ref_type_path: vec![],
            repeated: false,
            type_path: vec![],
        });
        self.set_name(value.to_string());
    }

    pub fn add_type_path(&mut self, type_str: &str) {
        if self.current.is_none() {
            self.current = Some(EnumItem {
                types: None,
                ref_type_id: None,
                name: String::new(),
                ref_type_path: vec![],
                repeated: false,
                type_path: vec![],
            });
        }
        if let Some(current) = self.current.as_mut() {
            current.add_type_path(type_str);
        } else {
            stop!("Cannot set path of enum item, because enum item wasn't opened");
        }
    }

    pub fn get_current_option(&self) -> Option<&EnumItem> {
        self.current.as_ref()
    }

    pub fn accept_type(&mut self, store: &Store, own_group_id: usize) {
        if let Some(current) = self.current.as_mut() {
            current.accept_type(store, own_group_id);
        } else {
            stop!("Attempt to accept type of enum item as soon as it isn't created");
        }
    }
}
