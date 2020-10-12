pub mod PrimitiveTypes {

    pub mod CTypes {
        pub const Tbool: &str = "bool";
        pub const Tchar: &str = "char";
        pub const Ti8: &str = "i8";
        pub const Ti16: &str = "i16";
        pub const Ti32: &str = "i32";
        pub const Ti64: &str = "i64";
        pub const Tisize: &str = "isize";
        pub const Tu8: &str = "u8";
        pub const Tu16: &str = "u16";
        pub const Tu32: &str = "u32";
        pub const Tu64: &str = "u64";
        pub const Tusize: &str = "usize";
        pub const Tf32: &str = "f32";
        pub const Tf64: &str = "f64";
        pub const Tstr: &str = "&str";
    }

    const Available: Vec<&str> = vec![
        CTypes::Tbool,
        CTypes::Tchar,
        CTypes::Ti8,
        CTypes::Ti16,
        CTypes::Ti32,
        CTypes::Ti64,
        CTypes::Tisize,
        CTypes::Tu8,
        CTypes::Tu16,
        CTypes::Tu32,
        CTypes::Tu64,
        CTypes::Tusize,
        CTypes::Tf32,
        CTypes::Tf64,
        CTypes::Tstr,
    ];

    pub fn isValid(str: &str) -> bool {
        match Available.iter().position(|&t| t == str) {
            Some(_) => true,
            _ => false,
        }
    }

}