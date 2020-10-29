use super::{ PrimitiveField, Enum, Struct, PrimitiveTypes };

#[derive(Debug)]
pub struct Store {
    sequence: usize,
    structs: Vec<Struct>,
    enums: Vec<Enum>,
    c_struct: Option<Struct>,
    c_enum: Option<Enum>,
    c_field: Option<PrimitiveField>,
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
        if let Some(c_struct) = self.c_struct.take() {
            parent = c_struct.id;
        }
        self.sequence += 1;
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
            self.c_field = Some(PrimitiveField::new(self.sequence, 0, type_str.to_string()));
        } else {
            panic!("Expecting type definition but has been gotten value {}", type_str)
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
        println!("close");
    }

}