
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::convert::TryFrom;
use std::io::Cursor;
use std::collections::{HashMap};
use bytes::{ Buf };

pub mod sizes {
    use std::mem;

    pub const U8_LEN: usize = mem::size_of::<u8>();
    pub const U16_LEN: usize = mem::size_of::<u16>();
    pub const U32_LEN: usize = mem::size_of::<u32>();
    pub const U64_LEN: usize = mem::size_of::<u64>();
    pub const I8_LEN: usize = mem::size_of::<i8>();
    pub const I16_LEN: usize = mem::size_of::<i16>();
    pub const I32_LEN: usize = mem::size_of::<i32>();
    pub const I64_LEN: usize = mem::size_of::<i64>();
    pub const F32_LEN: usize = mem::size_of::<f32>();
    pub const F64_LEN: usize = mem::size_of::<f64>();
    pub const BOOL_LEN: usize = mem::size_of::<bool>();

}

pub enum ESize {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

pub trait StructDecode {

    fn get_id() -> u32;
    fn defaults() -> Self;
    fn extract(&mut self, storage: Storage) -> Result<(), String>;

}

pub trait EnumDecode<T> {

    fn extract(buf: Vec<u8>) -> Result<T, String>;

    fn decode(storage: &mut Storage, id: u16) -> Result<T, String> {
        if let Some(buf) = storage.get(id) {
            Self::extract(buf.clone())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

pub trait Decode<T> {

    fn decode(storage: &mut Storage, id: u16) -> Result<T, String>;

}

impl Decode<u8> for u8 {
    fn decode(storage: &mut Storage, id: u16) -> Result<u8, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<u16> for u16 {
    fn decode(storage: &mut Storage, id: u16) -> Result<u16, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::U16_LEN {
                return Err(format!("To extract u16 value buffer should have length at least {} bytes, but length is {}", sizes::U16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<u32> for u32 {
    fn decode(storage: &mut Storage, id: u16) -> Result<u32, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::U32_LEN {
                return Err(format!("To extract u32 value buffer should have length at least {} bytes, but length is {}", sizes::U32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<u64> for u64 {
    fn decode(storage: &mut Storage, id: u16) -> Result<u64, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::U64_LEN {
                return Err(format!("To extract u64 value buffer should have length at least {} bytes, but length is {}", sizes::U64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<i8> for i8 {
    fn decode(storage: &mut Storage, id: u16) -> Result<i8, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::I8_LEN {
                return Err(format!("To extract i8 value buffer should have length at least {} bytes, but length is {}", sizes::I8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i8())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<i16> for i16 {
    fn decode(storage: &mut Storage, id: u16) -> Result<i16, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::I16_LEN {
                return Err(format!("To extract i16 value buffer should have length at least {} bytes, but length is {}", sizes::I16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<i32> for i32 {
    fn decode(storage: &mut Storage, id: u16) -> Result<i32, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::I32_LEN {
                return Err(format!("To extract i32 value buffer should have length at least {} bytes, but length is {}", sizes::I32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<i64> for i64 {
    fn decode(storage: &mut Storage, id: u16) -> Result<i64, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::I64_LEN {
                return Err(format!("To extract i64 value buffer should have length at least {} bytes, but length is {}", sizes::I64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<f32> for f32 {
    fn decode(storage: &mut Storage, id: u16) -> Result<f32, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::F32_LEN {
                return Err(format!("To extract f32 value buffer should have length at least {} bytes, but length is {}", sizes::F32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<f64> for f64 {
    fn decode(storage: &mut Storage, id: u16) -> Result<f64, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::F64_LEN {
                return Err(format!("To extract f64 value buffer should have length at least {} bytes, but length is {}", sizes::F64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<bool> for bool {
    fn decode(storage: &mut Storage, id: u16) -> Result<bool, String> {
        if let Some(buf) = storage.get(id) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8() != 0)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<String> for String {
    fn decode(storage: &mut Storage, id: u16) -> Result<String, String> {
        if let Some(buf) = storage.get(id) {
            Ok(String::from_utf8_lossy(buf).to_string())
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl<T> Decode<T> for T where T: StructDecode,  {
    fn decode(storage: &mut Storage, id: u16) -> Result<T, String> {
        if let Some(buf) = storage.get(id) {
            let sctruct_storage = match Storage::new(buf.to_vec()) {
                Ok(storage) => storage,
                Err(e) => {
                    return Err(e);
                }
            };
            let mut strct: T = T::defaults();
            match strct.extract(sctruct_storage) {
                Ok(_) => Ok(strct),
                Err(e) => Err(e),
            }
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<u8>> for Vec<u8> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<u8>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<u8> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            loop {
                if cursor.position() == buf.len() as u64 {
                    break;
                }
                res.push(cursor.get_u8());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<u16>> for Vec<u16> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<u16>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<u16> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::U16_LEN as u64 {
                    return Err(format!("To extract u16 value from array buffer should have length at least {} bytes, but length is {}", sizes::U16_LEN, buf.len()));
                }
                res.push(cursor.get_u16_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<u32>> for Vec<u32> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<u32>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<u32> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::U32_LEN as u64 {
                    return Err(format!("To extract u32 value from array buffer should have length at least {} bytes, but length is {}", sizes::U32_LEN, buf.len()));
                }
                res.push(cursor.get_u32_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<u64>> for Vec<u64> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<u64>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<u64> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::U64_LEN as u64 {
                    return Err(format!("To extract u64 value from array buffer should have length at least {} bytes, but length is {}", sizes::U64_LEN, buf.len()));
                }
                res.push(cursor.get_u64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<i8>> for Vec<i8> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<i8>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<i8> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            loop {
                if cursor.position() == buf.len() as u64 {
                    break;
                }
                res.push(cursor.get_i8());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<i16>> for Vec<i16> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<i16>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<i16> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::I16_LEN as u64 {
                    return Err(format!("To extract i16 value from array buffer should have length at least {} bytes, but length is {}", sizes::I16_LEN, buf.len()));
                }
                res.push(cursor.get_i16_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<i32>> for Vec<i32> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<i32>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<i32> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::I32_LEN as u64 {
                    return Err(format!("To extract i32 value from array buffer should have length at least {} bytes, but length is {}", sizes::I32_LEN, buf.len()));
                }
                res.push(cursor.get_i32_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<i64>> for Vec<i64> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<i64>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<i64> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::I64_LEN as u64 {
                    return Err(format!("To extract i64 value from array buffer should have length at least {} bytes, but length is {}", sizes::I64_LEN, buf.len()));
                }
                res.push(cursor.get_i64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<f32>> for Vec<f32> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<f32>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<f32> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::F32_LEN as u64 {
                    return Err(format!("To extract f32 value from array buffer should have length at least {} bytes, but length is {}", sizes::F32_LEN, buf.len()));
                }
                res.push(cursor.get_f32_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<f64>> for Vec<f64> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<f64>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<f64> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < sizes::F64_LEN as u64 {
                    return Err(format!("To extract f64 value from array buffer should have length at least {} bytes, but length is {}", sizes::F64_LEN, buf.len()));
                }
                res.push(cursor.get_f64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl Decode<Vec<String>> for Vec<String> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<String>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<String> = vec!();
            let mut buffer = vec![0; buf.len()];
            buffer.copy_from_slice(&buf[0..buf.len()]);
            loop {
                if buffer.is_empty() {
                    break;
                }
                let mut cursor: Cursor<&[u8]> = Cursor::new(&buffer);
                if buffer.len() < sizes::U32_LEN {
                    return Err(format!("To extract length of string (u32) value from array buffer should have length at least {} bytes, but length is {}", sizes::U32_LEN, buf.len()));
                }
                let item_len: u32 = cursor.get_u32_le();
                if buffer.len() < sizes::U32_LEN + item_len as usize {
                    return Err(format!("Cannot extract string, because expecting {} bytes, but length of buffer is {}", item_len, (buffer.len() - sizes::U32_LEN)));
                }
                let mut item_buf = vec![0; item_len as usize];
                item_buf.copy_from_slice(&buffer[sizes::U32_LEN..(sizes::U32_LEN + item_len as usize)]);
                buffer = buffer.drain((sizes::U32_LEN + item_len as usize)..).collect();
                res.push(String::from_utf8_lossy(&item_buf).to_string());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl<T> Decode<Vec<T>> for Vec<T> where T: StructDecode {
    fn decode(storage: &mut Storage, id: u16) -> Result<Vec<T>, String> {
        if let Some(buf) = storage.get(id) {
            let mut res: Vec<T> = vec!();
            let mut buffer = vec![0; buf.len()];
            buffer.copy_from_slice(&buf[0..buf.len()]);
            loop {
                if buffer.is_empty() {
                    break;
                }
                let mut cursor: Cursor<&[u8]> = Cursor::new(&buffer);
                if buffer.len() < sizes::U64_LEN {
                    return Err(format!("To extract length of string (u64) value from array buffer should have length at least {} bytes, but length is {}", sizes::U64_LEN, buf.len()));
                }
                let item_len: u64 = cursor.get_u64_le();
                if buffer.len() < sizes::U64_LEN + item_len as usize {
                    return Err(format!("Cannot extract string, because expecting {} bytes, but length of buffer is {}", item_len, (buffer.len() - sizes::U64_LEN)));
                }
                let mut item_buf = vec![0; item_len as usize];
                item_buf.copy_from_slice(&buffer[sizes::U64_LEN..(sizes::U64_LEN + item_len as usize)]);
                buffer = buffer.drain((sizes::U64_LEN + item_len as usize)..).collect();
                let sctruct_storage = match Storage::new(item_buf) {
                    Ok(storage) => storage,
                    Err(e) => {
                        return Err(e);
                    }
                };
                let mut strct: T = T::defaults();
                match strct.extract(sctruct_storage) {
                    Ok(_) => {},
                    Err(e) => { return Err(e); },
                }
                res.push(strct);
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

impl<T> Decode<Option<T>> for Option<T> where T: Decode<T> {
    fn decode(storage: &mut Storage, id: u16) -> Result<Option<T>, String> {
        if let Some(buf) = storage.get(id) {
            if buf.is_empty() {
                Ok(None)
            } else {
                match T::decode(storage, id) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => Err(e),
                }
            }
        } else {
            Err(format!("Buffer for property {} isn't found", id))
        }
    }
}

fn get_value_buffer(id: u16, size: ESize, mut value: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec!();
    buffer.append(&mut id.to_le_bytes().to_vec());
    match size {
        ESize::U8(size) => {
            buffer.append(&mut (8 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U16(size) => {
            buffer.append(&mut (16 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U32(size) => {
            buffer.append(&mut (32 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
        ESize::U64(size) => {
            buffer.append(&mut (64 as u8).to_le_bytes().to_vec());
            buffer.append(&mut size.to_le_bytes().to_vec());
        },
    };
    buffer.append(&mut value);
    Ok(buffer)
}

pub trait StructEncode {

    fn get_id(&self) -> u32;
    fn abduct(&mut self) -> Result<Vec<u8>, String>;

}

pub trait EnumEncode {
    
    fn abduct(&mut self) -> Result<Vec<u8>, String>;
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e)
        }
    }

}

pub trait Encode {

    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String>;

}

impl Encode for u8 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U8_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u16 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U16_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u32 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u64 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i8 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I8_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i16 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I16_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i32 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i64 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for f32 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::F32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for f64 {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::F64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for bool {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::BOOL_LEN as u8), if self == &true { vec![1] } else { vec![0] })
    }
}

impl Encode for String {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let buf = self.as_bytes();
        get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec())
    }
}

impl<T> Encode for T where T: StructEncode {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e)
        }
    }
}

impl Encode for Vec<u8> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u16> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u32> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u64> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i8> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i16> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i32> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i64> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f32> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f64> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<String> {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            let val_as_bytes = val.as_bytes();
            buffer.append(&mut (val_as_bytes.len() as u32).to_le_bytes().to_vec());
            buffer.append(&mut val_as_bytes.to_vec());
        }
        get_value_buffer(id, ESize::U64(buffer.len() as u64), buffer.to_vec())
    }
}

impl<T> Encode for Vec<T> where T: StructEncode {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter_mut() {
            let val_as_bytes = match val.abduct() {
                Ok(buf) => buf,
                Err(e) => { return Err(e); }
            };
            buffer.append(&mut (val_as_bytes.len() as u64).to_le_bytes().to_vec());
            buffer.append(&mut val_as_bytes.to_vec());
        }
        get_value_buffer(id, ESize::U64(buffer.len() as u64), buffer.to_vec())
    }
}

impl<T> Encode for Option<T> where T: Encode {
    fn encode(&mut self, id: u16) -> Result<Vec<u8>, String> {
        match self {
            Some(v) => v.encode(id),
            None => get_value_buffer(id, ESize::U8(0), vec!()),
        }
    }
}

pub struct Storage {
    map: HashMap<u16, Vec<u8>>,
}

#[allow(dead_code)]
impl Storage {

    pub fn from(map: HashMap<u16, Vec<u8>>) -> Self {
        Storage { map }
    }

    pub fn new(buf: Vec<u8>) -> Result<Self, String> {
        /* 
        | PROP_ID  | PROP_BODY_LEN_GRAD | PROP_BODY_LEN | PROP_BODY | ... |
        | 2 bytes  | 1 byte             | 1 - 8 bytes   | n bytes   | ... |
        */
        let mut position: usize = 0;
        let mut map: HashMap<u16, Vec<u8>> = HashMap::new();
        loop {
            match Storage::next(&buf, position) {
                Ok((id, body, pos)) => {
                    position = pos;
                    map.insert(id, body);
                    if pos == buf.len() {
                        break;
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Storage {
            map
        })
    }

    fn id(buf: &[u8], pos: usize) -> Result<(u16, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        if let Ok(pos) = u64::try_from(pos) {
            cursor.set_position(pos);
        } else {
            return Err("Fail to set cursor position".to_string());
        }
        let id = cursor.get_u16_le();
        Ok((id, pos + sizes::U16_LEN))
    }

    fn body(buf: &[u8], pos: usize) -> Result<(Vec<u8>, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        if let Ok(pos) = u64::try_from(pos) {
            cursor.set_position(pos);
        } else {
            return Err("Fail to set cursor position".to_string());
        }
        let prop_body_len_rank = cursor.get_u8();
        let prop_body_len_usize: usize;
        let prop_rank_len: usize = 1;
        let prop_size_len: usize;
        match prop_body_len_rank {
            8 => if let Ok(val) = usize::try_from(cursor.get_u8()) {
                prop_body_len_usize = val;
                prop_size_len = sizes::U8_LEN;
            } else {
                return Err("Fail convert length of name from u8 to usize".to_string());
            }
            16 => if let Ok(val) = usize::try_from(cursor.get_u16_le()) {
                prop_body_len_usize = val;
                prop_size_len = sizes::U16_LEN;
            } else {
                return Err("Fail convert length of name from u16 to usize".to_string());
            },
            32 => if let Ok(val) = usize::try_from(cursor.get_u32_le()) {
                prop_body_len_usize = val;
                prop_size_len = sizes::U32_LEN;
            } else {
                return Err("Fail convert length of name from u32 to usize".to_string());
            },
            64 => if let Ok(val) = usize::try_from(cursor.get_u64_le()) {
                prop_body_len_usize = val;
                prop_size_len = sizes::U64_LEN;
            } else {
                return Err("Fail convert length of name from u64 to usize".to_string());
            },
            v => {
                return Err(format!("Unknown rank has been gotten: {}", v));
            }
        };
        let mut prop_body_buf = vec![0; prop_body_len_usize];
        prop_body_buf.copy_from_slice(&buf[(pos + prop_rank_len + prop_size_len)..(pos + prop_rank_len + prop_size_len + prop_body_len_usize)]);
        Ok((prop_body_buf, pos + prop_rank_len + prop_size_len + prop_body_len_usize))
    }

    fn next(buf: &[u8], pos: usize) -> Result<(u16, Vec<u8>, usize), String> {
        match Storage::id(buf, pos) {
            Ok((id, pos)) => {
                match Storage::body(buf, pos) {
                    Ok((body, pos)) => Ok((id, body, pos)),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn get(&mut self, id: u16) -> Option<&Vec<u8>> {
        self.map.get(&id)
    }

}


#[derive(Debug, Clone, PartialEq)]
pub enum EnumWithSctructs {
    a(OptionA),
    b(OptionB),
    Defaults,
}
impl EnumDecode<EnumWithSctructs> for EnumWithSctructs {
    fn extract(buf: Vec<u8>) -> Result<EnumWithSctructs, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumWithSctructs because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let id = cursor.get_u16_le();
        let mut storage = match Storage::new(buf) {
            Ok(s) => s,
            Err(e) => { return Err(e); }
        };
        match id {
            0 => match OptionA::decode(&mut storage, id) {
                Ok(v) => Ok(EnumWithSctructs::a(v)),
                Err(e) => Err(e)
            },
            1 => match OptionB::decode(&mut storage, id) {
                Ok(v) => Ok(EnumWithSctructs::b(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumWithSctructs")),
        }
    }
}
impl EnumEncode for EnumWithSctructs {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        match match self {
            Self::a(v) => v.encode(0),
            Self::b(v) => v.encode(1),
            _ => Err(String::from("Not supportable option")),
        } {
            Ok(buf) => Ok(buf),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxSugarEnum {
    VariantA(String),
    VariantB(String),
    VariantC(String),
    Defaults,
}
impl EnumDecode<SyntaxSugarEnum> for SyntaxSugarEnum {
    fn extract(buf: Vec<u8>) -> Result<SyntaxSugarEnum, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for SyntaxSugarEnum because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let id = cursor.get_u16_le();
        let mut storage = match Storage::new(buf) {
            Ok(s) => s,
            Err(e) => { return Err(e); }
        };
        match id {
            0 => match String::decode(&mut storage, id) {
                Ok(v) => Ok(SyntaxSugarEnum::VariantA(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&mut storage, id) {
                Ok(v) => Ok(SyntaxSugarEnum::VariantB(v)),
                Err(e) => Err(e)
            },
            2 => match String::decode(&mut storage, id) {
                Ok(v) => Ok(SyntaxSugarEnum::VariantC(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for SyntaxSugarEnum")),
        }
    }
}
impl EnumEncode for SyntaxSugarEnum {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        match match self {
            Self::VariantA(v) => v.encode(0),
            Self::VariantB(v) => v.encode(1),
            Self::VariantC(v) => v.encode(2),
            _ => Err(String::from("Not supportable option")),
        } {
            Ok(buf) => Ok(buf),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserType {
    PointA(Vec<u8>),
    PointB(String),
    PointC(u16),
    Defaults,
}
impl EnumDecode<UserType> for UserType {
    fn extract(buf: Vec<u8>) -> Result<UserType, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for UserType because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let id = cursor.get_u16_le();
        let mut storage = match Storage::new(buf) {
            Ok(s) => s,
            Err(e) => { return Err(e); }
        };
        match id {
            0 => match Vec::<u8>::decode(&mut storage, id) {
                Ok(v) => Ok(UserType::PointA(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&mut storage, id) {
                Ok(v) => Ok(UserType::PointB(v)),
                Err(e) => Err(e)
            },
            2 => match u16::decode(&mut storage, id) {
                Ok(v) => Ok(UserType::PointC(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for UserType")),
        }
    }
}
impl EnumEncode for UserType {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        match match self {
            Self::PointA(v) => v.encode(0),
            Self::PointB(v) => v.encode(1),
            Self::PointC(v) => v.encode(2),
            _ => Err(String::from("Not supportable option")),
        } {
            Ok(buf) => Ok(buf),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructName {
    pub age: u8,
    pub name: String,
}
impl StructDecode for StructName {
    fn get_id() -> u32 {
        1
    }
    fn defaults() -> StructName {
        StructName {
            age: 0,
            name: String::from(""),
        }
    }
    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.age = match u8::decode(&mut storage, 2) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.name = match String::decode(&mut storage, 3) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
impl StructEncode for StructName {
    fn get_id(&self) -> u32 {
        1
    }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.age.encode(2) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.name.encode(3) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionA {
    pub option_a_field_a: String,
    pub option_a_field_b: String,
}
impl StructDecode for OptionA {
    fn get_id() -> u32 {
        4
    }
    fn defaults() -> OptionA {
        OptionA {
            option_a_field_a: String::from(""),
            option_a_field_b: String::from(""),
        }
    }
    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.option_a_field_a = match String::decode(&mut storage, 5) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.option_a_field_b = match String::decode(&mut storage, 6) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
impl StructEncode for OptionA {
    fn get_id(&self) -> u32 {
        4
    }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.option_a_field_a.encode(5) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.option_a_field_b.encode(6) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionB {
    pub option_b_field_a: String,
    pub option_b_field_b: String,
}
impl StructDecode for OptionB {
    fn get_id() -> u32 {
        7
    }
    fn defaults() -> OptionB {
        OptionB {
            option_b_field_a: String::from(""),
            option_b_field_b: String::from(""),
        }
    }
    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.option_b_field_a = match String::decode(&mut storage, 8) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.option_b_field_b = match String::decode(&mut storage, 9) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
impl StructEncode for OptionB {
    fn get_id(&self) -> u32 {
        7
    }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.option_b_field_a.encode(8) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.option_b_field_b.encode(9) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: Vec<String>,
    pub email: Option<String>,
    pub usertype: UserType,
    pub info: StructName,
}
impl StructDecode for User {
    fn get_id() -> u32 {
        13
    }
    fn defaults() -> User {
        User {
            username: vec![],
            email: None,
            usertype: UserType::Defaults,
            info: StructName {
                age: 0,
                name: String::from(""),
            }
,
        }
    }
    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.username = match Vec::<String>::decode(&mut storage, 14) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.email = match Option::<String>::decode(&mut storage, 15) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.usertype = match UserType::decode(&mut storage, 16) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.info = match StructName::decode(&mut storage, 17) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
impl StructEncode for User {
    fn get_id(&self) -> u32 {
        13
    }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.username.encode(14) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.email.encode(15) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.usertype.encode(16) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.info.encode(17) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
    pub users: Vec<User>,
}
impl StructDecode for Login {
    fn get_id() -> u32 {
        18
    }
    fn defaults() -> Login {
        Login {
            users: vec![],
        }
    }
    fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
        self.users = match Vec::<User>::decode(&mut storage, 19) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
impl StructEncode for Login {
    fn get_id(&self) -> u32 {
        18
    }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.users.encode(19) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}

pub mod GroupA {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };

    #[derive(Debug, Clone, PartialEq)]
    pub enum UserTypeTest {
        PointA(u8),
        PointB(u8),
        PointC(u8),
        Defaults,
    }
    impl EnumDecode<UserTypeTest> for UserTypeTest {
        fn extract(buf: Vec<u8>) -> Result<UserTypeTest, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for UserTypeTest because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let id = cursor.get_u16_le();
            let mut storage = match Storage::new(buf) {
                Ok(s) => s,
                Err(e) => { return Err(e); }
            };
            match id {
                0 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointA(v)),
                    Err(e) => Err(e)
                },
                1 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointB(v)),
                    Err(e) => Err(e)
                },
                2 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointC(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for UserTypeTest")),
            }
        }
    }
    impl EnumEncode for UserTypeTest {
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            match match self {
                Self::PointA(v) => v.encode(0),
                Self::PointB(v) => v.encode(1),
                Self::PointC(v) => v.encode(2),
                _ => Err(String::from("Not supportable option")),
            } {
                Ok(buf) => Ok(buf),
                Err(e) => Err(e),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserA {
        pub username: Vec<String>,
        pub email: Option<String>,
        pub usertype: UserType,
    }
    impl StructDecode for UserA {
        fn get_id() -> u32 {
            21
        }
        fn defaults() -> UserA {
            UserA {
                username: vec![],
                email: None,
                usertype: UserType::Defaults,
            }
        }
        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match Vec::<String>::decode(&mut storage, 22) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.email = match Option::<String>::decode(&mut storage, 23) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.usertype = match UserType::decode(&mut storage, 24) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    impl StructEncode for UserA {
        fn get_id(&self) -> u32 {
            21
        }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.encode(22) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.email.encode(23) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.usertype.encode(24) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LoginA {
        pub users: Vec<User>,
    }
    impl StructDecode for LoginA {
        fn get_id() -> u32 {
            25
        }
        fn defaults() -> LoginA {
            LoginA {
                users: vec![],
            }
        }
        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.users = match Vec::<User>::decode(&mut storage, 26) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    impl StructEncode for LoginA {
        fn get_id(&self) -> u32 {
            25
        }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.users.encode(26) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }

}

pub mod GroupB {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };

    #[derive(Debug, Clone, PartialEq)]
    pub enum UserTypeTest {
        PointA(u8),
        PointB(u8),
        PointC(u8),
        Defaults,
    }
    impl EnumDecode<UserTypeTest> for UserTypeTest {
        fn extract(buf: Vec<u8>) -> Result<UserTypeTest, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for UserTypeTest because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let id = cursor.get_u16_le();
            let mut storage = match Storage::new(buf) {
                Ok(s) => s,
                Err(e) => { return Err(e); }
            };
            match id {
                0 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointA(v)),
                    Err(e) => Err(e)
                },
                1 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointB(v)),
                    Err(e) => Err(e)
                },
                2 => match u8::decode(&mut storage, id) {
                    Ok(v) => Ok(UserTypeTest::PointC(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for UserTypeTest")),
            }
        }
    }
    impl EnumEncode for UserTypeTest {
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            match match self {
                Self::PointA(v) => v.encode(0),
                Self::PointB(v) => v.encode(1),
                Self::PointC(v) => v.encode(2),
                _ => Err(String::from("Not supportable option")),
            } {
                Ok(buf) => Ok(buf),
                Err(e) => Err(e),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserA {
        pub username: Vec<String>,
        pub email: Option<String>,
        pub usertype: UserType,
    }
    impl StructDecode for UserA {
        fn get_id() -> u32 {
            29
        }
        fn defaults() -> UserA {
            UserA {
                username: vec![],
                email: None,
                usertype: UserType::Defaults,
            }
        }
        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match Vec::<String>::decode(&mut storage, 30) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.email = match Option::<String>::decode(&mut storage, 31) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.usertype = match UserType::decode(&mut storage, 32) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    impl StructEncode for UserA {
        fn get_id(&self) -> u32 {
            29
        }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.encode(30) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.email.encode(31) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.usertype.encode(32) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LoginA {
        pub users: Vec<User>,
    }
    impl StructDecode for LoginA {
        fn get_id() -> u32 {
            33
        }
        fn defaults() -> LoginA {
            LoginA {
                users: vec![],
            }
        }
        fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
            self.users = match Vec::<User>::decode(&mut storage, 34) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    impl StructEncode for LoginA {
        fn get_id(&self) -> u32 {
            33
        }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.users.encode(34) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }

    pub mod GroupC {
        use super::*;
        use std::io::Cursor;
        use bytes::{ Buf };

        #[derive(Debug, Clone, PartialEq)]
        pub enum UserTypeTest {
            PointA(u8),
            PointB(u8),
            PointC(u8),
            Defaults,
        }
        impl EnumDecode<UserTypeTest> for UserTypeTest {
            fn extract(buf: Vec<u8>) -> Result<UserTypeTest, String> {
                if buf.len() <= sizes::U16_LEN {
                    return Err(String::from("Fail to extract value for UserTypeTest because buffer too small"));
                }
                let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
                let id = cursor.get_u16_le();
                let mut storage = match Storage::new(buf) {
                    Ok(s) => s,
                    Err(e) => { return Err(e); }
                };
                match id {
                    0 => match u8::decode(&mut storage, id) {
                        Ok(v) => Ok(UserTypeTest::PointA(v)),
                        Err(e) => Err(e)
                    },
                    1 => match u8::decode(&mut storage, id) {
                        Ok(v) => Ok(UserTypeTest::PointB(v)),
                        Err(e) => Err(e)
                    },
                    2 => match u8::decode(&mut storage, id) {
                        Ok(v) => Ok(UserTypeTest::PointC(v)),
                        Err(e) => Err(e)
                    },
                    _ => Err(String::from("Fail to find relevant value for UserTypeTest")),
                }
            }
        }
        impl EnumEncode for UserTypeTest {
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                match match self {
                    Self::PointA(v) => v.encode(0),
                    Self::PointB(v) => v.encode(1),
                    Self::PointC(v) => v.encode(2),
                    _ => Err(String::from("Not supportable option")),
                } {
                    Ok(buf) => Ok(buf),
                    Err(e) => Err(e),
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UserA {
            pub username: Vec<String>,
            pub email: Option<String>,
            pub usertype: UserType,
        }
        impl StructDecode for UserA {
            fn get_id() -> u32 {
                37
            }
            fn defaults() -> UserA {
                UserA {
                    username: vec![],
                    email: None,
                    usertype: UserType::Defaults,
                }
            }
            fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
                self.username = match Vec::<String>::decode(&mut storage, 38) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.email = match Option::<String>::decode(&mut storage, 39) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.usertype = match UserType::decode(&mut storage, 40) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        impl StructEncode for UserA {
            fn get_id(&self) -> u32 {
                37
            }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.username.encode(38) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.email.encode(39) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.usertype.encode(40) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoginA {
            pub users: Vec<User>,
        }
        impl StructDecode for LoginA {
            fn get_id() -> u32 {
                41
            }
            fn defaults() -> LoginA {
                LoginA {
                    users: vec![],
                }
            }
            fn extract(&mut self, mut storage: Storage) -> Result<(), String> {
                self.users = match Vec::<User>::decode(&mut storage, 42) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        impl StructEncode for LoginA {
            fn get_id(&self) -> u32 {
                41
            }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.users.encode(42) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }

    }

}

