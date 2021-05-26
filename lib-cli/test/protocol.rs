
#[derive(Debug, Clone)]
pub enum AvailableMessages {
    EnumExampleA(EnumExampleA),
    EnumExampleB(EnumExampleB),
    EnumExampleC(EnumExampleC),
    StructExampleA(StructExampleA),
    StructExampleB(StructExampleB),
    StructExampleC(StructExampleC),
    StructExampleD(StructExampleD),
    StructExampleE(StructExampleE),
    StructExampleF(StructExampleF),
    StructExampleG(StructExampleG),
    StructExampleJ(StructExampleJ),
    GroupA(GroupA::AvailableMessages),
    GroupB(GroupB::AvailableMessages),
}
#[derive(Debug, Clone, PartialEq)]
pub enum EnumExampleA {
    Option_a(String),
    Option_b(String),
    Defaults,
}
impl EnumDecode for EnumExampleA {
    fn get_id(&self) -> u32 { 1 }
    fn extract(buf: Vec<u8>) -> Result<EnumExampleA, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumExampleA because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleA::Option_a(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleA::Option_b(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumExampleA")),
        }
    }
}
impl EnumEncode for EnumExampleA {
    fn get_id(&self) -> u32 { 1 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let (buf, index) = match self {
            Self::Option_a(v) => (v.encode(), 0),
            Self::Option_b(v) => (v.encode(), 1),
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
impl PackingEnum for EnumExampleA {}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumExampleB {
    Option_str(String),
    Option_u8(u8),
    Option_u16(u16),
    Option_u32(u32),
    Option_u64(u64),
    Option_i8(i8),
    Option_i16(i16),
    Option_i32(i32),
    Option_i64(i64),
    Option_f32(f32),
    Option_f64(f64),
    Defaults,
}
impl EnumDecode for EnumExampleB {
    fn get_id(&self) -> u32 { 2 }
    fn extract(buf: Vec<u8>) -> Result<EnumExampleB, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumExampleB because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_str(v)),
                Err(e) => Err(e)
            },
            1 => match u8::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_u8(v)),
                Err(e) => Err(e)
            },
            2 => match u16::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_u16(v)),
                Err(e) => Err(e)
            },
            3 => match u32::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_u32(v)),
                Err(e) => Err(e)
            },
            4 => match u64::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_u64(v)),
                Err(e) => Err(e)
            },
            5 => match i8::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_i8(v)),
                Err(e) => Err(e)
            },
            6 => match i16::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_i16(v)),
                Err(e) => Err(e)
            },
            7 => match i32::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_i32(v)),
                Err(e) => Err(e)
            },
            8 => match i64::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_i64(v)),
                Err(e) => Err(e)
            },
            9 => match f32::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_f32(v)),
                Err(e) => Err(e)
            },
            10 => match f64::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleB::Option_f64(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumExampleB")),
        }
    }
}
impl EnumEncode for EnumExampleB {
    fn get_id(&self) -> u32 { 2 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let (buf, index) = match self {
            Self::Option_str(v) => (v.encode(), 0),
            Self::Option_u8(v) => (v.encode(), 1),
            Self::Option_u16(v) => (v.encode(), 2),
            Self::Option_u32(v) => (v.encode(), 3),
            Self::Option_u64(v) => (v.encode(), 4),
            Self::Option_i8(v) => (v.encode(), 5),
            Self::Option_i16(v) => (v.encode(), 6),
            Self::Option_i32(v) => (v.encode(), 7),
            Self::Option_i64(v) => (v.encode(), 8),
            Self::Option_f32(v) => (v.encode(), 9),
            Self::Option_f64(v) => (v.encode(), 10),
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
impl PackingEnum for EnumExampleB {}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumExampleC {
    Option_str(Vec<String>),
    Option_u8(Vec<u8>),
    Option_u16(Vec<u16>),
    Option_u32(Vec<u32>),
    Option_u64(Vec<u64>),
    Option_i8(Vec<i8>),
    Option_i16(Vec<i16>),
    Option_i32(Vec<i32>),
    Option_i64(Vec<i64>),
    Option_f32(Vec<f32>),
    Option_f64(Vec<f64>),
    Defaults,
}
impl EnumDecode for EnumExampleC {
    fn get_id(&self) -> u32 { 3 }
    fn extract(buf: Vec<u8>) -> Result<EnumExampleC, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumExampleC because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match Vec::<String>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_str(v)),
                Err(e) => Err(e)
            },
            1 => match Vec::<u8>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_u8(v)),
                Err(e) => Err(e)
            },
            2 => match Vec::<u16>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_u16(v)),
                Err(e) => Err(e)
            },
            3 => match Vec::<u32>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_u32(v)),
                Err(e) => Err(e)
            },
            4 => match Vec::<u64>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_u64(v)),
                Err(e) => Err(e)
            },
            5 => match Vec::<i8>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_i8(v)),
                Err(e) => Err(e)
            },
            6 => match Vec::<i16>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_i16(v)),
                Err(e) => Err(e)
            },
            7 => match Vec::<i32>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_i32(v)),
                Err(e) => Err(e)
            },
            8 => match Vec::<i64>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_i64(v)),
                Err(e) => Err(e)
            },
            9 => match Vec::<f32>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_f32(v)),
                Err(e) => Err(e)
            },
            10 => match Vec::<f64>::decode(&body_buf) {
                Ok(v) => Ok(EnumExampleC::Option_f64(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumExampleC")),
        }
    }
}
impl EnumEncode for EnumExampleC {
    fn get_id(&self) -> u32 { 3 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let (buf, index) = match self {
            Self::Option_str(v) => (v.encode(), 0),
            Self::Option_u8(v) => (v.encode(), 1),
            Self::Option_u16(v) => (v.encode(), 2),
            Self::Option_u32(v) => (v.encode(), 3),
            Self::Option_u64(v) => (v.encode(), 4),
            Self::Option_i8(v) => (v.encode(), 5),
            Self::Option_i16(v) => (v.encode(), 6),
            Self::Option_i32(v) => (v.encode(), 7),
            Self::Option_i64(v) => (v.encode(), 8),
            Self::Option_f32(v) => (v.encode(), 9),
            Self::Option_f64(v) => (v.encode(), 10),
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
impl PackingEnum for EnumExampleC {}

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleA {
    pub field_str: String,
    pub field_u8: u8,
    pub field_u16: u16,
    pub field_u32: u32,
    pub field_u64: u64,
    pub field_i8: i8,
    pub field_i16: i16,
    pub field_i32: i32,
    pub field_i64: i64,
    pub field_f32: f32,
    pub field_f64: f64,
    pub field_bool: bool,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleA {
    fn get_id() -> u32 {
        4
    }
    fn defaults() -> StructExampleA {
        StructExampleA {
            field_str: String::from(""),
            field_u8: 0,
            field_u16: 0,
            field_u32: 0,
            field_u64: 0,
            field_i8: 0,
            field_i16: 0,
            field_i32: 0,
            field_i64: 0,
            field_f32: 0.0,
            field_f64: 0.0,
            field_bool: true,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_str = match String::get_from_storage(Source::Storage(&mut storage), Some(5)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(6)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(7)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match u32::get_from_storage(Source::Storage(&mut storage), Some(8)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match u64::get_from_storage(Source::Storage(&mut storage), Some(9)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match i8::get_from_storage(Source::Storage(&mut storage), Some(10)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match i16::get_from_storage(Source::Storage(&mut storage), Some(11)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match i32::get_from_storage(Source::Storage(&mut storage), Some(12)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match i64::get_from_storage(Source::Storage(&mut storage), Some(13)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match f32::get_from_storage(Source::Storage(&mut storage), Some(14)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match f64::get_from_storage(Source::Storage(&mut storage), Some(15)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match bool::get_from_storage(Source::Storage(&mut storage), Some(16)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleA {
    fn get_id(&self) -> u32 { 4 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(5)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(6)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(7)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(8)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(9)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(10)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(11)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(12)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(13)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(14)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(15)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(16)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleA { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleB {
    pub field_str: Vec<String>,
    pub field_u8: Vec<u8>,
    pub field_u16: Vec<u16>,
    pub field_u32: Vec<u32>,
    pub field_u64: Vec<u64>,
    pub field_i8: Vec<i8>,
    pub field_i16: Vec<i16>,
    pub field_i32: Vec<i32>,
    pub field_i64: Vec<i64>,
    pub field_f32: Vec<f32>,
    pub field_f64: Vec<f64>,
    pub field_bool: Vec<bool>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleB {
    fn get_id() -> u32 {
        17
    }
    fn defaults() -> StructExampleB {
        StructExampleB {
            field_str: vec![],
            field_u8: vec![],
            field_u16: vec![],
            field_u32: vec![],
            field_u64: vec![],
            field_i8: vec![],
            field_i16: vec![],
            field_i32: vec![],
            field_i64: vec![],
            field_f32: vec![],
            field_f64: vec![],
            field_bool: vec![],
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_str = match Vec::<String>::get_from_storage(Source::Storage(&mut storage), Some(18)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Vec::<u8>::get_from_storage(Source::Storage(&mut storage), Some(19)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Vec::<u16>::get_from_storage(Source::Storage(&mut storage), Some(20)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Vec::<u32>::get_from_storage(Source::Storage(&mut storage), Some(21)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Vec::<u64>::get_from_storage(Source::Storage(&mut storage), Some(22)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Vec::<i8>::get_from_storage(Source::Storage(&mut storage), Some(23)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Vec::<i16>::get_from_storage(Source::Storage(&mut storage), Some(24)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Vec::<i32>::get_from_storage(Source::Storage(&mut storage), Some(25)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Vec::<i64>::get_from_storage(Source::Storage(&mut storage), Some(26)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Vec::<f32>::get_from_storage(Source::Storage(&mut storage), Some(27)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Vec::<f64>::get_from_storage(Source::Storage(&mut storage), Some(28)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Vec::<bool>::get_from_storage(Source::Storage(&mut storage), Some(29)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleB {
    fn get_id(&self) -> u32 { 17 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(18)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(19)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(20)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(21)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(22)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(23)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(24)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(25)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(26)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(27)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(28)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(29)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleB { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleC {
    pub field_str: Option<String>,
    pub field_u8: Option<u8>,
    pub field_u16: Option<u16>,
    pub field_u32: Option<u32>,
    pub field_u64: Option<u64>,
    pub field_i8: Option<i8>,
    pub field_i16: Option<i16>,
    pub field_i32: Option<i32>,
    pub field_i64: Option<i64>,
    pub field_f32: Option<f32>,
    pub field_f64: Option<f64>,
    pub field_bool: Option<bool>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleC {
    fn get_id() -> u32 {
        30
    }
    fn defaults() -> StructExampleC {
        StructExampleC {
            field_str: None,
            field_u8: None,
            field_u16: None,
            field_u32: None,
            field_u64: None,
            field_i8: None,
            field_i16: None,
            field_i32: None,
            field_i64: None,
            field_f32: None,
            field_f64: None,
            field_bool: None,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_str = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(31)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Option::<u8>::get_from_storage(Source::Storage(&mut storage), Some(32)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Option::<u16>::get_from_storage(Source::Storage(&mut storage), Some(33)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Option::<u32>::get_from_storage(Source::Storage(&mut storage), Some(34)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Option::<u64>::get_from_storage(Source::Storage(&mut storage), Some(35)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Option::<i8>::get_from_storage(Source::Storage(&mut storage), Some(36)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Option::<i16>::get_from_storage(Source::Storage(&mut storage), Some(37)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Option::<i32>::get_from_storage(Source::Storage(&mut storage), Some(38)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Option::<i64>::get_from_storage(Source::Storage(&mut storage), Some(39)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Option::<f32>::get_from_storage(Source::Storage(&mut storage), Some(40)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Option::<f64>::get_from_storage(Source::Storage(&mut storage), Some(41)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Option::<bool>::get_from_storage(Source::Storage(&mut storage), Some(42)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleC {
    fn get_id(&self) -> u32 { 30 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(31)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(32)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(33)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(34)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(35)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(36)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(37)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(38)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(39)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(40)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(41)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(42)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleC { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleD {
    pub field_str: Option<Vec<String>>,
    pub field_u8: Option<Vec<u8>>,
    pub field_u16: Option<Vec<u16>>,
    pub field_u32: Option<Vec<u32>>,
    pub field_u64: Option<Vec<u64>>,
    pub field_i8: Option<Vec<i8>>,
    pub field_i16: Option<Vec<i16>>,
    pub field_i32: Option<Vec<i32>>,
    pub field_i64: Option<Vec<i64>>,
    pub field_f32: Option<Vec<f32>>,
    pub field_f64: Option<Vec<f64>>,
    pub field_bool: Option<Vec<bool>>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleD {
    fn get_id() -> u32 {
        43
    }
    fn defaults() -> StructExampleD {
        StructExampleD {
            field_str: None,
            field_u8: None,
            field_u16: None,
            field_u32: None,
            field_u64: None,
            field_i8: None,
            field_i16: None,
            field_i32: None,
            field_i64: None,
            field_f32: None,
            field_f64: None,
            field_bool: None,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_str = match Option::<Vec::<String>>::get_from_storage(Source::Storage(&mut storage), Some(44)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Option::<Vec::<u8>>::get_from_storage(Source::Storage(&mut storage), Some(45)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Option::<Vec::<u16>>::get_from_storage(Source::Storage(&mut storage), Some(46)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Option::<Vec::<u32>>::get_from_storage(Source::Storage(&mut storage), Some(47)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Option::<Vec::<u64>>::get_from_storage(Source::Storage(&mut storage), Some(48)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Option::<Vec::<i8>>::get_from_storage(Source::Storage(&mut storage), Some(49)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Option::<Vec::<i16>>::get_from_storage(Source::Storage(&mut storage), Some(50)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Option::<Vec::<i32>>::get_from_storage(Source::Storage(&mut storage), Some(51)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Option::<Vec::<i64>>::get_from_storage(Source::Storage(&mut storage), Some(52)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Option::<Vec::<f32>>::get_from_storage(Source::Storage(&mut storage), Some(53)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Option::<Vec::<f64>>::get_from_storage(Source::Storage(&mut storage), Some(54)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Option::<Vec::<bool>>::get_from_storage(Source::Storage(&mut storage), Some(55)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleD {
    fn get_id(&self) -> u32 { 43 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(44)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(45)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(46)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(47)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(48)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(49)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(50)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(51)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(52)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(53)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(54)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(55)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleD { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleE {
    pub field_a: EnumExampleA,
    pub field_b: EnumExampleB,
    pub field_c: EnumExampleC,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleE {
    fn get_id() -> u32 {
        56
    }
    fn defaults() -> StructExampleE {
        StructExampleE {
            field_a: EnumExampleA::Defaults,
            field_b: EnumExampleB::Defaults,
            field_c: EnumExampleC::Defaults,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match EnumExampleA::get_from_storage(Source::Storage(&mut storage), Some(57)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match EnumExampleB::get_from_storage(Source::Storage(&mut storage), Some(58)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_c = match EnumExampleC::get_from_storage(Source::Storage(&mut storage), Some(59)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleE {
    fn get_id(&self) -> u32 { 56 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(57)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(58)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_c.get_buf_to_store(Some(59)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleE { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleF {
    pub field_a: Option<EnumExampleA>,
    pub field_b: Option<EnumExampleB>,
    pub field_c: Option<EnumExampleC>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleF {
    fn get_id() -> u32 {
        60
    }
    fn defaults() -> StructExampleF {
        StructExampleF {
            field_a: None,
            field_b: None,
            field_c: None,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        if let Some(buf) = storage.get(61) {
            if buf.is_empty() {
                self.field_a = None;
            } else {
                self.field_a = match EnumExampleA::get_from_storage(Source::Storage(&mut storage), Some(61)) {
                    Ok(val) => Some(val),
                    Err(e) => { return Err(e) },
                };
            }
        } else {
            return Err("Buffer for property field_a isn't found".to_string());
        }
        if let Some(buf) = storage.get(62) {
            if buf.is_empty() {
                self.field_b = None;
            } else {
                self.field_b = match EnumExampleB::get_from_storage(Source::Storage(&mut storage), Some(62)) {
                    Ok(val) => Some(val),
                    Err(e) => { return Err(e) },
                };
            }
        } else {
            return Err("Buffer for property field_b isn't found".to_string());
        }
        if let Some(buf) = storage.get(63) {
            if buf.is_empty() {
                self.field_c = None;
            } else {
                self.field_c = match EnumExampleC::get_from_storage(Source::Storage(&mut storage), Some(63)) {
                    Ok(val) => Some(val),
                    Err(e) => { return Err(e) },
                };
            }
        } else {
            return Err("Buffer for property field_c isn't found".to_string());
        }
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleF {
    fn get_id(&self) -> u32 { 60 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        if let Some(mut val) = self.field_a.clone() {
            match val.get_buf_to_store(Some(61)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(61)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        if let Some(mut val) = self.field_b.clone() {
            match val.get_buf_to_store(Some(62)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(62)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        if let Some(mut val) = self.field_c.clone() {
            match val.get_buf_to_store(Some(63)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(63)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleF { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleG {
    pub field_a: StructExampleA,
    pub field_b: StructExampleB,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleG {
    fn get_id() -> u32 {
        64
    }
    fn defaults() -> StructExampleG {
        StructExampleG {
            field_a: StructExampleA {
                field_str: String::from(""),
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: 0,
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: 0,
                field_f32: 0.0,
                field_f64: 0.0,
                field_bool: true,
            },
            field_b: StructExampleB {
                field_str: vec![],
                field_u8: vec![],
                field_u16: vec![],
                field_u32: vec![],
                field_u64: vec![],
                field_i8: vec![],
                field_i16: vec![],
                field_i32: vec![],
                field_i64: vec![],
                field_f32: vec![],
                field_f64: vec![],
                field_bool: vec![],
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(65)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match StructExampleB::get_from_storage(Source::Storage(&mut storage), Some(66)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleG {
    fn get_id(&self) -> u32 { 64 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(65)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(66)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleG { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleJ {
    pub field_a: Option<StructExampleA>,
    pub field_b: Option<StructExampleB>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleJ {
    fn get_id() -> u32 {
        67
    }
    fn defaults() -> StructExampleJ {
        StructExampleJ {
            field_a: None,
            field_b: None,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match Option::<StructExampleA>::get_from_storage(Source::Storage(&mut storage), Some(68)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match Option::<StructExampleB>::get_from_storage(Source::Storage(&mut storage), Some(69)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleJ {
    fn get_id(&self) -> u32 { 67 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(68)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(69)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleJ { }

pub mod GroupA {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        EnumExampleA(EnumExampleA),
        StructExampleA(StructExampleA),
        StructExampleB(StructExampleB),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum EnumExampleA {
        Option_a(String),
        Option_b(String),
        Defaults,
    }
    impl EnumDecode for EnumExampleA {
        fn get_id(&self) -> u32 { 71 }
        fn extract(buf: Vec<u8>) -> Result<EnumExampleA, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for EnumExampleA because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let index = cursor.get_u16_le();
            let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
            body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
            match index {
                0 => match String::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleA::Option_a(v)),
                    Err(e) => Err(e)
                },
                1 => match String::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleA::Option_b(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for EnumExampleA")),
            }
        }
    }
    impl EnumEncode for EnumExampleA {
        fn get_id(&self) -> u32 { 71 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let (buf, index) = match self {
                Self::Option_a(v) => (v.encode(), 0),
                Self::Option_b(v) => (v.encode(), 1),
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
    impl PackingEnum for EnumExampleA {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructExampleA {
        pub field_u8: u8,
        pub field_u16: u16,
        pub opt: EnumExampleA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructExampleA {
        fn get_id() -> u32 {
            72
        }
        fn defaults() -> StructExampleA {
            StructExampleA {
                field_u8: 0,
                field_u16: 0,
                opt: EnumExampleA::Defaults,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(73)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(74)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.opt = match EnumExampleA::get_from_storage(Source::Storage(&mut storage), Some(75)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleA {
        fn get_id(&self) -> u32 { 72 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(73)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(74)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.opt.get_buf_to_store(Some(75)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructExampleA { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructExampleB {
        pub field_u8: u8,
        pub field_u16: u16,
        pub strct: StructExampleA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructExampleB {
        fn get_id() -> u32 {
            76
        }
        fn defaults() -> StructExampleB {
            StructExampleB {
                field_u8: 0,
                field_u16: 0,
                strct: StructExampleA {
                    field_u8: 0,
                    field_u16: 0,
                    opt: EnumExampleA::Defaults,
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(77)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(78)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.strct = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(79)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleB {
        fn get_id(&self) -> u32 { 76 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(77)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(78)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.strct.get_buf_to_store(Some(79)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructExampleB { }

}

pub mod GroupB {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        StructExampleA(StructExampleA),
        GroupC(GroupC::AvailableMessages),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructExampleA {
        pub field_u8: u8,
        pub field_u16: u16,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructExampleA {
        fn get_id() -> u32 {
            81
        }
        fn defaults() -> StructExampleA {
            StructExampleA {
                field_u8: 0,
                field_u16: 0,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(82)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(83)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleA {
        fn get_id(&self) -> u32 { 81 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(82)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(83)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructExampleA { }

    pub mod GroupC {
        use super::*;
        use std::io::Cursor;
        use bytes::{ Buf };
        #[derive(Debug, Clone)]
        pub enum AvailableMessages {
            StructExampleA(StructExampleA),
            StructExampleB(StructExampleB),
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StructExampleA {
            pub field_u8: u8,
            pub field_u16: u16,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for StructExampleA {
            fn get_id() -> u32 {
                85
            }
            fn defaults() -> StructExampleA {
                StructExampleA {
                    field_u8: 0,
                    field_u16: 0,
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(86)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(87)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructExampleA {
            fn get_id(&self) -> u32 { 85 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(86)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(87)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for StructExampleA { }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StructExampleB {
            pub field_u8: u8,
            pub field_u16: u16,
            pub strct: StructExampleA,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for StructExampleB {
            fn get_id() -> u32 {
                88
            }
            fn defaults() -> StructExampleB {
                StructExampleB {
                    field_u8: 0,
                    field_u16: 0,
                    strct: StructExampleA {
                        field_u8: 0,
                        field_u16: 0,
                    },
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(89)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(90)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.strct = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(91)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructExampleB {
            fn get_id(&self) -> u32 { 88 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(89)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(90)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.strct.get_buf_to_store(Some(91)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for StructExampleB { }

    }

}

impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {
        match id {
            1 => match EnumExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumExampleA(m)),
                Err(e) => Err(e),
            },
            2 => match EnumExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumExampleB(m)),
                Err(e) => Err(e),
            },
            3 => match EnumExampleC::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumExampleC(m)),
                Err(e) => Err(e),
            },
            71 => match GroupA::EnumExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::EnumExampleA(m))),
                Err(e) => Err(e),
            },
            4 => match StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleA(m)),
                Err(e) => Err(e),
            },
            17 => match StructExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleB(m)),
                Err(e) => Err(e),
            },
            30 => match StructExampleC::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleC(m)),
                Err(e) => Err(e),
            },
            43 => match StructExampleD::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleD(m)),
                Err(e) => Err(e),
            },
            56 => match StructExampleE::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleE(m)),
                Err(e) => Err(e),
            },
            60 => match StructExampleF::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleF(m)),
                Err(e) => Err(e),
            },
            64 => match StructExampleG::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleG(m)),
                Err(e) => Err(e),
            },
            67 => match StructExampleJ::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleJ(m)),
                Err(e) => Err(e),
            },
            72 => match GroupA::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructExampleA(m))),
                Err(e) => Err(e),
            },
            76 => match GroupA::StructExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructExampleB(m))),
                Err(e) => Err(e),
            },
            81 => match GroupB::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::StructExampleA(m))),
                Err(e) => Err(e),
            },
            85 => match GroupB::GroupC::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructExampleA(m)))),
                Err(e) => Err(e),
            },
            88 => match GroupB::GroupC::StructExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructExampleB(m)))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

