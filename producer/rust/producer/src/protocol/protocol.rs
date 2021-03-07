
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::convert::TryFrom;
use std::io::Cursor;
use std::collections::{ HashMap };
use bytes::{ Buf };
use std::time::{ SystemTime, UNIX_EPOCH };

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

pub enum Source<'a> {
    Storage(&'a mut Storage),
    Buffer(&'a Vec<u8>),
}

pub trait StructDecode where Self: Sized {

    fn get_id() -> u32;
    fn defaults() -> Self;
    fn extract_from_storage(&mut self, storage: Storage) -> Result<(), String>;
    fn extract(buf: Vec<u8>) -> Result<Self, String> {
        let mut instance: Self = Self::defaults();
        let storage = match Storage::new(buf) {
            Ok(storage) => storage,
            Err(e) => {
                return Err(e);
            }
        };
        match instance.extract_from_storage(storage) {
            Ok(()) => Ok(instance),
            Err(e) => Err(e),
        }
    }
}

pub trait EnumDecode {

    fn get_id(&self) -> u32;
    fn extract(buf: Vec<u8>) -> Result<Self, String> where Self: std::marker::Sized;

}

pub trait DecodeEnum<T> {

    fn get_from_storage(source: Source, id: Option<u16>) -> Result<T, String>;
    fn get_buf_from_source(source: Source, id: Option<u16>) -> Result<&Vec<u8>, String> {
        match source {
            Source::Storage(storage) => {
                if let Some(id) = id {
                    if let Some(buf) = storage.get(id) {
                        Ok(buf)
                    } else {
                        Err(format!("Buffer for property {} isn't found", id))
                    }
                } else {
                    Err("Storage defined as source, but no id is defined".to_string())
                }
            },
            Source::Buffer(buf) => Ok(buf),
        }
    }
    fn decode(buf: &Vec<u8>) -> Result<T, String> {
        Self::get_from_storage(Source::Buffer(buf), None)
    }
}

impl<T> DecodeEnum<T> for T where T: EnumDecode,  {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<T, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            Self::extract(buf.clone())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> DecodeEnum<Vec<T>> for Vec<T> where T: EnumDecode {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<T>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
                match T::extract(item_buf) {
                    Ok(i) => res.push(i),
                    Err(e) => { return Err(e); },
                }
            }
            Ok(res)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

pub trait Decode<T> {

    fn get_from_storage(source: Source, id: Option<u16>) -> Result<T, String>;
    fn get_buf_from_source(source: Source, id: Option<u16>) -> Result<&Vec<u8>, String> {
        match source {
            Source::Storage(storage) => {
                if let Some(id) = id {
                    if let Some(buf) = storage.get(id) {
                        Ok(buf)
                    } else {
                        Err(format!("Buffer for property {} isn't found", id))
                    }
                } else {
                    Err("Storage defined as source, but no id is defined".to_string())
                }
            },
            Source::Buffer(buf) => Ok(buf),
        }
    }
    fn decode(buf: &Vec<u8>) -> Result<T, String> {
        Self::get_from_storage(Source::Buffer(buf), None)
    }

}

impl Decode<u8> for u8 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<u8, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}. Prop {:?}", sizes::U8_LEN, buf.len(), id));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<u16> for u16 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<u16, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::U16_LEN {
                return Err(format!("To extract u16 value buffer should have length at least {} bytes, but length is {}", sizes::U16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u16_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<u32> for u32 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<u32, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::U32_LEN {
                return Err(format!("To extract u32 value buffer should have length at least {} bytes, but length is {}", sizes::U32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u32_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<u64> for u64 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<u64, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::U64_LEN {
                return Err(format!("To extract u64 value buffer should have length at least {} bytes, but length is {}", sizes::U64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u64_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<i8> for i8 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<i8, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::I8_LEN {
                return Err(format!("To extract i8 value buffer should have length at least {} bytes, but length is {}", sizes::I8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i8())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<i16> for i16 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<i16, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::I16_LEN {
                return Err(format!("To extract i16 value buffer should have length at least {} bytes, but length is {}", sizes::I16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i16_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<i32> for i32 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<i32, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::I32_LEN {
                return Err(format!("To extract i32 value buffer should have length at least {} bytes, but length is {}", sizes::I32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i32_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<i64> for i64 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<i64, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::I64_LEN {
                return Err(format!("To extract i64 value buffer should have length at least {} bytes, but length is {}", sizes::I64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i64_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<f32> for f32 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<f32, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::F32_LEN {
                return Err(format!("To extract f32 value buffer should have length at least {} bytes, but length is {}", sizes::F32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f32_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<f64> for f64 {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<f64, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::F64_LEN {
                return Err(format!("To extract f64 value buffer should have length at least {} bytes, but length is {}", sizes::F64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f64_le())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<bool> for bool {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<bool, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8() != 0)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<String> for String {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<String, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            Ok(String::from_utf8_lossy(buf).to_string())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> Decode<T> for T where T: StructDecode,  {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<T, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            let sctruct_storage = match Storage::new(buf.to_vec()) {
                Ok(storage) => storage,
                Err(e) => {
                    return Err(e);
                }
            };
            let mut strct: T = T::defaults();
            match strct.extract_from_storage(sctruct_storage) {
                Ok(_) => Ok(strct),
                Err(e) => Err(e),
            }
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<u8>> for Vec<u8> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<u8>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<u16>> for Vec<u16> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<u16>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<u32>> for Vec<u32> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<u32>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<u64>> for Vec<u64> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<u64>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<i8>> for Vec<i8> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<i8>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<i16>> for Vec<i16> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<i16>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<i32>> for Vec<i32> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<i32>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<i64>> for Vec<i64> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<i64>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<f32>> for Vec<f32> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<f32>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<f64>> for Vec<f64> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<f64>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<bool>> for Vec<bool> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<bool>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            let mut res: Vec<bool> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            loop {
                if cursor.position() == buf.len() as u64 {
                    break;
                }
                res.push(cursor.get_u8() != 0);
            }
            Ok(res)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl Decode<Vec<String>> for Vec<String> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<String>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> Decode<Vec<T>> for Vec<T> where T: StructDecode {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<T>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
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
                match strct.extract_from_storage(sctruct_storage) {
                    Ok(_) => {},
                    Err(e) => { return Err(e); },
                }
                res.push(strct);
            }
            Ok(res)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> Decode<Option<T>> for Option<T> where T: Decode<T> {
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Option<T>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            if buf.is_empty() {
                Ok(None)
            } else {
                match T::get_from_storage(Source::Buffer(buf), id) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => Err(e),
                }
            }
        } else {
            Err("Fail get buffer".to_string())
        }
        
    }
}

fn get_value_buffer(id: Option<u16>, size: ESize, mut value: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut buffer: Vec<u8> = vec!();
    if let Some(id) = id {
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
    }
    buffer.append(&mut value);
    Ok(buffer)
}

pub fn get_empty_buffer_val(id: Option<u16>) -> Result<Vec<u8>, String> {
    get_value_buffer(id, ESize::U8(0), vec!())
}

pub trait StructEncode {

    fn get_id(&self) -> u32;
    fn get_signature(&self) -> u16;
    fn abduct(&mut self) -> Result<Vec<u8>, String>;

}

pub trait EnumEncode {
    
    fn get_id(&self) -> u32;
    fn get_signature(&self) -> u16;
    fn abduct(&mut self) -> Result<Vec<u8>, String>;

}

pub trait EncodeEnum {

    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String>;
    fn encode(&mut self) -> Result<Vec<u8>, String> {
        self.get_buf_to_store(None)
    }
}

impl<T> EncodeEnum for T where T: EnumEncode {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e)
        }
    }
}

impl<T> EncodeEnum for Vec<T> where T: EnumEncode {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
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

pub trait Encode {

    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String>;
    fn encode(&mut self) -> Result<Vec<u8>, String> {
        self.get_buf_to_store(None)
    }
}

impl Encode for u8 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U8_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u16 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U16_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for u64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::U64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i8 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I8_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i16 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I16_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for i64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::I64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for f32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::F32_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for f64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::F64_LEN as u8), self.to_le_bytes().to_vec())
    }
}

impl Encode for bool {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(id, ESize::U8(sizes::BOOL_LEN as u8), if self == &true { vec![1] } else { vec![0] })
    }
}

impl Encode for String {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let buf = self.as_bytes();
        get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec())
    }
}

impl<T> Encode for T where T: StructEncode {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e)
        }
    }
}

impl Encode for Vec<u8> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u16> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i8> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i16> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<String> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            let val_as_bytes = val.as_bytes();
            buffer.append(&mut (val_as_bytes.len() as u32).to_le_bytes().to_vec());
            buffer.append(&mut val_as_bytes.to_vec());
        }
        get_value_buffer(id, ESize::U64(buffer.len() as u64), buffer.to_vec())
    }
}

impl Encode for Vec<bool> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in self.iter() {
            let byte: u8 = if val.clone() {
                1
            } else {
                0
            };
            buffer.append(&mut byte.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl<T> Encode for Vec<T> where T: StructEncode {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
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
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        match self {
            Some(v) => v.get_buf_to_store(id),
            None => get_empty_buffer_val(id),
        }
    }
}

#[derive(Debug, Clone)]
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
        if buf.len() > 0 {
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

const MSG_HEADER_LEN: usize =   sizes::U32_LEN + // {u32} message ID
                                sizes::U16_LEN + // {u16} signature
                                sizes::U64_LEN + // {u64} body size
                                sizes::U64_LEN;  // {u64} timestamp

#[derive(Debug, Clone)]
pub struct PackageHeader {
    pub id: u32,
    pub signature: u16,
    pub len: u64,
    pub ts: u64,
    pub len_usize: usize,
}

pub fn has_buffer_header(buf: &[u8]) -> bool {
    buf.len() > MSG_HEADER_LEN
}

pub fn get_header_from_buffer(buf: &[u8]) -> Result<PackageHeader, String> {
    let mut header = Cursor::new(buf);
    if buf.len() < MSG_HEADER_LEN {
        return Err(format!("Cannot extract header of package because size of header {} bytes, but size of buffer {} bytes.", MSG_HEADER_LEN, buf.len()));
    }
    // Get message id
    let id: u32 = header.get_u32_le();
    // Get signature
    let signature: u16 = header.get_u16_le();
    // Get timestamp
    let ts: u64 = header.get_u64_le();
    // Get length of payload and payload
    let len: u64 = header.get_u64_le();
    let len_usize = match usize::try_from(len) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };
    Ok(PackageHeader { id, signature, ts, len, len_usize })
}

pub fn has_buffer_body(buf: &[u8], header: &PackageHeader) -> bool {
    buf.len() >= header.len_usize + MSG_HEADER_LEN
}

pub fn get_body_from_buffer(buf: &[u8], header: &PackageHeader) -> Result<(Vec<u8>, Vec<u8>), String> {
    if buf.len() < header.len_usize + MSG_HEADER_LEN {
        return Err(format!("Cannot extract body of package because size in header {} bytes, but size of buffer {} bytes.", header.len, buf.len() - MSG_HEADER_LEN));
    }
    // Get body
    let mut body = vec![0; header.len_usize];
    body.copy_from_slice(&buf[MSG_HEADER_LEN..(MSG_HEADER_LEN + header.len_usize)]);
    let mut rest = vec![0; buf.len() - MSG_HEADER_LEN - header.len_usize];
    rest.copy_from_slice(&buf[(MSG_HEADER_LEN + header.len_usize)..]);
    Ok((body, rest))
}

pub fn pack<T>(mut msg: T) -> Result<Vec<u8>, String> where T: StructEncode {
    match msg.abduct() {
        Ok(buffer) => pack_buffer(msg.get_id(), msg.get_signature(), buffer),
        Err(e) => Err(e),
    }
}

pub fn pack_buffer(msg_id: u32, signature: u16, msg_buf: Vec<u8>) -> Result<Vec<u8>, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let mut buf: Vec<u8> = vec!();
            buf.append(&mut msg_id.to_le_bytes().to_vec());
            buf.append(&mut signature.to_le_bytes().to_vec());
            buf.append(&mut duration.as_secs().to_le_bytes().to_vec());
            buf.append(&mut (msg_buf.len() as u64).to_le_bytes().to_vec());
            buf.append(&mut msg_buf.to_vec());
            Ok(buf)
        },
        Err(e) => Err(e.to_string()),
    }
}

pub trait PackingStruct: StructEncode {

    fn pack(&mut self) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), buf),
            Err(e) => Err(e),
        }
    }

}

pub trait PackingEnum: EnumEncode {

    fn pack(&mut self) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), buf),
            Err(e) => Err(e),
        }
    }

}

#[derive(Debug)]
pub enum ReadError {
    Header(String),
    Parsing(String),
    Signature(String),
}

#[derive(Clone)]
pub struct IncomeMessage<T: Clone> {
    pub header: PackageHeader,
    pub msg: T,
}

pub trait DecodeBuffer<T> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<T, String>;
    fn get_signature(&self) -> u16;
}

pub struct Buffer<T: Clone> {
    buffer: Vec<u8>,
    queue: Vec<IncomeMessage<T>>,
}

#[allow(clippy::len_without_is_empty)]
#[allow(clippy::new_without_default)]
impl<T: Clone> Buffer<T>
where
    Self: DecodeBuffer<T>,
{
    fn get_message(&self, header: &PackageHeader, buf: &[u8]) -> Result<T, ReadError> {
        if self.get_signature() != header.signature {
            Err(ReadError::Signature(format!(
                "Signature dismatch; expectation: {}; message: {}",
                self.get_signature(),
                header.signature
            )))
        } else {
            match self.get_msg(header.id, buf) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(ReadError::Parsing(format!(
                    "Fail get message id={}, signature={} due error: {}",
                    header.id, header.signature, e
                ))),
            }
        }
    }

    pub fn new() -> Self {
        Buffer {
            buffer: vec![],
            queue: vec![],
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn chunk(&mut self, buf: &Vec<u8>) -> Result<(), ReadError> {
        // Add data into buffer
        self.buffer.append(&mut buf.clone());
        if !has_buffer_header(&self.buffer) {
            return Ok(());
        }
        // Get header
        let header: PackageHeader = match get_header_from_buffer(&self.buffer) {
            Ok(v) => v,
            Err(e) => {
                return Err(ReadError::Header(e));
            }
        };
        if !has_buffer_body(&self.buffer, &header) {
            return Ok(());
        }
        let (body, rest) = match get_body_from_buffer(&self.buffer, &header) {
            Ok(v) => v,
            Err(e) => {
                return Err(ReadError::Parsing(e));
            }
        };
        self.buffer = rest;
        match Self::get_message(self, &header, &body) {
            Ok(msg) => {
                self.queue.push(IncomeMessage { header, msg });
                if !self.buffer.is_empty() {
                    self.chunk(&vec![])
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<IncomeMessage<T>> {
        if self.queue.is_empty() {
            return None;
        }
        let message = Some(self.queue[0].clone());
        if self.queue.len() > 1 {
            self.queue = self.queue.drain(1..).collect();
        } else {
            self.queue.clear();
        }
        message
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }
}


#[derive(Debug, Clone)]
pub enum AvailableMessages {
    UserRole(UserRole),
    UserConnected(UserConnected),
    UserDisconnected(UserDisconnected),
    UserSignIn(UserSignIn::AvailableMessages),
    UserJoin(UserJoin::AvailableMessages),
    UserLogout(UserLogout::AvailableMessages),
}
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin(String),
    User(String),
    Manager(String),
    Defaults,
}
impl EnumDecode for UserRole {
    fn get_id(&self) -> u32 { 1 }
    fn extract(buf: Vec<u8>) -> Result<UserRole, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for UserRole because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::Admin(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::User(v)),
                Err(e) => Err(e)
            },
            2 => match String::decode(&body_buf) {
                Ok(v) => Ok(UserRole::Manager(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for UserRole")),
        }
    }
}
impl EnumEncode for UserRole {
    fn get_id(&self) -> u32 { 1 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let (buf, index) = match self {
            Self::Admin(v) => (v.encode(), 0),
            Self::User(v) => (v.encode(), 1),
            Self::Manager(v) => (v.encode(), 2),
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
impl PackingEnum for UserRole {}

#[derive(Debug, Clone, PartialEq)]
pub struct UserConnected {
    pub username: String,
    pub uuid: String,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for UserConnected {
    fn get_id() -> u32 {
        2
    }
    fn defaults() -> UserConnected {
        UserConnected {
            username: String::from(""),
            uuid: String::from(""),
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(3)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(4)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for UserConnected {
    fn get_id(&self) -> u32 { 2 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.username.get_buf_to_store(Some(3)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.uuid.get_buf_to_store(Some(4)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for UserConnected { }

#[derive(Debug, Clone, PartialEq)]
pub struct UserDisconnected {
    pub username: String,
    pub uuid: String,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for UserDisconnected {
    fn get_id() -> u32 {
        5
    }
    fn defaults() -> UserDisconnected {
        UserDisconnected {
            username: String::from(""),
            uuid: String::from(""),
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(6)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(7)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for UserDisconnected {
    fn get_id(&self) -> u32 { 5 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.username.get_buf_to_store(Some(6)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.uuid.get_buf_to_store(Some(7)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for UserDisconnected { }

pub mod UserSignIn {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Accepted(Accepted),
        Denied(Denied),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub email: String,
        pub hash: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            9
        }
        fn defaults() -> Request {
            Request {
                email: String::from(""),
                hash: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.email = match String::get_from_storage(Source::Storage(&mut storage), Some(10)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.hash = match String::get_from_storage(Source::Storage(&mut storage), Some(11)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 9 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.email.get_buf_to_store(Some(10)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.hash.get_buf_to_store(Some(11)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            12
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(13)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 12 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(13)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Accepted { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Denied {
        pub reason: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Denied {
        fn get_id() -> u32 {
            14
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(15)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 14 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(15)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Denied { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            16
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(17)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 16 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(17)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserJoin {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Accepted(Accepted),
        Denied(Denied),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub email: String,
        pub username: String,
        pub role: UserRole,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            19
        }
        fn defaults() -> Request {
            Request {
                email: String::from(""),
                username: String::from(""),
                role: UserRole::Defaults,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.email = match String::get_from_storage(Source::Storage(&mut storage), Some(20)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(21)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.role = match UserRole::get_from_storage(Source::Storage(&mut storage), Some(22)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 19 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.email.get_buf_to_store(Some(20)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.username.get_buf_to_store(Some(21)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.role.get_buf_to_store(Some(22)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            23
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(24)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 23 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(24)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Accepted { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Denied {
        pub reason: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Denied {
        fn get_id() -> u32 {
            25
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(26)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 25 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(26)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Denied { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            27
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(28)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 27 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(28)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserLogout {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Request(Request),
        Done(Done),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            30
        }
        fn defaults() -> Request {
            Request {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(31)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 30 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(31)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Done {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Done {
        fn get_id() -> u32 {
            32
        }
        fn defaults() -> Done {
            Done {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Done {
        fn get_id(&self) -> u32 { 32 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Done { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            33
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(34)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 33 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(34)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {
        match id {
            1 => match UserRole::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserRole(m)),
                Err(e) => Err(e),
            },
            2 => match UserConnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserConnected(m)),
                Err(e) => Err(e),
            },
            5 => match UserDisconnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserDisconnected(m)),
                Err(e) => Err(e),
            },
            9 => match UserSignIn::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserSignIn(UserSignIn::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            12 => match UserSignIn::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserSignIn(UserSignIn::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            14 => match UserSignIn::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserSignIn(UserSignIn::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            16 => match UserSignIn::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserSignIn(UserSignIn::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            19 => match UserJoin::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserJoin(UserJoin::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            23 => match UserJoin::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserJoin(UserJoin::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            25 => match UserJoin::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserJoin(UserJoin::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            27 => match UserJoin::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserJoin(UserJoin::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            30 => match UserLogout::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogout(UserLogout::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            32 => match UserLogout::Done::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogout(UserLogout::AvailableMessages::Done(m))),
                Err(e) => Err(e),
            },
            33 => match UserLogout::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogout(UserLogout::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

