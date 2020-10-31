use super::{ Field, Enum, Struct, PrimitiveTypes, EReferenceToType };

#[derive(Debug)]
pub struct Store {
    sequence: usize,
    structs: Vec<Struct>,
    enums: Vec<Enum>,
    c_struct: Option<Struct>,
    c_enum: Option<Enum>,
    c_field: Option<Field>,
    path: Vec<usize>,
}

impl Store {

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Store {
            sequence: 0,
            structs: vec![],
            enums: vec![],
            c_struct: None,
            c_enum: None,
            c_field: None,
            path: vec![],
        }
    }

    pub fn open_struct(&mut self, name: String) {
        let mut parent: usize = 0;
        self.sequence += 1;
        if let Some(mut c_struct) = self.c_struct.take() {
            parent = c_struct.id;
            c_struct.bind_struct(self.sequence);
            self.structs.push(c_struct);
        }
        self.c_struct = Some(Struct::new(self.sequence, parent, name));
        self.path.push(self.sequence);
    }

    pub fn open_enum(&mut self, name: String) {
        let mut parent: usize = 0;
        if let Some(c_struct) = self.c_struct.take() {
            parent = c_struct.id;
            self.c_struct = Some(c_struct);
        }
        if self.c_enum.is_some() {
            panic!("Nested enums arn't supported");
        }
        self.sequence += 1;
        self.c_enum = Some(Enum::new(self.sequence, parent, name));
    }

    pub fn set_field_type(&mut self, type_str: &str) {
        if self.c_field.is_some() {
            panic!("Fail to create new field, while previous isn't closed.");
        }
        if self.c_struct.is_none() {
            panic!("Fail to create new field, because no open struct.");
        }
        if PrimitiveTypes::get_entity(type_str).is_some() {
            self.sequence += 1;
            self.c_field = Some(Field::new(self.sequence, 0, type_str.to_string()));
        } else {
            let mut c_field = Field::new(self.sequence, 0, type_str.to_string());
            if let Some(enum_ref) = self.enums.iter().find(|i| i.name == type_str) {
                c_field.set_type_ref(EReferenceToType::Enum, enum_ref.id);
            } else if let Some(struct_ref) = self.structs.iter().find(|i| i.name == type_str) {
                c_field.set_type_ref(EReferenceToType::Struct, struct_ref.id);
            } else {
                panic!("Expecting type definition but has been gotten value {}", type_str)
            }
            self.c_field = Some(c_field);
        }
    }

    pub fn set_field_type_as_repeated(&mut self) {
        if let Some(mut c_field) = self.c_field.take() {
            c_field.set_as_repeated();
            self.c_field = Some(c_field);
        } else {
            panic!("Fail to close field, while it wasn't opened.");
        }
    }

    pub fn set_field_name(&mut self, name_str: &str) {
        if let Some(mut c_struct) = self.c_struct.take() {
            if let Some(mut c_field) = self.c_field.take() {
                c_field.set_name(name_str.to_string());
                c_struct.add_field(c_field);
                self.c_struct = Some(c_struct);
                self.c_field = None;
            } else {
                panic!("Fail to close field, while it wasn't opened.");
            }
        } else {
            panic!("Fail to close new field, because no open struct.");
        }
    }

    pub fn set_enum_value(&mut self, val: &str) {
        if let Some(mut c_enum) = self.c_enum.take() {
            c_enum.add(val.to_string());
            self.c_enum = Some(c_enum);
        } else {
            panic!("Fail to add enum value because enum isn't opened");
        }
    }

    pub fn is_enum_opened(&mut self) -> bool {
        self.c_enum.is_some()
    }

    pub fn open(&mut self) {
        if self.c_struct.is_none() && self.c_enum.is_none() {
            panic!("No created struct or enum");
        }
        println!("open");
    }

    pub fn close(&mut self) {
        if self.c_struct.is_none() && self.c_enum.is_none() {
            panic!("No opened struct or enum");
        }
        if let Some(c_enum) = self.c_enum.take() {
            self.enums.push(c_enum);
            self.c_enum = None;
        } else if let Some(c_struct) = self.c_struct.take() {
            self.structs.push(c_struct);
            self.path.remove(self.path.len() - 1);
            if self.path.is_empty() {
                self.c_struct = None;
            } else if let Some(pos) = self.structs.iter().position(|s| s.id == self.path[self.path.len() - 1]) {
                self.c_struct = Some(self.structs.remove(pos));
            } else {
                panic!("Cannot find struct from path");
            }
        }
    }

}