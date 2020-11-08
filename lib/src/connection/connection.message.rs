use std::convert::TryFrom;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::io::Cursor;
use std::collections::{HashMap};
use bytes::{Buf};
use std::str;
use std::mem;

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingOutStruct {
    pub uuid: String,
}

pub struct PingOut {
    _msg: PingOutStruct,
}

Message:
| PACKAGE_SIGNATURE | PACKAGE_LEN | MSG_ID | MSG_BODY |
-------------------------------------------------------
| 8 byte            | 8 byte      | 4 byte | len      |

// Primitive (lengths are defined in map)
| PROP_ID | VALUE  |
| 4 byte  | n byte |
// Repeated
| PROP_ID | LEN    | ITEM_TYPE_ID | ITEMS_BODY         |
|         |        |              | ITEM  | ... | ITEM |
| 4 byte  | 8 byte | 4 byte       |       |     |      |
// Struct
| PROP_ID | VALUE  |
| 4 byte  | n byte |
*/

#[allow(non_snake_case)]
#[allow(dead_code)]
mod Sizes {

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

pub struct Storage {
    map: HashMap<String, Vec<u8>>,
}

#[allow(dead_code)]
impl Storage {

    fn new(buf: Vec<u8>) -> Result<Self, String> {
        /* 
        | PROP_NAME_LEN | NAME    | PROP_BODY_LEN | PROP_BODY | ... |
        | 2 bytes       | n bytes | 4 bytes       | n bytes   | ... |
        */
        let mut position: usize = 0;
        let mut map: HashMap<String, Vec<u8>> = HashMap::new();
        loop {
            match Storage::next(&buf, position) {
                Ok((name, body, pos)) => {
                    position = pos;
                    map.insert(name, body);
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

    fn name(buf: &[u8], pos: usize) -> Result<(String, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        if let Ok(pos) = u64::try_from(pos) {
            cursor.set_position(pos);
        } else {
            return Err("Fail to set cursor position".to_string());
        }
        let prop_name_len_u16 = cursor.get_u16_le();
        let prop_name_len_usize: usize;
        if let Ok(val) = usize::try_from(prop_name_len_u16) {
            prop_name_len_usize = val;
        } else {
            return Err("Fail convert length of name from u16 to usize".to_string());
        }
        let mut prop_name_buf = vec![0; prop_name_len_usize];
        prop_name_buf.copy_from_slice(&buf[(pos + Sizes::U16_LEN)..(pos + Sizes::U16_LEN + prop_name_len_usize)]);
        match str::from_utf8(&prop_name_buf) {
            Ok(name) => Ok((name.to_string(), pos + Sizes::U16_LEN + prop_name_len_usize)),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn body(buf: &[u8], pos: usize) -> Result<(Vec<u8>, usize), String> {
        let mut cursor: Cursor<&[u8]> = Cursor::new(&buf);
        if let Ok(pos) = u64::try_from(pos) {
            cursor.set_position(pos);
        } else {
            return Err("Fail to set cursor position".to_string());
        }
        let prop_body_len_u32 = cursor.get_u32_le();
        let prop_body_len_usize: usize;
        if let Ok(val) = usize::try_from(prop_body_len_u32) {
            prop_body_len_usize = val;
        } else {
            return Err("Fail convert length of name from u16 to usize".to_string());
        }
        let mut prop_body_buf = vec![0; prop_body_len_usize];
        prop_body_buf.copy_from_slice(&buf[(pos + Sizes::U32_LEN)..(pos + Sizes::U32_LEN + prop_body_len_usize)]);
        Ok((prop_body_buf, pos + Sizes::U32_LEN + prop_body_len_usize))
    }

    fn next(buf: &[u8], pos: usize) -> Result<(String, Vec<u8>, usize), String> {
        match Storage::name(buf, pos) {
            Ok((name, pos)) => {
                match Storage::body(buf, pos) {
                    Ok((body, pos)) => Ok((name, body, pos)),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn get(&mut self, name: String) -> Option<&Vec<u8>> {
        self.map.get(&name)
    }

}

#[allow(non_snake_case)]
#[allow(dead_code)]
mod DecodeTools {

    use std::io::Cursor;
    use bytes::{Buf};
    use super::{ Storage, Sizes };

    pub fn get_u8(storage: &mut Storage, name: String) -> Result<u8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", Sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u16(storage: &mut Storage, name: String) -> Result<u16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::U16_LEN {
                return Err(format!("To extract u16 value buffer should have length at least {} bytes, but length is {}", Sizes::U16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u32(storage: &mut Storage, name: String) -> Result<u32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::U32_LEN {
                return Err(format!("To extract u32 value buffer should have length at least {} bytes, but length is {}", Sizes::U32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u64(storage: &mut Storage, name: String) -> Result<u64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::U64_LEN {
                return Err(format!("To extract u64 value buffer should have length at least {} bytes, but length is {}", Sizes::U64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i8(storage: &mut Storage, name: String) -> Result<i8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::I8_LEN {
                return Err(format!("To extract i8 value buffer should have length at least {} bytes, but length is {}", Sizes::I8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i16(storage: &mut Storage, name: String) -> Result<i16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::I16_LEN {
                return Err(format!("To extract i16 value buffer should have length at least {} bytes, but length is {}", Sizes::I16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i32(storage: &mut Storage, name: String) -> Result<i32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::I32_LEN {
                return Err(format!("To extract i32 value buffer should have length at least {} bytes, but length is {}", Sizes::I32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i64(storage: &mut Storage, name: String) -> Result<i64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::I64_LEN {
                return Err(format!("To extract i64 value buffer should have length at least {} bytes, but length is {}", Sizes::I64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_bool(storage: &mut Storage, name: String) -> Result<bool, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", Sizes::U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8() != 0)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_f32(storage: &mut Storage, name: String) -> Result<f32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::F32_LEN {
                return Err(format!("To extract f32 value buffer should have length at least {} bytes, but length is {}", Sizes::F32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_f64(storage: &mut Storage, name: String) -> Result<f64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < Sizes::F64_LEN {
                return Err(format!("To extract f64 value buffer should have length at least {} bytes, but length is {}", Sizes::F64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_f64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_utf8_string(storage: &mut Storage, name: String) -> Result<String, String> {
        if let Some(buf) = storage.get(name.clone()) {
            Ok(String::from_utf8_lossy(buf).to_string())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    } 

    pub fn get_u8_vec(storage: &mut Storage, name: String) -> Result<Vec<u8>, String> {
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

    pub fn get_u16_vec(storage: &mut Storage, name: String) -> Result<Vec<u16>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<u16> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::U16_LEN as u64 {
                    return Err(format!("To extract u16 value from array buffer should have length at least {} bytes, but length is {}", Sizes::U16_LEN, buf.len()));
                }
                res.push(cursor.get_u16_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u32_vec(storage: &mut Storage, name: String) -> Result<Vec<u32>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<u32> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::U32_LEN as u64 {
                    return Err(format!("To extract u32 value from array buffer should have length at least {} bytes, but length is {}", Sizes::U32_LEN, buf.len()));
                }
                res.push(cursor.get_u32_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u64_vec(storage: &mut Storage, name: String) -> Result<Vec<u64>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<u64> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::U64_LEN as u64 {
                    return Err(format!("To extract u64 value from array buffer should have length at least {} bytes, but length is {}", Sizes::U64_LEN, buf.len()));
                }
                res.push(cursor.get_u64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i8_vec(storage: &mut Storage, name: String) -> Result<Vec<i8>, String> {
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

    pub fn get_i16_vec(storage: &mut Storage, name: String) -> Result<Vec<i16>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<i16> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::I16_LEN as u64 {
                    return Err(format!("To extract i16 value from array buffer should have length at least {} bytes, but length is {}", Sizes::I16_LEN, buf.len()));
                }
                res.push(cursor.get_i16_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i32_vec(storage: &mut Storage, name: String) -> Result<Vec<i32>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<i32> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::I32_LEN as u64 {
                    return Err(format!("To extract i32 value from array buffer should have length at least {} bytes, but length is {}", Sizes::I32_LEN, buf.len()));
                }
                res.push(cursor.get_i32_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i64_vec(storage: &mut Storage, name: String) -> Result<Vec<i64>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<i64> = vec!();
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            let len = buf.len() as u64;
            loop {
                if cursor.position() == len {
                    break;
                }
                if len - cursor.position() < Sizes::I64_LEN as u64 {
                    return Err(format!("To extract i64 value from array buffer should have length at least {} bytes, but length is {}", Sizes::I64_LEN, buf.len()));
                }
                res.push(cursor.get_i64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_utf8_string_vec(storage: &mut Storage, name: String) -> Result<Vec<String>, String> {
        if let Some(buf) = storage.get(name.clone()) {
            let mut res: Vec<String> = vec!();
            let mut buffer = vec![0; buf.len()];
            buffer.copy_from_slice(&buf[0..buf.len()]);
            loop {
                if buffer.is_empty() {
                    break;
                }
                let mut cursor: Cursor<&[u8]> = Cursor::new(&buffer);
                if buffer.len() < Sizes::U32_LEN {
                    return Err(format!("To extract length of string (u32) value from array buffer should have length at least {} bytes, but length is {}", Sizes::U32_LEN, buf.len()));
                }
                let item_len: u32 = cursor.get_u32_le();
                if buffer.len() < Sizes::U32_LEN + item_len as usize {
                    return Err(format!("Cannot extract string, because expecting {} bytes, but length of buffer is {}", item_len, (buffer.len() - Sizes::U32_LEN)));
                }
                let mut item_buf = vec![0; item_len as usize];
                item_buf.copy_from_slice(&buffer[Sizes::U32_LEN..(Sizes::U32_LEN + item_len as usize)]);
                buffer = buffer.drain((Sizes::U32_LEN + item_len as usize)..).collect();
                res.push(String::from_utf8_lossy(&item_buf).to_string());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

}

#[allow(non_snake_case)]
#[allow(dead_code)]
mod EncodeTools {

    use std::convert::TryFrom;
    use super::{ Sizes };

    pub fn get_name(name: String) -> Result<(Vec<u8>, u16), String> {
        let bytes = name.as_bytes();
        match u16::try_from(bytes.len()) {
            Ok(len) => Ok((bytes.to_vec(), len)),
            Err(e) => Err(format!("Looks like name of variable is too long. Error: {}", e))
        }
    }

    pub fn get_value_buffer(name: String, size: u32, mut value: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        let (buf, len) = match get_name(name) {
            Ok((name_buf, len)) => (name_buf, len),
            Err(e) => { return  Err(e); }
        };
        buffer.append(&mut len.to_le_bytes().to_vec());
        buffer.append(&mut buf.to_vec());
        buffer.append(&mut size.to_le_bytes().to_vec());
        buffer.append(&mut value);
        Ok(buffer)
    }

    pub fn get_u8(name: String, value: u8) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::U8_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u16(name: String, value: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::U16_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u32(name: String, value: u32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::U32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u64(name: String, value: u64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::U64_LEN as u32, value.to_le_bytes().to_vec())
    }
    pub fn get_i8(name: String, value: i8) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::I8_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i16(name: String, value: i16) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::I16_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i32(name: String, value: i32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::I32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i64(name: String, value: i64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::I64_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_f32(name: String, value: f32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::F32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_f64(name: String, value: f64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::F64_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_bool(name: String, value: bool) -> Result<Vec<u8>, String> {
        get_value_buffer(name, Sizes::BOOL_LEN as u32, if value { vec![1] } else { vec![0] })
    }

    pub fn get_utf8_string(name: String, value: String) -> Result<Vec<u8>, String> {
        let buf = value.as_bytes();
        get_value_buffer(name, buf.len() as u32, buf.to_vec())
    }

    pub fn get_u8_vec(name: String, value: Vec<u8>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::U8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u16_vec(name: String, value: Vec<u16>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::U16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u32_vec(name: String, value: Vec<u32>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::U32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u64_vec(name: String, value: Vec<u64>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::U64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i8_vec(name: String, value: Vec<i8>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::I8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i16_vec(name: String, value: Vec<i16>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::I16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i32_vec(name: String, value: Vec<i32>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::I32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i64_vec(name: String, value: Vec<i64>) -> Result<Vec<u8>, String> {
        let len = value.len() * Sizes::I64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_utf8_string_vec(name: String, value: Vec<String>) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            let val_as_bytes = val.as_bytes();
            buffer.append(&mut (val_as_bytes.len() as u32).to_le_bytes().to_vec());
            buffer.append(&mut val_as_bytes.to_vec());
        }
        get_value_buffer(name, buffer.len() as u32, buffer.to_vec())
    }

}

trait StructDecode {

    fn decode(&mut self, storage: Storage) -> Result<(), String>;

}

trait StructEncode {

    fn encode(&mut self) -> Result<Vec<u8>, String>;

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
    pub prop_string: String,
    pub prop_f32: f32,
    pub prop_f64: f64,
    pub prop_utf8_string_vec: Vec<String>,
}

impl StructDecode for Target {

    fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
        self.prop_u8 = match DecodeTools::get_u8(&mut storage, String::from("prop_u8")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u16 = match DecodeTools::get_u16(&mut storage, String::from("prop_u16")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u32 = match DecodeTools::get_u32(&mut storage, String::from("prop_u32")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u64 = match DecodeTools::get_u64(&mut storage, String::from("prop_u64")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i8 = match DecodeTools::get_i8(&mut storage, String::from("prop_i8")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i16 = match DecodeTools::get_i16(&mut storage, String::from("prop_i16")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i32 = match DecodeTools::get_i32(&mut storage, String::from("prop_i32")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i64 = match DecodeTools::get_i64(&mut storage, String::from("prop_i64")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u8_vec = match DecodeTools::get_u8_vec(&mut storage, String::from("prop_u8_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u16_vec = match DecodeTools::get_u16_vec(&mut storage, String::from("prop_u16_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u32_vec = match DecodeTools::get_u32_vec(&mut storage, String::from("prop_u32_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_u64_vec = match DecodeTools::get_u64_vec(&mut storage, String::from("prop_u64_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i8_vec = match DecodeTools::get_i8_vec(&mut storage, String::from("prop_i8_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i16_vec = match DecodeTools::get_i16_vec(&mut storage, String::from("prop_i16_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i32_vec = match DecodeTools::get_i32_vec(&mut storage, String::from("prop_i32_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_i64_vec = match DecodeTools::get_i64_vec(&mut storage, String::from("prop_i64_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_string = match DecodeTools::get_utf8_string(&mut storage, String::from("prop_string")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_f32 = match DecodeTools::get_f32(&mut storage, String::from("prop_f32")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_f64 = match DecodeTools::get_f64(&mut storage, String::from("prop_f64")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_utf8_string_vec = match DecodeTools::get_utf8_string_vec(&mut storage, String::from("prop_utf8_string_vec")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }

}

impl StructEncode for Target {

    fn encode(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match EncodeTools::get_u8(String::from("prop_u8"), self.prop_u8) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u16(String::from("prop_u16"), self.prop_u16) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u32(String::from("prop_u32"), self.prop_u32) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u64(String::from("prop_u64"), self.prop_u64) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i8(String::from("prop_i8"), self.prop_i8) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i16(String::from("prop_i16"), self.prop_i16) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i32(String::from("prop_i32"), self.prop_i32) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i64(String::from("prop_i64"), self.prop_i64) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u8_vec(String::from("prop_u8_vec"), self.prop_u8_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u16_vec(String::from("prop_u16_vec"), self.prop_u16_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u32_vec(String::from("prop_u32_vec"), self.prop_u32_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u64_vec(String::from("prop_u64_vec"), self.prop_u64_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i8_vec(String::from("prop_i8_vec"), self.prop_i8_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i16_vec(String::from("prop_i16_vec"), self.prop_i16_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i32_vec(String::from("prop_i32_vec"), self.prop_i32_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_i64_vec(String::from("prop_i64_vec"), self.prop_i64_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_utf8_string(String::from("prop_string"), self.prop_string.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_f32(String::from("prop_f32"), self.prop_f32) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_f64(String::from("prop_f64"), self.prop_f64) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_utf8_string_vec(String::from("prop_utf8_string_vec"), self.prop_utf8_string_vec.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        Ok(buffer)
    }

}

#[cfg(test)]
mod tests { 
    use super::*;

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
        let mut b: Target = Target {
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
            prop_string: String::from(""),
            prop_f32: 0.0,
            prop_f64: 0.0,
            prop_utf8_string_vec: vec![],
        };
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
        assert_eq!(a.prop_f32, b.prop_f32);
        assert_eq!(a.prop_f64, b.prop_f64);
        assert_eq!(a.prop_utf8_string_vec, b.prop_utf8_string_vec);
        assert_eq!(true, false);

    }
}

pub enum EPrimitive {

}

pub struct Prop {
    pub len: u64,
    pub name: String,
    pub optional: bool,
    pub repeated: bool,
    pub primitive: Option<EPrimitive>,
    pub enumed: Option<u32>,
    pub structed: Option<u32>,
}

pub trait Protocol {

    fn get_prop_desc(&mut self, id: u32) -> Result<Prop, String>;

}

const PROP_ID_LEN: u64 = 4;

pub trait Struct {

    type Msg;

    /*
    // Primitive (lengths are defined in map)
    | PROP_ID | VALUE  |
    | 4 byte  | n byte |

    // Repeated
    | PROP_ID | LEN    | ITEM_TYPE_ID | ITEMS_BODY         |
    |         |        |              | ITEM  | ... | ITEM |
    | 4 byte  | 8 byte | 4 byte       |       |     |      |

    // Struct
    | PROP_ID | VALUE  |
    | 4 byte  | n byte |
    */
    fn read(buffer: &[u8], mut protocol: impl Protocol) -> Result<(), String> {
        if let Ok(len) = u64::try_from(buffer.len()) {
            let mut cursor: Cursor<&[u8]> = Cursor::new(buffer);
            loop {
                if len - cursor.position() < PROP_ID_LEN {
                    return Err("Fail to read prop_id, not enought bytes in buffer".to_string());
                }
                let prop_id: u32 = cursor.get_u32_le();
                match protocol.get_prop_desc(prop_id) {
                    Ok(desc) => {

                    },
                    Err(e) => {
                        return Err(e);
                    },
                }
            }
        } else {
            Err("Fail to convert buffer len to u64".to_string())
        }
    }


}