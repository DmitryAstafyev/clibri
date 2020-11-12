use std::convert::TryFrom;
use std::io::Cursor;
use std::collections::{HashMap};
use bytes::{Buf};
use std::str;
use super::{ sizes };

pub struct Storage {
    map: HashMap<String, Vec<u8>>,
}

#[allow(dead_code)]
impl Storage {

    pub fn new(buf: Vec<u8>) -> Result<Self, String> {
        /* 
        | PROP_NAME_LEN | NAME    | PROP_BODY_LEN_GRAD | PROP_BODY_LEN | PROP_BODY | ... |
        | 2 bytes       | n bytes | 1 byte             | 1 - 8 bytes   | n bytes   | ... |
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
        prop_name_buf.copy_from_slice(&buf[(pos + sizes::U16_LEN)..(pos + sizes::U16_LEN + prop_name_len_usize)]);
        match str::from_utf8(&prop_name_buf) {
            Ok(name) => Ok((name.to_string(), pos + sizes::U16_LEN + prop_name_len_usize)),
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