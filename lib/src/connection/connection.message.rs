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

mod StructEncodeDecode {

    use std::convert::TryFrom;

    pub fn get_name(name: String) -> Result<(Vec<u8>, u16), String> {
        let bytes = name.as_bytes();
        match u16::try_from(bytes.len()) {
            Ok(len) => Ok((bytes.to_vec(), len)),
            Err(e) => Err(format!("Looks like name of variable is too long. Error: {}", e))
        }
    }

}

trait StructDecode {

    fn decode(&mut self, storage: Storage) -> Result<(), String>;

}

trait StructEncode {

    fn encode(self) -> Result<Vec<u8>, String>;

}

#[derive(Debug, Clone)]
struct Target {
    pub prop_a: u16,
    pub prop_b: u32,
}

impl StructDecode for Target {

    fn decode(&mut self, mut storage: Storage) -> Result<(), String> {
        let prop_a: u16;
        let prop_b: u32;
        if let Some(buf) = storage.get(String::from("prop_a")) {
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            prop_a = cursor.get_u16_le();
        } else {
            return Err("Buffer for property prop_a isn't found".to_string());
        }
        if let Some(buf) = storage.get(String::from("prop_b")) {
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            prop_b = cursor.get_u32_le();
        } else {
            return Err("Buffer for property prop_b isn't found".to_string());
        }
        self.prop_a = prop_a;
        self.prop_b = prop_b;
        Ok(())
    }

}

impl StructEncode for Target {

    fn encode(self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = vec!();
        let (buf, len) = match StructEncodeDecode::get_name(String::from("prop_a")) {
            Ok((name_buf, len)) => (name_buf, len),
            Err(e) => { return  Err(e); }
        };
        buffer.append(&mut len.to_le_bytes().to_vec());
        buffer.append(&mut buf.to_vec());
        buffer.append(&mut (U16_LEN as u32).to_le_bytes().to_vec());
        buffer.append(&mut self.prop_a.to_le_bytes().to_vec());
        let (buf, len) = match StructEncodeDecode::get_name(String::from("prop_b")) {
            Ok((name_buf, len)) => (name_buf, len),
            Err(e) => { return  Err(e); }
        };
        buffer.append(&mut len.to_le_bytes().to_vec());
        buffer.append(&mut buf.to_vec());
        buffer.append(&mut (U32_LEN as u32).to_le_bytes().to_vec());
        buffer.append(&mut self.prop_b.to_le_bytes().to_vec());
        Ok(buffer)
    }

}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn encode_decode() {
        let a: Target = Target {
            prop_a: 9,
            prop_b: 99,
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
        assert_eq!(false, true);
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