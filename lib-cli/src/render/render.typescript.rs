use super::parser::enums::{Enum};
use super::parser::fields::Field;
use super::parser::groups::Group;
use super::parser::store::Store;
use super::parser::structs::Struct;
use super::parser::types::PrimitiveTypes;
use super::Render;
use std::{include_str};
use regex::Regex;

pub struct TypescriptRender {
    embedded: bool,
}

impl TypescriptRender {

    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}export namespace {} {{\n", self.spaces(level), group.name);
        for enum_id in &group.enums {
            if let Some(enums) = store.get_enum(*enum_id) {
                body = format!(
                    "{}\n{}",
                    body,
                    self.enums(&enums, &mut store.clone(), level + 1)
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
                if store.get_enum(ref_type_id).is_some() {
                    body = format!(
                        "{}\n{}{}",
                        body,
                        self.spaces(level + 1),
                        format!(
                            "private _{}: Primitives.Enum;",
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
        body = format!("{}\n{}}}", body, self.spaces(level + 1));

        body = format!("{}\n{}public defaults(): {} {{", body, self.spaces(level + 1), strct.name);
        body = format!("{}\n{}return {}.defaults();", body, self.spaces(level + 2), strct.name);
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
                    body = format!("{}\n{}this._{} = new Primitives.Enum([", body, self.spaces(level + 1), field.name);
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
                        } else if variant.ref_type_id.is_some() {
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
                        };
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
        let mut body = format!("{}interface {} {{\n", self.spaces(level), enums.name);
        for variant in &enums.variants {
            let variant_type = if let Some(prim_type_ref) = variant.types.clone() {
                self.etype_ts(prim_type_ref.clone(), variant.repeated)
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    strct.name
                } else {
                    panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {}", enums.name, variant.name, ref_type_id);
                }
            } else {
                panic!("Unknown type of data in scope of enum {} / {} ", enums.name, variant.name);
            };
            body = format!(
                "{}{}{}?: {};\n",
                body,
                self.spaces(level + 1),
                variant.name,
                variant_type
            );
        }
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
            PrimitiveTypes::ETypes::Ei8 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Ei16 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Ei32 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Ei64 => if repeated { "[BigInt(0)]" } else { "BigInt(0)" },
            PrimitiveTypes::ETypes::Eu8 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Eu16 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Eu32 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Eu64 => if repeated { "[BigInt(0)]" } else { "BigInt(0)" },
            PrimitiveTypes::ETypes::Ef32 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Ef64 => if repeated { "[0]" } else { "0" },
            PrimitiveTypes::ETypes::Ebool => if repeated { "[true]" } else { "true" },
            PrimitiveTypes::ETypes::Estr => if repeated { "['']" } else { "''" },
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
        } else if store.get_enum(entity_id).is_some() {
            "{}".to_string()
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
        if let Some(entity_id) = field.ref_type_id {
            if let Some(strct) = store.get_struct(entity_id) {
                if field.repeated {
                    body = format!("{}const arr{}Inst: {} = {}.defaults();", self.spaces(level), strct.name, strct.name, strct.name);
                    body = format!("{}\n{}const arr{}: Array<any> | Error = this.getValue<{}[]>(storage, {}, arr{}Inst.decodeSelfArray.bind(arr{}Inst));", body, self.spaces(level), strct.name, strct.name, field.id, strct.name, strct.name);
                    body = format!("{}\n{}if (arr{} instanceof Error) {{", body, self.spaces(level), strct.name);
                    body = format!("{}\n{}return arr{};", body, self.spaces(level + 1), strct.name);
                    body = format!("{}\n{}}} else {{", body, self.spaces(level));
                    body = format!("{}\n{}this.{} = arr{} as {}[];", body, self.spaces(level + 1), field.name, strct.name, strct.name);
                    body = format!("{}\n{}}}", body, self.spaces(level));
                } else {
                    body = format!("{}const {}: {} = {};", self.spaces(level), field.name, field.kind, self.entity_default(entity_id, &mut store.clone(), level));
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
                }
            } else if let Some(enums) = store.get_enum(entity_id) {
                body = format!("{}this.{} = {{}};", self.spaces(level), field.name);
                body = format!("{}\n{}const {}Buf: ArrayBufferLike = storage.get({});", body, self.spaces(level), field.name, field.id);
                body = format!("{}\n{}if ({}Buf === undefined) {{", body, self.spaces(level), field.name);
                body = format!("{}\n{}return new Error(`Fail to get property \"{}\"`);", body, self.spaces(level + 1), field.name);
                body = format!("{}\n{}}}", body, self.spaces(level));
                body = format!("{}\n{}if ({}Buf.byteLength > 0) {{", body, self.spaces(level), field.name);
                body = format!("{}\n{}const {}Err: Error | undefined = this._{}.decode({}Buf);", body, self.spaces(level + 1), field.name, field.name, field.name);
                body = format!("{}\n{}if ({}Err instanceof Error) {{", body, self.spaces(level + 1), field.name);
                body = format!("{}\n{}return {}Err;", body, self.spaces(level + 2), field.name);
                body = format!("{}\n{}}} else {{", body, self.spaces(level + 1));
                body = format!("{}\n{}switch (this._{}.getValueIndex()) {{", body, self.spaces(level + 2), field.name);
                for (pos, variant) in enums.variants.iter().enumerate() {
                    let types = if let Some(prim_type_ref) = variant.types.clone() {
                        self.etype_ts(prim_type_ref, variant.repeated)
                    } else if let Some(ref_type_id) = variant.ref_type_id {
                        if let Some(strct) = store.get_struct(ref_type_id) {
                            strct.name
                        } else {
                            panic!("Unknown type of data in scope of enum {} / {}, ref_type_id: {}. Failed to find a struct. ", enums.name, variant.name, ref_type_id);
                        }
                    } else {
                        panic!("Unknown type of data in scope of enum {} / {}", enums.name, variant.name);
                    };
                    body = format!("{}\n{}case {}: this.{}.{} = this._{}.get<{}>(); break;", body, self.spaces(level + 3), pos, field.name, variant.name, field.name, types);
                }
                body = format!("{}\n{}}}", body, self.spaces(level + 2));
                body = format!("{}\n{}}}", body, self.spaces(level + 1));
                body = format!("{}\n{}}}", body, self.spaces(level));
            } else {
                panic!("Fail to find a type by ref {} for field {}", entity_id, field.name);
            }
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
        if let Some(entity_id) = field.ref_type_id {
            let optional = if field.optional {
                format!("if (this.{} === undefined) {{ return this.getBuffer({}, Protocol.ESize.u8, 0, new Uint8Array()); }}", field.name, field.id)
            } else {
                format!("")
            };
            if let Some(strct) = store.get_struct(entity_id) {
                if field.repeated {
                    body = format!("() => {{{} const self: {} = {}.defaults(); return this.getBufferFromBuf<{}[]>({}, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.{}); }}", optional, strct.name, strct.name, strct.name, field.id, field.name);
                } else {
                    body = format!("() => {{{} const buffer = this.{}.encode(); return this.getBuffer({}, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); }}", optional, field.name, field.id);
                }
            } else if store.get_enum(entity_id).is_some() {
                body = format!("() => {{{} const buffer = this._{}.encode(); return this.getBuffer({}, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); }}", optional, field.name, field.id);
            } else {
                panic!("Fail to find a type by ref {} for field {}", entity_id, field.name);
            }
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

    fn includes(&self) -> String {
        if self.embedded {
            format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
                self.get_injectable(include_str!("../../../protocol/typescript/src/tools/index.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/tools/tools.arraybuffer.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.sizes.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.interface.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.u8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.u16.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.u32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.u64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.i8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.i16.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.i32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.i64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.f32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.f64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.bool.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.string.utf8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.u8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.u16.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.u32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.u64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.i8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.i16.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.i32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.i64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.f32.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.f64.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.bool.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.array.string.utf8.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.enum.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.primitives.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.convertor.storage.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/protocol.convertor.ts")),
                self.get_injectable(include_str!("../../../protocol/typescript/src/index.ts")),
            )
        } else {
            include_str!("../../../protocol/typescript/src/protocol.injection.ts").to_string()
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

impl Render for TypescriptRender {

    fn new(embedded: bool) -> Self {
        TypescriptRender { embedded }
    }

    fn render(&self, store: Store) -> String {
        let mut body = format!("{}\n", self.includes());
        for enums in &store.enums {
            if enums.parent == 0 {
                body =
                    format!("{}{}\n", body, self.enums(enums, &mut store.clone(), 0)).to_string();
            }
        }
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
