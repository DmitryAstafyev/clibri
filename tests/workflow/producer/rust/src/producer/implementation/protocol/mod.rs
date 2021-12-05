
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
    EnumA(EnumA),
    EnumB(EnumB),
    EnumC(EnumC),
    StructA(StructA),
    StructB(StructB),
    StructC(StructC),
    StructD(StructD),
    StructE(StructE),
    StructF(StructF),
    StructG(StructG),
    TriggerBeaconsEmitter(TriggerBeaconsEmitter),
    StructEmpty(StructEmpty),
    StructEmptyA(StructEmptyA),
    StructEmptyB(StructEmptyB),
    StructJ(StructJ),
    TriggerBeacons(TriggerBeacons),
    FinishConsumerTest(FinishConsumerTest),
    FinishConsumerTestBroadcast(FinishConsumerTestBroadcast),
    BeaconA(BeaconA),
    EventA(EventA),
    EventB(EventB),
    Beacons(Beacons::AvailableMessages),
    GroupA(GroupA::AvailableMessages),
    GroupB(GroupB::AvailableMessages),
    GroupD(GroupD::AvailableMessages),
    Events(Events::AvailableMessages),
    InternalServiceGroup(InternalServiceGroup::AvailableMessages),
}
#[derive(Debug, Clone, PartialEq)]
pub enum EnumA {
    Option_a(String),
    Option_b(String),
    Defaults,
}
impl EnumDecode for EnumA {
    fn get_id(&self) -> u32 { 1 }
    fn extract(buf: Vec<u8>) -> Result<EnumA, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumA because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumA::Option_a(v)),
                Err(e) => Err(e)
            },
            1 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumA::Option_b(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumA")),
        }
    }
}
impl EnumEncode for EnumA {
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
impl PackingEnum for EnumA {}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumB {
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
impl EnumDecode for EnumB {
    fn get_id(&self) -> u32 { 2 }
    fn extract(buf: Vec<u8>) -> Result<EnumB, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumB because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match String::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_str(v)),
                Err(e) => Err(e)
            },
            1 => match u8::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_u8(v)),
                Err(e) => Err(e)
            },
            2 => match u16::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_u16(v)),
                Err(e) => Err(e)
            },
            3 => match u32::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_u32(v)),
                Err(e) => Err(e)
            },
            4 => match u64::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_u64(v)),
                Err(e) => Err(e)
            },
            5 => match i8::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_i8(v)),
                Err(e) => Err(e)
            },
            6 => match i16::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_i16(v)),
                Err(e) => Err(e)
            },
            7 => match i32::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_i32(v)),
                Err(e) => Err(e)
            },
            8 => match i64::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_i64(v)),
                Err(e) => Err(e)
            },
            9 => match f32::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_f32(v)),
                Err(e) => Err(e)
            },
            10 => match f64::decode(&body_buf) {
                Ok(v) => Ok(EnumB::Option_f64(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumB")),
        }
    }
}
impl EnumEncode for EnumB {
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
impl PackingEnum for EnumB {}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumC {
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
impl EnumDecode for EnumC {
    fn get_id(&self) -> u32 { 3 }
    fn extract(buf: Vec<u8>) -> Result<EnumC, String> {
        if buf.len() <= sizes::U16_LEN {
            return Err(String::from("Fail to extract value for EnumC because buffer too small"));
        }
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let index = cursor.get_u16_le();
        let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
        body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
        match index {
            0 => match Vec::<String>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_str(v)),
                Err(e) => Err(e)
            },
            1 => match Vec::<u8>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_u8(v)),
                Err(e) => Err(e)
            },
            2 => match Vec::<u16>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_u16(v)),
                Err(e) => Err(e)
            },
            3 => match Vec::<u32>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_u32(v)),
                Err(e) => Err(e)
            },
            4 => match Vec::<u64>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_u64(v)),
                Err(e) => Err(e)
            },
            5 => match Vec::<i8>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_i8(v)),
                Err(e) => Err(e)
            },
            6 => match Vec::<i16>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_i16(v)),
                Err(e) => Err(e)
            },
            7 => match Vec::<i32>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_i32(v)),
                Err(e) => Err(e)
            },
            8 => match Vec::<i64>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_i64(v)),
                Err(e) => Err(e)
            },
            9 => match Vec::<f32>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_f32(v)),
                Err(e) => Err(e)
            },
            10 => match Vec::<f64>::decode(&body_buf) {
                Ok(v) => Ok(EnumC::Option_f64(v)),
                Err(e) => Err(e)
            },
            _ => Err(String::from("Fail to find relevant value for EnumC")),
        }
    }
}
impl EnumEncode for EnumC {
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
impl PackingEnum for EnumC {}

#[derive(Debug, Clone, PartialEq)]
pub struct StructA {
    pub field_str: String,
    pub field_str_empty: String,
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
impl StructDecode for StructA {
    fn get_id() -> u32 {
        4
    }
    fn defaults() -> StructA {
        StructA {
            field_str: String::from(""),
            field_str_empty: String::from(""),
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
        self.field_str_empty = match String::get_from_storage(Source::Storage(&mut storage), Some(6)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(7)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(8)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match u32::get_from_storage(Source::Storage(&mut storage), Some(9)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match u64::get_from_storage(Source::Storage(&mut storage), Some(10)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match i8::get_from_storage(Source::Storage(&mut storage), Some(11)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match i16::get_from_storage(Source::Storage(&mut storage), Some(12)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match i32::get_from_storage(Source::Storage(&mut storage), Some(13)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match i64::get_from_storage(Source::Storage(&mut storage), Some(14)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match f32::get_from_storage(Source::Storage(&mut storage), Some(15)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match f64::get_from_storage(Source::Storage(&mut storage), Some(16)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match bool::get_from_storage(Source::Storage(&mut storage), Some(17)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructA {
    fn get_id(&self) -> u32 { 4 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(5)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_str_empty.get_buf_to_store(Some(6)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(7)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(8)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(9)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(10)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(11)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(12)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(13)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(14)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(15)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(16)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(17)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructA { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructB {
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
    pub field_struct: Vec<StructA>,
    pub field_str_empty: Vec<String>,
    pub field_u8_empty: Vec<u8>,
    pub field_u16_empty: Vec<u16>,
    pub field_u32_empty: Vec<u32>,
    pub field_u64_empty: Vec<u64>,
    pub field_i8_empty: Vec<i8>,
    pub field_i16_empty: Vec<i16>,
    pub field_i32_empty: Vec<i32>,
    pub field_i64_empty: Vec<i64>,
    pub field_f32_empty: Vec<f32>,
    pub field_f64_empty: Vec<f64>,
    pub field_bool_empty: Vec<bool>,
    pub field_struct_empty: Vec<StructA>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructB {
    fn get_id() -> u32 {
        18
    }
    fn defaults() -> StructB {
        StructB {
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
            field_struct: vec![],
            field_str_empty: vec![],
            field_u8_empty: vec![],
            field_u16_empty: vec![],
            field_u32_empty: vec![],
            field_u64_empty: vec![],
            field_i8_empty: vec![],
            field_i16_empty: vec![],
            field_i32_empty: vec![],
            field_i64_empty: vec![],
            field_f32_empty: vec![],
            field_f64_empty: vec![],
            field_bool_empty: vec![],
            field_struct_empty: vec![],
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_str = match Vec::<String>::get_from_storage(Source::Storage(&mut storage), Some(19)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Vec::<u8>::get_from_storage(Source::Storage(&mut storage), Some(20)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Vec::<u16>::get_from_storage(Source::Storage(&mut storage), Some(21)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Vec::<u32>::get_from_storage(Source::Storage(&mut storage), Some(22)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Vec::<u64>::get_from_storage(Source::Storage(&mut storage), Some(23)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Vec::<i8>::get_from_storage(Source::Storage(&mut storage), Some(24)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Vec::<i16>::get_from_storage(Source::Storage(&mut storage), Some(25)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Vec::<i32>::get_from_storage(Source::Storage(&mut storage), Some(26)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Vec::<i64>::get_from_storage(Source::Storage(&mut storage), Some(27)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Vec::<f32>::get_from_storage(Source::Storage(&mut storage), Some(28)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Vec::<f64>::get_from_storage(Source::Storage(&mut storage), Some(29)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Vec::<bool>::get_from_storage(Source::Storage(&mut storage), Some(30)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_struct = match Vec::<StructA>::get_from_storage(Source::Storage(&mut storage), Some(31)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_str_empty = match Vec::<String>::get_from_storage(Source::Storage(&mut storage), Some(32)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8_empty = match Vec::<u8>::get_from_storage(Source::Storage(&mut storage), Some(33)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16_empty = match Vec::<u16>::get_from_storage(Source::Storage(&mut storage), Some(34)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32_empty = match Vec::<u32>::get_from_storage(Source::Storage(&mut storage), Some(35)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64_empty = match Vec::<u64>::get_from_storage(Source::Storage(&mut storage), Some(36)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8_empty = match Vec::<i8>::get_from_storage(Source::Storage(&mut storage), Some(37)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16_empty = match Vec::<i16>::get_from_storage(Source::Storage(&mut storage), Some(38)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32_empty = match Vec::<i32>::get_from_storage(Source::Storage(&mut storage), Some(39)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64_empty = match Vec::<i64>::get_from_storage(Source::Storage(&mut storage), Some(40)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32_empty = match Vec::<f32>::get_from_storage(Source::Storage(&mut storage), Some(41)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64_empty = match Vec::<f64>::get_from_storage(Source::Storage(&mut storage), Some(42)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool_empty = match Vec::<bool>::get_from_storage(Source::Storage(&mut storage), Some(43)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_struct_empty = match Vec::<StructA>::get_from_storage(Source::Storage(&mut storage), Some(44)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructB {
    fn get_id(&self) -> u32 { 18 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(19)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(20)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(21)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(22)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(23)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(24)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(25)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(26)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(27)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(28)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(29)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(30)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_struct.get_buf_to_store(Some(31)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_str_empty.get_buf_to_store(Some(32)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8_empty.get_buf_to_store(Some(33)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16_empty.get_buf_to_store(Some(34)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32_empty.get_buf_to_store(Some(35)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64_empty.get_buf_to_store(Some(36)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8_empty.get_buf_to_store(Some(37)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16_empty.get_buf_to_store(Some(38)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32_empty.get_buf_to_store(Some(39)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64_empty.get_buf_to_store(Some(40)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32_empty.get_buf_to_store(Some(41)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64_empty.get_buf_to_store(Some(42)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool_empty.get_buf_to_store(Some(43)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_struct_empty.get_buf_to_store(Some(44)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructB { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructC {
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
impl StructDecode for StructC {
    fn get_id() -> u32 {
        45
    }
    fn defaults() -> StructC {
        StructC {
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
        self.field_str = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(46)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Option::<u8>::get_from_storage(Source::Storage(&mut storage), Some(47)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Option::<u16>::get_from_storage(Source::Storage(&mut storage), Some(48)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Option::<u32>::get_from_storage(Source::Storage(&mut storage), Some(49)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Option::<u64>::get_from_storage(Source::Storage(&mut storage), Some(50)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Option::<i8>::get_from_storage(Source::Storage(&mut storage), Some(51)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Option::<i16>::get_from_storage(Source::Storage(&mut storage), Some(52)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Option::<i32>::get_from_storage(Source::Storage(&mut storage), Some(53)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Option::<i64>::get_from_storage(Source::Storage(&mut storage), Some(54)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Option::<f32>::get_from_storage(Source::Storage(&mut storage), Some(55)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Option::<f64>::get_from_storage(Source::Storage(&mut storage), Some(56)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Option::<bool>::get_from_storage(Source::Storage(&mut storage), Some(57)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructC {
    fn get_id(&self) -> u32 { 45 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(46)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(47)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(48)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(49)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(50)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(51)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(52)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(53)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(54)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(55)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(56)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(57)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructC { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructD {
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
impl StructDecode for StructD {
    fn get_id() -> u32 {
        58
    }
    fn defaults() -> StructD {
        StructD {
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
        self.field_str = match Option::<Vec::<String>>::get_from_storage(Source::Storage(&mut storage), Some(59)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u8 = match Option::<Vec::<u8>>::get_from_storage(Source::Storage(&mut storage), Some(60)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u16 = match Option::<Vec::<u16>>::get_from_storage(Source::Storage(&mut storage), Some(61)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u32 = match Option::<Vec::<u32>>::get_from_storage(Source::Storage(&mut storage), Some(62)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_u64 = match Option::<Vec::<u64>>::get_from_storage(Source::Storage(&mut storage), Some(63)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i8 = match Option::<Vec::<i8>>::get_from_storage(Source::Storage(&mut storage), Some(64)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i16 = match Option::<Vec::<i16>>::get_from_storage(Source::Storage(&mut storage), Some(65)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i32 = match Option::<Vec::<i32>>::get_from_storage(Source::Storage(&mut storage), Some(66)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_i64 = match Option::<Vec::<i64>>::get_from_storage(Source::Storage(&mut storage), Some(67)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f32 = match Option::<Vec::<f32>>::get_from_storage(Source::Storage(&mut storage), Some(68)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_f64 = match Option::<Vec::<f64>>::get_from_storage(Source::Storage(&mut storage), Some(69)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_bool = match Option::<Vec::<bool>>::get_from_storage(Source::Storage(&mut storage), Some(70)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructD {
    fn get_id(&self) -> u32 { 58 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_str.get_buf_to_store(Some(59)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u8.get_buf_to_store(Some(60)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u16.get_buf_to_store(Some(61)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u32.get_buf_to_store(Some(62)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_u64.get_buf_to_store(Some(63)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i8.get_buf_to_store(Some(64)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i16.get_buf_to_store(Some(65)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i32.get_buf_to_store(Some(66)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_i64.get_buf_to_store(Some(67)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f32.get_buf_to_store(Some(68)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_f64.get_buf_to_store(Some(69)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_bool.get_buf_to_store(Some(70)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructD { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructE {
    pub field_a: EnumA,
    pub field_b: EnumB,
    pub field_c: EnumC,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructE {
    fn get_id() -> u32 {
        71
    }
    fn defaults() -> StructE {
        StructE {
            field_a: EnumA::Defaults,
            field_b: EnumB::Defaults,
            field_c: EnumC::Defaults,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match EnumA::get_from_storage(Source::Storage(&mut storage), Some(72)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match EnumB::get_from_storage(Source::Storage(&mut storage), Some(73)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_c = match EnumC::get_from_storage(Source::Storage(&mut storage), Some(74)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructE {
    fn get_id(&self) -> u32 { 71 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(72)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(73)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_c.get_buf_to_store(Some(74)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructE { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructF {
    pub field_a: Option<EnumA>,
    pub field_b: Option<EnumB>,
    pub field_c: Option<EnumC>,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructF {
    fn get_id() -> u32 {
        75
    }
    fn defaults() -> StructF {
        StructF {
            field_a: None,
            field_b: None,
            field_c: None,
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        if let Some(buf) = storage.get(76) {
            if buf.is_empty() {
                self.field_a = None;
            } else {
                self.field_a = match EnumA::get_from_storage(Source::Storage(&mut storage), Some(76)) {
                    Ok(val) => Some(val),
                    Err(e) => { return Err(e) },
                };
            }
        } else {
            return Err("Buffer for property field_a isn't found".to_string());
        }
        if let Some(buf) = storage.get(77) {
            if buf.is_empty() {
                self.field_b = None;
            } else {
                self.field_b = match EnumB::get_from_storage(Source::Storage(&mut storage), Some(77)) {
                    Ok(val) => Some(val),
                    Err(e) => { return Err(e) },
                };
            }
        } else {
            return Err("Buffer for property field_b isn't found".to_string());
        }
        if let Some(buf) = storage.get(78) {
            if buf.is_empty() {
                self.field_c = None;
            } else {
                self.field_c = match EnumC::get_from_storage(Source::Storage(&mut storage), Some(78)) {
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
impl StructEncode for StructF {
    fn get_id(&self) -> u32 { 75 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        if let Some(mut val) = self.field_a.clone() {
            match val.get_buf_to_store(Some(76)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(76)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        if let Some(mut val) = self.field_b.clone() {
            match val.get_buf_to_store(Some(77)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(77)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        if let Some(mut val) = self.field_c.clone() {
            match val.get_buf_to_store(Some(78)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        } else {
            match get_empty_buffer_val(Some(78)) {
                Ok(mut buf) => { buffer.append(&mut buf); },
                Err(e) => { return  Err(e); },
            };
        }
        Ok(buffer)
    }
}
impl PackingStruct for StructF { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructG {
    pub field_a: StructA,
    pub field_b: StructB,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructG {
    fn get_id() -> u32 {
        79
    }
    fn defaults() -> StructG {
        StructG {
            field_a: StructA {
                field_str: String::from(""),
                field_str_empty: String::from(""),
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
            field_b: StructB {
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
                field_struct: vec![],
                field_str_empty: vec![],
                field_u8_empty: vec![],
                field_u16_empty: vec![],
                field_u32_empty: vec![],
                field_u64_empty: vec![],
                field_i8_empty: vec![],
                field_i16_empty: vec![],
                field_i32_empty: vec![],
                field_i64_empty: vec![],
                field_f32_empty: vec![],
                field_f64_empty: vec![],
                field_bool_empty: vec![],
                field_struct_empty: vec![],
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match StructA::get_from_storage(Source::Storage(&mut storage), Some(80)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match StructB::get_from_storage(Source::Storage(&mut storage), Some(81)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructG {
    fn get_id(&self) -> u32 { 79 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(80)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(81)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructG { }

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerBeaconsEmitter {
    pub uuid: String,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for TriggerBeaconsEmitter {
    fn get_id() -> u32 {
        82
    }
    fn defaults() -> TriggerBeaconsEmitter {
        TriggerBeaconsEmitter {
            uuid: String::from(""),
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(83)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for TriggerBeaconsEmitter {
    fn get_id(&self) -> u32 { 82 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.get_buf_to_store(Some(83)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for TriggerBeaconsEmitter { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructEmpty {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructEmpty {
    fn get_id() -> u32 {
        84
    }
    fn defaults() -> StructEmpty {
        StructEmpty {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructEmpty {
    fn get_id(&self) -> u32 { 84 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for StructEmpty { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructEmptyA {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructEmptyA {
    fn get_id() -> u32 {
        85
    }
    fn defaults() -> StructEmptyA {
        StructEmptyA {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructEmptyA {
    fn get_id(&self) -> u32 { 85 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for StructEmptyA { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructEmptyB {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructEmptyB {
    fn get_id() -> u32 {
        86
    }
    fn defaults() -> StructEmptyB {
        StructEmptyB {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructEmptyB {
    fn get_id(&self) -> u32 { 86 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for StructEmptyB { }

#[derive(Debug, Clone, PartialEq)]
pub struct StructJ {
    pub field_a: Option<StructA>,
    pub field_b: Option<StructB>,
    pub field_c: StructEmpty,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for StructJ {
    fn get_id() -> u32 {
        87
    }
    fn defaults() -> StructJ {
        StructJ {
            field_a: None,
            field_b: None,
            field_c: StructEmpty {
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field_a = match Option::<StructA>::get_from_storage(Source::Storage(&mut storage), Some(88)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match Option::<StructB>::get_from_storage(Source::Storage(&mut storage), Some(89)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_c = match StructEmpty::get_from_storage(Source::Storage(&mut storage), Some(90)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for StructJ {
    fn get_id(&self) -> u32 { 87 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field_a.get_buf_to_store(Some(88)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(89)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_c.get_buf_to_store(Some(90)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for StructJ { }

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerBeacons {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for TriggerBeacons {
    fn get_id() -> u32 {
        91
    }
    fn defaults() -> TriggerBeacons {
        TriggerBeacons {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for TriggerBeacons {
    fn get_id(&self) -> u32 { 91 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for TriggerBeacons { }

#[derive(Debug, Clone, PartialEq)]
pub struct FinishConsumerTest {
    pub uuid: String,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for FinishConsumerTest {
    fn get_id() -> u32 {
        92
    }
    fn defaults() -> FinishConsumerTest {
        FinishConsumerTest {
            uuid: String::from(""),
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(93)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for FinishConsumerTest {
    fn get_id(&self) -> u32 { 92 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.get_buf_to_store(Some(93)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for FinishConsumerTest { }

#[derive(Debug, Clone, PartialEq)]
pub struct FinishConsumerTestBroadcast {
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for FinishConsumerTestBroadcast {
    fn get_id() -> u32 {
        94
    }
    fn defaults() -> FinishConsumerTestBroadcast {
        FinishConsumerTestBroadcast {
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for FinishConsumerTestBroadcast {
    fn get_id(&self) -> u32 { 94 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        Ok(buffer)
    }
}
impl PackingStruct for FinishConsumerTestBroadcast { }

#[derive(Debug, Clone, PartialEq)]
pub struct BeaconA {
    pub field: StructA,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for BeaconA {
    fn get_id() -> u32 {
        95
    }
    fn defaults() -> BeaconA {
        BeaconA {
            field: StructA {
                field_str: String::from(""),
                field_str_empty: String::from(""),
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
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.field = match StructA::get_from_storage(Source::Storage(&mut storage), Some(96)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for BeaconA {
    fn get_id(&self) -> u32 { 95 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.field.get_buf_to_store(Some(96)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for BeaconA { }

#[derive(Debug, Clone, PartialEq)]
pub struct EventA {
    pub uuid: String,
    pub field_a: StructB,
    pub field_b: StructC,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for EventA {
    fn get_id() -> u32 {
        133
    }
    fn defaults() -> EventA {
        EventA {
            uuid: String::from(""),
            field_a: StructB {
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
                field_struct: vec![],
                field_str_empty: vec![],
                field_u8_empty: vec![],
                field_u16_empty: vec![],
                field_u32_empty: vec![],
                field_u64_empty: vec![],
                field_i8_empty: vec![],
                field_i16_empty: vec![],
                field_i32_empty: vec![],
                field_i64_empty: vec![],
                field_f32_empty: vec![],
                field_f64_empty: vec![],
                field_bool_empty: vec![],
                field_struct_empty: vec![],
            },
            field_b: StructC {
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
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(134)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_a = match StructB::get_from_storage(Source::Storage(&mut storage), Some(135)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_b = match StructC::get_from_storage(Source::Storage(&mut storage), Some(136)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for EventA {
    fn get_id(&self) -> u32 { 133 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.get_buf_to_store(Some(134)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_a.get_buf_to_store(Some(135)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_b.get_buf_to_store(Some(136)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for EventA { }

#[derive(Debug, Clone, PartialEq)]
pub struct EventB {
    pub uuid: String,
    pub field_a: StructC,
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructDecode for EventB {
    fn get_id() -> u32 {
        137
    }
    fn defaults() -> EventB {
        EventB {
            uuid: String::from(""),
            field_a: StructC {
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
            },
        }
    }
    fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
        self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(138)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.field_a = match StructC::get_from_storage(Source::Storage(&mut storage), Some(139)) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }
}
#[allow(unused_variables)]
#[allow(unused_mut)]
impl StructEncode for EventB {
    fn get_id(&self) -> u32 { 137 }
    fn get_signature(&self) -> u16 { 0 }
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match self.uuid.get_buf_to_store(Some(138)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        match self.field_a.get_buf_to_store(Some(139)) {
            Ok(mut buf) => { buffer.append(&mut buf); }
            Err(e) => { return Err(e) },
        };
        Ok(buffer)
    }
}
impl PackingStruct for EventB { }

pub mod Beacons {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        ShutdownServer(ShutdownServer),
        BeaconA(BeaconA),
        BeaconB(BeaconB),
        Sub(Sub::AvailableMessages),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ShutdownServer {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for ShutdownServer {
        fn get_id() -> u32 {
            98
        }
        fn defaults() -> ShutdownServer {
            ShutdownServer {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for ShutdownServer {
        fn get_id(&self) -> u32 { 98 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for ShutdownServer { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BeaconA {
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for BeaconA {
        fn get_id() -> u32 {
            99
        }
        fn defaults() -> BeaconA {
            BeaconA {
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for BeaconA {
        fn get_id(&self) -> u32 { 99 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            Ok(buffer)
        }
    }
    impl PackingStruct for BeaconA { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BeaconB {
        pub field: StructB,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for BeaconB {
        fn get_id() -> u32 {
            100
        }
        fn defaults() -> BeaconB {
            BeaconB {
                field: StructB {
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
                    field_struct: vec![],
                    field_str_empty: vec![],
                    field_u8_empty: vec![],
                    field_u16_empty: vec![],
                    field_u32_empty: vec![],
                    field_u64_empty: vec![],
                    field_i8_empty: vec![],
                    field_i16_empty: vec![],
                    field_i32_empty: vec![],
                    field_i64_empty: vec![],
                    field_f32_empty: vec![],
                    field_f64_empty: vec![],
                    field_bool_empty: vec![],
                    field_struct_empty: vec![],
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field = match StructB::get_from_storage(Source::Storage(&mut storage), Some(101)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for BeaconB {
        fn get_id(&self) -> u32 { 100 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field.get_buf_to_store(Some(101)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for BeaconB { }

    pub mod Sub {
        use super::*;
        use std::io::Cursor;
        use bytes::{ Buf };
        #[derive(Debug, Clone)]
        pub enum AvailableMessages {
            BeaconA(BeaconA),
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BeaconA {
            pub field: StructG,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for BeaconA {
            fn get_id() -> u32 {
                103
            }
            fn defaults() -> BeaconA {
                BeaconA {
                    field: StructG {
                        field_a: StructA {
                            field_str: String::from(""),
                            field_str_empty: String::from(""),
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
                        field_b: StructB {
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
                            field_struct: vec![],
                            field_str_empty: vec![],
                            field_u8_empty: vec![],
                            field_u16_empty: vec![],
                            field_u32_empty: vec![],
                            field_u64_empty: vec![],
                            field_i8_empty: vec![],
                            field_i16_empty: vec![],
                            field_i32_empty: vec![],
                            field_i64_empty: vec![],
                            field_f32_empty: vec![],
                            field_f64_empty: vec![],
                            field_bool_empty: vec![],
                            field_struct_empty: vec![],
                        },
                    },
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field = match StructG::get_from_storage(Source::Storage(&mut storage), Some(104)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for BeaconA {
            fn get_id(&self) -> u32 { 103 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field.get_buf_to_store(Some(104)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for BeaconA { }

    }

}

pub mod GroupA {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        EnumA(EnumA),
        StructA(StructA),
        StructB(StructB),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum EnumA {
        Option_a(String),
        Option_b(String),
        Defaults,
    }
    impl EnumDecode for EnumA {
        fn get_id(&self) -> u32 { 106 }
        fn extract(buf: Vec<u8>) -> Result<EnumA, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for EnumA because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let index = cursor.get_u16_le();
            let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
            body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
            match index {
                0 => match String::decode(&body_buf) {
                    Ok(v) => Ok(EnumA::Option_a(v)),
                    Err(e) => Err(e)
                },
                1 => match String::decode(&body_buf) {
                    Ok(v) => Ok(EnumA::Option_b(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for EnumA")),
            }
        }
    }
    impl EnumEncode for EnumA {
        fn get_id(&self) -> u32 { 106 }
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
    impl PackingEnum for EnumA {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructA {
        pub field_u8: u8,
        pub field_u16: u16,
        pub opt: EnumA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructA {
        fn get_id() -> u32 {
            107
        }
        fn defaults() -> StructA {
            StructA {
                field_u8: 0,
                field_u16: 0,
                opt: GroupA::EnumA::Defaults,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(108)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(109)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.opt = match EnumA::get_from_storage(Source::Storage(&mut storage), Some(110)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructA {
        fn get_id(&self) -> u32 { 107 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(108)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(109)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.opt.get_buf_to_store(Some(110)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructA { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructB {
        pub field_u8: u8,
        pub field_u16: u16,
        pub strct: StructA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructB {
        fn get_id() -> u32 {
            111
        }
        fn defaults() -> StructB {
            StructB {
                field_u8: 0,
                field_u16: 0,
                strct: GroupA::StructA {
                    field_u8: 0,
                    field_u16: 0,
                    opt: GroupA::EnumA::Defaults,
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(112)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(113)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.strct = match StructA::get_from_storage(Source::Storage(&mut storage), Some(114)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructB {
        fn get_id(&self) -> u32 { 111 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(112)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(113)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.strct.get_buf_to_store(Some(114)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructB { }

}

pub mod GroupB {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        StructA(StructA),
        GroupC(GroupC::AvailableMessages),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructA {
        pub field_u8: u8,
        pub field_u16: u16,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructA {
        fn get_id() -> u32 {
            116
        }
        fn defaults() -> StructA {
            StructA {
                field_u8: 0,
                field_u16: 0,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(117)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(118)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructA {
        fn get_id(&self) -> u32 { 116 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_u8.get_buf_to_store(Some(117)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_u16.get_buf_to_store(Some(118)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructA { }

    pub mod GroupC {
        use super::*;
        use std::io::Cursor;
        use bytes::{ Buf };
        #[derive(Debug, Clone)]
        pub enum AvailableMessages {
            StructA(StructA),
            StructB(StructB),
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StructA {
            pub field_u8: u8,
            pub field_u16: u16,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for StructA {
            fn get_id() -> u32 {
                120
            }
            fn defaults() -> StructA {
                StructA {
                    field_u8: 0,
                    field_u16: 0,
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(121)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(122)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructA {
            fn get_id(&self) -> u32 { 120 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(121)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(122)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for StructA { }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StructB {
            pub field_u8: u8,
            pub field_u16: u16,
            pub strct: StructA,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for StructB {
            fn get_id() -> u32 {
                123
            }
            fn defaults() -> StructB {
                StructB {
                    field_u8: 0,
                    field_u16: 0,
                    strct: GroupB::GroupC::StructA {
                        field_u8: 0,
                        field_u16: 0,
                    },
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.field_u8 = match u8::get_from_storage(Source::Storage(&mut storage), Some(124)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_u16 = match u16::get_from_storage(Source::Storage(&mut storage), Some(125)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.strct = match StructA::get_from_storage(Source::Storage(&mut storage), Some(126)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for StructB {
            fn get_id(&self) -> u32 { 123 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.field_u8.get_buf_to_store(Some(124)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_u16.get_buf_to_store(Some(125)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.strct.get_buf_to_store(Some(126)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for StructB { }

    }

}

pub mod GroupD {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        EnumP(EnumP),
        StructP(StructP),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum EnumP {
        Option_a(StructA),
        Option_b(StructP),
        Option_c(GroupB::StructA),
        Option_d(GroupB::GroupC::StructA),
        Defaults,
    }
    impl EnumDecode for EnumP {
        fn get_id(&self) -> u32 { 132 }
        fn extract(buf: Vec<u8>) -> Result<EnumP, String> {
            if buf.len() <= sizes::U16_LEN {
                return Err(String::from("Fail to extract value for EnumP because buffer too small"));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
            let index = cursor.get_u16_le();
            let mut body_buf = vec![0; buf.len() - sizes::U16_LEN];
            body_buf.copy_from_slice(&buf[sizes::U16_LEN..]);
            match index {
                0 => match StructA::decode(&body_buf) {
                    Ok(v) => Ok(EnumP::Option_a(v)),
                    Err(e) => Err(e)
                },
                1 => match StructP::decode(&body_buf) {
                    Ok(v) => Ok(EnumP::Option_b(v)),
                    Err(e) => Err(e)
                },
                2 => match GroupB::StructA::decode(&body_buf) {
                    Ok(v) => Ok(EnumP::Option_c(v)),
                    Err(e) => Err(e)
                },
                3 => match GroupB::GroupC::StructA::decode(&body_buf) {
                    Ok(v) => Ok(EnumP::Option_d(v)),
                    Err(e) => Err(e)
                },
                _ => Err(String::from("Fail to find relevant value for EnumP")),
            }
        }
    }
    impl EnumEncode for EnumP {
        fn get_id(&self) -> u32 { 132 }
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
    impl PackingEnum for EnumP {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct StructP {
        pub field_a: StructA,
        pub field_b: GroupB::StructA,
        pub field_c: GroupB::GroupC::StructA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for StructP {
        fn get_id() -> u32 {
            128
        }
        fn defaults() -> StructP {
            StructP {
                field_a: StructA {
                    field_str: String::from(""),
                    field_str_empty: String::from(""),
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
                field_b: GroupB::StructA {
                    field_u8: 0,
                    field_u16: 0,
                },
                field_c: GroupB::GroupC::StructA {
                    field_u8: 0,
                    field_u16: 0,
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.field_a = match StructA::get_from_storage(Source::Storage(&mut storage), Some(129)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_b = match GroupB::StructA::get_from_storage(Source::Storage(&mut storage), Some(130)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_c = match GroupB::GroupC::StructA::get_from_storage(Source::Storage(&mut storage), Some(131)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for StructP {
        fn get_id(&self) -> u32 { 128 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.field_a.get_buf_to_store(Some(129)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_b.get_buf_to_store(Some(130)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_c.get_buf_to_store(Some(131)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for StructP { }

}

pub mod Events {
    use super::*;
    use std::io::Cursor;
    use bytes::{ Buf };
    #[derive(Debug, Clone)]
    pub enum AvailableMessages {
        EventA(EventA),
        EventB(EventB),
        Sub(Sub::AvailableMessages),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct EventA {
        pub uuid: String,
        pub field_a: StructA,
        pub field_b: StructB,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for EventA {
        fn get_id() -> u32 {
            141
        }
        fn defaults() -> EventA {
            EventA {
                uuid: String::from(""),
                field_a: StructA {
                    field_str: String::from(""),
                    field_str_empty: String::from(""),
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
                field_b: StructB {
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
                    field_struct: vec![],
                    field_str_empty: vec![],
                    field_u8_empty: vec![],
                    field_u16_empty: vec![],
                    field_u32_empty: vec![],
                    field_u64_empty: vec![],
                    field_i8_empty: vec![],
                    field_i16_empty: vec![],
                    field_i32_empty: vec![],
                    field_i64_empty: vec![],
                    field_f32_empty: vec![],
                    field_f64_empty: vec![],
                    field_bool_empty: vec![],
                    field_struct_empty: vec![],
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(142)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_a = match StructA::get_from_storage(Source::Storage(&mut storage), Some(143)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_b = match StructB::get_from_storage(Source::Storage(&mut storage), Some(144)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for EventA {
        fn get_id(&self) -> u32 { 141 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(142)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_a.get_buf_to_store(Some(143)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_b.get_buf_to_store(Some(144)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for EventA { }

    #[derive(Debug, Clone, PartialEq)]
    pub struct EventB {
        pub uuid: String,
        pub field_a: GroupA::StructA,
        pub field_b: GroupA::StructB,
        pub field_c: GroupB::StructA,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for EventB {
        fn get_id() -> u32 {
            145
        }
        fn defaults() -> EventB {
            EventB {
                uuid: String::from(""),
                field_a: GroupA::StructA {
                    field_u8: 0,
                    field_u16: 0,
                    opt: GroupA::EnumA::Defaults,
                },
                field_b: GroupA::StructB {
                    field_u8: 0,
                    field_u16: 0,
                    strct: GroupA::StructA {
                        field_u8: 0,
                        field_u16: 0,
                        opt: GroupA::EnumA::Defaults,
                    },
                },
                field_c: GroupB::StructA {
                    field_u8: 0,
                    field_u16: 0,
                },
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(146)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_a = match GroupA::StructA::get_from_storage(Source::Storage(&mut storage), Some(147)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_b = match GroupA::StructB::get_from_storage(Source::Storage(&mut storage), Some(148)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.field_c = match GroupB::StructA::get_from_storage(Source::Storage(&mut storage), Some(149)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for EventB {
        fn get_id(&self) -> u32 { 145 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(146)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_a.get_buf_to_store(Some(147)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_b.get_buf_to_store(Some(148)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.field_c.get_buf_to_store(Some(149)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for EventB { }

    pub mod Sub {
        use super::*;
        use std::io::Cursor;
        use bytes::{ Buf };
        #[derive(Debug, Clone)]
        pub enum AvailableMessages {
            EventA(EventA),
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventA {
            pub uuid: String,
            pub field_a: GroupB::GroupC::StructA,
            pub field_b: GroupB::GroupC::StructB,
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructDecode for EventA {
            fn get_id() -> u32 {
                151
            }
            fn defaults() -> EventA {
                EventA {
                    uuid: String::from(""),
                    field_a: GroupB::GroupC::StructA {
                        field_u8: 0,
                        field_u16: 0,
                    },
                    field_b: GroupB::GroupC::StructB {
                        field_u8: 0,
                        field_u16: 0,
                        strct: GroupB::GroupC::StructA {
                            field_u8: 0,
                            field_u16: 0,
                        },
                    },
                }
            }
            fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
                self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(152)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_a = match GroupB::GroupC::StructA::get_from_storage(Source::Storage(&mut storage), Some(153)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                self.field_b = match GroupB::GroupC::StructB::get_from_storage(Source::Storage(&mut storage), Some(154)) {
                    Ok(val) => val,
                    Err(e) => { return Err(e) },
                };
                Ok(())
            }
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        impl StructEncode for EventA {
            fn get_id(&self) -> u32 { 151 }
            fn get_signature(&self) -> u16 { 0 }
            fn abduct(&mut self) -> Result<Vec<u8>, String> {
                let mut buffer: Vec<u8> = vec!();
                match self.uuid.get_buf_to_store(Some(152)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_a.get_buf_to_store(Some(153)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                match self.field_b.get_buf_to_store(Some(154)) {
                    Ok(mut buf) => { buffer.append(&mut buf); }
                    Err(e) => { return Err(e) },
                };
                Ok(buffer)
            }
        }
        impl PackingStruct for EventA { }

    }

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
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SelfKeyResponse {
        pub uuid: String,
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructDecode for SelfKeyResponse {
        fn get_id() -> u32 {
            156
        }
        fn defaults() -> SelfKeyResponse {
            SelfKeyResponse {
                uuid: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.uuid = match String::get_from_storage(Source::Storage(&mut storage), Some(157)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for SelfKeyResponse {
        fn get_id(&self) -> u32 { 156 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.uuid.get_buf_to_store(Some(157)) {
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
            158
        }
        fn defaults() -> HashRequest {
            HashRequest {
                protocol: String::from(""),
                workflow: String::from(""),
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.protocol = match String::get_from_storage(Source::Storage(&mut storage), Some(159)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            self.workflow = match String::get_from_storage(Source::Storage(&mut storage), Some(160)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashRequest {
        fn get_id(&self) -> u32 { 158 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.protocol.get_buf_to_store(Some(159)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            match self.workflow.get_buf_to_store(Some(160)) {
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
            161
        }
        fn defaults() -> HashResponse {
            HashResponse {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(162)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for HashResponse {
        fn get_id(&self) -> u32 { 161 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(162)) {
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
            163
        }
        fn defaults() -> BeaconConfirmation {
            BeaconConfirmation {
                error: None,
            }
        }
        fn extract_from_storage(&mut self, mut storage: Storage) -> Result<(), String> {
            self.error = match Option::<String>::get_from_storage(Source::Storage(&mut storage), Some(164)) {
                Ok(val) => val,
                Err(e) => { return Err(e) },
            };
            Ok(())
        }
    }
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    impl StructEncode for BeaconConfirmation {
        fn get_id(&self) -> u32 { 163 }
        fn get_signature(&self) -> u16 { 0 }
        fn abduct(&mut self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = vec!();
            match self.error.get_buf_to_store(Some(164)) {
                Ok(mut buf) => { buffer.append(&mut buf); }
                Err(e) => { return Err(e) },
            };
            Ok(buffer)
        }
    }
    impl PackingStruct for BeaconConfirmation { }

}

impl DecodeBuffer<AvailableMessages> for Buffer<AvailableMessages> {
    fn get_msg(&self, id: u32, buf: &[u8]) -> Result<AvailableMessages, String> {
        match id {
            1 => match EnumA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumA(m)),
                Err(e) => Err(e),
            },
            2 => match EnumB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumB(m)),
                Err(e) => Err(e),
            },
            3 => match EnumC::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EnumC(m)),
                Err(e) => Err(e),
            },
            106 => match GroupA::EnumA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::EnumA(m))),
                Err(e) => Err(e),
            },
            132 => match GroupD::EnumP::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupD(GroupD::AvailableMessages::EnumP(m))),
                Err(e) => Err(e),
            },
            4 => match StructA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructA(m)),
                Err(e) => Err(e),
            },
            18 => match StructB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructB(m)),
                Err(e) => Err(e),
            },
            45 => match StructC::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructC(m)),
                Err(e) => Err(e),
            },
            58 => match StructD::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructD(m)),
                Err(e) => Err(e),
            },
            71 => match StructE::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructE(m)),
                Err(e) => Err(e),
            },
            75 => match StructF::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructF(m)),
                Err(e) => Err(e),
            },
            79 => match StructG::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructG(m)),
                Err(e) => Err(e),
            },
            82 => match TriggerBeaconsEmitter::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::TriggerBeaconsEmitter(m)),
                Err(e) => Err(e),
            },
            84 => match StructEmpty::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructEmpty(m)),
                Err(e) => Err(e),
            },
            85 => match StructEmptyA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructEmptyA(m)),
                Err(e) => Err(e),
            },
            86 => match StructEmptyB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructEmptyB(m)),
                Err(e) => Err(e),
            },
            87 => match StructJ::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::StructJ(m)),
                Err(e) => Err(e),
            },
            91 => match TriggerBeacons::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::TriggerBeacons(m)),
                Err(e) => Err(e),
            },
            92 => match FinishConsumerTest::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::FinishConsumerTest(m)),
                Err(e) => Err(e),
            },
            94 => match FinishConsumerTestBroadcast::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::FinishConsumerTestBroadcast(m)),
                Err(e) => Err(e),
            },
            95 => match BeaconA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::BeaconA(m)),
                Err(e) => Err(e),
            },
            98 => match Beacons::ShutdownServer::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::ShutdownServer(m))),
                Err(e) => Err(e),
            },
            99 => match Beacons::BeaconA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::BeaconA(m))),
                Err(e) => Err(e),
            },
            100 => match Beacons::BeaconB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::BeaconB(m))),
                Err(e) => Err(e),
            },
            103 => match Beacons::Sub::BeaconA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Beacons(Beacons::AvailableMessages::Sub(Beacons::Sub::AvailableMessages::BeaconA(m)))),
                Err(e) => Err(e),
            },
            107 => match GroupA::StructA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructA(m))),
                Err(e) => Err(e),
            },
            111 => match GroupA::StructB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupA(GroupA::AvailableMessages::StructB(m))),
                Err(e) => Err(e),
            },
            116 => match GroupB::StructA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::StructA(m))),
                Err(e) => Err(e),
            },
            120 => match GroupB::GroupC::StructA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructA(m)))),
                Err(e) => Err(e),
            },
            123 => match GroupB::GroupC::StructB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupB(GroupB::AvailableMessages::GroupC(GroupB::GroupC::AvailableMessages::StructB(m)))),
                Err(e) => Err(e),
            },
            128 => match GroupD::StructP::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::GroupD(GroupD::AvailableMessages::StructP(m))),
                Err(e) => Err(e),
            },
            133 => match EventA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EventA(m)),
                Err(e) => Err(e),
            },
            137 => match EventB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::EventB(m)),
                Err(e) => Err(e),
            },
            141 => match Events::EventA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::EventA(m))),
                Err(e) => Err(e),
            },
            145 => match Events::EventB::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::EventB(m))),
                Err(e) => Err(e),
            },
            151 => match Events::Sub::EventA::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::Events(Events::AvailableMessages::Sub(Events::Sub::AvailableMessages::EventA(m)))),
                Err(e) => Err(e),
            },
            156 => match InternalServiceGroup::SelfKeyResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::SelfKeyResponse(m))),
                Err(e) => Err(e),
            },
            158 => match InternalServiceGroup::HashRequest::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashRequest(m))),
                Err(e) => Err(e),
            },
            161 => match InternalServiceGroup::HashResponse::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::HashResponse(m))),
                Err(e) => Err(e),
            },
            163 => match InternalServiceGroup::BeaconConfirmation::extract(buf.to_vec()) {
                Ok(m) => Ok(AvailableMessages::InternalServiceGroup(InternalServiceGroup::AvailableMessages::BeaconConfirmation(m))),
                Err(e) => Err(e),
            },
            _ => Err(String::from("No message has been found"))
        }
    }
    fn get_signature(&self) -> u16 { 0 }
}

pub fn hash() -> String { String::from("2FE9D6137375F6B74B81143B6CA65EEAE6124B6C03C78937C4583DF0B0EF757A") }
