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
    use encode::{ StructEncode, Encode };
    use decode::{ StructDecode, Decode };
    use storage::{ Storage };

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
            }
        }
        fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
            self.prop_u8 = match u8::decode(&mut storage, String::from("prop_u8")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16 = match u16::decode(&mut storage, String::from("prop_u16")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32 = match u32::decode(&mut storage, String::from("prop_u32")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64 = match u64::decode(&mut storage, String::from("prop_u64")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8 = match i8::decode(&mut storage, String::from("prop_i8")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16 = match i16::decode(&mut storage, String::from("prop_i16")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32 = match i32::decode(&mut storage, String::from("prop_i32")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64 = match i64::decode(&mut storage, String::from("prop_i64")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u8_vec = match Vec::<u8>::decode(&mut storage, String::from("prop_u8_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u16_vec = match Vec::<u16>::decode(&mut storage, String::from("prop_u16_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u32_vec = match Vec::<u32>::decode(&mut storage, String::from("prop_u32_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_u64_vec = match Vec::<u64>::decode(&mut storage, String::from("prop_u64_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i8_vec = match Vec::<i8>::decode(&mut storage, String::from("prop_i8_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i16_vec = match Vec::<i16>::decode(&mut storage, String::from("prop_i16_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i32_vec = match Vec::<i32>::decode(&mut storage, String::from("prop_i32_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_i64_vec = match Vec::<i64>::decode(&mut storage, String::from("prop_i64_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32_vec = match Vec::<f32>::decode(&mut storage, String::from("prop_f32_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64_vec = match Vec::<f64>::decode(&mut storage, String::from("prop_f64_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_string = match String::decode(&mut storage, String::from("prop_string")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f32 = match f32::decode(&mut storage, String::from("prop_f32")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_f64 = match f64::decode(&mut storage, String::from("prop_f64")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.prop_utf8_string_vec = match Vec::<String>::decode(&mut storage, String::from("prop_utf8_string_vec")) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }

    impl StructEncode for Target {

        fn encode(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.prop_u8.encode(String::from("prop_u8")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16.encode(String::from("prop_u16")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32.encode(String::from("prop_u32")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64.encode(String::from("prop_u64")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8.encode(String::from("prop_i8")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16.encode(String::from("prop_i16")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32.encode(String::from("prop_i32")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64.encode(String::from("prop_i64")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u8_vec.encode(String::from("prop_u8_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u16_vec.encode(String::from("prop_u16_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u32_vec.encode(String::from("prop_u32_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_u64_vec.encode(String::from("prop_u64_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32_vec.encode(String::from("prop_f32_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64_vec.encode(String::from("prop_f64_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i8_vec.encode(String::from("prop_i8_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i16_vec.encode(String::from("prop_i16_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i32_vec.encode(String::from("prop_i32_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_i64_vec.encode(String::from("prop_i64_vec")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_string.encode(String::from("prop_string")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f32.encode(String::from("prop_f32")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_f64.encode(String::from("prop_f64")) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); }
            };
            match self.prop_utf8_string_vec.encode(String::from("prop_utf8_string_vec")) {
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
        };
        let buf = match a.encode() {
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
        // assert_eq!(true, false);
    }

}