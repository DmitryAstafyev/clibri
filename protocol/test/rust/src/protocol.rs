
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
                                sizes::U32_LEN + // {u32} sequence
                                sizes::U64_LEN + // {u64} body size
                                sizes::U64_LEN;  // {u64} timestamp

#[derive(Debug, Clone)]
pub struct PackageHeader {
    pub id: u32,
    pub signature: u16,
    pub sequence: u32,
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
    // Get sequence
    let sequence: u32 = header.get_u32_le();
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
    Ok(PackageHeader { id, signature, sequence, ts, len, len_usize })
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

pub fn pack<T>(mut msg: T, sequence: u32) -> Result<Vec<u8>, String> where T: StructEncode {
    match msg.abduct() {
        Ok(buffer) => pack_buffer(msg.get_id(), msg.get_signature(), sequence, buffer),
        Err(e) => Err(e),
    }
}

pub fn pack_buffer(msg_id: u32, signature: u16, sequence: u32, msg_buf: Vec<u8>) -> Result<Vec<u8>, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let mut buf: Vec<u8> = vec!();
            buf.append(&mut msg_id.to_le_bytes().to_vec());
            buf.append(&mut signature.to_le_bytes().to_vec());
            buf.append(&mut sequence.to_le_bytes().to_vec());
            buf.append(&mut duration.as_secs().to_le_bytes().to_vec());
            buf.append(&mut (msg_buf.len() as u64).to_le_bytes().to_vec());
            buf.append(&mut msg_buf.to_vec());
            Ok(buf)
        },
        Err(e) => Err(e.to_string()),
    }
}

pub trait PackingStruct: StructEncode {

    fn pack(&mut self, sequence: u32) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf),
            Err(e) => Err(e),
        }
    }

}

pub trait PackingEnum: EnumEncode {

    fn pack(&mut self, sequence: u32) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf),
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
    StructExampleEmpty(StructExampleEmpty),
    StructExampleJ(StructExampleJ),
    GroupA(GroupA::AvailableMessages),
    GroupB(GroupB::AvailableMessages),
    GroupD(GroupD::AvailableMessages),
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
pub struct StructExampleEmpty {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleEmpty {
    fn get_id() -> u32 {
        67
    }
    fn defaults() -> StructExampleEmpty {
        StructExampleEmpty {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleEmpty {
    fn get_id(&self) -> u32 { 67 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for StructExampleEmpty { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructExampleJ {
    pub field_a: Option<StructExampleA>,
    pub field_b: Option<StructExampleB>,
    pub field_c: StructExampleEmpty,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructExampleJ {
    fn get_id() -> u32 {
        68
    }
    fn defaults() -> StructExampleJ {
        StructExampleJ {
            field_a: None,
            field_b: None,
            field_c: StructExampleEmpty {
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match Option::<StructExampleA>::get_from_storage(Source::Storage(&mut storage), Some(69)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match Option::<StructExampleB>::get_from_storage(Source::Storage(&mut storage), Some(70)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_c = match StructExampleEmpty::get_from_storage(Source::Storage(&mut storage), Some(71)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructExampleJ {
    fn get_id(&self) -> u32 { 68 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(69)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(70)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_c.get_buf_to_store(Some(71)) {
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
        fn get_id(&self) -> u32 { 73 }
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
        fn get_id(&self) -> u32 { 73 }
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
            74
        }
        fn defaults() -> StructExampleA {
            StructExampleA {
                field_u8: 0,
                field_u16: 0,
                opt: EnumExampleA::Defaults,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(75)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(76)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.opt = match EnumExampleA::get_from_storage(Source::Storage(&mut storage), Some(77)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleA {
        fn get_id(&self) -> u32 { 74 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(75)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(76)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.opt.get_buf_to_store(Some(77)) {
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
            78
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
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(79)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(80)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.strct = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(81)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleB {
        fn get_id(&self) -> u32 { 78 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(79)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(80)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.strct.get_buf_to_store(Some(81)) {
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
            83
        }
        fn defaults() -> StructExampleA {
            StructExampleA {
                field_u8: 0,
                field_u16: 0,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(84)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(85)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleA {
        fn get_id(&self) -> u32 { 83 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(84)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(85)) {
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
                87
            }
            fn defaults() -> StructExampleA {
                StructExampleA {
                    field_u8: 0,
                    field_u16: 0,
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(88)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(89)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructExampleA {
            fn get_id(&self) -> u32 { 87 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(88)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(89)) {
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
                90
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
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(91)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(92)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.strct = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(93)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructExampleB {
            fn get_id(&self) -> u32 { 90 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(91)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(92)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.strct.get_buf_to_store(Some(93)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for StructExampleB { }

    }

}

pub mod GroupD {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        EnumExampleP(EnumExampleP),
        StructExampleP(StructExampleP),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum EnumExampleP {
        Option_a(StructExampleA),
        Option_b(StructExampleP),
        Option_c(GroupB::StructExampleA),
        Option_d(GroupB::GroupC::StructExampleA),
        Defaults,
    }
    impl EnumDecode for EnumExampleP {
        fn get_id(&self) -> u32 { 99 }
        fn extract(buf: Vec<u8>) -> Result<EnumExampleP, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for EnumExampleP because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let index = cursor.get_u16_le();
            let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
            body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
            match index {
                0 => match StructExampleA::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleP::Option_a(v)),
                    Err(e) => Err(e)
                },
                1 => match StructExampleP::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleP::Option_b(v)),
                    Err(e) => Err(e)
                },
                2 => match GroupB::StructExampleA::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleP::Option_c(v)),
                    Err(e) => Err(e)
                },
                3 => match GroupB::GroupC::StructExampleA::decode(&body_buf) {
                    Ok(v) => Ok(EnumExampleP::Option_d(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for EnumExampleP")),
            }
        }
    }
    impl EnumEncode for EnumExampleP {
        fn get_id(&self) -> u32 { 99 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let (buf, index) = match self {
                Self::Option_a(v) => (v.encode(), 0),
                Self::Option_b(v) => (v.encode(), 1),
                Self::Option_c(v) => (v.encode(), 2),
                Self::Option_d(v) => (v.encode(), 3),
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
    impl PackingEnum for EnumExampleP {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructExampleP {
        pub field_a: StructExampleA,
        pub field_b: GroupB::StructExampleA,
        pub field_c: GroupB::GroupC::StructExampleA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructExampleP {
        fn get_id() -> u32 {
            95
        }
        fn defaults() -> StructExampleP {
            StructExampleP {
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
                field_b: GroupB::StructExampleA {
                    field_u8: 0,
                    field_u16: 0,
                },
                field_c: GroupB::GroupC::StructExampleA {
                    field_u8: 0,
                    field_u16: 0,
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_a = match StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(96)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_b = match GroupB::StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(97)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_c = match GroupB::GroupC::StructExampleA::get_from_storage(Source::Storage(&mut storage), Some(98)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructExampleP {
        fn get_id(&self) -> u32 { 95 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_a.get_buf_to_store(Some(96)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_b.get_buf_to_store(Some(97)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_c.get_buf_to_store(Some(98)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructExampleP { }

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
            73 => match GroupA::EnumExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::EnumExampleA(m))),
                Err(e) => Err(e),
            },
            99 => match GroupD::EnumExampleP::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupD(GroupD::AvailableMessages::EnumExampleP(m))),
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
            67 => match StructExampleEmpty::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleEmpty(m)),
                Err(e) => Err(e),
            },
            68 => match StructExampleJ::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructExampleJ(m)),
                Err(e) => Err(e),
            },
            74 => match GroupA::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructExampleA(m))),
                Err(e) => Err(e),
            },
            78 => match GroupA::StructExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructExampleB(m))),
                Err(e) => Err(e),
            },
            83 => match GroupB::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::StructExampleA(m))),
                Err(e) => Err(e),
            },
            87 => match GroupB::GroupC::StructExampleA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructExampleA(m)))),
                Err(e) => Err(e),
            },
            90 => match GroupB::GroupC::StructExampleB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructExampleB(m)))),
                Err(e) => Err(e),
            },
            95 => match GroupD::StructExampleP::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupD(GroupD::AvailableMessages::StructExampleP(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

