use std::io::Cursor;
use bytes::{Buf};
use super::{ sizes, storage };
use storage::{ Storage };

pub trait StructDecode {

    fn defaults() -> Self;
    fn decode(&mut self, storage: Storage) -> Result<(), String>;

}

pub trait Decode<T> {

    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<T, String>;

}

impl Decode<u8> for u8 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<u8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<u16> for u16 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<u16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::U16_LEN {
                return Err(format!("To extract u16 value buffer should have length at least {} bytes, but length is {}", sizes::U16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<u32> for u32 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<u32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::U32_LEN {
                return Err(format!("To extract u32 value buffer should have length at least {} bytes, but length is {}", sizes::U32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<u64> for u64 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<u64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::U64_LEN {
                return Err(format!("To extract u64 value buffer should have length at least {} bytes, but length is {}", sizes::U64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<i8> for i8 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<i8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::I8_LEN {
                return Err(format!("To extract i8 value buffer should have length at least {} bytes, but length is {}", sizes::I8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<i16> for i16 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<i16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::I16_LEN {
                return Err(format!("To extract i16 value buffer should have length at least {} bytes, but length is {}", sizes::I16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<i32> for i32 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<i32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::I32_LEN {
                return Err(format!("To extract i32 value buffer should have length at least {} bytes, but length is {}", sizes::I32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<i64> for i64 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<i64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::I64_LEN {
                return Err(format!("To extract i64 value buffer should have length at least {} bytes, but length is {}", sizes::I64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<f32> for f32 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<f32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::F32_LEN {
                return Err(format!("To extract f32 value buffer should have length at least {} bytes, but length is {}", sizes::F32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<f64> for f64 {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<f64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::F64_LEN {
                return Err(format!("To extract f64 value buffer should have length at least {} bytes, but length is {}", sizes::F64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<bool> for bool {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<bool, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8() != 0)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<String> for String {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<String, String> {
        if let Some(buf) = storage.get(name.clone()) {
            Ok(String::from_utf8_lossy(buf).to_string())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl<T> Decode<T> for T where T: StructDecode,  {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<T, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let sctruct_storage = match Storage::new(buf.to_vec()) {
                Ok(storage) => storage,
                Err(e) => {
                    return Err(e);
                }
            };
            let mut strct: T = T::defaults();
            match strct.decode(sctruct_storage) {
                Ok(_) => Ok(strct),
                Err(e) => Err(e),
            }
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<u8>> for Vec<u8> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<u8>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<u16>> for Vec<u16> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<u16>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<u32>> for Vec<u32> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<u32>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<u64>> for Vec<u64> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<u64>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<i8>> for Vec<i8> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<i8>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<i16>> for Vec<i16> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<i16>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<i32>> for Vec<i32> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<i32>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<i64>> for Vec<i64> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<i64>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl Decode<Vec<String>> for Vec<String> {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<String>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}

impl<T> Decode<Vec<T>> for Vec<T> where T: StructDecode {
    fn decode(&mut self, storage: &mut Storage, name: String) -> Result<Vec<T>, String> {
        if let Some(buf) = storage.get(name.clone()) {
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
                match strct.decode(sctruct_storage) {
                    Ok(_) => {},
                    Err(e) => { return Err(e); },
                }
                res.push(strct);
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }
}