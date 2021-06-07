use super::{ PrimitiveTypes, stop, Store };

#[derive(Debug, Clone)]
pub enum EReferenceToType {
    Struct,
    Enum,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub kind: String,
    pub ref_type: Option<EReferenceToType>,
    pub ref_type_id: Option<usize>,
    pub ref_type_path: Vec<usize>,
    pub repeated: bool,
    pub optional: bool,
    type_path: Vec<String>,
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
            ref_type_path: vec![],
            repeated: false,
            optional: false,
            type_path: vec![],
        }
    }

    pub fn create_not_assigned_primitive(name: String, kind: PrimitiveTypes::ETypes, optional: bool) -> Self {
        let kind = if let Some(primitive) = PrimitiveTypes::get_entity_as_string(kind) {
            primitive
        } else {
            stop!("Unknown type");
        };
        Field {
            id: 0,
            parent: 0,
            name,
            kind: kind.clone(),
            ref_type: None,
            ref_type_id: None,
            ref_type_path: vec![],
            repeated: false,
            optional,
            type_path: vec![kind],
        }
    }


    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_type(&mut self, kind: PrimitiveTypes::ETypes) {
        if let Some(primitive) = PrimitiveTypes::get_entity_as_string(kind) {
            self.kind = primitive;
        } else {
            stop!("Unknown type");
        }
    }

    pub fn add_type_path(&mut self, type_str: &str) {
        self.type_path.push(type_str.to_owned());
    }

    pub fn add_ref_type_path(&mut self, ref_type_id: usize) {
        self.ref_type_path.push(ref_type_id);
    }

    pub fn set_as_repeated(&mut self) {
        self.repeated = true;
    }

    pub fn set_as_optional(&mut self) {
        self.optional = true;
    }

    pub fn get_full_name(&self) -> Vec<String> {
        self.type_path.clone()
    }

    pub fn get_path(&self) -> Vec<String> {
        (self.type_path[0..self.type_path.len() - 1]).to_vec()
    }

    pub fn accept_type(&mut self, store: &Store, own_group_id: usize) {
        if self.type_path.is_empty() {
            stop!("Fail to accept field type because no any type references were provided");
        }
        let first = self.type_path[0].clone();
        if self.type_path.len() == 1 && PrimitiveTypes::get_entity(&first).is_some() {
            self.kind = first;
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
            let (type_name, type_id) = path[path.len() - 1].clone();
            self.ref_type_id = Some(type_id);
            self.kind = type_name;
            self.ref_type_path = path[0..path.len() - 1].iter().map(|(_name, id)| id.clone() ).collect();
        }
    }

}