use std::convert::TryFrom;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::io::Cursor;
use std::collections::{HashMap};
use bytes::{Buf};

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

struct Target {
    pub propA: u16,
    pub propB: u32,
}

impl Target {

    pub fn new(props: HashMap<String, &[u8]>) -> Result<Self, String> {
        let propA: u16;
        let propB: u32;
        if let Some(buf) = props.get("propA") {
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            propA = cursor.get_u16_le();
        } else {
            return Err("Buffer for property propA isn't found".to_string());
        }
        if let Some(buf) = props.get("propB") {
            let mut cursor: Cursor<&[u8]> = Cursor::new(buf);
            propB = cursor.get_u32_le();
        } else {
            return Err("Buffer for property propB isn't found".to_string());
        }
        Ok(Target {
            propA,
            propB,
        })
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