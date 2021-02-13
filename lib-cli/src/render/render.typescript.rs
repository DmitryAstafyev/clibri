use super::parser::enums::Enum;
use super::parser::fields::Field;
use super::parser::groups::Group;
use super::parser::store::Store;
use super::parser::structs::Struct;
use super::parser::types::PrimitiveTypes;
use super::{ Render, stop };
use regex::Regex;
use std::include_str;

pub struct TypescriptRender {
    embedded: bool,
    signature: u16,
}

impl TypescriptRender {
    fn groups(&self, group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}export namespace {} {{\n", self.spaces(level), group.name);
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
        let mut body = format!("{}export interface I{} {{", self.spaces(level), strct.name);
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
            "{}{}export class {} extends Protocol.Convertor implements I{}, ISigned<{}> {{\n",
            body,
            self.spaces(level),
            strct.name,
            strct.name,
            strct.name,
        );
        body = format!(
            "{}\n{}",
            body,
            self.struct_map(&strct, &mut store.clone(), level + 1)
        );

        body = format!(
            "{}\n{}public static defaults(): {} {{",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!(
            "{}\n{}return {};",
            body,
            self.spaces(level + 2),
            self.entity_default(strct.id, &mut store.clone(), level + 2)
        );
        body = format!("{}\n{}}}\n", body, self.spaces(level + 1));

        body = format!("{}\n{}", body, self.struct_validator(&strct, level + 1));

        body = format!("{}\n{}", body, self.struct_from(&strct, level + 1));

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
                        format!("private _{}: Primitives.Enum;", field.name,),
                    );
                }
            }
        }
        body = format!("{}\n", body);
        body = format!(
            "{}{}public static getSignature(): string {{ return '{}'; }}\n",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!("{}\n", body);
        body = format!(
            "{}\n{}",
            body,
            self.struct_constructor(&strct, &mut store.clone(), level + 1)
        );
        body = format!("{}\n", body);
        body = format!(
            "{}{}public signature(): number {{ return {}; }}\n",
            body,
            self.spaces(level + 1),
            self.signature
        );
        body = format!("{}\n", body);
        body = format!(
            "{}{}public getSignature(): string {{ return '{}'; }}\n",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!("{}\n", body);
        body = format!(
            "{}{}public get(): {} {{ return this; }}\n",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!("{}\n", body);
        body = format!(
            "{}{}public getId(): number {{ return {}; }}\n",
            body,
            self.spaces(level + 1),
            strct.id
        );
        body = format!("{}\n", body);
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
        body = format!("{}\n", body);
        body = format!(
            "{}{}public decode(buffer: ArrayBufferLike): Error | undefined {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}const storage = this.getStorage(buffer);",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}\n{}if (storage instanceof Error) {{",
            body,
            self.spaces(level + 2)
        );
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
        body = format!("{}\n", body);
        body = format!(
            "{}\n{}public defaults(): {} {{",
            body,
            self.spaces(level + 1),
            strct.name
        );
        body = format!(
            "{}\n{}return {}.defaults();",
            body,
            self.spaces(level + 2),
            strct.name
        );
        body = format!("{}\n{}}}\n", body, self.spaces(level + 1));

        body = format!("{}{}}}\n", body, self.spaces(level));
        body
    }

    fn enum_declaration(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = "[".to_string();
        for variant in &enums.variants {
            if let Some(prim_type_ref) = variant.types.clone() {
                body = format!(
                    "{}\n{}Protocol.Primitives.{}.getSignature(),",
                    body,
                    self.spaces(level),
                    self.etype(prim_type_ref, variant.repeated)
                );
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    body = format!(
                        "{}\n{}{}.getSignature(),",
                        body,
                        self.spaces(level),
                        store.get_struct_path(strct.id).join(".")
                    );
                } else {
                    stop!(
                        "Unknown type of data in scope of enum {} / {}, ref_type_id: {} ",
                        enums.name, variant.name, ref_type_id
                    );
                }
            }
        }
        body = format!("{}\n{}]", body, self.spaces(level - 1));
        body
    }

    fn enum_getter(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}switch (id) {{", self.spaces(level));
        for (pos, variant) in enums.variants.iter().enumerate() {
            if let Some(prim_type_ref) = variant.types.clone() {
                body = format!(
                    "{}\n{}case {}: return new Protocol.Primitives.{}({});",
                    body,
                    self.spaces(level + 1),
                    pos,
                    self.etype(prim_type_ref.clone(), variant.repeated),
                    self.etype_def(prim_type_ref, variant.repeated)
                );
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    body = format!(
                        "{}\n{}case {}: return {}.defaults();",
                        body,
                        self.spaces(level + 1),
                        pos,
                        store.get_struct_path(strct.id).join(".")
                    );
                } else {
                    stop!(
                        "Unknown type of data in scope of enum {} / {}, ref_type_id: {} ",
                        enums.name, variant.name, ref_type_id
                    );
                }
            }
        }
        body = format!("{}\n{}}}", body, self.spaces(level));
        body
    }

    fn enum_setter(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}if (Object.keys(src).length > 1) {{", self.spaces(level),);
        body = format!(
            "{}\n{}return new Error(`Option cannot have more then 1 value.`);",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}\n{}}}", body, self.spaces(level));
        for (pos, variant) in enums.variants.iter().enumerate() {
            let value = if let Some(prim_type_ref) = variant.types.clone() {
                format!(
                    "new Protocol.Primitives.{}(src.{})",
                    self.etype(prim_type_ref.clone(), variant.repeated),
                    variant.name
                )
            } else if variant.ref_type_id.is_some() {
                format!("src.{}", variant.name)
            } else {
                stop!(
                    "Unknown type of data in scope of enum {} / {}",
                    enums.name, variant.name
                );
            };
            //
            let types = if let Some(prim_type_ref) = variant.types.clone() {
                self.etype_ts(prim_type_ref, variant.repeated)
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    store.get_struct_path(strct.id).join(".")
                } else {
                    stop!("Unknown type of data in scope of enum {} / {}, ref_type_id: {}. Failed to find a struct. ", enums.name, variant.name, ref_type_id);
                }
            } else {
                stop!(
                    "Unknown type of data in scope of enum {} / {}",
                    enums.name, variant.name
                );
            };
            body = format!(
                "{}\n{}if (src.{} !== undefined) {{",
                body,
                self.spaces(level),
                variant.name
            );
            body = format!("{}\n{}const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<{}>({}, {}));", body, self.spaces(level + 1), types, pos, value);
            body = format!(
                "{}\n{}if (err instanceof Error) {{",
                body,
                self.spaces(level + 1)
            );
            body = format!("{}\n{}return err;", body, self.spaces(level + 2));
            body = format!("{}\n{}}}", body, self.spaces(level + 1));
            body = format!("{}\n{}}}", body, self.spaces(level));
        }
        body
    }

    fn get_enum_decode(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}switch (this.getValueIndex()) {{", self.spaces(level),);
        for (pos, variant) in enums.variants.iter().enumerate() {
            let types = if let Some(prim_type_ref) = variant.types.clone() {
                self.etype_ts(prim_type_ref, variant.repeated)
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    store.get_struct_path(strct.id).join(".")
                } else {
                    stop!("Unknown type of data in scope of enum {} / {}, ref_type_id: {}. Failed to find a struct. ", enums.name, variant.name, ref_type_id);
                }
            } else {
                stop!(
                    "Unknown type of data in scope of enum {} / {}",
                    enums.name, variant.name
                );
            };
            body = format!(
                "{}\n{}case {}: target.{} = this.getValue<{}>(); break;",
                body,
                self.spaces(level + 1),
                pos,
                variant.name,
                types
            );
        }
        body = format!("{}\n{}}}", body, self.spaces(level));
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
                    // -------
                    body = format!(
                        "{}\n{}this._{} = new {}()",
                        body,
                        self.spaces(level + 1),
                        field.name,
                        enums.name,
                    );
                    body = format!(
                        "{}\n{}{}this._{}.set(this.{});",
                        body,
                        self.spaces(level + 1),
                        if field.optional {
                            format!("this.{} !== undefined && ", field.name)
                        } else {
                            "".to_string()
                        },
                        field.name,
                        field.name,
                    );
                }
            }
        }
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body
    }

    fn struct_map(&self, strct: &Struct, store: &mut Store, level: u8) -> String {
        let mut body = format!(
            "{}public static scheme: Protocol.IPropScheme[] = [",
            self.spaces(level)
        );
        for field in &strct.fields {
            body = format!(
                "{}{}",
                body,
                self.get_field_map_def(field, &mut store.clone(), level + 1),
            );
        }
        body = format!("{}\n{}];\n", body, self.spaces(level));
        body
    }

    fn struct_validator(&self, strct: &Struct, level: u8) -> String {
        let mut body = format!("{}public static getValidator(array: boolean): {{ validate(value: any): Error | undefined }} {{", self.spaces(level));
        body = format!("{}\n{}if (array) {{", body, self.spaces(level + 1));
        body = format!(
            "{}\n{}return {{ validate(obj: any): Error | undefined {{",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}\n{}if (!(obj instanceof Array)) {{",
            body,
            self.spaces(level + 3)
        );
        body = format!(
            "{}\n{}return new Error(`Expecting Array<{}>`);",
            body,
            self.spaces(level + 4),
            strct.name
        );
        body = format!("{}\n{}}}", body, self.spaces(level + 3));
        body = format!("{}\n{}try {{", body, self.spaces(level + 3));
        body = format!(
            "{}\n{}obj.forEach((o, index: number) => {{",
            body,
            self.spaces(level + 4)
        );
        body = format!(
            "{}\n{}if (!(o instanceof {})) {{",
            body,
            self.spaces(level + 5),
            strct.name
        );
        body = format!(
            "{}\n{}throw new Error(`Expecting instance of {} on index #${{index}}`);",
            body,
            self.spaces(level + 6),
            strct.name
        );
        body = format!("{}\n{}}}", body, self.spaces(level + 5));
        body = format!("{}\n{}}});", body, self.spaces(level + 4));
        body = format!("{}\n{}}} catch (e) {{", body, self.spaces(level + 3));
        body = format!("{}\n{}return e;", body, self.spaces(level + 4));
        body = format!("{}\n{}}}", body, self.spaces(level + 3));
        body = format!("{}\n{}}}}};", body, self.spaces(level + 2));
        body = format!("{}\n{}}} else {{", body, self.spaces(level + 1));
        body = format!(
            "{}\n{}return {{ validate(obj: any): Error | undefined {{",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}\n{}return obj instanceof {} ? undefined : new Error(`Expecting instance of {}`);",
            body,
            self.spaces(level + 3),
            strct.name,
            strct.name
        );
        body = format!("{}\n{}}}}};", body, self.spaces(level + 2));
        body = format!("{}\n{}}}", body, self.spaces(level + 1));
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body
    }

    fn struct_from(&self, strct: &Struct, level: u8) -> String {
        let mut body = format!(
            "{}public static from(obj: any): {} | Error {{",
            self.spaces(level),
            strct.name
        );
        body = format!("{}\n{}if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {{", body, self.spaces(level + 1));
        body = format!(
            "{}\n{}const inst = {}.defaults();",
            body,
            self.spaces(level + 2),
            strct.name
        );
        body = format!(
            "{}\n{}const err = inst.decode(obj);",
            body,
            self.spaces(level + 2)
        );
        body = format!(
            "{}\n{}return err instanceof Error ? err : inst;",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}\n{}}} else {{", body, self.spaces(level + 1));
        body = format!(
            "{}\n{}const error: Error | undefined = Protocol.validate(obj, {}.scheme);",
            body,
            self.spaces(level + 2),
            strct.name
        );
        body = format!(
            "{}\n{}return error instanceof Error ? error : new {}({{",
            body,
            self.spaces(level + 2),
            strct.name
        );
        for field in &strct.fields {
            body = format!(
                "{}\n{}{}: obj.{},",
                body,
                self.spaces(level + 3),
                field.name,
                field.name
            );
        }
        body = format!("{}\n{}}});", body, self.spaces(level + 2));
        body = format!("{}\n{}}}", body, self.spaces(level + 1));
        body = format!("{}\n{}}}\n", body, self.spaces(level));
        body
    }

    fn enums(&self, enums: &Enum, store: &mut Store, level: u8) -> String {
        let mut body = format!(
            "{}export interface I{} {{\n",
            self.spaces(level),
            enums.name
        );
        for variant in &enums.variants {
            let variant_type = if let Some(prim_type_ref) = variant.types.clone() {
                self.etype_ts(prim_type_ref.clone(), variant.repeated)
            } else if let Some(ref_type_id) = variant.ref_type_id {
                if let Some(strct) = store.get_struct(ref_type_id) {
                    store.get_struct_path(strct.id).join(".")
                } else {
                    stop!(
                        "Unknown type of data in scope of enum {} / {}, ref_type_id: {}",
                        enums.name, variant.name, ref_type_id
                    );
                }
            } else {
                stop!(
                    "Unknown type of data in scope of enum {} / {} ",
                    enums.name, variant.name
                );
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
        body = format!("{}\n", body);
        body = format!(
            "{}{}export class {} extends Protocol.Primitives.Enum<I{}> {{\n",
            body,
            self.spaces(level),
            enums.name,
            enums.name
        );
        body = format!(
            "{}{}public static from(obj: any): I{} | Error {{\n",
            body,
            self.spaces(level + 1),
            enums.name
        );
        body = format!(
            "{}{}const inst = new {}();\n",
            body,
            self.spaces(level + 2),
            enums.name
        );
        body = format!(
            "{}{}let err: Error | undefined;\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {{\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}err = inst.decode(obj);\n",
            body,
            self.spaces(level + 3)
        );
        body = format!("{}{}}} else {{\n", body, self.spaces(level + 2));
        body = format!("{}{}err = inst.set(obj);\n", body, self.spaces(level + 3));
        body = format!("{}{}}}\n", body, self.spaces(level + 2));
        body = format!(
            "{}{}return err instanceof Error ? err : inst.get();\n",
            body,
            self.spaces(level + 2)
        );
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}public from(obj: any): I{} | Error {{\n",
            body,
            self.spaces(level + 1),
            enums.name
        );
        body = format!(
            "{}{}return {}.from(obj);\n",
            body,
            self.spaces(level + 2),
            enums.name
        );
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}public signature(): number {{ return {}; }}\n",
            body,
            self.spaces(level + 1),
            self.signature
        );
        body = format!(
            "{}{}public getId(): number {{ return {}; }}\n",
            body,
            self.spaces(level + 1),
            enums.id
        );
        body = format!(
            "{}{}public getAllowed(): string[] {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!(
            "{}{}return {};\n",
            body,
            self.spaces(level + 2),
            self.enum_declaration(enums, store, level + 3)
        );
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}public getOptionValue(id: number): ISigned<any> {{\n",
            body,
            self.spaces(level + 1)
        );
        body = format!("{}{}\n", body, self.enum_getter(enums, store, level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}public get(): I{} {{\n",
            body,
            self.spaces(level + 1),
            enums.name
        );
        body = format!(
            "{}{}const target: I{} = {{}};\n",
            body,
            self.spaces(level + 2),
            enums.name
        );
        body = format!(
            "{}{}\n",
            body,
            self.get_enum_decode(enums, store, level + 2)
        );
        body = format!("{}{}return target;\n", body, self.spaces(level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!(
            "{}{}public set(src: I{}): Error | undefined{{\n",
            body,
            self.spaces(level + 1),
            enums.name
        );
        body = format!("{}{}\n", body, self.enum_setter(enums, store, level + 2));
        body = format!("{}{}}}\n", body, self.spaces(level + 1));
        body = format!("{}{}}}\n", body, self.spaces(level));
        body
    }

    fn etype(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => {
                if repeated {
                    "ArrayI8"
                } else {
                    "i8"
                }
            }
            PrimitiveTypes::ETypes::Ei16 => {
                if repeated {
                    "ArrayI16"
                } else {
                    "i16"
                }
            }
            PrimitiveTypes::ETypes::Ei32 => {
                if repeated {
                    "ArrayI32"
                } else {
                    "i32"
                }
            }
            PrimitiveTypes::ETypes::Ei64 => {
                if repeated {
                    "ArrayI64"
                } else {
                    "i64"
                }
            }
            PrimitiveTypes::ETypes::Eu8 => {
                if repeated {
                    "ArrayU8"
                } else {
                    "u8"
                }
            }
            PrimitiveTypes::ETypes::Eu16 => {
                if repeated {
                    "ArrayU16"
                } else {
                    "u16"
                }
            }
            PrimitiveTypes::ETypes::Eu32 => {
                if repeated {
                    "ArrayU32"
                } else {
                    "u32"
                }
            }
            PrimitiveTypes::ETypes::Eu64 => {
                if repeated {
                    "ArrayU64"
                } else {
                    "u64"
                }
            }
            PrimitiveTypes::ETypes::Ef32 => {
                if repeated {
                    "ArrayF32"
                } else {
                    "f32"
                }
            }
            PrimitiveTypes::ETypes::Ef64 => {
                if repeated {
                    "ArrayF64"
                } else {
                    "f64"
                }
            }
            PrimitiveTypes::ETypes::Ebool => {
                if repeated {
                    "ArrayBool"
                } else {
                    "bool"
                }
            }
            PrimitiveTypes::ETypes::Estr => {
                if repeated {
                    "ArrayStrUTF8"
                } else {
                    "StrUTF8"
                }
            }
            _ => {
                stop!("Unknown type ref {:?}", etype);
            }
        }
        .to_string()
    }

    fn etype_def(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Ei16 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Ei32 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Ei64 => {
                if repeated {
                    "[BigInt(0)]"
                } else {
                    "BigInt(0)"
                }
            }
            PrimitiveTypes::ETypes::Eu8 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Eu16 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Eu32 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Eu64 => {
                if repeated {
                    "[BigInt(0)]"
                } else {
                    "BigInt(0)"
                }
            }
            PrimitiveTypes::ETypes::Ef32 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Ef64 => {
                if repeated {
                    "[0]"
                } else {
                    "0"
                }
            }
            PrimitiveTypes::ETypes::Ebool => {
                if repeated {
                    "[true]"
                } else {
                    "true"
                }
            }
            PrimitiveTypes::ETypes::Estr => {
                if repeated {
                    "['']"
                } else {
                    "''"
                }
            }
            _ => {
                stop!("Unknown type ref {:?}", etype);
            }
        }
        .to_string()
    }

    fn etype_ts(&self, etype: PrimitiveTypes::ETypes, repeated: bool) -> String {
        match etype {
            PrimitiveTypes::ETypes::Ei8 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Ei16 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Ei32 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Ei64 => {
                if repeated {
                    "Array<bigint>"
                } else {
                    "bigint"
                }
            }
            PrimitiveTypes::ETypes::Eu8 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Eu16 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Eu32 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Eu64 => {
                if repeated {
                    "Array<bigint>"
                } else {
                    "bigint"
                }
            }
            PrimitiveTypes::ETypes::Ef32 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Ef64 => {
                if repeated {
                    "Array<number>"
                } else {
                    "number"
                }
            }
            PrimitiveTypes::ETypes::Ebool => {
                if repeated {
                    "Array<boolean>"
                } else {
                    "boolean"
                }
            }
            PrimitiveTypes::ETypes::Estr => {
                if repeated {
                    "Array<string>"
                } else {
                    "string"
                }
            }
            _ => {
                stop!("Unknown type ref {:?}", etype);
            }
        }
        .to_string()
    }

    fn entity_default(&self, entity_id: usize, store: &mut Store, level: u8) -> String {
        if let Some(strct) = store.get_struct(entity_id) {
            let mut body = format!("new {}({{", store.get_struct_path(entity_id).join("."));
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
            stop!("Fail to find a struct/enum id: {}", entity_id);
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

    fn get_field_map_def(&self, field: &Field, store: &mut Store, level: u8) -> String {
        let mut body: String = String::from("");
        if let Some(entity_id) = field.ref_type_id {
            if let Some(strct) = store.get_struct(entity_id) {
                body = format!(
                    "{}\n{}{{ prop: '{}', types: {}.getValidator({}), optional: {} }},",
                    body,
                    self.spaces(level),
                    field.name,
                    store.get_struct_path(strct.id).join("."),
                    if field.repeated { "true" } else { "false" },
                    if field.optional { "true" } else { "false" }
                );
            } else if let Some(enums) = store.get_enum(entity_id) {
                body = format!(
                    "{}\n{}{{ prop: '{}', optional: {}, options: [",
                    body,
                    self.spaces(level),
                    field.name,
                    if field.optional { "true" } else { "false" }
                );
                for variant in &enums.variants {
                    if let Some(struct_id) = variant.ref_type_id {
                        if let Some(strct) = store.get_struct(struct_id) {
                            body = format!("{}\n{}{{ prop: '{}', types: {}.getValidator({}), optional: false }},", body, self.spaces(level + 1), variant.name, strct.name, if variant.repeated { "true" } else { "false" });
                        } else {
                            stop!("Nested enums aren't supported.");
                        }
                    } else if let Some(etype) = variant.types.clone() {
                        body = format!("{}\n{}{{ prop: '{}', types: Protocol.Primitives.{}, optional: false, }},", body, self.spaces(level + 1), variant.name, self.etype(etype, variant.repeated));
                    } else {
                        stop!("Incorrect option definition for enum {}", enums.name);
                    }
                }
                body = format!("{}\n{}] }},", body, self.spaces(level));
            }
        } else {
            body = format!(
                "{}\n{}{{ prop: '{}', types: Protocol.Primitives.{}, optional: {}, }},",
                body,
                self.spaces(level),
                field.name,
                self.get_primitive_ref(field),
                if field.optional { "true" } else { "false" }
            );
        }
        body
    }

    fn get_field_decode_wrap(&self, field: &Field, store: &mut Store, level: u8) -> String {
        if field.optional {
            let mut body = format!(
                "{}const {}Buf: ArrayBufferLike | undefined = storage.get({});",
                self.spaces(level),
                field.name,
                field.id
            );
            body = format!(
                "{}\n{}if ({}Buf === undefined) {{",
                body,
                self.spaces(level),
                field.name
            );
            body = format!(
                "{}\n{}return new Error(`Fail to get property {}`);",
                body,
                self.spaces(level + 1),
                field.name
            );
            body = format!("{}\n{}}}", body, self.spaces(level));
            body = format!(
                "{}\n{}if ({}Buf.byteLength === 0) {{",
                body,
                self.spaces(level),
                field.name
            );
            body = format!(
                "{}\n{}this.{} = undefined;",
                body,
                self.spaces(level + 1),
                field.name
            );
            body = format!("{}\n{}}} else {{", body, self.spaces(level));
            body = format!(
                "{}\n{}",
                body,
                self.get_field_decode(field, store, level + 1)
            );
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
                    body = format!(
                        "{}const arr{}Inst: {} = {}.defaults();",
                        self.spaces(level),
                        strct.name,
                        store.get_struct_path(strct.id).join("."),
                        store.get_struct_path(strct.id).join(".")
                    );
                    body = format!("{}\n{}const arr{}: Array<any> | Error = this.getValue<{}[]>(storage, {}, arr{}Inst.decodeSelfArray.bind(arr{}Inst));", body, self.spaces(level), strct.name, strct.name, field.id, strct.name, strct.name);
                    body = format!(
                        "{}\n{}if (arr{} instanceof Error) {{",
                        body,
                        self.spaces(level),
                        strct.name
                    );
                    body = format!(
                        "{}\n{}return arr{};",
                        body,
                        self.spaces(level + 1),
                        strct.name
                    );
                    body = format!("{}\n{}}} else {{", body, self.spaces(level));
                    body = format!(
                        "{}\n{}this.{} = arr{} as {}[];",
                        body,
                        self.spaces(level + 1),
                        field.name,
                        strct.name,
                        store.get_struct_path(strct.id).join(".")
                    );
                    body = format!("{}\n{}}}", body, self.spaces(level));
                } else {
                    body = format!(
                        "{}const {}: {} = {};",
                        self.spaces(level),
                        field.name,
                        field.get_full_name().join("."),
                        self.entity_default(entity_id, &mut store.clone(), level)
                    );
                    body = format!(
                        "{}\n{}const {}Buf: ArrayBufferLike = storage.get({});",
                        body,
                        self.spaces(level),
                        field.name,
                        field.id
                    );
                    body = format!(
                        "{}\n{}if ({}Buf instanceof Error) {{",
                        body,
                        self.spaces(level),
                        field.name
                    );
                    body = format!(
                        "{}\n{}return {}Buf;",
                        body,
                        self.spaces(level + 1),
                        field.name
                    );
                    body = format!("{}\n{}}}", body, self.spaces(level));
                    body = format!(
                        "{}\n{}const {}Err: Error | undefined = {}.decode({}Buf);",
                        body,
                        self.spaces(level),
                        field.name,
                        field.name,
                        field.name
                    );
                    body = format!(
                        "{}\n{}if ({}Err instanceof Error) {{",
                        body,
                        self.spaces(level),
                        field.name
                    );
                    body = format!(
                        "{}\n{}return {}Err;",
                        body,
                        self.spaces(level + 1),
                        field.name
                    );
                    body = format!("{}\n{}}} else {{", body, self.spaces(level));
                    body = format!(
                        "{}\n{}this.{} = {};",
                        body,
                        self.spaces(level + 1),
                        field.name,
                        field.name
                    );
                    body = format!("{}\n{}}}", body, self.spaces(level));
                }
            } else if let Some(_enums) = store.get_enum(entity_id) {
                body = format!("{}this.{} = {{}};", self.spaces(level), field.name);
                body = format!(
                    "{}\n{}const {}Buf: ArrayBufferLike = storage.get({});",
                    body,
                    self.spaces(level),
                    field.name,
                    field.id
                );
                body = format!(
                    "{}\n{}if ({}Buf === undefined) {{",
                    body,
                    self.spaces(level),
                    field.name
                );
                body = format!(
                    "{}\n{}return new Error(`Fail to get property \"{}\"`);",
                    body,
                    self.spaces(level + 1),
                    field.name
                );
                body = format!("{}\n{}}}", body, self.spaces(level));
                body = format!(
                    "{}\n{}if ({}Buf.byteLength > 0) {{",
                    body,
                    self.spaces(level),
                    field.name
                );
                body = format!(
                    "{}\n{}const {}Err: Error | undefined = this._{}.decode({}Buf);",
                    body,
                    self.spaces(level + 1),
                    field.name,
                    field.name,
                    field.name
                );
                body = format!(
                    "{}\n{}if ({}Err instanceof Error) {{",
                    body,
                    self.spaces(level + 1),
                    field.name
                );
                body = format!(
                    "{}\n{}return {}Err;",
                    body,
                    self.spaces(level + 2),
                    field.name
                );
                body = format!("{}\n{}}} else {{", body, self.spaces(level + 1));
                body = format!(
                    "{}\n{}this.{} = this._{}.get();",
                    body,
                    self.spaces(level + 2),
                    field.name,
                    field.name
                );
                body = format!("{}\n{}}}", body, self.spaces(level + 1));
                body = format!("{}\n{}}}", body, self.spaces(level));
            } else {
                stop!(
                    "Fail to find a type by ref {} for field {}",
                    entity_id, field.name
                );
            }
        } else {
            let mut type_str = self.get_type_ref(field, &mut store.clone());
            let primitive = self.get_primitive_ref(field);
            if field.repeated {
                type_str = format!("Array<{}>", type_str);
            }
            body = format!("{}const {}: {} | Error = this.getValue<{}>(storage, {}, Protocol.Primitives.{}.decode);", self.spaces(level), field.name, type_str, type_str, field.id, primitive);
            body = format!(
                "{}\n{}if ({} instanceof Error) {{",
                body,
                self.spaces(level),
                field.name
            );
            body = format!("{}\n{}return {};", body, self.spaces(level + 1), field.name);
            body = format!("{}\n{}}} else {{", body, self.spaces(level));
            body = format!(
                "{}\n{}this.{} = {};",
                body,
                self.spaces(level + 1),
                field.name,
                field.name
            );
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
                stop!(
                    "Fail to find a type by ref {} for field {}",
                    entity_id, field.name
                );
            }
        } else {
            let type_str = self.get_type_ref(field, &mut store.clone());
            let size_ref = self.get_size_ref(field);
            let primitive = self.get_primitive_ref(field);
            if field.repeated {
                body = format!("this.getBufferFromBuf<Array<{}>>({}, Protocol.ESize.u64, Protocol.Primitives.{}.encode, this.{})", type_str, field.id, primitive, field.name);
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
            "i64" => Some("BigInt(0)"),
            "u8" => Some("0"),
            "u16" => Some("0"),
            "u32" => Some("0"),
            "u64" => Some("BigInt(0)"),
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
                    if let Some(_) = store.get_struct(ref_type_id) {
                        store.get_struct_path(ref_type_id).join(".")
                    } else if let Some(enums) = store.get_enum(ref_type_id) {
                        format!("I{}", enums.name)
                    } else {
                        stop!(
                            "Fail to find a struct/enum id: {} for field {}",
                            ref_type_id, field.name
                        );
                    }
                } else {
                    stop!("Invalid type definition for field {}", field.name);
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
                _ => stop!("{} type isn't recognized", field.kind),
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
                _ => stop!("{} type isn't recognized", field.kind),
            }
        }
    }

    fn get_messages_list(&self, group: Option<&Group>, store: &mut Store, level: u8) -> String {
        let mut body = String::from("");
        if let Some(group) = group {
            body = format!(
                "{}{}export interface IAvailableMessages {{\n",
                body,
                self.spaces(level)
            );
            for enum_id in &group.enums {
                if let Some(enums) = store.get_enum(*enum_id) {
                    body = format!(
                        "{}{}{}?: I{},\n",
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
                        "{}{}{}?: {},\n",
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
                        "{}{}{}?: {}.IAvailableMessages,\n",
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
                "{}{}export interface IAvailableMessages {{\n",
                body,
                self.spaces(level)
            );
            for enums in &store.enums {
                if enums.parent == 0 {
                    body = format!(
                        "{}{}{}?: I{},\n",
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
                        "{}{}{}?: {},\n",
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
                        "{}{}{}?: {}.IAvailableMessages,\n",
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
            format!("{}.{}", path.join("."), name)
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
        //GroupB: { GroupC: { StructExampleB: instance } }
        let path = self.get_entity_path(parent, store);
        if path.is_empty() {
            result = format!("{}: instance ", name);
        } else {
            for part in path.iter() {
                result = format!("{}{}: {{ ", result, part);
            }
            result = format!("{}{}: instance {}", result, name, "} ".repeat(path.len()));
        }
        result
    }

    fn buffer(&self, store: &mut Store) -> String {
        let mut body = format!(
            "{}export class BufferReaderMessages extends BufferReader<IAvailableMessage<IAvailableMessages>> {{\n",
            self.spaces(0)
        );
        body = format!(
            "{}{}public signature(): number {{ return {}; }}\n",
            body,
            self.spaces(1),
            self.signature
        );
        body = format!("{}{}public getMessage(header: MessageHeader, buffer: Buffer | ArrayBuffer | ArrayBufferLike): IAvailableMessage<IAvailableMessages> | Error {{\n", body, self.spaces(1));
        body = format!("{}{}let instance: any;\n", body, self.spaces(2));
        body = format!("{}{}let enum_instance: any = {{}};\n", body, self.spaces(2));
        body = format!("{}{}let err: Error | undefined;\n", body, self.spaces(2));
        body = format!("{}{}switch (header.id) {{\n", body, self.spaces(2));
        for enums in &store.enums {
            body = format!("{}{}case {}:\n", body, self.spaces(3), enums.id);
            body = format!(
                "{}{}instance = new {}();\n",
                body,
                self.spaces(4),
                store.get_enum_path(enums.id).join(".")
            );
            body = format!(
                "{}{}if (instance.decode(buffer) instanceof Error) {{ return err; }}\n",
                body,
                self.spaces(4)
            );
            body = format!(
                "{}{}enum_instance = instance.get();\n",
                body,
                self.spaces(4)
            );
            body = format!("{}{}instance = enum_instance;\n", body, self.spaces(4));
            body = format!("{}{}return {{ header: {{ id: header.id, timestamp: header.ts }}, msg: {{ {}}} }};\n", body, self.spaces(4), self.get_available_entity(enums.parent, &enums.name, &mut store.clone()));
        }
        for structs in &store.structs {
            body = format!("{}{}case {}:\n", body, self.spaces(3), structs.id);
            body = format!(
                "{}{}instance = {}.defaults();\n",
                body,
                self.spaces(4),
                store.get_struct_path(structs.id).join(".")
            );
            body = format!("{}{}err = instance.decode(buffer);\n", body, self.spaces(4));
            body = format!("{}{}return err instanceof Error ? err : {{ header: {{ id: header.id, timestamp: header.ts }}, msg: {{ {}}} }};\n", body, self.spaces(4), self.get_available_entity(structs.parent, &structs.name, &mut store.clone()));
        }
        body = format!("{}{}}}\n", body, self.spaces(2));
        body = format!("{}{}}}\n", body, self.spaces(1));
        body = format!("{}{}}}\n", body, self.spaces(0));
        body
    }

    fn includes(&self) -> String {
        if self.embedded {
            format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.injection.embedded.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/tools/index.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/tools/tools.arraybuffer.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.sizes.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.interface.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.u8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.u16.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.u32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.u64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.i8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.i16.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.i32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.i64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.f32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.f64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.bool.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.string.utf8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.u8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.u16.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.u32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.u64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.i8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.i16.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.i32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.i64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.f32.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.f64.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.bool.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.array.string.utf8.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.enum.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.validator.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.primitives.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.convertor.storage.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/protocol.convertor.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/packing.header.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/packing.ts")),
                self.get_injectable(include_str!("../../../protocol/implementations/typescript/src/index.ts")),
            )
        } else {
            include_str!("../../../protocol/implementations/typescript/src/protocol.injection.ts")
                .to_string()
        }
    }

    fn get_injectable(&self, content: &str) -> String {
        let re_injectable = Regex::new(r"^([\n\r]|.)*(//\s?injectable)").unwrap();
        re_injectable.replace_all(content, "").to_string()
    }

    fn spaces(&self, level: u8) -> String {
        "    ".repeat(level as usize)
    }
}

impl Render for TypescriptRender {
    fn new(embedded: bool, signature: u16) -> Self {
        TypescriptRender {
            embedded,
            signature,
        }
    }

    fn render(&self, store: Store) -> String {
        let mut body = format!("{}\n", self.includes());
        body = format!(
            "{}{}",
            body,
            self.get_messages_list(None, &mut store.clone(), 0)
        );
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
        body = format!("{}{}\n", body, self.buffer(&mut store.clone()));
        body
    }
}
