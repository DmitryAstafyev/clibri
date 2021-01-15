#[allow(unused_imports)]
use super::*;

#[cfg(test)]
mod tests { 
    use super::*;
    use encode::{ StructEncode, EnumEncode, Encode, EncodeEnum, get_empty_buffer_val };
    use decode::{ StructDecode, EnumDecode, Decode, DecodeEnum, Source };
    use storage::{ Storage };
    use packing::{ PackingStruct, PackingEnum };
    use buffer::{ DecodeBuffer, Buffer };
    use sizes::{ U16_LEN };
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

        fn get_id(&self) -> u32 { 1001 }

        fn get_signature(&self) -> u16 { 0 }

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let (buf, index) = match self {
                Self::OptionString(v) => (v.encode(), 1),
                Self::Optionu8(v) => (v.encode(), 2),
                Self::Optionu16(v) => (v.encode(), 3),
                Self::Optionu32(v) => (v.encode(), 4),
                Self::Optionu64(v) => (v.encode(), 5),
                Self::Optioni8(v) => (v.encode(), 6),
                Self::Optioni16(v) => (v.encode(), 7),
                Self::Optioni32(v) => (v.encode(), 8),
                Self::Optioni64(v) => (v.encode(), 9),
                Self::Optionf32(v) => (v.encode(), 10),
                Self::Optionf64(v) => (v.encode(), 11),
                Self::OptionBool(v) => (v.encode(), 12),
                Self::OptionStruct(v) => (v.encode(), 13),
                Self::Optionu8Vec(v) => (v.encode(), 14),
                Self::Optionu16Vec(v) => (v.encode(), 15),
                Self::Optionu32Vec(v) => (v.encode(), 16),
                Self::Optionu64Vec(v) => (v.encode(), 17),
                Self::Optioni8Vec(v) => (v.encode(), 18),
                Self::Optioni16Vec(v) => (v.encode(), 19),
                Self::Optioni32Vec(v) => (v.encode(), 20),
                Self::Optioni64Vec(v) => (v.encode(), 21),
                Self::Optionf32Vec(v) => (v.encode(), 22),
                Self::Optionf64Vec(v) => (v.encode(), 23),
                Self::OptionStructVec(v) => (v.encode(), 24),
                _ => { return Err(String::from("Not supportable option")); },
            };
            let mut buf = match buf {
                Ok(buf) => buf,
                Err(e) => { return Err(e); },
            };
            let mut buffer: Vec<u8> = vec!();
            buffer.append(&mut (index as u16).to_le_bytes().to_vec());
            buffer.append(&mut buf);
            Ok(buffer)
        }

    }

    impl EnumDecode for TargetEnum {

        fn get_id(&self) -> u32 { 1001 }

        fn extract(buf: Vec<u8>) -> Result<TargetEnum, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for TargetEnum because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let index = cursor.get_u16_le();
            let mut body_buf = vec![0; buf.len() - U16_LEN];
            body_buf.copy_from_slice(&buf[U16_LEN..]);
            match index {
                1 => match String::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::OptionString(v)),
                    Err(e) => Err(e),
                },
                2 => match u8::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu8(v)),
                    Err(e) => Err(e),
                },
                3 => match u16::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu16(v)),
                    Err(e) => Err(e),
                },
                4 => match u32::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu32(v)),
                    Err(e) => Err(e),
                },
                5 => match u64::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu64(v)),
                    Err(e) => Err(e),
                },
                6 => match i8::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni8(v)),
                    Err(e) => Err(e),
                },
                7 => match i16::decode(&body_buf){
                    Ok(v) => Ok(TargetEnum::Optioni16(v)),
                    Err(e) => Err(e),
                },
                8 => match i32::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni32(v)),
                    Err(e) => Err(e),
                },
                9 => match i64::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni64(v)),
                    Err(e) => Err(e),
                },
                10 => match f32::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionf32(v)),
                    Err(e) => Err(e),
                },
                11 => match f64::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionf64(v)),
                    Err(e) => Err(e),
                },
                12 => match bool::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::OptionBool(v)),
                    Err(e) => Err(e),
                },
                13 => match Nested::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::OptionStruct(v)),
                    Err(e) => Err(e),
                },
                14 => match Vec::<u8>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu8Vec(v)),
                    Err(e) => Err(e),
                },
                15 => match Vec::<u16>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu16Vec(v)),
                    Err(e) => Err(e),
                },
                16 => match Vec::<u32>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu32Vec(v)),
                    Err(e) => Err(e),
                },
                17 => match Vec::<u64>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionu64Vec(v)),
                    Err(e) => Err(e),
                },
                18 => match Vec::<i8>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni8Vec(v)),
                    Err(e) => Err(e),
                },
                19 => match Vec::<i16>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni16Vec(v)),
                    Err(e) => Err(e),
                },
                20 => match Vec::<i32>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni32Vec(v)),
                    Err(e) => Err(e),
                },
                21 => match Vec::<i64>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optioni64Vec(v)),
                    Err(e) => Err(e),
                },
                22 => match Vec::<f32>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionf32Vec(v)),
                    Err(e) => Err(e),
                },
                23 => match Vec::<f64>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::Optionf64Vec(v)),
                    Err(e) => Err(e),
                },
                24 => match Vec::<Nested>::decode(&body_buf) {
                    Ok(v) => Ok(TargetEnum::OptionStructVec(v)),
                    Err(e) => Err(e),
                },
                _ => Err(String::from("Fail to find relevant value for TargetEnum"))
            }
        }

    }

    impl PackingEnum for TargetEnum {

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

        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(1)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_utf8_string = match String::get_from_storage(Source::Storage(&mut storage), Some(2)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_optional = match Option::<u8>::get_from_storage(Source::Storage(&mut storage), Some(3)) {
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

        fn get_signature(&self) -> u16 { 0 }

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u16.get_buf_to_store(Some(1)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.field_utf8_string.get_buf_to_store(Some(2)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.field_optional.get_buf_to_store(Some(3)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            Ok(buffer)
        }
    }

    impl PackingStruct for Nested {
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
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.prop_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(1)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(2)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32 = match u32::get_from_storage(Source::Storage(&mut storage), Some(3)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64 = match u64::get_from_storage(Source::Storage(&mut storage), Some(4)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8 = match i8::get_from_storage(Source::Storage(&mut storage), Some(5)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16 = match i16::get_from_storage(Source::Storage(&mut storage), Some(6)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32 = match i32::get_from_storage(Source::Storage(&mut storage), Some(7)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64 = match i64::get_from_storage(Source::Storage(&mut storage), Some(8)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u8_vec = match Vec::<u8>::get_from_storage(Source::Storage(&mut storage), Some(9)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16_vec = match Vec::<u16>::get_from_storage(Source::Storage(&mut storage), Some(10)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32_vec = match Vec::<u32>::get_from_storage(Source::Storage(&mut storage), Some(11)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64_vec = match Vec::<u64>::get_from_storage(Source::Storage(&mut storage), Some(12)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32_vec = match Vec::<f32>::get_from_storage(Source::Storage(&mut storage), Some(13)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64_vec = match Vec::<f64>::get_from_storage(Source::Storage(&mut storage), Some(14)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8_vec = match Vec::<i8>::get_from_storage(Source::Storage(&mut storage), Some(15)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16_vec = match Vec::<i16>::get_from_storage(Source::Storage(&mut storage), Some(16)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32_vec = match Vec::<i32>::get_from_storage(Source::Storage(&mut storage), Some(17)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64_vec = match Vec::<i64>::get_from_storage(Source::Storage(&mut storage), Some(18)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_string = match String::get_from_storage(Source::Storage(&mut storage), Some(19)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32 = match f32::get_from_storage(Source::Storage(&mut storage), Some(20)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64 = match f64::get_from_storage(Source::Storage(&mut storage), Some(21)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_utf8_string_vec = match Vec::<String>::get_from_storage(Source::Storage(&mut storage), Some(22)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_nested = match <Nested as Decode<Nested>>::get_from_storage(Source::Storage(&mut storage), Some(23)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_nested_vec = match Vec::<Nested>::get_from_storage(Source::Storage(&mut storage), Some(24)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_enum = match TargetEnum::get_from_storage(Source::Storage(&mut storage), Some(25)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_optional_strct = match Option::<Nested>::get_from_storage(Source::Storage(&mut storage), Some(26)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            if let Some(buf) = storage.get(27) {
                if buf.is_empty() {
                    self.prop_optional_enum = None;
                } else {
                    self.prop_optional_enum = match TargetEnum::get_from_storage(Source::Storage(&mut storage), Some(27)) {
                        Ok(val) => Some(val),
                        Err(e) => { return Err(e) },
                    };
                }
            } else {
                return Err("Buffer for property prop_optional_enum isn\'t found".to_string());
            }
            self.prop_enum_vec = match Vec::<TargetEnum>::get_from_storage(Source::Storage(&mut storage), Some(28)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            if let Some(buf) = storage.get(29) {
                if buf.is_empty() {
                    self.prop_optional_enum_vec = None;
                } else {
                    self.prop_optional_enum_vec = match Vec::<TargetEnum>::get_from_storage(Source::Storage(&mut storage), Some(29)) {
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

        fn get_signature(&self) -> u16 { 0 }

        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.prop_u8.get_buf_to_store(Some(1)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16.get_buf_to_store(Some(2)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32.get_buf_to_store(Some(3)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64.get_buf_to_store(Some(4)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8.get_buf_to_store(Some(5)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16.get_buf_to_store(Some(6)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32.get_buf_to_store(Some(7)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64.get_buf_to_store(Some(8)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u8_vec.get_buf_to_store(Some(9)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16_vec.get_buf_to_store(Some(10)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32_vec.get_buf_to_store(Some(11)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64_vec.get_buf_to_store(Some(12)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32_vec.get_buf_to_store(Some(13)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64_vec.get_buf_to_store(Some(14)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8_vec.get_buf_to_store(Some(15)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16_vec.get_buf_to_store(Some(16)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32_vec.get_buf_to_store(Some(17)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64_vec.get_buf_to_store(Some(18)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_string.get_buf_to_store(Some(19)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32.get_buf_to_store(Some(20)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64.get_buf_to_store(Some(21)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_utf8_string_vec.get_buf_to_store(Some(22)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match Encode::get_buf_to_store(&mut self.prop_nested, Some(23)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_nested_vec.get_buf_to_store(Some(24)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_enum.get_buf_to_store(Some(25)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_optional_strct.get_buf_to_store(Some(26)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            if let Some(mut val) = self.prop_optional_enum.clone() {
                match val.get_buf_to_store(Some(27)) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            } else {
                match get_empty_buffer_val(Some(27)) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            }
            match self.prop_enum_vec.get_buf_to_store(Some(28)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            if let Some(mut val) = self.prop_optional_enum_vec.clone() {
                match val.get_buf_to_store(Some(29)) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            } else {
                match get_empty_buffer_val(Some(29)) {
                    Ok(mut buf) => { buffer.append(&mut buf); },
                    Err(e) => { return  Err(e); }
                };
            }
            Ok(buffer)
        }

    }

    impl PackingStruct for Target {
    }

    #[derive(Debug, Clone)]
    enum Messages {
        Nested(Nested),
        Target(Target),
        TargetEnum(TargetEnum),
    }

    impl DecodeBuffer<Messages> for Buffer<Messages> {

        fn get_msg(&self, id: u32, buf: &[u8]) -> Result<Messages, String> {
            match id {
                1 => match Nested::extract(buf.to_vec()) {
                    Ok(structs) => Ok(Messages::Nested(structs)),
                    Err(e) => Err(e),
                },
                2 => match Target::extract(buf.to_vec()) {
                    Ok(structs) => Ok(Messages::Target(structs)),
                    Err(e) => Err(e),
                },
                1001 => match TargetEnum::extract(buf.to_vec()) {
                    Ok(enums) => Ok(Messages::TargetEnum(enums)),
                    Err(e) => Err(e), 
                },
                _ => Err(String::from("No message has been found"))
            }
        }

        fn get_signature(&self) -> u16 { 0 }

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
        println!("{:?}", s);

        match b.extract_from_storage(s) {
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
        let mut buffer = Buffer::new();
        let mut buf: Vec<u8> = vec!();
        let mut c = Nested {
            field_u16: 999,
            field_utf8_string: String::from("Hello, from Nested!"),
            field_optional: Some(2),
        };
        buf.append(&mut a.pack().unwrap());
        buf.append(&mut c.pack().unwrap());
        for item in enums.iter() {
            let mut item = item.clone();
            buf.append(&mut item.pack().unwrap());
        }
        if let Err(e) = buffer.chunk(&buf) {
            println!("{:?}", e);
            assert_eq!(true, false);
        }
        let mut count = 10 + 20 + enums.len();
        loop {
            if let Some(msg) = buffer.next() {
                match msg.msg {
                    Messages::Nested(b) => {
                        count -= 10;
                        assert_eq!(a.prop_nested, b);
                        assert_eq!(a.prop_nested.field_optional, b.field_optional);
                    },
                    Messages::Target(b) => {
                        count -= 20;
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
                    },
                    Messages::TargetEnum(pack) => {
                        count -= 1;
                    },
                };
            } else {
                break;
            }
        }
        assert_eq!(count, 0);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.pending(), 0);
        //assert_eq!(true, false);
    }

}