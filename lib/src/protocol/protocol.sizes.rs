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

pub enum ESize {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}
