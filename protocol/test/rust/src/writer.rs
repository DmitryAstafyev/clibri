#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;

pub fn write() -> Result<(), String> {
    StructExampleA {
        field_str: String::from("test"),
        field_u8: 1,
        field_u16: 2,
        field_u32: 3,
        field_u64: 4,
        field_i8: -1,
        field_i16: -2,
        field_i32: -3,
        field_i64: -4,
        field_f32: 0.1,
        field_f64: 0.2,
        field_bool: true,
    }.encode();
    Ok(())
}