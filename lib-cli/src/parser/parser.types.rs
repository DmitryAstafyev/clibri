pub mod PrimitiveTypes {
    
    #[derive(Debug)]
    pub enum ETypes {
        Ebool,
        Echar,
        Ei8,
        Ei16,
        Ei32,
        Ei64,
        Eisize,
        Eu8,
        Eu16,
        Eu32,
        Eu64,
        Eusize,
        Ef32,
        Ef64,
        Estr,
    }

    #[allow(non_upper_case_globals)]
    pub mod ctypes {
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
        pub const Tstr: &str = "str";
    }

    pub fn is_valid(str: &str) -> bool {
        get_entity(str).is_some()
    }

    pub fn get_entity(str: &str) -> Option<ETypes> {
        match str {
            ctypes::Tbool => Some(ETypes::Ebool),
            ctypes::Tchar => Some(ETypes::Echar),
            ctypes::Ti8 => Some(ETypes::Ei8),
            ctypes::Ti16 => Some(ETypes::Ei16),
            ctypes::Ti32 => Some(ETypes::Ei32),
            ctypes::Ti64 => Some(ETypes::Ei64),
            ctypes::Tisize => Some(ETypes::Eisize),
            ctypes::Tu8 => Some(ETypes::Eu8),
            ctypes::Tu16 => Some(ETypes::Eu16),
            ctypes::Tu32 => Some(ETypes::Eu32),
            ctypes::Tu64 => Some(ETypes::Eu64),
            ctypes::Tusize => Some(ETypes::Eusize),
            ctypes::Tf32 => Some(ETypes::Ef32),
            ctypes::Tf64 => Some(ETypes::Ef64),
            ctypes::Tstr => Some(ETypes::Estr),
            _ => None
        }
    }

}