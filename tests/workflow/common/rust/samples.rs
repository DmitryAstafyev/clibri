#[path = "../../producer/rust/src/producer/implementation/protocol/mod.rs"]
pub mod protocol;

pub mod struct_a {
    use super::*;
    pub fn get() -> protocol::StructA {
        protocol::StructA {
            field_str: String::from("test"),
            field_str_empty: String::from(""),
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
        }
    }
    pub fn equal(strct: protocol::StructA) -> bool {
        get() == strct
    }
}
pub mod struct_b {
    use super::*;
    pub fn get() -> protocol::StructB {
        protocol::StructB {
            field_str: vec![String::from("test_a"), String::from("test_b")],
            field_u8: vec![1, 2, 3, 4],
            field_u16: vec![1, 2, 3, 4],
            field_u32: vec![1, 2, 3, 4],
            field_u64: vec![1, 2],
            field_i8: vec![-1, -2, -3, -4],
            field_i16: vec![-1, -2, -3, -4],
            field_i32: vec![-1, -2, -3, -4],
            field_i64: vec![-1, -2],
            field_f32: vec![0.1, 0.2, 0.3, 0.4],
            field_f64: vec![0.1, 0.2, 0.3, 0.4],
            field_bool: vec![true, false, true],
            field_struct: vec![struct_a::get(), struct_a::get(), struct_a::get()],
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
    pub fn equal(strct: protocol::StructB) -> bool {
        get() == strct
    }
}
pub mod struct_c {
    use super::*;
    pub fn get() -> protocol::StructC {
        protocol::StructC {
            field_str: Some(String::from("test")),
            field_u8: Some(1),
            field_u16: Some(2),
            field_u32: Some(3),
            field_u64: Some(4),
            field_i8: None,
            field_i16: None,
            field_i32: None,
            field_i64: None,
            field_f32: None,
            field_f64: None,
            field_bool: None,
        }
    }
    pub fn equal(strct: protocol::StructC) -> bool {
        get() == strct
    }
}
pub mod struct_d {
    use super::*;
    pub fn get() -> protocol::StructD {
        protocol::StructD {
            field_str: Some(vec![String::from("test_a"), String::from("test_b")]),
            field_u8: Some(vec![1, 2, 3, 4]),
            field_u16: Some(vec![1, 2, 3, 4]),
            field_u32: Some(vec![1, 2, 3, 4]),
            field_u64: Some(vec![1, 2]),
            field_i8: None,
            field_i16: None,
            field_i32: None,
            field_i64: None,
            field_f32: None,
            field_f64: None,
            field_bool: None,
        }
    }
    pub fn equal(strct: protocol::StructD) -> bool {
        get() == strct
    }
}
pub mod struct_e {
    use super::*;
    pub fn get() -> protocol::StructE {
        protocol::StructE {
            field_a: protocol::EnumA::Option_a(String::from("Option_a")),
            field_b: protocol::EnumB::Option_u8(1),
            field_c: protocol::EnumC::Option_u8(vec![1]),
        }
    }
    pub fn equal(strct: protocol::StructE) -> bool {
        get() == strct
    }
}
pub mod struct_f {
    use super::*;
    pub fn get() -> protocol::StructF {
        protocol::StructF {
            field_a: None,
            field_b: None,
            field_c: Some(protocol::EnumC::Option_u8(vec![1])),
        }
    }
    pub fn equal(strct: protocol::StructF) -> bool {
        get() == strct
    }
}
pub mod struct_g {
    use super::*;
    pub fn get() -> protocol::StructG {
        protocol::StructG {
            field_a: struct_a::get(),
            field_b: struct_b::get(),
        }
    }
    pub fn equal(strct: protocol::StructG) -> bool {
        get() == strct
    }
}
pub mod struct_j {
    use super::*;
    pub fn get() -> protocol::StructJ {
        protocol::StructJ {
            field_a: Some(struct_a::get()),
            field_b: None,
            field_c: protocol::StructEmpty {},
        }
    }
    pub fn equal(strct: protocol::StructJ) -> bool {
        get() == strct
    }
}
