use super::parser::enums::{Enum, EnumItem};
use super::parser::fields::Field;
use super::parser::groups::Group;
use super::parser::store::Store;
use super::parser::structs::Struct;
use super::parser::types::PrimitiveTypes;
use super::Render;

pub struct TypescriptRender {}

impl TypescriptRender {
    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}export namespace {} {{\n", self.spaces(level), group.name);
        /*
        for enum_id in &group.enums {
            if let Some(enums) = store.get_enum(*enum_id) {
                body = format!(
                    "{}\n{}",
                    body,
                    self.enums(&enums, &mut store.clone(), level + 1)
                );
            }
        }
        */
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
        let mut body = format!("{}interface I{} {{", self.spaces(level), strct.name);
        for field in &strct.fields {
            body = format!(
                "{}\n{}{}",
                body,
                self.spaces(level + 1),
                format!(
                    "{}: {};",
                    field.name,
                    self.get_declare_type_ref(field, &mut store.clone())
                ),
            );
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body = format!(
            "{}{}class {} extends Protocol.Convertor implements I{} {{\n",
            body,
            self.spaces(level),
            strct.name,
            strct.name
        );
        for field in &strct.fields {
            body = format!(
                "{}\n{}{}",
                body,
                self.spaces(level + 1),
                format!(
                    "public {}: {};",
                    field.name,
                    self.get_declare_type_ref(field, &mut store.clone())
                ),
            );
        }

        body = format!(
            "{}\n{}constructor(params: I{})  {{\n",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!("{}{}super();\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}Object.keys(params).forEach((key: string) => {{\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}this[key] = params[key];\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}});\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));

        body = format!(
            "{}{}public getSignature(): string {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}return '{}';\n",
            body,
            self.spaces(level + 2),
            strct.name
        );
        body = format!("{}{}}}\n", body, self.spaces(level + 1));

        body = format!(
            "{}{}public getId(): number {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}return {};\n", body, self.spaces(level + 2), strct.id);
        body = format!("{}{}}}\n", body, self.spaces(level + 1));

        body = format!(
            "{}{}public encode(): ArrayBufferLike {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}return this.collect([", body, self.spaces(level + 2));
        for field in &strct.fields {
            body = format!(
                "{}\n{}{},",
                body,
                self.spaces(level + 3),
                self.get_field_encode(field, &mut store.clone()),
            );
        }
        body = format!("{}\n{}]);\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));

        body = format!(
            "{}{}public decode(buffer: ArrayBufferLike): Error | undefined {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}const storage = this.getStorage(buffer);", body, self.spaces(level + 2));
        body = format!("{}\n{}if (storage instanceof Error) {{", body, self.spaces(level + 2));
        body = format!("{}\n{}return storage;", body, self.spaces(level + 3));
        body = format!("{}\n{}}}", body, self.spaces(level + 2));
        for field in &strct.fields {
            body = format!(
                "{}\n{}",
                body,
                self.get_field_decode_wrap(field, &mut store.clone(), level + 2),
            );
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level + 1));

        body = format!("{}{}}}\n", body, self.spaces(level));
        body
    }

    fn enums(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}#[derive(Debug, Clone, PartialEq)]\n", self.spaces(level));
        body = format!("{}{}pub enum {} {{\n", body, self.spaces(level), enums.name);
        for item in &enums.variants {
            let item_type = self.enum_item_type(item.clone(), store);
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
            "{}{}impl EnumDecode<{}> for {} {{\n",
            body,
            self.spaces(level),
            enums.name,
            enums.name
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
            "{}{}let id = cursor.get_u16_le();\n",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}{}let mut storage = match Storage::new(buf) {{\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}Ok(s) => s,\n", body, self.spaces(level + 3));
        body = format!(
            "{}{}Err(e) => {{ return Err(e); }}\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}};\n", body, self.spaces(level + 2));
        body = format!("{}{}match id {{\n", body, self.spaces(level + 2));
        for (index, item) in enums.variants.iter().enumerate() {
            let item_type = self.enum_item_type(item.clone(), store);
            body = format!(
                "{}{}{} => match {}::decode(&mut storage, id)\n",
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
            "{}{}fn abduct(&mut self) -> Result<Vec<u8>, String> {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}match self {{\n", body, self.spaces(level + 2));
        for (index, item) in enums.variants.iter().enumerate() {
            body = format!(
                "{}{}Self::{}(v) => v.encode({}),\n",
                body,
                self.spaces(level + 3),
                item.name,
                index
            );
        }
        body = format!(
            "{}{}_ => Err(String::from(\"Not supportable option\")),\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}} {{\n", body, self.spaces(level + 2));
        body = format!("{}{}Ok(buf) => Ok(buf),\n", body, self.spaces(level + 3));
        body = format!("{}{}Err(e) => Err(e),,\n", body, self.spaces(level + 3));
        body = format!("{}{}}}\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body
    }

    fn enum_item_type(&self, item: EnumItem, store: &mut Store) -> String {
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
        } else if let Some(ref_type_id) = item.ref_type_id {
            if let Some(strct) = store.get_struct(ref_type_id) {
                return strct.name;
            }
        }
        panic!("Fail to find a type ref for {}", item.name);
    }

    fn entity_default(&self, entity_id: usize, store: &mut Store) -> String {
        if let Some(strct) = store.get_struct(entity_id) {
            let mut body = format!("new {}({{ ", strct.name);
            for field in &strct.fields {
                body = format!(
                    "{}{}, ",
                    body,
                    self.field_default(field, &mut store.clone())
                );
            }
            format!("{}}})", body)
        } else if let Some(enums) = store.get_enum(entity_id) {
            format!("{}::Defaults", enums.name)
        } else {
            panic!(format!("Fail to find a struct/enum id: {}", entity_id));
        }
    }

    fn field_default(&self, field: &Field, store: &mut Store) -> String {
        let mut body = format!("{}: ", field.name);
        if field.optional {
            body = format!("{}undefined", body);
        } else if field.repeated {
            body = format!("{}[]", body);
        } else if let Some(default_value) = self.type_default_value(&field.kind) {
            body = format!("{}{}", body, default_value);
        } else if let Some(struct_id) = field.ref_type_id {
            body = format!(
                "{}{}",
                body,
                self.entity_default(struct_id, store)
            );
        }
        body
    }

    fn get_field_decode_wrap(&self, field: &Field, store: &mut Store, level: u8) -> String {
        if field.optional {
            let mut body = format!("{}const {}Buf: ArrayBufferLike | undefined = storage.get({});", self.spaces(level), field.name, field.id);
            body = format!("{}\n{}if ({}Buf === undefined) {{", body, self.spaces(level), field.name);
            body = format!("{}\n{}return new Error(`Fail to get property {}`);", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}}", body, self.spaces(level));
            body = format!("{}\n{}if ({}Buf.byteLength === 0) {{", body, self.spaces(level), field.name);
            body = format!("{}\n{}this.{} = undefined;", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}} else {{", body, self.spaces(level));
            body = format!("{}\n{}", body, self.get_field_decode(field, store, level + 1));
            body = format!("{}\n{}}}", body, self.spaces(level));
            body
        } else {
            self.get_field_decode(field, store, level)
        }
    }

    fn get_field_decode(&self, field: &Field, store: &mut Store, level: u8) -> String {
        let mut body: String;
        if let Some(struct_id) = field.ref_type_id {
            body = format!("{}const {}: {} = {};", self.spaces(level), field.name, field.kind, self.entity_default(struct_id, &mut store.clone()));
            body = format!("{}\n{}const {}Buf: ArrayBufferLike = storage.get({});", body, self.spaces(level), field.name, field.id);
            body = format!("{}\n{}if ({}Buf instanceof Error) {{", body, self.spaces(level), field.name);
            body = format!("{}\n{}return {}Buf;", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}}", body, self.spaces(level));
            body = format!("{}\n{}const {}Err: Error | undefined = {}.decode({}Buf);", body, self.spaces(level), field.name, field.name, field.name);
            body = format!("{}\n{}if ({}Err instanceof Error) {{", body, self.spaces(level), field.name);
            body = format!("{}\n{}return {}Err;", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}} else {{", body, self.spaces(level));
            body = format!("{}\n{}this.{} = {};", body, self.spaces(level + 1), field.name, field.name);
            body = format!("{}\n{}}}", body, self.spaces(level));
        } else {
            let mut type_str = self.get_type_ref(field, &mut store.clone());
            let primitive = self.get_primitive_ref(field);
            if field.repeated {
                type_str = format!("Array<{}>", type_str);
            }
            body = format!("{}const {}: {} | Error = this.getValue<{}>(storage, {}, Protocol.Primitives.{}.decode);", self.spaces(level), field.name, type_str, type_str, field.id, primitive);
            body = format!("{}\n{}if ({} instanceof Error) {{", body, self.spaces(level), field.name);
            body = format!("{}\n{}return {};", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}} else {{", body, self.spaces(level));
            body = format!("{}\n{}this.{} = {};", body, self.spaces(level + 1), field.name, field.name);
            body = format!("{}\n{}}}", body, self.spaces(level));
        }
        body
    }

    fn get_field_encode(&self, field: &Field, store: &mut Store) -> String {
        let mut body: String;
        if let Some(struct_id) = field.ref_type_id {
            let optional = if field.optional {
                format!("if (this.{} === undefined) {{ return this.getBuffer({}, Protocol.ESize.u8, 0, new Uint8Array()); }}", field.name, field.id)
            } else {
                format!("")
            };
            body = format!("() => {{{} const buffer = this.{}.encode(); return this.getBuffer({}, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); }}", optional, field.name, field.id);
        } else {
            let type_str = self.get_type_ref(field, &mut store.clone());
            let size_ref = self.get_size_ref(field);
            let primitive = self.get_primitive_ref(field);
            if field.repeated {
                body = format!("this.getBufferFromBuf<Array<{}>>({}, {}, Protocol.Primitives.{}.encode, this.{})", type_str, field.id, size_ref, primitive, field.name);
            } else {
                body = if field.kind == "str" {
                    format!("this.getBufferFromBuf<string>({}, {}, Protocol.Primitives.{}.encode, this.{})", field.id, size_ref, primitive, field.name)
                } else {
                    format!("this.getBuffer({}, {}, Protocol.Primitives.{}.getSize(), Protocol.Primitives.{}.encode(this.{}))", field.id, size_ref, primitive, primitive, field.name)
                }
            }
            if field.optional {
                body = format!("() => this.{} === undefined ? this.getBuffer({}, Protocol.ESize.u8, 0, new Uint8Array()) : {}", field.name, field.id, body);
            } else {
                body = format!("() => {}", body);
            }
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
            "str" => Some("''"),
            _ => None,
        }
    }

    fn get_decode_type_ref(&self, field: &Field, store: &mut Store) -> String {
        let mut type_str = self.get_type_ref(field, &mut store.clone());
        if field.optional {
            type_str = format!("Option::<{}>", type_str);
        }
        if field.repeated {
            type_str = format!("Vec::<{}>", type_str);
        }
        type_str
    }

    fn get_declare_type_ref(&self, field: &Field, store: &mut Store) -> String {
        let mut type_str = self.get_type_ref(field, &mut store.clone());
        if field.optional {
            type_str = format!("{} | undefined", type_str);
        }
        if field.repeated {
            type_str = format!("Array<{}>", type_str);
        }
        type_str
    }

    fn get_type_ref(&self, field: &Field, store: &mut Store) -> String {
        match field.kind.clone().as_str() {
            "bool" => String::from("boolean"),
            "i8" => String::from("number"),
            "i16" => String::from("number"),
            "i32" => String::from("number"),
            "i64" => String::from("bigint"),
            "u8" => String::from("number"),
            "u16" => String::from("number"),
            "u32" => String::from("number"),
            "u64" => String::from("bigint"),
            "f32" => String::from("number"),
            "f64" => String::from("number"),
            "str" => String::from("string"),
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
        }
    }

    fn get_size_ref(&self, field: &Field) -> String {
        match field.kind.clone().as_str() {
            "bool" => String::from("Protocol.ESize.u8"),
            "i8" => String::from("Protocol.ESize.u8"),
            "i16" => String::from("Protocol.ESize.u8"),
            "i32" => String::from("Protocol.ESize.u8"),
            "i64" => String::from("Protocol.ESize.u8"),
            "u8" => String::from("Protocol.ESize.u8"),
            "u16" => String::from("Protocol.ESize.u8"),
            "u32" => String::from("Protocol.ESize.u8"),
            "u64" => String::from("Protocol.ESize.u8"),
            "f32" => String::from("Protocol.ESize.u8"),
            "f64" => String::from("Protocol.ESize.u8"),
            "str" => String::from("Protocol.ESize.u64"),
            _ => String::from("Protocol.ESize.u64"),
        }
    }

    fn get_primitive_ref(&self, field: &Field) -> String {
        if !field.repeated {
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
                "str" => String::from("StrUTF8"),
                _ => panic!("{} type isn't recognized", field.kind),
            }
        } else {
            match field.kind.clone().as_str() {
                "bool" => String::from("ArrayBool"),
                "i8" => String::from("ArrayI8"),
                "i16" => String::from("ArrayI16"),
                "i32" => String::from("ArrayI32"),
                "i64" => String::from("ArrayI64"),
                "u8" => String::from("ArrayU8"),
                "u16" => String::from("ArrayU16"),
                "u32" => String::from("ArrayU32"),
                "u64" => String::from("ArrayU64"),
                "f32" => String::from("ArrayF32"),
                "f64" => String::from("ArrayF64"),
                "str" => String::from("ArrayStrUTF8"),
                _ => panic!("{} type isn't recognized", field.kind),
            }
        }
    }

    fn spaces(&self, level: u8) -> String {
        "    ".repeat(level as usize)
    }
}

impl Render for TypescriptRender {
    fn render(&self, store: Store) -> String {
        let mut body = String::new();
        /*
        for enums in &store.enums {
            if enums.parent == 0 {
                body =
                    format!("{}{}\n", body, self.enums(enums, &mut store.clone(), 0)).to_string();
            }
        }*/
        for strct in &store.structs {
            if strct.parent == 0 {
                body =
                    format!("{}{}\n", body, self.structs(strct, &mut store.clone(), 0)).to_string();
            }
        }
        for group in &store.groups {
            if group.parent == 0 {
                body =
                    format!("{}{}\n", body, self.groups(group, &mut store.clone(), 0)).to_string();
            }
        }
        body
    }
}
