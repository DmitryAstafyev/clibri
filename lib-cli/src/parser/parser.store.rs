use super::{ Field, Enum, Struct, Group, PrimitiveTypes, EReferenceToType, stop };

#[derive(Debug, Clone)]
pub struct Store {
    sequence: usize,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub groups: Vec<Group>,
    c_struct: Option<Struct>,
    c_group: Option<Group>,
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
            groups: vec![],
            c_struct: None,
            c_enum: None,
            c_field: None,
            c_group: None,
            path: vec![],
        }
    }

    pub fn get_struct(&mut self, id: usize) -> Option<Struct> {
        if let Some(strct) = self.structs.iter().find(|s| s.id == id) {
            Some(strct.clone())
        } else {
            None
        }
    }

    pub fn get_enum(&mut self, id: usize) -> Option<Enum> {
        if let Some(enums) = self.enums.iter().find(|s| s.id == id) {
            Some(enums.clone())
        } else {
            None
        }
    }

    pub fn get_group(&mut self, id: usize) -> Option<Group> {
        if let Some(group) = self.groups.iter().find(|s| s.id == id) {
            Some(group.clone())
        } else {
            None
        }
    }

    pub fn get_child_groups(&mut self, parent_id: usize) -> Vec<Group> {
        let mut groups = vec!();
        for group in self.groups.iter() {
            if group.parent == parent_id {
                groups.push(group.clone());
            }
        }
        groups
    }

    pub fn open_struct(&mut self, name: String) {
        if self.c_struct.is_some() {
            stop!("Struct cannot be defined inside struct");
        }
        if self.c_enum.is_some() {
            stop!("Struct cannot be defined inside enum");
        }
        self.sequence += 1;
        self.bind_struct_with_group(self.sequence);
        self.c_struct = Some(Struct::new(self.sequence, self.get_group_id(), name));
    }

    pub fn open_enum(&mut self, name: String) {
        if self.c_struct.is_some() {
            stop!("Enum cannot be defined inside struct");
        }
        if self.c_enum.is_some() {
            stop!("Enum cannot be defined inside enum");
        }
        self.sequence += 1;
        self.bind_enum_with_group(self.sequence);
        self.c_enum = Some(Enum::new(self.sequence, self.get_group_id(), name));
    }

    pub fn open_group(&mut self, name: String) {
        if self.c_struct.is_some() {
            stop!("Group cannot be defined inside struct");
        }
        if self.c_enum.is_some() {
            stop!("Group cannot be defined inside enum");
        }
        let parent: usize = self.get_group_id();
        self.sequence += 1;
        self.bind_group_with_group(self.sequence);
        self.c_group = Some(Group::new(self.sequence, parent, name));
        self.path.push(self.sequence);
    }

    pub fn set_field_type(&mut self, type_str: &str) {
        if self.c_field.is_some() {
            stop!("Fail to create new field, while previous isn't closed.");
        }
        if self.c_struct.is_none() {
            stop!("Fail to create new field, because no open struct.");
        }
        self.sequence += 1;
        if PrimitiveTypes::get_entity(type_str).is_some() {
            self.c_field = Some(Field::new(self.sequence, 0, type_str.to_string()));
        } else {
            let mut c_field = Field::new(self.sequence, 0, type_str.to_string());
            let group_id = if let Some(group) = self.c_group.clone() { group.id } else { 0 };
            if let Some(enum_ref) = self.enums.iter().find(|i| i.name == type_str && i.parent == group_id) {
                c_field.set_type_ref(EReferenceToType::Enum, enum_ref.id);
            } else if let Some(struct_ref) = self.structs.iter().find(|i| i.name == type_str && i.parent == group_id) {
                c_field.set_type_ref(EReferenceToType::Struct, struct_ref.id);
            } else {
                stop!("Expecting type definition but has been gotten value {}", type_str)
            }
            self.c_field = Some(c_field);
        }
    }

    pub fn set_field_type_as_repeated(&mut self) {
        if let Some(mut c_enum) = self.c_enum.take() {
            c_enum.set_as_repeated();
            self.c_enum = Some(c_enum);
        } else if let Some(mut c_field) = self.c_field.take() {
            c_field.set_as_repeated();
            self.c_field = Some(c_field);
        } else {
            stop!("Fail to set field as repeated, because it wasn't opened.");
        }
    }

    pub fn set_field_type_as_optional(&mut self) {
        if let Some(mut c_field) = self.c_field.take() {
            c_field.set_as_optional();
            self.c_field = Some(c_field);
        } else {
            stop!("Fail to set field as optional, because it wasn't opened.");
        }
    }

    pub fn set_field_name(&mut self, name_str: &str) {
        if self.c_struct.is_none() {
            stop!("Fail to set name of field, because no open struct.");
        }
        if let Some(mut c_field) = self.c_field.take() {
            c_field.set_name(name_str.to_string());
            self.c_field = Some(c_field);
        } else {
            stop!("Fail to set name of field, while it wasn't opened.");
        }
    }

    pub fn close_field(&mut self) {
        if let Some(mut c_struct) = self.c_struct.take() {
            if let Some(c_field) = self.c_field.take() {
                c_struct.add_field(c_field);
                self.c_struct = Some(c_struct);
                self.c_field = None;
            } else {
                stop!("Fail to close field, while it wasn't opened.");
            }
        } else {
            stop!("Fail to close new field, because no open struct.");
        }
    }

    pub fn set_enum_type(&mut self, type_str: &str) {
        if let Some(mut c_enum) = self.c_enum.take() {
            let group_id = if let Some(group) = self.c_group.clone() { group.id } else { 0 };
            if let Some(types) = PrimitiveTypes::get_entity(type_str) {
                c_enum.set_type(types);
            } else if let Some(struct_ref) = self.structs.iter().find(|i| i.name == type_str && i.parent == group_id) {
                c_enum.set_type_ref(struct_ref.id);
            } else {
                stop!("Expecting type definition but has been gotten value {}", type_str)
            }
            self.c_enum = Some(c_enum);
        } else {
            stop!("Fail to create new enum item, because no open enum.");
        }
    }

    pub fn set_simple_enum_item(&mut self, word: &str) {
        if let Some(mut c_enum) = self.c_enum.take() {
            c_enum.set_simple(word);
            self.c_enum = Some(c_enum);
        } else {
            stop!("Fail to create new enum item, because no open enum.");
        }
    }

    pub fn set_enum_name(&mut self, name: &str) {
        if let Some(mut c_enum) = self.c_enum.take() {
            c_enum.set_name(name.to_string());
            self.c_enum = Some(c_enum);
        } else {
            stop!("Fail to set enum item name, because no open enum.");
        }
    }

    pub fn is_enum_opened(&mut self) -> bool {
        self.c_enum.is_some()
    }

    pub fn open(&mut self) {
        if self.c_group.is_none() && self.c_struct.is_none() && self.c_enum.is_none() {
            stop!("No created struct or enum");
        }
    }

    pub fn close(&mut self) {
        if self.c_group.is_none() && self.c_struct.is_none() && self.c_enum.is_none() {
            stop!("No opened group or struct or enum");
        }
        if let Some(c_enum) = self.c_enum.take() {
            self.enums.push(c_enum);
            self.c_enum = None;
        } else if let Some(c_struct) = self.c_struct.take() {
            self.structs.push(c_struct);
            self.c_struct = None;
        } else if let Some(c_group) = self.c_group.take() {
            self.groups.push(c_group);
            self.path.remove(self.path.len() - 1);
            if self.path.is_empty() {
                self.c_group = None;
            } else if let Some(pos) = self.groups.iter().position(|s| s.id == self.path[self.path.len() - 1]) {
                self.c_group = Some(self.groups.remove(pos));
            } else {
                stop!("Cannot find group from path");
            }
        }
    }

    pub fn order(&mut self) -> Result<(), String> {
        let mut parents: Vec<usize> = vec!();
        for strct in &self.structs {
            if strct.parent != 0 && parents.iter().find(|id| id == &&strct.parent).is_none() {
                parents.push(strct.parent);
            }
        }
        Ok(())
    }

    fn get_group_id(&mut self) -> usize {
        if let Some(c_group) = self.c_group.clone() {
            c_group.id
        } else {
            0
        }
    }

    fn bind_struct_with_group(&mut self, id: usize) {
        if let Some(mut c_group) = self.c_group.take() {
            c_group.bind_struct(id);
            self.c_group = Some(c_group);
        }
    }

    fn bind_enum_with_group(&mut self, id: usize) {
        if let Some(mut c_group) = self.c_group.take() {
            c_group.bind_enum(id);
            self.c_group = Some(c_group);
        }
    }

    fn bind_group_with_group(&mut self, id: usize) {
        if let Some(mut c_group) = self.c_group.take() {
            c_group.bind_group(id);
            self.c_group = Some(c_group.clone());
            self.groups.push(c_group);
        }
    }

}