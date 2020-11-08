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

const U16_LEN: usize = mem::size_of::<u16>();
const U32_LEN: usize = mem::size_of::<u32>();

pub struct Storage {
    map: HashMap<String, Vec<u8>>,
}

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
        prop_name_buf.copy_from_slice(&buf[(pos + U16_LEN)..(pos + U16_LEN + prop_name_len_usize)]);
        match str::from_utf8(&prop_name_buf) {
            Ok(name) => Ok((name.to_string(), pos + U16_LEN + prop_name_len_usize)),
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
        prop_body_buf.copy_from_slice(&buf[(pos + U32_LEN)..(pos + U32_LEN + prop_body_len_usize)]);
        Ok((prop_body_buf, pos + U32_LEN + prop_body_len_usize))
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

mod DecodeTools {

    use std::io::Cursor;
    use std::mem;
    use bytes::{Buf};
    use super::{ Storage };

    const U8_LEN: usize = mem::size_of::<u8>();
    const U16_LEN: usize = mem::size_of::<u16>();
    const U32_LEN: usize = mem::size_of::<u32>();
    const U64_LEN: usize = mem::size_of::<u64>();
    const I8_LEN: usize = mem::size_of::<i8>();
    const I16_LEN: usize = mem::size_of::<i16>();
    const I32_LEN: usize = mem::size_of::<i32>();
    const I64_LEN: usize = mem::size_of::<i64>();
    const USIZE_LEN: usize = mem::size_of::<usize>();
    const F32_LEN: usize = mem::size_of::<f32>();
    const F64_LEN: usize = mem::size_of::<f64>();
    const BOOL_LEN: usize = mem::size_of::<bool>();

    pub fn get_u8(storage: &mut Storage, name: String) -> Result<u8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u16(storage: &mut Storage, name: String) -> Result<u16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < U16_LEN {
                return Err(format!("To extract u16 value buffer should have length at least {} bytes, but length is {}", U16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u32(storage: &mut Storage, name: String) -> Result<u32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < U32_LEN {
                return Err(format!("To extract u32 value buffer should have length at least {} bytes, but length is {}", U32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_u64(storage: &mut Storage, name: String) -> Result<u64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < U64_LEN {
                return Err(format!("To extract u64 value buffer should have length at least {} bytes, but length is {}", U64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i8(storage: &mut Storage, name: String) -> Result<i8, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < I8_LEN {
                return Err(format!("To extract i8 value buffer should have length at least {} bytes, but length is {}", I8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i8())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i16(storage: &mut Storage, name: String) -> Result<i16, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < I16_LEN {
                return Err(format!("To extract i16 value buffer should have length at least {} bytes, but length is {}", I16_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i16_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i32(storage: &mut Storage, name: String) -> Result<i32, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < I32_LEN {
                return Err(format!("To extract i32 value buffer should have length at least {} bytes, but length is {}", I32_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i32_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_i64(storage: &mut Storage, name: String) -> Result<i64, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < I64_LEN {
                return Err(format!("To extract i64 value buffer should have length at least {} bytes, but length is {}", I64_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_i64_le())
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

    pub fn get_bool(storage: &mut Storage, name: String) -> Result<bool, String> {
        if let Some(buf) = storage.get(name.clone()) {
            if buf.len() < U8_LEN {
                return Err(format!("To extract u8 value buffer should have length at least {} bytes, but length is {}", U8_LEN, buf.len()));
            }
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            Ok(cursor.get_u8() != 0)
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
                if len - cursor.position() < U16_LEN as u64 {
                    return Err(format!("To extract u16 value from array buffer should have length at least {} bytes, but length is {}", U16_LEN, buf.len()));
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
                if len - cursor.position() < U32_LEN as u64 {
                    return Err(format!("To extract u32 value from array buffer should have length at least {} bytes, but length is {}", U32_LEN, buf.len()));
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
                if len - cursor.position() < U64_LEN as u64 {
                    return Err(format!("To extract u64 value from array buffer should have length at least {} bytes, but length is {}", U64_LEN, buf.len()));
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
                if len - cursor.position() < I16_LEN as u64 {
                    return Err(format!("To extract i16 value from array buffer should have length at least {} bytes, but length is {}", I16_LEN, buf.len()));
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
                if len - cursor.position() < I32_LEN as u64 {
                    return Err(format!("To extract i32 value from array buffer should have length at least {} bytes, but length is {}", I32_LEN, buf.len()));
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
                if len - cursor.position() < I64_LEN as u64 {
                    return Err(format!("To extract i64 value from array buffer should have length at least {} bytes, but length is {}", I64_LEN, buf.len()));
                }
                res.push(cursor.get_i64_le());
            }
            Ok(res)
        } else {
            Err(format!("Buffer for property {} isn't found", name))
        }
    }

}

mod EncodeTools {

    use std::convert::TryFrom;
    use std::mem;

    const U8_LEN: usize = mem::size_of::<u8>();
    const U16_LEN: usize = mem::size_of::<u16>();
    const U32_LEN: usize = mem::size_of::<u32>();
    const U64_LEN: usize = mem::size_of::<u64>();
    const I8_LEN: usize = mem::size_of::<i8>();
    const I16_LEN: usize = mem::size_of::<i16>();
    const I32_LEN: usize = mem::size_of::<i32>();
    const I64_LEN: usize = mem::size_of::<i64>();
    const USIZE_LEN: usize = mem::size_of::<usize>();
    const F32_LEN: usize = mem::size_of::<f32>();
    const F64_LEN: usize = mem::size_of::<f64>();
    const BOOL_LEN: usize = mem::size_of::<bool>();

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
        get_value_buffer(name, U8_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u16(name: String, value: u16) -> Result<Vec<u8>, String> {
        get_value_buffer(name, U16_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u32(name: String, value: u32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, U32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_u64(name: String, value: u64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, U64_LEN as u32, value.to_le_bytes().to_vec())
    }
    pub fn get_i8(name: String, value: i8) -> Result<Vec<u8>, String> {
        get_value_buffer(name, I8_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i16(name: String, value: i16) -> Result<Vec<u8>, String> {
        get_value_buffer(name, I16_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i32(name: String, value: i32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, I32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_i64(name: String, value: i64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, I64_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_usize(name: String, value: usize) -> Result<Vec<u8>, String> {
        get_value_buffer(name, USIZE_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_f32(name: String, value: f32) -> Result<Vec<u8>, String> {
        get_value_buffer(name, F32_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_f64(name: String, value: f64) -> Result<Vec<u8>, String> {
        get_value_buffer(name, F64_LEN as u32, value.to_le_bytes().to_vec())
    }

    pub fn get_bool(name: String, value: bool) -> Result<Vec<u8>, String> {
        get_value_buffer(name, BOOL_LEN as u32, if value { vec![1] } else { vec![0] })
    }

    pub fn get_string(name: String, value: String) -> Result<Vec<u8>, String> {
        let buf = value.as_bytes();
        get_value_buffer(name, buf.len() as u32, buf.to_vec())
    } 

    pub fn get_u8_vec(name: String, value: Vec<u8>) -> Result<Vec<u8>, String> {
        let len = value.len() * U8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u16_vec(name: String, value: Vec<u16>) -> Result<Vec<u8>, String> {
        let len = value.len() * U16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u32_vec(name: String, value: Vec<u32>) -> Result<Vec<u8>, String> {
        let len = value.len() * U32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_u64_vec(name: String, value: Vec<u64>) -> Result<Vec<u8>, String> {
        let len = value.len() * U64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i8_vec(name: String, value: Vec<i8>) -> Result<Vec<u8>, String> {
        let len = value.len() * I8_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i16_vec(name: String, value: Vec<i16>) -> Result<Vec<u8>, String> {
        let len = value.len() * I16_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i32_vec(name: String, value: Vec<i32>) -> Result<Vec<u8>, String> {
        let len = value.len() * I32_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
    }

    pub fn get_i64_vec(name: String, value: Vec<u64>) -> Result<Vec<u8>, String> {
        let len = value.len() * I64_LEN;
        let mut buffer: Vec<u8> = vec!();
        for val in value.iter() {
            buffer.append(&mut val.to_le_bytes().to_vec());
        }
        get_value_buffer(name, len as u32, buffer.to_vec())
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
    pub prop_a: u16,
    pub prop_b: u32,
    pub prop_c: Vec<u8>,
    pub prop_d: Vec<u16>,
    pub prop_e: Vec<u32>,
    pub prop_f: Vec<u64>,
}

impl StructDecode for Target {

    fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
        self.prop_a = match DecodeTools::get_u16(&mut storage, String::from("prop_a")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_b = match DecodeTools::get_u32(&mut storage, String::from("prop_b")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_c = match DecodeTools::get_u8_vec(&mut storage, String::from("prop_c")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_d = match DecodeTools::get_u16_vec(&mut storage, String::from("prop_d")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_e = match DecodeTools::get_u32_vec(&mut storage, String::from("prop_e")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        self.prop_f = match DecodeTools::get_u64_vec(&mut storage, String::from("prop_f")) {
            Ok(val) => val,
            Err(e) => { return Err(e) },
        };
        Ok(())
    }

}

impl StructEncode for Target {

    fn encode(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        match EncodeTools::get_u16(String::from("prop_a"), self.prop_a) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u32(String::from("prop_b"), self.prop_b) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u8_vec(String::from("prop_c"), self.prop_c.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u16_vec(String::from("prop_d"), self.prop_d.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u32_vec(String::from("prop_e"), self.prop_e.clone()) {
            Ok(mut buf) => { buffer.append(&mut buf); },
            Err(e) => { return  Err(e); }
        };
        match EncodeTools::get_u64_vec(String::from("prop_f"), self.prop_f.clone()) {
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
            prop_a: 9,
            prop_b: 99,
            prop_c: vec![0, 1, 2, 3, 4],
            prop_d: vec![5, 6, 7, 8, 9],
            prop_e: vec![10, 11, 12, 13, 14],
            prop_f: vec![15, 16, 17, 18, 19],
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
            prop_a: 0,
            prop_b: 0,
            prop_c: vec![],
            prop_d: vec![],
            prop_e: vec![],
            prop_f: vec![],
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
        assert_eq!(a.prop_a, b.prop_a);
        assert_eq!(a.prop_b, b.prop_b);
        assert_eq!(a.prop_c, b.prop_c);
        assert_eq!(a.prop_d, b.prop_d);
        assert_eq!(a.prop_e, b.prop_e);
        assert_eq!(a.prop_f, b.prop_f);
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