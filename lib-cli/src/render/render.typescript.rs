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
        body = format!("{}\n{}public static defaults(): {} {{", body, self.spaces(level + 1), strct.name);
        body = format!("{}\n{}return {};", body, self.spaces(level + 2), self.entity_default(strct.id, &mut store.clone(), level + 2));
        body = format!("{}\n{}}}", body, self.spaces(level + 1));

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

        for field in &strct.fields {
            if let Some(ref_type_id) = field.ref_type_id {
                if let Some(enums) = store.get_enum(ref_type_id) {
                    body = format!(
                        "{}\n{}{}",
                        body,
                        self.spaces(level + 1),
                        format!(
                            "private _{}: Protocol.Primitives.Enum;",
                            field.name,
                        ),
                    );
                }
            }
        }
    
        body = format!(
            "{}\n{}",
            body,
            self.struct_constructor(&strct, &mut store.clone(), level + 1)
        );

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

    fn struct_constructor(&self, strct: &Struct, store: &mut Store, level: u8) -> String {
        let mut body = format!(
            "{}constructor(params: I{})  {{\n",
            self.spaces(level),
            strct.name
        );
        body = format!("{}{}super();\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}Object.keys(params).forEach((key: string) => {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}this[key] = params[key];\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}}});", body, self.spaces(level + 1));
        for field in &strct.fields {
            if let Some(ref_type_id) = field.ref_type_id {
                if let Some(enums) = store.get_enum(ref_type_id) {
                    body = format!("{}\n{}this._{} = new Protocol.Primitives.Enum([", body, self.spaces(level + 1), field.name);
                    for variant in &enums.variants {
                        if let Some(prim_type_ref) = variant.types.clone() {
                            body = format!("{}\n{}Protocol.Primitives.{}.getSignature(),", body, self.spaces(level + 2), self.etype(prim_type_ref, variant.repeated));
                        } else if let Some(ref_type_id) = variant.ref_type_id {
                            if let Some(strct) = store.get_struct(ref_type_id) {
                                body = format!("{}\n{}{}.getSignature(),", body, self.spaces(level + 2), strct.name);
                            } else {
                                panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {} ", enums.name, variant.name, ref_type_id);
                            }
                        }
                    }
                    body = format!("{}\n{}], (id: number): ISigned<any> | undefined => {{", body, self.spaces(level + 1));
                    body = format!("{}\n{}switch (id) {{", body, self.spaces(level + 2));
                    for (pos, variant) in enums.variants.iter().enumerate() {
                        if let Some(prim_type_ref) = variant.types.clone() {
                            body = format!("{}\n{}case {}: return new Protocol.Primitives.{}({});", body, self.spaces(level + 3), pos, self.etype(prim_type_ref.clone(), variant.repeated), self.etype_def(prim_type_ref, variant.repeated));
                        } else if let Some(ref_type_id) = variant.ref_type_id {
                            if let Some(strct) = store.get_struct(ref_type_id) {
                                body = format!("{}\n{}case {}: return {}.defaults();", body, self.spaces(level + 3), pos, strct.name);
                            } else {
                                panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {} ", enums.name, variant.name, ref_type_id);
                            }
                        }
                    }
                    body = format!("{}\n{}}}", body, self.spaces(level + 2));
                    body = format!("{}\n{}}});", body, self.spaces(level + 1));
                    body = format!("{}\n{}if (Object.keys(this.{}).length > 1) {{", body, self.spaces(level + 1), field.name);
                    body = format!("{}\n{}throw new Error(`Option cannot have more then 1 value. Property \"{}\" or class \"{}\"`);", body, self.spaces(level + 2), field.name, strct.name);
                    body = format!("{}\n{}}}", body, self.spaces(level + 1));
                    for (pos, variant) in enums.variants.iter().enumerate() {
                        let value = if let Some(prim_type_ref) = variant.types.clone() {
                            format!("new Protocol.Primitives.{}(this.{}.{})", self.etype(prim_type_ref.clone(), variant.repeated), field.name, variant.name)
                        } else if let Some(ref_type_id) = variant.ref_type_id {
                            format!("this.{}.{}", field.name, variant.name)
                        } else {
                            panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {} ", enums.name, variant.name, ref_type_id);
                        };
                        //
                        let types = if let Some(prim_type_ref) = variant.types.clone() {
                            self.etype_ts(prim_type_ref, variant.repeated)
                        } else if let Some(ref_type_id) = variant.ref_type_id {
                            if let Some(strct) = store.get_struct(ref_type_id) {
                                strct.name
                            } else {
                                panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {}. Failed to find a struct. ", enums.name, variant.name, ref_type_id);
                            }
                        } else {
                            panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {} ", enums.name, variant.name, ref_type_id);
                        };;
                        body = format!("{}\n{}if (this.{}.{} !== undefined) {{", body, self.spaces(level + 1), field.name, variant.name);
                        body = format!("{}\n{}const err: Error | undefined = this._{}.set(new Protocol.Primitives.Option<{}>({}, {}));", body, self.spaces(level + 2), field.name, types, pos, value);
                        body = format!("{}\n{}if (err instanceof Error) {{", body, self.spaces(level + 2));
                        body = format!("{}\n{}throw err;", body, self.spaces(level + 3));
                        body = format!("{}\n{}}}", body, self.spaces(level + 2));
                        body = format!("{}\n{}}}", body, self.spaces(level + 1));
                    }
                }
            }
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level));
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

    fn etype(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => if repeated { "ArrayI8" } else  { "i8" },
            PrimitiveTypes::ETypes::Ei16 => if repeated { "ArrayI16" } else  { "i16" },
            PrimitiveTypes::ETypes::Ei32 => if repeated { "ArrayI32" } else  { "i32" },
            PrimitiveTypes::ETypes::Ei64 => if repeated { "ArrayI64" } else  { "i64" },
            PrimitiveTypes::ETypes::Eu8 => if repeated { "ArrayU8" } else  { "u8" },
            PrimitiveTypes::ETypes::Eu16 => if repeated { "ArrayU16" } else  { "u16" },
            PrimitiveTypes::ETypes::Eu32 => if repeated { "ArrayU32" } else  { "u32" },
            PrimitiveTypes::ETypes::Eu64 => if repeated { "ArrayU64" } else  { "u64" },
            PrimitiveTypes::ETypes::Ef32 => if repeated { "ArrayF32" } else  { "f32" },
            PrimitiveTypes::ETypes::Ef64 => if repeated { "ArrayF64" } else  { "f64" },
            PrimitiveTypes::ETypes::Ebool => if repeated { "ArrayBool" } else  { "bool" },
            PrimitiveTypes::ETypes::Estr => if repeated { "ArrayStrUTF8" } else  { "StrUTF8" },
            _ => {
                panic!("Unknown type ref {:?}", etype);
            }
        }.to_string()
    }

    fn etype_def(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => "0",
            PrimitiveTypes::ETypes::Ei16 => "0",
            PrimitiveTypes::ETypes::Ei32 => "0",
            PrimitiveTypes::ETypes::Ei64 => "BigInt(0)",
            PrimitiveTypes::ETypes::Eu8 => "0",
            PrimitiveTypes::ETypes::Eu16 => "0",
            PrimitiveTypes::ETypes::Eu32 => "0",
            PrimitiveTypes::ETypes::Eu64 => "BigInt(0)",
            PrimitiveTypes::ETypes::Ef32 => "0",
            PrimitiveTypes::ETypes::Ef64 => "0",
            PrimitiveTypes::ETypes::Ebool => "true",
            PrimitiveTypes::ETypes::Estr => "''",
            _ => {
                panic!("Unknown type ref {:?}", etype);
            }
        }
        .to_string()
    }

    fn etype_ts(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Ei16 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Ei32 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Ei64 => if repeated { "Array<bigint>" } else  { "bigint" },
            PrimitiveTypes::ETypes::Eu8 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Eu16 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Eu32 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Eu64 => if repeated { "Array<bigint>" } else  { "bigint" },
            PrimitiveTypes::ETypes::Ef32 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Ef64 => if repeated { "Array<number>" } else  { "number" },
            PrimitiveTypes::ETypes::Ebool => if repeated { "Array<boolean>" } else  { "boolean" },
            PrimitiveTypes::ETypes::Estr => if repeated { "Array<string>" } else  { "string" },
            _ => {
                panic!("Unknown type ref {:?}", etype);
            }
        }.to_string()
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

    fn entity_default(&self, entity_id: usize, store: &mut Store, level: u8) -> String {
        if let Some(strct) = store.get_struct(entity_id) {
            let mut body = format!("new {}({{ ", strct.name);
            for field in &strct.fields {
                body = format!(
                    "{}\n{}{},",
                    body,
                    self.spaces(level + 1),
                    self.field_default(field, &mut store.clone(), level)
                );
            }
            format!("{}\n{}}})", body, self.spaces(level))
        } else if let Some(enums) = store.get_enum(entity_id) {
            format!("{{}}")
        } else {
            panic!(format!("Fail to find a struct/enum id: {}", entity_id));
        }
    }

    fn field_default(&self, field: &Field, store: &mut Store, level: u8) -> String {
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
                self.entity_default(struct_id, store, level + 1)
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
            body = format!("{}const {}: {} = {};", self.spaces(level), field.name, field.kind, self.entity_default(struct_id, &mut store.clone(), level));
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
