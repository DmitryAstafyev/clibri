use super::{stop, Enum, Field, Group, Struct};

pub const INTERNAL_SERVICE_GROUP: &str = "InternalServiceGroup";

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

    pub fn add_service_struct(&mut self, name: String, mut fields: Vec<Field>) {
        if (self
            .groups
            .iter()
            .find(|i| i.name == INTERNAL_SERVICE_GROUP && i.parent == 0))
        .is_none()
        {
            self.sequence += 1;
            self.groups.push(Group::new(
                self.sequence,
                0,
                String::from(INTERNAL_SERVICE_GROUP),
            ));
        }
        if let Some(service_group) = self
            .groups
            .iter_mut()
            .find(|i| i.name == INTERNAL_SERVICE_GROUP && i.parent == 0)
        {
            self.sequence += 1;
            let mut strct: Struct = Struct::new(self.sequence, service_group.id, name);
            for field in fields.iter_mut() {
                self.sequence += 1;
                field.id = self.sequence;
            }
            strct.set_fields(fields);
            service_group.bind_struct(strct.id);
            self.structs.push(strct);
        }
    }

    pub fn get_struct(&self, id: usize) -> Option<Struct> {
        self.structs.iter().find(|s| s.id == id).cloned()
    }

    pub fn get_enum(&self, id: usize) -> Option<Enum> {
        self.enums.iter().find(|s| s.id == id).cloned()
    }

    pub fn get_group(&self, id: usize) -> Option<Group> {
        self.groups.iter().find(|s| s.id == id).cloned()
    }

    pub fn get_child_groups(&mut self, parent_id: usize) -> Vec<Group> {
        let mut groups = vec![];
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
        if self.c_struct.is_none() {
            stop!("Fail to create new field, because no open struct.");
        }
        let mut c_field = if let Some(field) = self.c_field.take() {
            field
        } else {
            self.sequence += 1;
            Field::new(self.sequence, 0, type_str.to_string())
        };
        c_field.add_type_path(type_str);
        self.c_field = Some(c_field);
    }

    pub fn find_by_str_path(&self, from: usize, path: &str) -> Option<Vec<(String, usize)>> {
        let parts: Vec<String> = path
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| String::from(*v))
            .collect();
        self.find_by_path(from, &parts)
    }

    pub fn find_by_path(&self, from: usize, path: &[String]) -> Option<Vec<(String, usize)>> {
        let mut results: Vec<(String, usize)> = vec![];
        let last = path.len() - 1;
        let mut parent: usize = from;
        for (pos, type_str) in path.iter().enumerate() {
            if pos == last {
                if let Some(enum_ref) = self
                    .enums
                    .iter()
                    .find(|i| i.name == *type_str && i.parent == parent)
                {
                    // Check enum in own group
                    results.push((String::from(type_str), enum_ref.id));
                } else if let Some(struct_ref) = self
                    .structs
                    .iter()
                    .find(|i| i.name == *type_str && i.parent == parent)
                {
                    // Check struct in own group
                    results.push((String::from(type_str), struct_ref.id));
                } else {
                    return None;
                }
            } else if let Some(group_ref) = self
                .groups
                .iter()
                .find(|i| i.name == *type_str && i.parent == parent)
            {
                results.push((String::from(type_str), group_ref.id));
                parent = group_ref.id;
            } else {
                return None;
            }
        }
        Some(results)
    }

    pub fn get_struct_by_str_path(&self, from: usize, path: &str) -> Option<&Struct> {
        let path: Vec<String> = path
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| String::from(*v))
            .collect();
        let last = path.len() - 1;
        let mut parent: usize = from;
        for (pos, type_str) in path.iter().enumerate() {
            if pos == last {
                if let Some(struct_ref) = self
                    .structs
                    .iter()
                    .find(|i| i.name == *type_str && i.parent == parent)
                {
                    return Some(struct_ref);
                } else {
                    return None;
                }
            } else if let Some(group_ref) = self
                .groups
                .iter()
                .find(|i| i.name == *type_str && i.parent == parent)
            {
                parent = group_ref.id;
            } else {
                return None;
            }
        }
        None
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
            c_field.accept_type(
                &self,
                if let Some(group) = self.c_group.clone() {
                    group.id
                } else {
                    0
                },
            );
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
            c_enum.add_type_path(type_str);
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
            c_enum.accept_type(
                &self,
                if let Some(group) = self.c_group.clone() {
                    group.id
                } else {
                    0
                },
            );
            c_enum.set_name(name.to_string());
            self.c_enum = Some(c_enum);
        } else {
            stop!("Fail to set enum item name, because no open enum.");
        }
    }

    pub fn is_enum_opened(&mut self) -> bool {
        self.c_enum.is_some()
    }

    pub fn is_field_opened(&mut self) -> bool {
        self.c_field.is_some()
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
            } else if let Some(pos) = self
                .groups
                .iter()
                .position(|s| s.id == self.path[self.path.len() - 1])
            {
                self.c_group = Some(self.groups.remove(pos));
            } else {
                stop!("Cannot find group from path");
            }
        }
    }

    pub fn order(&mut self) -> Result<(), String> {
        let mut parents: Vec<usize> = vec![];
        for strct in &self.structs {
            if strct.parent != 0 && parents.iter().find(|id| id == &&strct.parent).is_none() {
                parents.push(strct.parent);
            }
        }
        Ok(())
    }

    pub fn get_struct_path(&self, id: usize) -> Vec<String> {
        if let Some(strct) = self.structs.iter().find(|s| s.id == id) {
            let mut path: Vec<String> = vec![strct.name.clone()];
            let mut parent = strct.parent;
            loop {
                if parent == 0 {
                    break;
                }
                if let Some(group) = self.groups.iter().find(|s| s.id == parent) {
                    path.push(group.name.clone());
                    parent = group.parent;
                } else {
                    stop!(
                        "Fail to find a group id: {} for struct {}",
                        strct.parent,
                        strct.name
                    );
                }
            }
            path.reverse();
            path
        } else {
            stop!("Fail to find a struct {}", id);
        }
    }

    pub fn get_enum_path(&self, id: usize) -> Vec<String> {
        if let Some(strct) = self.enums.iter().find(|s| s.id == id) {
            let mut path: Vec<String> = vec![strct.name.clone()];
            let mut parent = strct.parent;
            loop {
                if parent == 0 {
                    break;
                }
                if let Some(group) = self.groups.iter().find(|s| s.id == parent) {
                    path.push(group.name.clone());
                    parent = group.parent;
                } else {
                    stop!(
                        "Fail to find a group id: {} for struct {}",
                        strct.parent,
                        strct.name
                    );
                }
            }
            path.reverse();
            path
        } else {
            stop!("Fail to find a struct {}", id);
        }
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
