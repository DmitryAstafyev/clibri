use super::sizes;
use sizes::ESize;

// injectable
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
