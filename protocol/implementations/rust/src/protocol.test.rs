#[allow(unused_imports)]
use super::*;

#[cfg(test)]
mod tests { 
    use super::*;
    use encode::{ StructEncode, EnumEncode, Encode, EncodeEnum, get_empty_buffer_val };
    use decode::{ StructDecode, EnumDecode, Decode, DecodeEnum };
    use storage::{ Storage };
    use std::io::Cursor;
    use bytes::{ Buf };

    #[derive(Debug, Clone, PartialEq)]
    pub enum TargetEnum {
        OptionString(String),
        Optionu8(u8),
        Optionu16(u16),
        Optionu32(u32),
        Optionu64(u64),
        Optioni8(i8),
        Optioni16(i16),
        Optioni32(i32),
        Optioni64(i64),
        Optionf32(f32),
        Optionf64(f64),
        OptionBool(bool),
        OptionStruct(Nested),
        Optionu8Vec(Vec<u8>),
        Optionu16Vec(Vec<u16>),
        Optionu32Vec(Vec<u32>),
        Optionu64Vec(Vec<u64>),
        Optioni8Vec(Vec<i8>),
        Optioni16Vec(Vec<i16>),
        Optioni32Vec(Vec<i32>),
        Optioni64Vec(Vec<i64>),
        Optionf32Vec(Vec<f32>),
        Optionf64Vec(Vec<f64>),
        OptionStructVec(Vec<Nested>),
        Defaults,
    }

    impl EnumEncode for TargetEnum {

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            match match self {
                Self::OptionString(v) => v.encode(1),
                Self::Optionu8(v) => v.encode(2),
                Self::Optionu16(v) => v.encode(3),
                Self::Optionu32(v) => v.encode(4),
                Self::Optionu64(v) => v.encode(5),
                Self::Optioni8(v) => v.encode(6),
                Self::Optioni16(v) => v.encode(7),
                Self::Optioni32(v) => v.encode(8),
                Self::Optioni64(v) => v.encode(9),
                Self::Optionf32(v) => v.encode(10),
                Self::Optionf64(v) => v.encode(11),
                Self::OptionBool(v) => v.encode(12),
                Self::OptionStruct(v) => v.encode(13),
                Self::Optionu8Vec(v) => v.encode(14),
                Self::Optionu16Vec(v) => v.encode(15),
                Self::Optionu32Vec(v) => v.encode(16),
                Self::Optionu64Vec(v) => v.encode(17),
                Self::Optioni8Vec(v) => v.encode(18),
                Self::Optioni16Vec(v) => v.encode(19),
                Self::Optioni32Vec(v) => v.encode(20),
                Self::Optioni64Vec(v) => v.encode(21),
                Self::Optionf32Vec(v) => v.encode(22),
                Self::Optionf64Vec(v) => v.encode(23),
                Self::OptionStructVec(v) => v.encode(24),
                _ => Err(String::from("Not supportable option")),
            } {
                Ok(buf) => Ok(buf),
                Err(e) => Err(e),
            }
        }

    }

    impl EnumDecode for TargetEnum {

        fn extract(buf: Vec<u8>) -> Result<TargetEnum, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for TargetEnum because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let id = cursor.get_u16_le();
            let mut storage = match Storage::new(buf) {
                Ok(s) => s,
                Err(e) => { return Err(e); }
            };
            match id {
                1 => match String::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::OptionString(v)),
                    Err(e) => Err(e),
                },
                2 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu8(v)),
                    Err(e) => Err(e),
                },
                3 => match u16::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu16(v)),
                    Err(e) => Err(e),
                },
                4 => match u32::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu32(v)),
                    Err(e) => Err(e),
                },
                5 => match u64::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu64(v)),
                    Err(e) => Err(e),
                },
                6 => match i8::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni8(v)),
                    Err(e) => Err(e),
                },
                7 => match i16::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni16(v)),
                    Err(e) => Err(e),
                },
                8 => match i32::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni32(v)),
                    Err(e) => Err(e),
                },
                9 => match i64::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni64(v)),
                    Err(e) => Err(e),
                },
                10 => match f32::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionf32(v)),
                    Err(e) => Err(e),
                },
                11 => match f64::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionf64(v)),
                    Err(e) => Err(e),
                },
                12 => match bool::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::OptionBool(v)),
                    Err(e) => Err(e),
                },
                13 => match Nested::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::OptionStruct(v)),
                    Err(e) => Err(e),
                },
                14 => match Vec::<u8>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu8Vec(v)),
                    Err(e) => Err(e),
                },
                15 => match Vec::<u16>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu16Vec(v)),
                    Err(e) => Err(e),
                },
                16 => match Vec::<u32>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu32Vec(v)),
                    Err(e) => Err(e),
                },
                17 => match Vec::<u64>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionu64Vec(v)),
                    Err(e) => Err(e),
                },
                18 => match Vec::<i8>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni8Vec(v)),
                    Err(e) => Err(e),
                },
                19 => match Vec::<i16>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni16Vec(v)),
                    Err(e) => Err(e),
                },
                20 => match Vec::<i32>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni32Vec(v)),
                    Err(e) => Err(e),
                },
                21 => match Vec::<i64>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optioni64Vec(v)),
                    Err(e) => Err(e),
                },
                22 => match Vec::<f32>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionf32Vec(v)),
                    Err(e) => Err(e),
                },
                23 => match Vec::<f64>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::Optionf64Vec(v)),
                    Err(e) => Err(e),
                },
                24 => match Vec::<Nested>::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::OptionStructVec(v)),
                    Err(e) => Err(e),
                },
                _ => Err(String::from("Fail to find relevant value for TargetEnum"))
            }
        }

    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Nested {
        field_u16: u16,
        field_utf8_string: String,
        field_optional: Option<u8>,
    }

    impl StructDecode for Nested {

        fn get_id() -> u32 {
            1
        }

        fn defaults() -> Nested {
            Nested {
                field_u16: 0,
                field_utf8_string: String::from(""),
                field_optional: None,
            }
        }

        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u16 = match u16::decode(&mut storage, 1) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_utf8_string = match String::decode(&mut storage, 2) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_optional = match Option::<u8>::decode(&mut storage, 3) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }

    impl StructEncode for Nested {

        fn get_id(&self) -> u32 {
            1
        }

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u16.encode(1) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.field_utf8_string.encode(2) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.field_optional.encode(3) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            Ok(buffer)
        }
    }

    #[derive(Debug, Clone)]
    struct Target {
        pub prop_u8: u8,
        pub prop_u16: u16,
        pub prop_u32: u32,
        pub prop_u64: u64,
        pub prop_i8: i8,
        pub prop_i16: i16,
        pub prop_i32: i32,
        pub prop_i64: i64,
        pub prop_u8_vec: Vec<u8>,
        pub prop_u16_vec: Vec<u16>,
        pub prop_u32_vec: Vec<u32>,
        pub prop_u64_vec: Vec<u64>,
        pub prop_i8_vec: Vec<i8>,
        pub prop_i16_vec: Vec<i16>,
        pub prop_i32_vec: Vec<i32>,
        pub prop_i64_vec: Vec<i64>,
        pub prop_f32_vec: Vec<f32>,
        pub prop_f64_vec: Vec<f64>,
        pub prop_string: String,
        pub prop_f32: f32,
        pub prop_f64: f64,
        pub prop_utf8_string_vec: Vec<String>,
        pub prop_nested: Nested,
        pub prop_nested_vec: Vec<Nested>,
        pub prop_enum: TargetEnum,
        pub prop_optional_strct: Option<Nested>,
        pub prop_optional_enum: Option<TargetEnum>,
        pub prop_enum_vec: Vec<TargetEnum>,
        pub prop_optional_enum_vec: Option<Vec<TargetEnum>>,
    }

    impl StructDecode for Target {
        fn get_id() -> u32 {
            2
        }
        fn defaults() -> Target {
            Target {
                prop_u8: 0,
                prop_u16: 0,
                prop_u32: 0,
                prop_u64: 0,
                prop_i8: 0,
                prop_i16: 0,
                prop_i32: 0,
                prop_i64: 0,
                prop_u8_vec: vec![],
                prop_u16_vec: vec![],
                prop_u32_vec: vec![],
                prop_u64_vec: vec![],
                prop_i8_vec: vec![],
                prop_i16_vec: vec![],
                prop_i32_vec: vec![],
                prop_i64_vec: vec![],
                prop_f32_vec: vec![],
                prop_f64_vec: vec![],
                prop_string: String::from(""),
                prop_f32: 0.0,
                prop_f64: 0.0,
                prop_utf8_string_vec: vec![],
                prop_nested: Nested::defaults(),
                prop_nested_vec: vec![],
                prop_enum: TargetEnum::Defaults,
                prop_optional_strct: None,
                prop_optional_enum: None,
                prop_enum_vec: vec![],
                prop_optional_enum_vec: None,
            }
        }
        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.prop_u8 = match u8::decode(&mut storage, 1) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16 = match u16::decode(&mut storage, 2) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32 = match u32::decode(&mut storage, 3) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64 = match u64::decode(&mut storage, 4) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8 = match i8::decode(&mut storage, 5) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16 = match i16::decode(&mut storage, 6) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32 = match i32::decode(&mut storage, 7) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64 = match i64::decode(&mut storage, 8) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u8_vec = match Vec::<u8>::decode(&mut storage, 9) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16_vec = match Vec::<u16>::decode(&mut storage, 10) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32_vec = match Vec::<u32>::decode(&mut storage, 11) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64_vec = match Vec::<u64>::decode(&mut storage, 12) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32_vec = match Vec::<f32>::decode(&mut storage, 13) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64_vec = match Vec::<f64>::decode(&mut storage, 14) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8_vec = match Vec::<i8>::decode(&mut storage, 15) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16_vec = match Vec::<i16>::decode(&mut storage, 16) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32_vec = match Vec::<i32>::decode(&mut storage, 17) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64_vec = match Vec::<i64>::decode(&mut storage, 18) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_string = match String::decode(&mut storage, 19) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32 = match f32::decode(&mut storage, 20) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64 = match f64::decode(&mut storage, 21) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_utf8_string_vec = match Vec::<String>::decode(&mut storage, 22) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_nested = match <Nested as Decode<Nested>>::decode(&mut storage, 23) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_nested_vec = match Vec::<Nested>::decode(&mut storage, 24) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_enum = match TargetEnum::decode(&mut storage, 25) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_optional_strct = match Option::<Nested>::decode(&mut storage, 26) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            if let Some(buf) = storage.get(27) {
                if buf.is_empty() {
                    self.prop_optional_enum = None;
                } else {
                    self.prop_optional_enum = match TargetEnum::decode(&mut storage, 27) {
                        Ok(val) => Some(val),
                        Err(e) => { return Err(e) },
                    };
                }
            } else {
                return Err("Buffer for property prop_optional_enum isn\'t found".to_string())
            }
            self.prop_enum_vec = match Vec::<TargetEnum>::decode(&mut storage, 28) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            if let Some(buf) = storage.get(29) {
                if buf.is_empty() {
                    self.prop_optional_enum_vec = None;
                } else {
                    self.prop_optional_enum_vec = match Vec::<TargetEnum>::decode(&mut storage, 29) {
                        Ok(val) => Some(val),
                        Err(e) => { return Err(e) },
                    };
                }
            } else {
                return Err("Buffer for property prop_optional_enum_vec isn\'t found".to_string())
            }
            Ok(())
        }
    }

    impl StructEncode for Target {

        fn get_id(&self) -> u32 {
            2
        }

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.prop_u8.encode(1) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16.encode(2) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32.encode(3) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64.encode(4) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8.encode(5) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16.encode(6) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32.encode(7) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64.encode(8) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u8_vec.encode(9) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16_vec.encode(10) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32_vec.encode(11) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64_vec.encode(12) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32_vec.encode(13) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64_vec.encode(14) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8_vec.encode(15) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16_vec.encode(16) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32_vec.encode(17) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64_vec.encode(18) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_string.encode(19) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32.encode(20) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64.encode(21) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_utf8_string_vec.encode(22) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match Encode::encode(&mut self.prop_nested, 23) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_nested_vec.encode(24) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_enum.encode(25) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_optional_strct.encode(26) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            if let Some(mut val) = self.prop_optional_enum.clone() {
                match val.encode(27) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            } else {
                match get_empty_buffer_val(27) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            }
            match self.prop_enum_vec.encode(28) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            if let Some(mut val) = self.prop_optional_enum_vec.clone() {
                match val.encode(29) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            } else {
                match get_empty_buffer_val(29) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            }
            Ok(buffer)
        }

    }

    #[test]
    fn encode_decode() {
        let mut a: Target = Target {
            prop_u8: 1,
            prop_u16: 2,
            prop_u32: 3,
            prop_u64: 4,
            prop_i8: -1,
            prop_i16: -2,
            prop_i32: -3,
            prop_i64: -4,
            prop_u8_vec: vec![0, 1, 2, 3, 4],
            prop_u16_vec: vec![5, 6, 7, 8, 9],
            prop_u32_vec: vec![10, 11, 12, 13, 14],
            prop_u64_vec: vec![15, 16, 17, 18, 19],
            prop_i8_vec: vec![0, -1, -2, -3, -4],
            prop_i16_vec: vec![-5, -6, -7, -8, -9],
            prop_i32_vec: vec![-10, -11, -12, -13, -14],
            prop_i64_vec: vec![-15, -16, -17, -18, -19],
            prop_f32_vec: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            prop_f64_vec: vec![0.6, 0.7, 0.8, 0.9],
            prop_string: String::from("Hello, World!"),
            prop_f32: 0.1,
            prop_f64: 0.00002,
            prop_utf8_string_vec: vec![String::from("UTF8 String 1"), String::from("UTF8 String 2")],
            prop_nested: Nested {
                field_u16: 999,
                field_utf8_string: String::from("Hello, from Nested!"),
                field_optional: Some(2),
            },
            prop_nested_vec: vec![
                Nested {
                    field_u16: 333,
                    field_utf8_string: String::from("Hello, from Nested (333)!"),
                    field_optional: None,
                },
                Nested {
                    field_u16: 444,
                    field_utf8_string: String::from("Hello, from Nested (444)!"),
                    field_optional: Some(99),
                },
                Nested {
                    field_u16: 555,
                    field_utf8_string: String::from("Hello, from Nested (555)!"),
                    field_optional: Some(100),
                },
            ],
            prop_enum: TargetEnum::OptionString(String::from("Hello, from Enum (666)!")),
            prop_optional_strct: Some(Nested {
                field_u16: 555,
                field_utf8_string: String::from("Hello, from Nested (555)!"),
                field_optional: Some(100),
            }),
            prop_optional_enum: Some(TargetEnum::OptionString(String::from("Hello, from Enum (666)!"))),
            prop_enum_vec: vec![TargetEnum::OptionString(String::from("Hello, from Enum (666)!"))],
            prop_optional_enum_vec: Some(vec![TargetEnum::OptionString(String::from("Hello, from Enum (666)!")), TargetEnum::Optioni16(666)])
        };
        let buf = match StructEncode::abduct(&mut a) {
            Ok(buf) => buf,
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        println!("{:?}", buf);
        let mut b: Target = Target::defaults();
        let s = match Storage::new(buf) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        match b.extract(s) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        }
        println!("{:?}", b);
        assert_eq!(a.prop_u8, b.prop_u8);
        assert_eq!(a.prop_u16, b.prop_u16);
        assert_eq!(a.prop_u32, b.prop_u32);
        assert_eq!(a.prop_u64, b.prop_u64);
        assert_eq!(a.prop_i8, b.prop_i8);
        assert_eq!(a.prop_i16, b.prop_i16);
        assert_eq!(a.prop_i32, b.prop_i32);
        assert_eq!(a.prop_i64, b.prop_i64);
        assert_eq!(a.prop_u8_vec, b.prop_u8_vec);
        assert_eq!(a.prop_u16_vec, b.prop_u16_vec);
        assert_eq!(a.prop_u32_vec, b.prop_u32_vec);
        assert_eq!(a.prop_u64_vec, b.prop_u64_vec);
        assert_eq!(a.prop_i8_vec, b.prop_i8_vec);
        assert_eq!(a.prop_i16_vec, b.prop_i16_vec);
        assert_eq!(a.prop_i32_vec, b.prop_i32_vec);
        assert_eq!(a.prop_i64_vec, b.prop_i64_vec);
        assert_eq!(a.prop_f32_vec, b.prop_f32_vec);
        assert_eq!(a.prop_f64_vec, b.prop_f64_vec);
        assert_eq!(a.prop_f32, b.prop_f32);
        assert_eq!(a.prop_f64, b.prop_f64);
        assert_eq!(a.prop_utf8_string_vec, b.prop_utf8_string_vec);
        assert_eq!(a.prop_nested, b.prop_nested);
        assert_eq!(a.prop_nested.field_optional, b.prop_nested.field_optional);
        assert_eq!(a.prop_nested_vec, b.prop_nested_vec);
        assert_eq!(a.prop_enum, b.prop_enum);
        assert_eq!(a.prop_optional_strct, b.prop_optional_strct);
        assert_eq!(a.prop_optional_enum, b.prop_optional_enum);
        assert_eq!(a.prop_optional_enum_vec, b.prop_optional_enum_vec);
        let enums: Vec<TargetEnum> = vec![
            TargetEnum::OptionString(String::from("Hello from enum!")),
            TargetEnum::Optionu8(1),
            TargetEnum::Optionu16(2),
            TargetEnum::Optionu32(3),
            TargetEnum::Optionu64(4),
            TargetEnum::Optioni8(5),
            TargetEnum::Optioni16(6),
            TargetEnum::Optioni32(7),
            TargetEnum::Optioni64(8),
            TargetEnum::Optionf32(0.1),
            TargetEnum::Optionf64(0.2),
            TargetEnum::OptionBool(true),
            TargetEnum::OptionStruct(Nested {
                field_u16: 999,
                field_utf8_string: String::from("Hello, from Nested in enum!"),
                field_optional: None,
            }),
            TargetEnum::Optionu8Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optionu16Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optionu32Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optionu64Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optioni8Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optioni16Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optioni32Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optioni64Vec(vec![1, 2, 3, 4]),
            TargetEnum::Optionf32Vec(vec![0.1, 0.2, 0.3, 0.4]),
            TargetEnum::Optionf64Vec(vec![0.1, 0.2, 0.3, 0.4]),
            TargetEnum::OptionStructVec(vec![
                Nested {
                    field_u16: 111,
                    field_utf8_string: String::from("Hello, from Nested in enum!"),
                    field_optional: Some(1),
                },
                Nested {
                    field_u16: 222,
                    field_utf8_string: String::from("Hello, from Nested in enum!"),
                    field_optional: Some(2),
                }
            ]),
        ];
        let mut enums_bufs: Vec<Vec<u8>> = vec![];
        for item in enums.iter() {
            let mut item = item.clone();
            enums_bufs.push(match &mut item.abduct() {
                Ok(buf) => buf.clone(),
                Err(e) => {
                    println!("{}", e);
                    assert_eq!(true, false);
                    return;
                }
            });
        }
        for (pos, buf) in enums_bufs.iter().enumerate() {
            match TargetEnum::extract(buf.clone()) {
                Ok(v) => {
                    println!("{:?}", v);
                    assert_eq!(v, enums[pos]);
                },
                Err(e) => {
                    println!("{}", e);
                    assert_eq!(true, false);
                    return;
                }
            };
        }
        //assert_eq!(true, false);
    }

}