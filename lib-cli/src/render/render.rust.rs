use super::parser::enums::Enum;
use super::parser::fields::Field;
use super::parser::groups::Group;
use super::parser::store::Store;
use super::parser::structs::Struct;
use super::Render;

pub struct RustRender {}

impl RustRender {
    fn optional(&self, ref_type: String, opt: bool) -> String {
        if opt {
            format!("Option<{}>", ref_type)
        } else {
            ref_type
        }
    }

    fn struct_default(&self, entity_id: usize, store: &mut Store, level: u8) -> String {
        if let Some(strct) = store.get_struct(entity_id) {
            let mut body = format!("{} {{\n", strct.name);
            for field in &strct.fields {
                body = format!(
                    "{}{}{}\n",
                    body,
                    self.spaces(level),
                    self.field_default(field, &mut store.clone(), level)
                );
            }
            format!("{}{}}}\n", body, self.spaces(level - 1))
        } else if let Some(enums) = store.get_enum(entity_id) {
            format!("{}::Defaults", enums.name)
        } else {
            panic!(format!("Fail to find a struct/enum id: {}", entity_id));
        }
    }

    fn field_declare(&self, field: &Field) -> String {
        let mut body = format!("pub {}:", field.name);
        if field.repeated {
            body = format!(
                "{} Vec<{}>;",
                body,
                self.optional(field.kind.clone(), field.optional)
            );
        } else {
            body = format!(
                "{} {};",
                body,
                self.optional(field.kind.clone(), field.optional)
            );
        }
        body
    }

    fn field_default(&self, field: &Field, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}: ", field.name);
        if field.repeated {
            body = format!("{}[],", body);
        } else if field.optional {
            body = format!("{}None,", body);
        } else if let Some(default_value) = self.type_default_value(&field.kind) {
            body = format!("{}{},", body, default_value);
        } else if let Some(struct_id) = field.ref_type_id {
            body = format!(
                "{}{},",
                body,
                self.struct_default(struct_id, store, level + 1)
            );
        }
        body
    }

    fn type_default_value(&self, type_ref: &str) -> Option<&str> {
        match type_ref {
            "bool" => Some("true"),
            "i8" => Some("0"),
            "i16" => Some("0"),
            "i32" => Some("0"),
            "i64" => Some("0"),
            "u8" => Some("0"),
            "u16" => Some("0"),
            "u32" => Some("0"),
            "u64" => Some("0"),
            "f32" => Some("0"),
            "f64" => Some("0"),
            "str" => Some("String::from(\"\")"),
            _ => None,
        }
    }

    fn get_decode_type_ref(&self, field: &Field, store: &mut Store) -> String {
        let mut type_str = match field.kind.clone().as_str() {
            "bool" => String::from("bool"),
            "i8" => String::from("i8"),
            "i16" => String::from("i16"),
            "i32" => String::from("i32"),
            "i64" => String::from("i64"),
            "u8" => String::from("u8"),
            "u16" => String::from("u16"),
            "u32" => String::from("u32"),
            "u64" => String::from("u64"),
            "f32" => String::from("f32"),
            "f64" => String::from("f64"),
            "str" => String::from("String"),
            _ => {
                if let Some(ref_type_id) = field.ref_type_id {
                    if let Some(strct) = store.get_struct(ref_type_id) {
                        strct.name
                    } else if let Some(enums) = store.get_enum(ref_type_id) {
                        enums.name
                    } else {
                        panic!(format!(
                            "Fail to find a struct/enum id: {} for field {}",
                            ref_type_id, field.name
                        ));
                    }
                } else {
                    panic!("Invalid type definition for field {}", field.name);
                }
            }
        };
        if field.optional {
            type_str = format!("Option::<{}>", type_str);
        }
        if field.repeated {
            type_str = format!("Vec::<{}>", type_str);
        }
        type_str
    }
}

impl Render for RustRender {
    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}pub mod {} {{\n", self.spaces(level), group.name);
        body = format!("{}{}use super::*;\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}use encode::{{ StructEncode, EnumEncode, Encode }};\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}use decode::{{ StructDecode, EnumDecode, Decode }};\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}use storage::{{ Storage }};\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}use std::io::Cursor;\n", body, self.spaces(level + 1));
        body = format!("{}{}use bytes::{{ Buf }};\n", body, self.spaces(level + 1));
        for struct_id in &group.structs {
            if let Some(strct) = store.get_struct(*struct_id) {
                body = format!(
                    "{}\n{}",
                    body,
                    self.structs(&strct, &mut store.clone(), level + 1)
                );
            }
        }
        let childs = store.get_child_groups(group.id);
        for group in childs {
            body = format!(
                "{}\n{}",
                body,
                self.groups(&group, &mut store.clone(), level + 1)
            );
        }
        format!("{}\n{}}}\n", body, self.spaces(level))
    }

    fn structs(&self, strct: &Struct, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}#[derive(Debug, Clone, PartialEq)]\n", self.spaces(level));
        body = format!("{}{}pub struct {} {{", body, self.spaces(level), strct.name);
        for field in &strct.fields {
            body = format!(
                "{}\n{}{}",
                body,
                self.spaces(level + 1),
                self.field_declare(field)
            );
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}impl StructDecode for {} {{\n",
            body,
            self.spaces(level),
            strct.name
        );
        body = format!("{}{}fn get_id() -> u32 {{\n", body, self.spaces(level + 1));
        body = format!("{}{}{}\n", body, self.spaces(level + 2), strct.id);
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}fn defaults() -> {} {{\n",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!("{}{}{} {{\n", body, self.spaces(level + 2), strct.name);
        for field in &strct.fields {
            body = format!(
                "{}{}{}\n",
                body,
                self.spaces(level + 3),
                self.field_default(field, &mut store.clone(), level + 3)
            );
        }
        body = format!("{}{}}}\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}fn extract(&mut self, mut storage: Storage) -> Result<(), String> {{\n",
            body,
            self.spaces(level + 1)
        );
        for field in &strct.fields {
            body = format!(
                "{}{}self.{} = match {}::decode(&mut storage, {})\n",
                body,
                self.spaces(level + 2),
                field.name,
                self.get_decode_type_ref(&field, store),
                field.id
            );
            body = format!("{}{}Ok(val) => val,\n", body, self.spaces(level + 3));
            body = format!(
                "{}{}Err(e) => {{ return Err(e) }},\n",
                body,
                self.spaces(level + 3)
            );
            body = format!("{}{}}};\n", body, self.spaces(level + 2));
        }
        body = format!("{}{}Ok(())\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}impl StructEncode for {} {{\n",
            body,
            self.spaces(level),
            strct.name
        );
        body = format!("{}{}fn get_id() -> u32 {{\n", body, self.spaces(level + 1));
        body = format!("{}{}{}\n", body, self.spaces(level + 2), strct.id);
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}fn abduct(&mut self) -> Result<Vec<u8>, String> {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}let mut buffer: Vec<u8> = vec!();)\n",
            body,
            self.spaces(level + 2)
        );
        for field in &strct.fields {
            body = format!(
                "{}{}match self.{}.encode({}) {{\n",
                body,
                self.spaces(level + 2),
                field.name,
                field.id
            );
            body = format!(
                "{}{}Ok(mut buf) => {{ buffer.append(&mut buf); }}\n",
                body,
                self.spaces(level + 3)
            );
            body = format!(
                "{}{}Err(e) => {{ return Err(e) }},\n",
                body,
                self.spaces(level + 3)
            );
            body = format!("{}{}}};\n", body, self.spaces(level + 2));
        }
        body = format!("{}{}Ok(buffer)\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body
    }
}

/*
pub fn get_str(store: Store) -> String {
    let mut body = String::new();
    for strct in &store.structs {
        if strct.parent == 0 {
            body = format!("{}{}\n", body, render::structs(strct, 0)).to_string();
        }
    }
    for group in &store.groups {
        if group.parent == 0 {
            body = format!("{}{}\n", body, render::groups(group, &mut store.clone(), 0)).to_string();
        }
    }
    body
}

mod render {
    use super::{ Store, Struct, Field, Group };

    pub fn groups(group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}mod {} {{", spaces(level), group.name);
        body = format!("{}\n{}// id={}", body, spaces(level + 1), group.id);
        body = format!("{}\n{}// parent={}", body, spaces(level + 1), group.parent);
        for struct_id in &group.structs {
            if let Some(strct) = store.get_struct(*struct_id) {
                body = format!("{}\n{}", body, structs(&strct, level + 1));
            }
        }
        let childs = store.get_child_groups(group.id);
        for group in childs {
            body = format!("{}\n{}", body, groups(&group, &mut store.clone(), level + 1));
        }
        format!("{}\n{}}}\n", body, spaces(level))
    }
    pub fn structs(strct: &Struct, level: u8) -> String {
        let mut body = format!("{}struct {} {{", spaces(level), strct.name);
        body = format!("{}\n{}// id={}", body, spaces(level + 1), strct.id);
        body = format!("{}\n{}// parent={}", body, spaces(level + 1), strct.parent);
        for field in &strct.fields {
            body = format!("{}\n{}{}", body, spaces(level + 1), fields(field));
        }
        format!("{}\n{}}}\n", body, spaces(level))
    }

    pub fn fields(field: &Field) -> String {
        let mut body = format!("pub {}:", field.name);
        if field.repeated {
            body = format!("{} Vec<{}>;", body, optional(field.kind.clone(), field.optional));
        } else {
            body = format!("{} {};", body, optional(field.kind.clone(), field.optional));
        }
        body
    }

    fn spaces(level: u8) -> String {
        "    ".repeat(level as usize)
    }

    fn optional(ref_type: String, opt: bool) -> String {
        if opt {
            format!("Option<{}>", ref_type)
        } else {
            ref_type
        }
    }

}
*/
