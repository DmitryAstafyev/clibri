
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
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

pub trait StructDecode
where
    Self: Sized,
{
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
    fn extract(buf: Vec<u8>) -> Result<Self, String>
    where
        Self: std::marker::Sized;
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
            }
            Source::Buffer(buf) => Ok(buf),
        }
    }
    fn decode(buf: &[u8]) -> Result<T, String> {
        Self::get_from_storage(Source::Buffer(&buf.to_vec()), None)
    }
}

impl<T> DecodeEnum<T> for T
where
    T: EnumDecode,
{
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<T, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            Self::extract(buf.clone())
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> DecodeEnum<Vec<T>> for Vec<T>
where
    T: EnumDecode,
{
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<T>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            let mut res: Vec<T> = vec![];
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
                item_buf
                    .copy_from_slice(&buffer[sizes::U64_LEN..(sizes::U64_LEN + item_len as usize)]);
                buffer = buffer
                    .drain((sizes::U64_LEN + item_len as usize)..)
                    .collect();
                match T::extract(item_buf) {
                    Ok(i) => res.push(i),
                    Err(e) => {
                        return Err(e);
                    }
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
            }
            Source::Buffer(buf) => Ok(buf),
        }
    }
    fn decode(buf: &[u8]) -> Result<T, String> {
        Self::get_from_storage(Source::Buffer(&buf.to_vec()), None)
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

impl<T> Decode<T> for T
where
    T: StructDecode,
{
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
            let mut res: Vec<u8> = vec![];
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
            let mut res: Vec<u16> = vec![];
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
            let mut res: Vec<u32> = vec![];
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
            let mut res: Vec<u64> = vec![];
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
            let mut res: Vec<i8> = vec![];
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
            let mut res: Vec<i16> = vec![];
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
            let mut res: Vec<i32> = vec![];
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
            let mut res: Vec<i64> = vec![];
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
            let mut res: Vec<f32> = vec![];
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
            let mut res: Vec<f64> = vec![];
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
            let mut res: Vec<bool> = vec![];
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
            let mut res: Vec<String> = vec![];
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
                item_buf
                    .copy_from_slice(&buffer[sizes::U32_LEN..(sizes::U32_LEN + item_len as usize)]);
                buffer = buffer
                    .drain((sizes::U32_LEN + item_len as usize)..)
                    .collect();
                res.push(String::from_utf8_lossy(&item_buf).to_string());
            }
            Ok(res)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> Decode<Vec<T>> for Vec<T>
where
    T: StructDecode,
{
    fn get_from_storage(source: Source, id: Option<u16>) -> Result<Vec<T>, String> {
        if let Ok(buf) = Self::get_buf_from_source(source, id) {
            let mut res: Vec<T> = vec![];
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
                item_buf
                    .copy_from_slice(&buffer[sizes::U64_LEN..(sizes::U64_LEN + item_len as usize)]);
                buffer = buffer
                    .drain((sizes::U64_LEN + item_len as usize)..)
                    .collect();
                let sctruct_storage = match Storage::new(item_buf) {
                    Ok(storage) => storage,
                    Err(e) => {
                        return Err(e);
                    }
                };
                let mut strct: T = T::defaults();
                match strct.extract_from_storage(sctruct_storage) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
                res.push(strct);
            }
            Ok(res)
        } else {
            Err("Fail get buffer".to_string())
        }
    }
}

impl<T> Decode<Option<T>> for Option<T>
where
    T: Decode<T>,
{
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
    let mut buffer: Vec<u8> = vec![];
    if let Some(id) = id {
        buffer.append(&mut id.to_le_bytes().to_vec());
        match size {
            ESize::U8(size) => {
                buffer.append(&mut 8_u8.to_le_bytes().to_vec());
                buffer.append(&mut size.to_le_bytes().to_vec());
            }
            ESize::U16(size) => {
                buffer.append(&mut 16_u8.to_le_bytes().to_vec());
                buffer.append(&mut size.to_le_bytes().to_vec());
            }
            ESize::U32(size) => {
                buffer.append(&mut 32_u8.to_le_bytes().to_vec());
                buffer.append(&mut size.to_le_bytes().to_vec());
            }
            ESize::U64(size) => {
                buffer.append(&mut 64_u8.to_le_bytes().to_vec());
                buffer.append(&mut size.to_le_bytes().to_vec());
            }
        };
    }
    buffer.append(&mut value);
    Ok(buffer)
}

pub fn get_empty_buffer_val(id: Option<u16>) -> Result<Vec<u8>, String> {
    get_value_buffer(id, ESize::U8(0), vec![])
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

impl<T> EncodeEnum for T
where
    T: EnumEncode,
{
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e),
        }
    }
}

impl<T> EncodeEnum for Vec<T>
where
    T: EnumEncode,
{
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter_mut() {
            let val_as_bytes = match val.abduct() {
                Ok(buf) => buf,
                Err(e) => {
                    return Err(e);
                }
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
        get_value_buffer(
            id,
            ESize::U8(sizes::U8_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for u16 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::U16_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for u32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::U32_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for u64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::U64_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for i8 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::I8_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for i16 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::I16_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for i32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::I32_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for i64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::I64_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for f32 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::F32_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for f64 {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::F64_LEN as u8),
            self.to_le_bytes().to_vec(),
        )
    }
}

impl Encode for bool {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        get_value_buffer(
            id,
            ESize::U8(sizes::BOOL_LEN as u8),
            if self == &true { vec![1] } else { vec![0] },
        )
    }
}

impl Encode for String {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let buf = self.as_bytes();
        get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec())
    }
}

impl<T> Encode for T
where
    T: StructEncode,
{
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => get_value_buffer(id, ESize::U64(buf.len() as u64), buf.to_vec()),
            Err(e) => Err(e),
        }
    }
}

impl Encode for Vec<u8> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U8_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u16> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U16_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U32_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<u64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::U64_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i8> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I8_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i16> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I16_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I32_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<i64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::I64_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f32> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F32_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<f64> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let len = self.len() * sizes::F64_LEN;
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl Encode for Vec<String> {
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec![];
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
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter() {
            let byte: u8 = if *val { 1 } else { 0 };
            buffer.append(&mut byte.to_le_bytes().to_vec());
        }
        get_value_buffer(id, ESize::U64(len as u64), buffer.to_vec())
    }
}

impl<T> Encode for Vec<T>
where
    T: StructEncode,
{
    fn get_buf_to_store(&mut self, id: Option<u16>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec![];
        for val in self.iter_mut() {
            let val_as_bytes = match val.abduct() {
                Ok(buf) => buf,
                Err(e) => {
                    return Err(e);
                }
            };
            buffer.append(&mut (val_as_bytes.len() as u64).to_le_bytes().to_vec());
            buffer.append(&mut val_as_bytes.to_vec());
        }
        get_value_buffer(id, ESize::U64(buffer.len() as u64), buffer.to_vec())
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
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
        if !buf.is_empty() {
            loop {
                match Storage::next(&buf, position) {
                    Ok((id, body, pos)) => {
                        position = pos;
                        map.insert(id, body);
                        if pos == buf.len() {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(Storage { map })
    }

    fn id(buf: &[u8], pos: usize) -> Result<(u16, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
        if let Ok(pos) = u64::try_from(pos) {
            cursor.set_position(pos);
        } else {
            return Err("Fail to set cursor position".to_string());
        }
        let id = cursor.get_u16_le();
        Ok((id, pos + sizes::U16_LEN))
    }

    fn body(buf: &[u8], pos: usize) -> Result<(Vec<u8>, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
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
            8 => {
                if let Ok(val) = usize::try_from(cursor.get_u8()) {
                    prop_body_len_usize = val;
                    prop_size_len = sizes::U8_LEN;
                } else {
                    return Err("Fail convert length of name from u8 to usize".to_string());
                }
            }
            16 => {
                if let Ok(val) = usize::try_from(cursor.get_u16_le()) {
                    prop_body_len_usize = val;
                    prop_size_len = sizes::U16_LEN;
                } else {
                    return Err("Fail convert length of name from u16 to usize".to_string());
                }
            }
            32 => {
                if let Ok(val) = usize::try_from(cursor.get_u32_le()) {
                    prop_body_len_usize = val;
                    prop_size_len = sizes::U32_LEN;
                } else {
                    return Err("Fail convert length of name from u32 to usize".to_string());
                }
            }
            64 => {
                if let Ok(val) = usize::try_from(cursor.get_u64_le()) {
                    prop_body_len_usize = val;
                    prop_size_len = sizes::U64_LEN;
                } else {
                    return Err("Fail convert length of name from u64 to usize".to_string());
                }
            }
            v => {
                return Err(format!("Unknown rank has been gotten: {}", v));
            }
        };
        let mut prop_body_buf = vec![0; prop_body_len_usize];
        prop_body_buf.copy_from_slice(
            &buf[(pos + prop_rank_len + prop_size_len)
                ..(pos + prop_rank_len + prop_size_len + prop_body_len_usize)],
        );
        Ok((
            prop_body_buf,
            pos + prop_rank_len + prop_size_len + prop_body_len_usize,
        ))
    }

    fn next(buf: &[u8], pos: usize) -> Result<(u16, Vec<u8>, usize), String> {
        match Storage::id(buf, pos) {
            Ok((id, pos)) => match Storage::body(buf, pos) {
                Ok((body, pos)) => Ok((id, body, pos)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn get(&mut self, id: u16) -> Option<&Vec<u8>> {
        self.map.get(&id)
    }
}

const MSG_HEADER_LEN: usize = sizes::U32_LEN + // {u32} message ID
                                sizes::U16_LEN + // {u16} signature
                                sizes::U32_LEN + // {u32} sequence
                                sizes::U64_LEN + // {u64} body size
                                sizes::U64_LEN; // {u64} timestamp

pub trait PackingMiddlewareInterface {
    fn decode(
        buffer: Vec<u8>,
        _id: u32,
        _sequence: u32,
        _uuid: Option<String>,
    ) -> Result<Vec<u8>, String> {
        Ok(buffer)
    }
    fn encode(
        buffer: Vec<u8>,
        _id: u32,
        _sequence: u32,
        _uuid: Option<String>,
    ) -> Result<Vec<u8>, String> {
        Ok(buffer)
    }
}

pub struct PackingMiddleware {}

impl PackingMiddlewareInterface for PackingMiddleware {
    fn decode(
        buffer: Vec<u8>,
        _id: u32,
        _sequence: u32,
        _uuid: Option<String>,
    ) -> Result<Vec<u8>, String> {
        Ok(buffer)
    }
    fn encode(
        buffer: Vec<u8>,
        _id: u32,
        _sequence: u32,
        _uuid: Option<String>,
    ) -> Result<Vec<u8>, String> {
        Ok(buffer)
    }
}

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
    buf.len() >= MSG_HEADER_LEN
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
    Ok(PackageHeader {
        id,
        signature,
        sequence,
        len,
        ts,
        len_usize,
    })
}

pub fn has_buffer_body(buf: &[u8], header: &PackageHeader) -> bool {
    buf.len() >= header.len_usize + MSG_HEADER_LEN
}

pub fn get_body_from_buffer(
    buf: &[u8],
    header: &PackageHeader,
    uuid: Option<String>,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    if buf.len() < header.len_usize + MSG_HEADER_LEN {
        return Err(format!("Cannot extract body of package because size in header {} bytes, but size of buffer {} bytes.", header.len, buf.len() - MSG_HEADER_LEN));
    }
    // Get body
    let mut body = vec![0; header.len_usize];
    body.copy_from_slice(&buf[MSG_HEADER_LEN..(MSG_HEADER_LEN + header.len_usize)]);
    let mut rest = vec![0; buf.len() - MSG_HEADER_LEN - header.len_usize];
    rest.copy_from_slice(&buf[(MSG_HEADER_LEN + header.len_usize)..]);
    match PackingMiddleware::decode(body, header.id, header.sequence, uuid) {
        Ok(buffer) => Ok((buffer, rest)),
        Err(e) => Err(e),
    }
}

pub fn pack<T>(mut msg: T, sequence: u32, uuid: Option<String>) -> Result<Vec<u8>, String>
where
    T: StructEncode,
{
    match msg.abduct() {
        Ok(buffer) => pack_buffer(msg.get_id(), msg.get_signature(), sequence, buffer, uuid),
        Err(e) => Err(e),
    }
}

pub fn pack_buffer(
    msg_id: u32,
    signature: u16,
    sequence: u32,
    msg_buf: Vec<u8>,
    uuid: Option<String>,
) -> Result<Vec<u8>, String> {
    let buffer = match PackingMiddleware::encode(msg_buf, msg_id, sequence, uuid) {
        Ok(buffer) => buffer,
        Err(e) => {
            return Err(e);
        }
    };
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let mut buf: Vec<u8> = vec![];
            buf.append(&mut msg_id.to_le_bytes().to_vec());
            buf.append(&mut signature.to_le_bytes().to_vec());
            buf.append(&mut sequence.to_le_bytes().to_vec());
            buf.append(&mut duration.as_secs().to_le_bytes().to_vec());
            buf.append(&mut (buffer.len() as u64).to_le_bytes().to_vec());
            buf.append(&mut buffer.to_vec());
            Ok(buf)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub trait PackingStruct: StructEncode {
    fn pack(&mut self, sequence: u32, uuid: Option<String>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf, uuid),
            Err(e) => Err(e),
        }
    }
}

pub trait PackingEnum: EnumEncode {
    fn pack(&mut self, sequence: u32, uuid: Option<String>) -> Result<Vec<u8>, String> {
        match self.abduct() {
            Ok(buf) => pack_buffer(self.get_id(), self.get_signature(), sequence, buf, uuid),
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
    pub fn chunk(&mut self, buf: &Vec<u8>, uuid: Option<String>) -> Result<(), ReadError> {
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
        let (body, rest) = match get_body_from_buffer(&self.buffer, &header, uuid.clone()) {
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
                    self.chunk(&vec![], uuid)
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
    Identification(Identification::AvailableMessages),
    Events(Events::AvailableMessages),
    Beacons(Beacons::AvailableMessages),
    ServerEvents(ServerEvents::AvailableMessages),
    Message(Message::AvailableMessages),
    Messages(Messages::AvailableMessages),
    UserLogin(UserLogin::AvailableMessages),
    UserInfo(UserInfo::AvailableMessages),
    Users(Users::AvailableMessages),
    InternalServiceGroup(InternalServiceGroup::AvailableMessages),
}
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin(String),
    User(String),
    Manager(String),
    Defaults,
}
impl EnumDecode for UserRole {
    fn get_id(&self) -> u32 { 11 }
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
    fn get_id(&self) -> u32 { 11 }
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

pub mod Identification {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        SelfKey(SelfKey),
        SelfKeyResponse(SelfKeyResponse),
        AssignedKey(AssignedKey),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKey {
        pub uuid: Option<String>,
        pub id: Option<u64>,
        pub location: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKey {
        fn get_id() -> u32 {
            2
        }
        fn defaults() -> SelfKey {
            SelfKey {
                uuid: None,
                id: None,
                location: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(3)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.id = match Option::<u64>::get_from_storage(Source::Storage(&mut storage), Some(4)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.location = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(5)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKey {
        fn get_id(&self) -> u32 { 2 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(3)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.id.get_buf_to_store(Some(4)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.location.get_buf_to_store(Some(5)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKey { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKeyResponse {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKeyResponse {
        fn get_id() -> u32 {
            6
        }
        fn defaults() -> SelfKeyResponse {
            SelfKeyResponse {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(7)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKeyResponse {
        fn get_id(&self) -> u32 { 6 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(7)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKeyResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct AssignedKey {
        pub uuid: Option<String>,
        pub auth: Option<bool>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for AssignedKey {
        fn get_id() -> u32 {
            8
        }
        fn defaults() -> AssignedKey {
            AssignedKey {
                uuid: None,
                auth: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(9)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.auth = match Option::<bool>::get_from_storage(Source::Storage(&mut storage), Some(10)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for AssignedKey {
        fn get_id(&self) -> u32 { 8 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(9)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.auth.get_buf_to_store(Some(10)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for AssignedKey { }

}

pub mod Events {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        UserConnected(UserConnected),
        UserDisconnected(UserDisconnected),
        Message(Message),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserConnected {
        pub username: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserConnected {
        fn get_id() -> u32 {
            13
        }
        fn defaults() -> UserConnected {
            UserConnected {
                username: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(14)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(15)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserConnected {
        fn get_id(&self) -> u32 { 13 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(14)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(15)) {
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
            16
        }
        fn defaults() -> UserDisconnected {
            UserDisconnected {
                username: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(17)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(18)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserDisconnected {
        fn get_id(&self) -> u32 { 16 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(17)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(18)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserDisconnected { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub timestamp: u64,
        pub user: String,
        pub message: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Message {
        fn get_id() -> u32 {
            19
        }
        fn defaults() -> Message {
            Message {
                timestamp: 0,
                user: String::from(""),
                message: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.timestamp = match u64::get_from_storage(Source::Storage(&mut storage), Some(20)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(21)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(22)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(23)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Message {
        fn get_id(&self) -> u32 { 19 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.timestamp.get_buf_to_store(Some(20)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.user.get_buf_to_store(Some(21)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(22)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(23)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Message { }

}

pub mod Beacons {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        LikeUser(LikeUser),
        LikeMessage(LikeMessage),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LikeUser {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for LikeUser {
        fn get_id() -> u32 {
            25
        }
        fn defaults() -> LikeUser {
            LikeUser {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(26)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for LikeUser {
        fn get_id(&self) -> u32 { 25 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(26)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for LikeUser { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct LikeMessage {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for LikeMessage {
        fn get_id() -> u32 {
            27
        }
        fn defaults() -> LikeMessage {
            LikeMessage {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(28)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for LikeMessage {
        fn get_id(&self) -> u32 { 27 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(28)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for LikeMessage { }

}

pub mod ServerEvents {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        UserKickOff(UserKickOff),
        UserAlert(UserAlert),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserKickOff {
        pub reason: Option<String>,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserKickOff {
        fn get_id() -> u32 {
            30
        }
        fn defaults() -> UserKickOff {
            UserKickOff {
                reason: None,
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(31)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(32)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserKickOff {
        fn get_id(&self) -> u32 { 30 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(31)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(32)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserKickOff { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct UserAlert {
        pub reason: Option<String>,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for UserAlert {
        fn get_id() -> u32 {
            33
        }
        fn defaults() -> UserAlert {
            UserAlert {
                reason: None,
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(34)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(35)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for UserAlert {
        fn get_id(&self) -> u32 { 33 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(34)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(35)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for UserAlert { }

}

pub mod Message {
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
        pub user: String,
        pub message: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            37
        }
        fn defaults() -> Request {
            Request {
                user: String::from(""),
                message: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(38)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(39)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 37 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.user.get_buf_to_store(Some(38)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(39)) {
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
            40
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(41)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 40 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(41)) {
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
            42
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(43)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 42 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(43)) {
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
            44
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(45)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 44 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(45)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod Messages {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        Message(Message),
        Request(Request),
        Response(Response),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub timestamp: u64,
        pub user: String,
        pub uuid: String,
        pub message: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Message {
        fn get_id() -> u32 {
            47
        }
        fn defaults() -> Message {
            Message {
                timestamp: 0,
                user: String::from(""),
                uuid: String::from(""),
                message: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.timestamp = match u64::get_from_storage(Source::Storage(&mut storage), Some(48)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.user = match String::get_from_storage(Source::Storage(&mut storage), Some(49)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(50)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.message = match String::get_from_storage(Source::Storage(&mut storage), Some(51)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Message {
        fn get_id(&self) -> u32 { 47 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.timestamp.get_buf_to_store(Some(48)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.user.get_buf_to_store(Some(49)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(50)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.message.get_buf_to_store(Some(51)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Message { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            52
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 52 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub messages: Vec<Message>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Response {
        fn get_id() -> u32 {
            53
        }
        fn defaults() -> Response {
            Response {
                messages: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.messages = match Vec::<Message>::get_from_storage(Source::Storage(&mut storage), Some(54)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 53 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.messages.get_buf_to_store(Some(54)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Response { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            55
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(56)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 55 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(56)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserLogin {
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
        pub username: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            58
        }
        fn defaults() -> Request {
            Request {
                username: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.username = match String::get_from_storage(Source::Storage(&mut storage), Some(59)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 58 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.username.get_buf_to_store(Some(59)) {
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
            60
        }
        fn defaults() -> Accepted {
            Accepted {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(61)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 60 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(61)) {
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
            62
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(63)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 62 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(63)) {
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
            64
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(65)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 64 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(65)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod UserInfo {
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
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            67
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 67 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Accepted {
        pub browser: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Accepted {
        fn get_id() -> u32 {
            68
        }
        fn defaults() -> Accepted {
            Accepted {
                browser: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.browser = match String::get_from_storage(Source::Storage(&mut storage), Some(69)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Accepted {
        fn get_id(&self) -> u32 { 68 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.browser.get_buf_to_store(Some(69)) {
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
            70
        }
        fn defaults() -> Denied {
            Denied {
                reason: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.reason = match String::get_from_storage(Source::Storage(&mut storage), Some(71)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Denied {
        fn get_id(&self) -> u32 { 70 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.reason.get_buf_to_store(Some(71)) {
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
            72
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(73)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 72 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(73)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod Users {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        User(User),
        Request(Request),
        Response(Response),
        Err(Err),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub name: String,
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for User {
        fn get_id() -> u32 {
            75
        }
        fn defaults() -> User {
            User {
                name: String::from(""),
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.name = match String::get_from_storage(Source::Storage(&mut storage), Some(76)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(77)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for User {
        fn get_id(&self) -> u32 { 75 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.name.get_buf_to_store(Some(76)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.uuid.get_buf_to_store(Some(77)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for User { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Request {
        fn get_id() -> u32 {
            78
        }
        fn defaults() -> Request {
            Request {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Request {
        fn get_id(&self) -> u32 { 78 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for Request { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub users: Vec<User>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Response {
        fn get_id() -> u32 {
            79
        }
        fn defaults() -> Response {
            Response {
                users: vec![],
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.users = match Vec::<User>::get_from_storage(Source::Storage(&mut storage), Some(80)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Response {
        fn get_id(&self) -> u32 { 79 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.users.get_buf_to_store(Some(80)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Response { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Err {
        pub error: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for Err {
        fn get_id() -> u32 {
            81
        }
        fn defaults() -> Err {
            Err {
                error: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match String::get_from_storage(Source::Storage(&mut storage), Some(82)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for Err {
        fn get_id(&self) -> u32 { 81 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(82)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for Err { }

}

pub mod InternalServiceGroup {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        SelfKeyResponse(SelfKeyResponse),
        HashRequest(HashRequest),
        HashResponse(HashResponse),
        BeaconConfirmation(BeaconConfirmation),
        ConnectConfirmationBeacon(ConnectConfirmationBeacon),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKeyResponse {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKeyResponse {
        fn get_id() -> u32 {
            84
        }
        fn defaults() -> SelfKeyResponse {
            SelfKeyResponse {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(85)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKeyResponse {
        fn get_id(&self) -> u32 { 84 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(85)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for SelfKeyResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HashRequest {
        pub protocol: String,
        pub workflow: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for HashRequest {
        fn get_id() -> u32 {
            86
        }
        fn defaults() -> HashRequest {
            HashRequest {
                protocol: String::from(""),
                workflow: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.protocol = match String::get_from_storage(Source::Storage(&mut storage), Some(87)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.workflow = match String::get_from_storage(Source::Storage(&mut storage), Some(88)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashRequest {
        fn get_id(&self) -> u32 { 86 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.protocol.get_buf_to_store(Some(87)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.workflow.get_buf_to_store(Some(88)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for HashRequest { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct HashResponse {
        pub error: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for HashResponse {
        fn get_id() -> u32 {
            89
        }
        fn defaults() -> HashResponse {
            HashResponse {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(90)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashResponse {
        fn get_id(&self) -> u32 { 89 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(90)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for HashResponse { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BeaconConfirmation {
        pub error: Option<String>,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for BeaconConfirmation {
        fn get_id() -> u32 {
            91
        }
        fn defaults() -> BeaconConfirmation {
            BeaconConfirmation {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(92)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for BeaconConfirmation {
        fn get_id(&self) -> u32 { 91 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(92)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for BeaconConfirmation { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ConnectConfirmationBeacon {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for ConnectConfirmationBeacon {
        fn get_id() -> u32 {
            93
        }
        fn defaults() -> ConnectConfirmationBeacon {
            ConnectConfirmationBeacon {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for ConnectConfirmationBeacon {
        fn get_id(&self) -> u32 { 93 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for ConnectConfirmationBeacon { }

}

impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {
        match id {
            11 => match UserRole::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserRole(m)),
                Err(e) => Err(e),
            },
            2 => match Identification::SelfKey::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::SelfKey(m))),
                Err(e) => Err(e),
            },
            6 => match Identification::SelfKeyResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::SelfKeyResponse(m))),
                Err(e) => Err(e),
            },
            8 => match Identification::AssignedKey::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Identification(Identification::AvailableMessages::AssignedKey(m))),
                Err(e) => Err(e),
            },
            13 => match Events::UserConnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::UserConnected(m))),
                Err(e) => Err(e),
            },
            16 => match Events::UserDisconnected::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::UserDisconnected(m))),
                Err(e) => Err(e),
            },
            19 => match Events::Message::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::Message(m))),
                Err(e) => Err(e),
            },
            25 => match Beacons::LikeUser::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::LikeUser(m))),
                Err(e) => Err(e),
            },
            27 => match Beacons::LikeMessage::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::LikeMessage(m))),
                Err(e) => Err(e),
            },
            30 => match ServerEvents::UserKickOff::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::ServerEvents(ServerEvents::AvailableMessages::UserKickOff(m))),
                Err(e) => Err(e),
            },
            33 => match ServerEvents::UserAlert::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::ServerEvents(ServerEvents::AvailableMessages::UserAlert(m))),
                Err(e) => Err(e),
            },
            37 => match Message::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            40 => match Message::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            42 => match Message::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            44 => match Message::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Message(Message::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            47 => match Messages::Message::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Message(m))),
                Err(e) => Err(e),
            },
            52 => match Messages::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            53 => match Messages::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            55 => match Messages::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Messages(Messages::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            58 => match UserLogin::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            60 => match UserLogin::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            62 => match UserLogin::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            64 => match UserLogin::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserLogin(UserLogin::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            67 => match UserInfo::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            68 => match UserInfo::Accepted::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Accepted(m))),
                Err(e) => Err(e),
            },
            70 => match UserInfo::Denied::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Denied(m))),
                Err(e) => Err(e),
            },
            72 => match UserInfo::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::UserInfo(UserInfo::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            75 => match Users::User::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::User(m))),
                Err(e) => Err(e),
            },
            78 => match Users::Request::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Request(m))),
                Err(e) => Err(e),
            },
            79 => match Users::Response::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Response(m))),
                Err(e) => Err(e),
            },
            81 => match Users::Err::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Users(Users::AvailableMessages::Err(m))),
                Err(e) => Err(e),
            },
            84 => match InternalServiceGroup::SelfKeyResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::SelfKeyResponse(m))),
                Err(e) => Err(e),
            },
            86 => match InternalServiceGroup::HashRequest::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashRequest(m))),
                Err(e) => Err(e),
            },
            89 => match InternalServiceGroup::HashResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashResponse(m))),
                Err(e) => Err(e),
            },
            91 => match InternalServiceGroup::BeaconConfirmation::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::BeaconConfirmation(m))),
                Err(e) => Err(e),
            },
            93 => match InternalServiceGroup::ConnectConfirmationBeacon::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::ConnectConfirmationBeacon(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

pub fn hash() -> String { String::from("F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE") }
