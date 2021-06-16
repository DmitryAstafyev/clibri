use super::protocol::enums::{Enum, EnumItem};
use super::protocol::fields::Field;
use super::protocol::groups::Group;
use super::protocol::store::Store;
use super::protocol::structs::Struct;
use super::protocol::types::PrimitiveTypes;
use super::Render;
use super::helpers;
use regex::Regex;
use std::{
    include_str,
    path::{
        Path,
    }
};
pub struct RustRender {
    embedded: bool,
    signature: u16,
}

impl RustRender {
    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}pub mod {} {{\n", self.spaces(level), group.name);
        body = format!("{}{}use super::*;\n", body, self.spaces(level + 1));
        body = format!("{}{}use std::io::Cursor;\n", body, self.spaces(level + 1));
        body = format!("{}{}use bytes::{{ Buf }};\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}",
            body,
            self.get_messages_list(Some(group), &mut store.clone(), level + 1)
        );
        for enum_id in &group.enums {
            if let Some(enums) = store.get_enum(*enum_id) {
                body = format!(
                    "{}\n{}",
                    body,
                    self.enums(&enums, level + 1)
                );
            }
        }
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
                format!(
                    "pub {}: {},",
                    field.name,
                    self.get_declare_type_ref(field)
                ),
            );
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body = format!("{}{}#[allow(unused_variables)]\n", body, self.spaces(level));
        body = format!("{}{}#[allow(unused_mut)]\n", body, self.spaces(level));
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
            "{}{}fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {{\n",
            body,
            self.spaces(level + 1)
        );
        for field in &strct.fields {
            if field.optional {
                if let Some(id) = field.ref_type_id {
                    if let Some(enums) = store.get_enum(id) {
                        body = format!(
                            "{}{}if let Some(buf) = storage.get({}) {{\n",
                            body,
                            self.spaces(level + 2),
                            field.id
                        );
                        body = format!("{}{}if buf.is_empty() {{\n", body, self.spaces(level + 3));
                        body = format!(
                            "{}{}self.{} = None;\n",
                            body,
                            self.spaces(level + 4),
                            field.name
                        );
                        body = format!("{}{}}} else {{\n", body, self.spaces(level + 3));
                        body = format!("{}{}self.{} = match {}::get_from_storage(Source::Storage(&mut storage), Some({})) {{\n", body, self.spaces(level + 4), field.name, enums.name, field.id);
                        body = format!("{}{}Ok(val) => Some(val),\n", body, self.spaces(level + 5));
                        body = format!(
                            "{}{}Err(e) => {{ return Err(e) }},\n",
                            body,
                            self.spaces(level + 5)
                        );
                        body = format!("{}{}}};\n", body, self.spaces(level + 4));
                        body = format!("{}{}}}\n", body, self.spaces(level + 3));
                        body = format!("{}{}}} else {{\n", body, self.spaces(level + 2));
                        body = format!(
                            "{}{}return Err(\"Buffer for property {} isn't found\".to_string());\n",
                            body,
                            self.spaces(level + 3),
                            field.name
                        );
                        body = format!("{}{}}}\n", body, self.spaces(level + 2));
                        continue;
                    }
                }
            }
            body = format!(
                "{}{}self.{} = match {}::get_from_storage(Source::Storage(&mut storage), Some({})) {{\n",
                body,
                self.spaces(level + 2),
                field.name,
                self.get_decode_type_ref(&field),
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
        body = format!("{}{}#[allow(unused_variables)]\n", body, self.spaces(level));
        body = format!("{}{}#[allow(unused_mut)]\n", body, self.spaces(level));
        body = format!(
            "{}{}impl StructEncode for {} {{\n",
            body,
            self.spaces(level),
            strct.name
        );
        body = format!(
            "{}{}fn get_id(&self) -> u32 {{ {} }}\n",
            body,
            self.spaces(level + 1),
            strct.id
        );
        body = format!(
            "{}{}fn get_signature(&self) -> u16 {{ {} }}\n",
            body,
            self.spaces(level + 1),
            self.signature,
        );
        body = format!(
            "{}{}fn abduct(&mut self) -> Result<Vec<u8>, String> {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}let mut buffer: Vec<u8> = vec!();\n",
            body,
            self.spaces(level + 2)
        );
        for field in &strct.fields {
            if field.optional {
                if let Some(id) = field.ref_type_id {
                    if store.get_enum(id).is_some() {
                        body = format!(
                            "{}{}if let Some(mut val) = self.{}.clone() {{\n",
                            body,
                            self.spaces(level + 2),
                            field.name
                        );
                        body = format!(
                            "{}{}match val.get_buf_to_store(Some({})) {{\n",
                            body,
                            self.spaces(level + 3),
                            field.id
                        );
                        body = format!(
                            "{}{}Ok(mut buf) => {{ buffer.append(&mut buf); }},\n",
                            body,
                            self.spaces(level + 4)
                        );
                        body = format!(
                            "{}{}Err(e) => {{ return  Err(e); }},\n",
                            body,
                            self.spaces(level + 4)
                        );
                        body = format!("{}{}}};\n", body, self.spaces(level + 3));
                        body = format!("{}{}}} else {{\n", body, self.spaces(level + 2));
                        body = format!(
                            "{}{}match get_empty_buffer_val(Some({})) {{\n",
                            body,
                            self.spaces(level + 3),
                            field.id
                        );
                        body = format!(
                            "{}{}Ok(mut buf) => {{ buffer.append(&mut buf); }},\n",
                            body,
                            self.spaces(level + 4)
                        );
                        body = format!(
                            "{}{}Err(e) => {{ return  Err(e); }},\n",
                            body,
                            self.spaces(level + 4)
                        );
                        body = format!("{}{}}};\n", body, self.spaces(level + 3));
                        body = format!("{}{}}}\n", body, self.spaces(level + 2));
                        continue;
                    }
                }
            }
            body = format!(
                "{}{}match self.{}.get_buf_to_store(Some({})) {{\n",
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
        body = format!(
            "{}{}impl PackingStruct for {} {{ }}\n",
            body,
            self.spaces(level),
            strct.name
        );
        body
    }

    fn enums(&self, enums: &Enum, level: u8) -> String {
        let mut body = format!("{}#[derive(Debug, Clone, PartialEq)]\n", self.spaces(level));
        body = format!("{}{}pub enum {} {{\n", body, self.spaces(level), enums.name);
        for item in &enums.variants {
            let item_type = self.enum_item_type(item.clone());
            body = format!(
                "{}{}{},\n",
                body,
                self.spaces(level + 1),
                format!(
                    "{}({})",
                    item.name,
                    if item.repeated {
                        format!("Vec<{}>", item_type)
                    } else {
                        item_type
                    }
                ),
            );
        }
        body = format!("{}{}Defaults,\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}impl EnumDecode for {} {{\n",
            body,
            self.spaces(level),
            enums.name
        );
        body = format!(
            "{}{}fn get_id(&self) -> u32 {{ {} }}\n",
            body,
            self.spaces(level + 1),
            enums.id
        );
        body = format!(
            "{}{}fn extract(buf: Vec<u8>) -> Result<{}, String> {{\n",
            body,
            self.spaces(level + 1),
            enums.name
        );
        body = format!(
            "{}{}if buf.len() <= sizes::U16_LEN {{\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}return Err(String::from(\"Fail to extract value for {} because buffer too small\"));\n", body, self.spaces(level + 3), enums.name);
        body = format!("{}{}}}\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}let index = cursor.get_u16_le();\n",
            body,
            self.spaces(level + 2)
        );

        body = format!(
            "{}{}let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}match index {{\n", body, self.spaces(level + 2));
        for (index, item) in enums.variants.iter().enumerate() {
            let item_type = self.enum_item_type(item.clone());
            body = format!(
                "{}{}{} => match {}::decode(&body_buf) {{\n",
                body,
                self.spaces(level + 3),
                index,
                if item.repeated {
                    format!("Vec::<{}>", item_type)
                } else {
                    item_type
                }
            );
            body = format!(
                "{}{}Ok(v) => Ok({}::{}(v)),\n",
                body,
                self.spaces(level + 4),
                enums.name,
                item.name
            );
            body = format!("{}{}Err(e) => Err(e)\n", body, self.spaces(level + 4));
            body = format!("{}{}}},\n", body, self.spaces(level + 3));
        }
        body = format!(
            "{}{}_ => Err(String::from(\"Fail to find relevant value for {}\")),\n",
            body,
            self.spaces(level + 3),
            enums.name
        );
        body = format!("{}{}}}\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}impl EnumEncode for {} {{\n",
            body,
            self.spaces(level),
            enums.name
        );
        body = format!(
            "{}{}fn get_id(&self) -> u32 {{ {} }}\n",
            body,
            self.spaces(level + 1),
            enums.id,
        );
        body = format!(
            "{}{}fn get_signature(&self) -> u16 {{ {} }}\n",
            body,
            self.spaces(level + 1),
            self.signature,
        );
        body = format!(
            "{}{}fn abduct(&mut self) -> Result<Vec<u8>, String> {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}let (buf, index) = match self {{\n",
            body,
            self.spaces(level + 2)
        );
        for (index, item) in enums.variants.iter().enumerate() {
            body = format!(
                "{}{}Self::{}(v) => (v.encode(), {}),\n",
                body,
                self.spaces(level + 3),
                item.name,
                index
            );
        }
        body = format!(
            "{}{}_ => {{ return Err(String::from(\"Not supportable option\")); }},\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}};\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}let mut buf = match buf {{\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}Ok(buf) => buf,\n", body, self.spaces(level + 3));
        body = format!(
            "{}{}Err(e) => {{ return Err(e); }},\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}};\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}let mut buffer: Vec<u8> = vec!();\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}buffer.append(&mut (index as u16).to_le_bytes().to_vec());\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}buffer.append(&mut buf);\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}Ok(buffer)\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}impl PackingEnum for {} {{}}\n",
            body,
            self.spaces(level),
            enums.name
        );
        body
    }

    fn enum_item_type(&self, item: EnumItem) -> String {
        if let Some(type_ref) = item.types {
            return match type_ref {
                PrimitiveTypes::ETypes::Ei8 => "i8",
                PrimitiveTypes::ETypes::Ei16 => "i16",
                PrimitiveTypes::ETypes::Ei32 => "i32",
                PrimitiveTypes::ETypes::Ei64 => "i64",
                PrimitiveTypes::ETypes::Eu8 => "u8",
                PrimitiveTypes::ETypes::Eu16 => "u16",
                PrimitiveTypes::ETypes::Eu32 => "u32",
                PrimitiveTypes::ETypes::Eu64 => "u64",
                PrimitiveTypes::ETypes::Ef32 => "f32",
                PrimitiveTypes::ETypes::Ef64 => "f64",
                PrimitiveTypes::ETypes::Ebool => "bool",
                PrimitiveTypes::ETypes::Estr => "String",
                _ => {
                    panic!("Unknown type ref {:?} for {}", type_ref, item.name);
                }
            }
            .to_string();
        } else {
            return item.get_full_name();
        }
    }

    fn entity_default(&self, entity_id: usize, store: &mut Store, level: u8) -> String {
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
            format!("{}{}}}", body, self.spaces(level - 1))
        } else if let Some(enums) = store.get_enum(entity_id) {
            format!("{}::Defaults", enums.name)
        } else {
            panic!("Fail to find a struct/enum id: {}", entity_id);
        }
    }

    fn field_default(&self, field: &Field, store: &mut Store, level: u8) -> String {
        let path: String = if field.get_path().is_empty() { String::from("") } else { format!("{}::", field.get_path().join("::")) };
        let mut body = format!("{}: {}", field.name, path);
        if field.repeated && !field.optional {
            body = format!("{}vec![],", body);
        } else if field.optional {
            body = format!("{}None,", body);
        } else if let Some(default_value) = self.type_default_value(&field.kind) {
            body = format!("{}{},", body, default_value);
        } else if let Some(struct_id) = field.ref_type_id {
            body = format!(
                "{}{},",
                body,
                self.entity_default(struct_id, store, level + 1)
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
            "f32" => Some("0.0"),
            "f64" => Some("0.0"),
            "str" => Some("String::from(\"\")"),
            _ => None,
        }
    }

    fn get_decode_type_ref(&self, field: &Field) -> String {
        let mut type_str = self.get_type_ref(field);
        if field.repeated {
            type_str = format!("Vec::<{}>", type_str);
        }
        if field.optional {
            type_str = format!("Option::<{}>", type_str);
        }
        type_str
    }

    fn get_declare_type_ref(&self, field: &Field) -> String {
        let mut type_str = self.get_type_ref(field);
        if field.repeated {
            type_str = format!("Vec<{}>", type_str);
        }
        if field.optional {
            type_str = format!("Option<{}>", type_str);
        }
        type_str
    }

    fn get_type_ref(&self, field: &Field) -> String {
        match field.kind.clone().as_str() {
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
                if let Some(_ref_type_id) = field.ref_type_id {
                    field.get_full_name().join("::")
                } else {
                    panic!("Invalid type definition for field {}", field.name);
                }
            }
        }
    }

    fn get_messages_list(&self, group: Option<&Group>, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}#[derive(Debug, Clone)]\n", self.spaces(level));
        if let Some(group) = group {
            body = format!(
                "{}{}pub enum AvailableMessages {{\n",
                body,
                self.spaces(level)
            );
            for enum_id in &group.enums {
                if let Some(enums) = store.get_enum(*enum_id) {
                    body = format!(
                        "{}{}{}({}),\n",
                        body,
                        self.spaces(level + 1),
                        enums.name,
                        enums.name
                    );
                }
            }
            for struct_id in &group.structs {
                if let Some(strct) = store.get_struct(*struct_id) {
                    body = format!(
                        "{}{}{}({}),\n",
                        body,
                        self.spaces(level + 1),
                        strct.name,
                        strct.name
                    );
                }
            }
            let childs = store.get_child_groups(group.id);
            for child in childs {
                if child.parent == group.id {
                    body = format!(
                        "{}{}{}({}::AvailableMessages),\n",
                        body,
                        self.spaces(level + 1),
                        child.name,
                        child.name
                    );
                }
            }
            body = format!("{}{}}}\n", body, self.spaces(level));
        } else {
            body = format!(
                "{}{}pub enum AvailableMessages {{\n",
                body,
                self.spaces(level)
            );
            for enums in &store.enums {
                if enums.parent == 0 {
                    body = format!(
                        "{}{}{}({}),\n",
                        body,
                        self.spaces(level + 1),
                        enums.name,
                        enums.name
                    );
                }
            }
            for strct in &store.structs {
                if strct.parent == 0 {
                    body = format!(
                        "{}{}{}({}),\n",
                        body,
                        self.spaces(level + 1),
                        strct.name,
                        strct.name
                    );
                }
            }
            for group in &store.groups {
                if group.parent == 0 {
                    body = format!(
                        "{}{}{}({}::AvailableMessages),\n",
                        body,
                        self.spaces(level + 1),
                        group.name,
                        group.name
                    );
                }
            }
            body = format!("{}{}}}\n", body, self.spaces(level));
        }
        body
    }

    fn get_path(&self, mut parent: usize, store: &mut Store) -> Vec<String> {
        let mut path: Vec<String> = vec![];
        loop {
            if parent == 0 {
                break;
            }
            if let Some(group) = store.get_group(parent) {
                path.push(group.name.clone());
                parent = group.parent;
            } else {
                break;
            }
        }
        path.reverse();
        path
    }

    fn get_full_name(&self, name: String, parent: usize, store: &mut Store) -> String {
        let path: Vec<String> = self.get_path(parent, store);
        if path.is_empty() {
            name
        } else {
            format!("{}::{}", path.join("::"), name)
        }
    }

    fn get_entity_path(&self, parent: usize, store: &mut Store) -> Vec<String> {
        let mut path: Vec<String> = vec![];
        let mut parent = parent;
        loop {
            if parent == 0 {
                break;
            }
            if let Some(group) = store.get_group(parent) {
                path.push(group.name.clone());
                parent = group.parent;
            } else {
                break;
            }
        }
        path.reverse();
        path
    }

    fn get_available_entity(&self, parent: usize, name: &str, store: &mut Store) -> String {
        let mut result = String::from("");
        let path = self.get_entity_path(parent, store);
        if path.is_empty() {
            result = format!("AvailableMessages::{}(m)", name);
        } else {
            let mut chain = String::from("");
            for part in path.iter() {
                result = format!(
                    "{}{}AvailableMessages::{}(",
                    result,
                    chain,
                    part
                );
                chain = format!(
                    "{}{}::",
                    chain,
                    part
                );
            }
            result = format!(
                "{}{}AvailableMessages::{}(m){}",
                result,
                chain,
                name, 
                ")".repeat(path.len())
            );
        }
        result
    }

    fn buffer(&self, store: &mut Store) -> String {
        let mut body = format!(
            "{}impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {{\n",
            self.spaces(0)
        );
        body = format!("{}{}fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {{\n", body, self.spaces(1));
        body = format!("{}{}match id {{\n", body, self.spaces(2));
        for enums in &store.enums {
            body = format!(
                "{}{}{} => match {}::extract(buf.to_vec()) {{\n",
                body,
                self.spaces(3),
                enums.id,
                self.get_full_name(enums.name.clone(), enums.parent, &mut store.clone())
            );
            body = format!(
                "{}{}Ok(m) => Ok({}),\n",
                body,
                self.spaces(4),
                self.get_available_entity(enums.parent, &enums.name, &mut store.clone())
            );
            body = format!("{}{}Err(e) => Err(e),\n", body, self.spaces(4));
            body = format!("{}{}}},\n", body, self.spaces(3));
        }
        for structs in &store.structs {
            body = format!(
                "{}{}{} => match {}::extract(buf.to_vec()) {{\n",
                body,
                self.spaces(3),
                structs.id,
                self.get_full_name(structs.name.clone(), structs.parent, &mut store.clone())
            );
            body = format!(
                "{}{}Ok(m) => Ok({}),\n",
                body,
                self.spaces(4),
                self.get_available_entity(structs.parent, &structs.name, &mut store.clone())
            );
            body = format!("{}{}Err(e) => Err(e),\n", body, self.spaces(4));
            body = format!("{}{}}},\n", body, self.spaces(3));
        }
        body = format!(
            "{}{}_ => Err(String::from(\"No message has been found\"))\n",
            body,
            self.spaces(3)
        );
        body = format!("{}{}}}\n", body, self.spaces(2));
        body = format!("{}{}}}\n", body, self.spaces(1));
        body = format!("{}{}fn get_signature(&self) -> u16 {{ {} }}\n", body, self.spaces(1), self.signature);
        body = format!("{}{}}}\n", body, self.spaces(0));
        body
    }

    fn includes(&self) -> String {
        if self.embedded {
            format!(
                "{}{}{}{}{}{}{}\n",
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.uses.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.sizes.mod.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.decode.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.encode.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.storage.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.packing.rs"
                )),
                self.get_injectable(include_str!(
                    "../../../../protocol/implementations/rust/src/protocol.buffer.rs"
                )),
            )
        } else {
            String::new()
        }
    }

    fn get_injectable(&self, content: &str) -> String {
        let re = Regex::new(r"^([\n\r]|.)*(//\s?injectable)").unwrap();
        re.replace_all(content, "").to_string()
    }

    fn spaces(&self, level: u8) -> String {
        "    ".repeat(level as usize)
    }
}

impl Render for RustRender {
    fn new(embedded: bool, signature: u16) -> Self {
        RustRender {
            embedded,
            signature,
        }
    }

    fn render(&self, store: &mut Store, dest: &Path) -> Result<(), String> {
        let mut body = format!("{}\n", self.includes());
        body = format!(
            "{}{}",
            body,
            self.get_messages_list(None, &mut store.clone(), 0)
        );
        for enums in &store.enums {
            if enums.parent == 0 {
                body = format!("{}{}\n", body, self.enums(enums, 0));
            }
        }
        for strct in &store.structs {
            if strct.parent == 0 {
                body = format!("{}{}\n", body, self.structs(strct, &mut store.clone(), 0));
            }
        }
        for group in &store.groups {
            if group.parent == 0 {
                body = format!("{}{}\n", body, self.groups(group, &mut store.clone(), 0));
            }
        }
        body = format!("{}{}\n", body, self.buffer(&mut store.clone()));
        helpers::fs::write(dest.to_path_buf(), body, true)
    }
}
