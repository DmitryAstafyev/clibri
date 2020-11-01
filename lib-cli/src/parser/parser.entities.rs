#[allow(non_snake_case)]
pub mod Entities {

    #[derive(Debug)]
    pub enum EEntities {
        EStruct,
        EEnum,
    }

    #[allow(non_upper_case_globals)]
    pub mod centities {
        pub const TStruct: &str = "struct";
        pub const TEnum: &str = "enum";
    }

    pub fn is_valid(str: &str) -> bool {
        get_entity(str).is_some()
    }

    pub fn get_entity(str: &str) -> Option<EEntities> {
        match str {
            centities::TStruct => Some(EEntities::EStruct),
            centities::TEnum => Some(EEntities::EEnum),
            _ => None
        }
    }

}

