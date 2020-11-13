#[path = "./protocol.sizes.rs"]
pub mod sizes;

#[path = "./protocol.encode.rs"]
pub mod encode;

#[path = "./protocol.decode.rs"]
pub mod decode;

#[path = "./protocol.storage.rs"]
pub mod storage;

#[cfg(test)]
mod tests { 
    use super::*;
    use encode::{ StructEncode, EnumEncode, Encode };
    use decode::{ StructDecode, EnumDecode, Decode };
    use storage::{ Storage };
    use std::io::Cursor;
    use bytes::{ Buf };

    #[derive(Debug, Clone, PartialEq)]
    pub enum TargetEnum {
        OptionA(String),
        OptionB(u32),
    }

    impl EnumEncode for TargetEnum {

        fn encode(&mut self) -> Result<Vec<u8>, String> {
            match match self {
                Self::OptionA(v) => v.encode(1),
                Self::OptionB(v) => v.encode(2),
            } {
                Ok(buf) => Ok(buf),
                Err(e) => Err(e),
            }
        }

    }

    impl EnumDecode<TargetEnum> for TargetEnum {

        fn decode(buf: Vec<u8>) -> Result<TargetEnum, String> {
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
                    Ok(v) => Ok(TargetEnum::OptionA(v)),
                    Err(e) => Err(e),
                },
                2 => match u32::decode(&mut storage, id) {
                    Ok(v) => Ok(TargetEnum::OptionB(v)),
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
    }

    impl StructDecode for Nested {

        fn defaults() -> Nested {
            Nested {
                field_u16: 0,
                field_utf8_string: String::from(""),
            }
        }

        fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u16 = match u16::decode(&mut storage, 1) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_utf8_string = match String::decode(&mut storage, 2) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }

    impl StructEncode for Nested {

        fn encode(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u16.encode(1) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.field_utf8_string.encode(2) {
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
        prop_nested: Nested,
        prop_nested_vec: Vec<Nested>,
    }

    impl StructDecode for Target {
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
            }
        }
        fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
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
            Ok(())
        }
    }

    impl StructEncode for Target {

        fn encode(&mut self) -> Result<Vec<u8>, String> {
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
                field_utf8_string: String::from("Hello, from Nested!")
            },
            prop_nested_vec: vec![
                Nested {
                    field_u16: 333,
                    field_utf8_string: String::from("Hello, from Nested (333)!")
                },
                Nested {
                    field_u16: 444,
                    field_utf8_string: String::from("Hello, from Nested (444)!")
                },
                Nested {
                    field_u16: 555,
                    field_utf8_string: String::from("Hello, from Nested (555)!")
                },
            ],
        };
        let buf = match StructEncode::encode(&mut a) {
            Ok(buf) => buf,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        println!("{:?}", buf);
        let mut b: Target = Target::defaults();
        let s = match Storage::new(buf) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        b.decode(s);
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
        assert_eq!(a.prop_nested_vec, b.prop_nested_vec);

        let mut target_enum: TargetEnum = TargetEnum::OptionA(String::from("Hello from enum!"));
        let buf = match target_enum.encode() {
            Ok(buf) => buf,
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        match TargetEnum::decode(buf) {
            Ok(v) => {
                assert_eq!(v, target_enum);
            },
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        let mut target_enum: TargetEnum = TargetEnum::OptionB(999);
        let buf = match target_enum.encode() {
            Ok(buf) => buf,
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        match TargetEnum::decode(buf) {
            Ok(v) => {
                assert_eq!(v, target_enum);
            },
            Err(e) => {
                println!("{}", e);
                assert_eq!(true, false);
                return;
            }
        };
        // assert_eq!(true, false);
    }

}