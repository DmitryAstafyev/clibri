#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{File, create_dir};
use std::path::{PathBuf};
use std::io::prelude::*;

pub fn get_ts_bin_dir() -> Result<PathBuf, String> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dest) = exe.as_path().parent() {
            let dest = dest.join("../../../typescript/binary");
            if !dest.exists() {
                if let Err(e) = create_dir(dest.clone()) {
                    return Err(format!("{}", e));
                }
            }
            Ok(dest)
        } else {
            Err("Fail to find ts-bin path".to_string())
        }
    } else {
        Err("Fail to find ts-bin path".to_string())
    }
}

pub fn read_file(path: PathBuf) -> Result<Vec<u8>, String> {
    if !path.exists() {
        return Err(format!("File {:?} doesn't exist", path))
    }
    let mut file = match File::open(path.clone()) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Fail to open file {:?} due error: {}", path, e));
        }
    };
    let mut buffer = Vec::new();
    // read the whole file
    if let Err(e) = file.read_to_end(&mut buffer) {
        Err(format!("Fail to read file {:?} due error: {}", path, e))
    } else {
        Ok(buffer)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleA(entity: StructExampleA) {
    let src = StructExampleA {
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
    };
    if entity != src {
        panic!("StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleB(entity: StructExampleB) {
    let src = StructExampleB {
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
        field_struct: vec![
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
            },
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
            },
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
            }
        ],
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
    };
    if entity != src {
        panic!("StructExampleB: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleC(entity: StructExampleC) {
    let src = StructExampleC {
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
    };
    if entity != src {
        panic!("StructExampleC: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleD(entity: StructExampleD) {
    let src = StructExampleD {
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
    };
    if entity != src {
        panic!("StructExampleD: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleE(entity: StructExampleE) {
    let src = StructExampleE {
        field_a: EnumExampleA::Option_a(String::from("Option_a")),
        field_b: EnumExampleB::Option_u8(1),
        field_c: EnumExampleC::Option_u8(vec![1]),
    };
    if entity != src {
        panic!("StructExampleE: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleF(entity: StructExampleF) {
    let src = StructExampleF {
        field_a: None,
        field_b: None,
        field_c: Some(EnumExampleC::Option_u8(vec![1])),
    };
    if entity != src {
        panic!("StructExampleF: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleG(entity: StructExampleG) {
    let src = StructExampleG {
        field_a: StructExampleA {
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
        },
        field_b: StructExampleB {
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
            field_struct: vec![
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
                },
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
                },
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
                }
            ],
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
        },
    };
    if entity != src {
        panic!("StructExampleG: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleJ(entity: StructExampleJ) {
    let src = StructExampleJ {
        field_a: Some(StructExampleA {
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
        }),
        field_b: None,
        field_c: StructExampleEmpty {},
    };
    if entity != src {
        panic!("StructExampleJ: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_StructExampleEmpty(entity: StructExampleEmpty) {
    let src = StructExampleEmpty {
    };
    if entity != src {
        panic!("StructExampleEmpty: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}


#[allow(non_snake_case)]
fn check_GroupAStructExampleA(entity: GroupA::StructExampleA) {
    let src = GroupA::StructExampleA {
        field_u8: 1,
        field_u16: 2,
        opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
    };
    if entity != src {
        panic!("GroupA::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupAStructExampleB(entity: GroupA::StructExampleB) {
    let src = GroupA::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupA::StructExampleA {
            field_u8: 1,
            field_u16: 2,
            opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
        },
    };
    if entity != src {
        panic!("GroupA::StructExampleB: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupBStructExampleA(entity: GroupB::StructExampleA) {
    let src = GroupB::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    };
    if entity != src {
        panic!("GroupB::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupCStructExampleA(entity: GroupB::GroupC::StructExampleA) {
    let src = GroupB::GroupC::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    };
    if entity != src {
        panic!("GroupB::GroupC::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupCStructExampleB(entity: GroupB::GroupC::StructExampleB) {
    let src = GroupB::GroupC::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupB::GroupC::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        }
    };
    if entity != src {
        panic!("GroupB::GroupC::StructExampleB: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupDStructExampleP(entity: GroupD::StructExampleP) {
    let src = GroupD::StructExampleP {
        field_a: StructExampleA {
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
        },
        field_b: GroupB::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        },
        field_c: GroupB::GroupC::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        }
    };
    if entity != src {
        panic!("GroupD::StructExampleP: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupDEnumExamplePOption_a(entity: GroupD::EnumExampleP) {
    let src = GroupD::EnumExampleP::Option_a(StructExampleA {
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
    });
    if entity != src {
        panic!("GroupD::EnumExampleP: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupDEnumExamplePOption_b(entity: GroupD::EnumExampleP) {
    let src = GroupD::EnumExampleP::Option_b(GroupD::StructExampleP {
        field_a: StructExampleA {
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
        },
        field_b: GroupB::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        },
        field_c: GroupB::GroupC::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        }
    });
    if entity != src {
        panic!("GroupD::EnumExampleP: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupDEnumExamplePOption_c(entity: GroupD::EnumExampleP) {
    let src = GroupD::EnumExampleP::Option_c(GroupB::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    });
    if entity != src {
        panic!("GroupD::EnumExampleP: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_GroupDEnumExamplePOption_d(entity: GroupD::EnumExampleP) {
    let src = GroupD::EnumExampleP::Option_d(GroupB::GroupC::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    });
    if entity != src {
        panic!("GroupD::EnumExampleP: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleA_a(entity: EnumExampleA) {
    let src = EnumExampleA::Option_a(String::from("Option_a"));
    if entity != src {
        panic!("EnumExampleA.a: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleA_b(entity: EnumExampleA) {
    let src = EnumExampleA::Option_b(String::from("Option_b"));
    if entity != src {
        panic!("EnumExampleA.b: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_str(entity: EnumExampleB) {
    let src = EnumExampleB::Option_str(String::from("Option_str"));
    if entity != src {
        panic!("EnumExampleB.str: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_u8(entity: EnumExampleB) {
    let src = EnumExampleB::Option_u8(8);
    if entity != src {
        panic!("EnumExampleB.u8: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_u16(entity: EnumExampleB) {
    let src = EnumExampleB::Option_u16(16);
    if entity != src {
        panic!("EnumExampleB.u16: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_u32(entity: EnumExampleB) {
    let src = EnumExampleB::Option_u32(32);
    if entity != src {
        panic!("EnumExampleB.u32: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_u64(entity: EnumExampleB) {
    let src = EnumExampleB::Option_u64(64);
    if entity != src {
        panic!("EnumExampleB.u64: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_i8(entity: EnumExampleB) {
    let src = EnumExampleB::Option_i8(-8);
    if entity != src {
        panic!("EnumExampleB.i8: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_i16(entity: EnumExampleB) {
    let src = EnumExampleB::Option_i16(-16);
    if entity != src {
        panic!("EnumExampleB.i16: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_i32(entity: EnumExampleB) {
    let src = EnumExampleB::Option_i32(-32);
    if entity != src {
        panic!("EnumExampleB.i32: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_i64(entity: EnumExampleB) {
    let src = EnumExampleB::Option_i64(-64);
    if entity != src {
        panic!("EnumExampleB.i64: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_f32(entity: EnumExampleB) {
    let src = EnumExampleB::Option_f32(0.02);
    if entity != src {
        panic!("EnumExampleB.f32: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

#[allow(non_snake_case)]
fn check_EnumExampleB_f64(entity: EnumExampleB) {
    let src = EnumExampleB::Option_f64(0.02);
    if entity != src {
        panic!("EnumExampleB.f64: failed: \n\t{:?}\n\t{:?})", entity, src)
    }
}

pub fn read() -> Result<(), String> {
    let ts_bin = match get_ts_bin_dir() {
        Ok(root) => root,
        Err(e) => panic!(e),
    };
    match read_file(ts_bin.join("./EnumExampleA.a.prot.bin")) {
        Ok(buf) => {
            match EnumExampleA::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleA_a(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleA.a.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleA.b.prot.bin")) {
        Ok(buf) => {
            match EnumExampleA::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleA_b(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleA.b.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.str.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_str(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.str.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.u8.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_u8(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.u8.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.u16.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_u16(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.u16.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.u32.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_u32(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.u32.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.u64.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_u64(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.u64.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.i8.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_i8(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.i8.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.i16.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_i16(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.i16.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.i32.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_i32(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.i32.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.i64.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_i64(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.i64.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.f32.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_f32(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.f32.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./EnumExampleB.f64.prot.bin")) {
        Ok(buf) => {
            match EnumExampleB::decode(&buf) {
                Ok(entity) => {
                    check_EnumExampleB_f64(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./EnumExampleB.f64.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleA.prot.bin")) {
        Ok(buf) => {
            match StructExampleA::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleA(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleA.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleB.prot.bin")) {
        Ok(buf) => {
            match StructExampleB::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleB(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleB.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleC.prot.bin")) {
        Ok(buf) => {
            match StructExampleC::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleC(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleC.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleD.prot.bin")) {
        Ok(buf) => {
            match StructExampleD::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleD(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleD.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleE.prot.bin")) {
        Ok(buf) => {
            match StructExampleE::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleE(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleE.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleF.prot.bin")) {
        Ok(buf) => {
            match StructExampleF::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleF(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleF.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleG.prot.bin")) {
        Ok(buf) => {
            match StructExampleG::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleG(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleG.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleJ.prot.bin")) {
        Ok(buf) => {
            match StructExampleJ::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleJ(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleJ.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./StructExampleEmpty.prot.bin")) {
        Ok(buf) => {
            match StructExampleEmpty::decode(&buf) {
                Ok(entity) => {
                    check_StructExampleEmpty(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleEmpty.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    //
    match read_file(ts_bin.join("./GroupAStructExampleA.prot.bin")) {
        Ok(buf) => {
            match GroupA::StructExampleA::decode(&buf) {
                Ok(entity) => {
                    check_GroupAStructExampleA(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./GroupAStructExampleA.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupAStructExampleB.prot.bin")) {
        Ok(buf) => {
            match GroupA::StructExampleB::decode(&buf) {
                Ok(entity) => {
                    check_GroupAStructExampleB(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./GroupAStructExampleB.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupBStructExampleA.prot.bin")) {
        Ok(buf) => {
            match GroupB::StructExampleA::decode(&buf) {
                Ok(entity) => {
                    check_GroupBStructExampleA(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./GroupBStructExampleA.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupCStructExampleA.prot.bin")) {
        Ok(buf) => {
            match GroupB::GroupC::StructExampleA::decode(&buf) {
                Ok(entity) => {
                    check_GroupCStructExampleA(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./GroupCStructExampleA.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupCStructExampleB.prot.bin")) {
        Ok(buf) => {
            match GroupB::GroupC::StructExampleB::decode(&buf) {
                Ok(entity) => {
                    check_GroupCStructExampleB(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./GroupCStructExampleB.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupDStructExampleP.prot.bin")) {
        Ok(buf) => {
            match GroupD::StructExampleP::decode(&buf) {
                Ok(entity) => {
                    check_GroupDStructExampleP(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./check_GroupDStructExampleP.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupD.EnumExampleP.Option_a.prot.bin")) {
        Ok(buf) => {
            match GroupD::EnumExampleP::decode(&buf) {
                Ok(entity) => {
                    check_GroupDEnumExamplePOption_a(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./check_GroupDEnumExamplePOption_a.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupD.EnumExampleP.Option_b.prot.bin")) {
        Ok(buf) => {
            match GroupD::EnumExampleP::decode(&buf) {
                Ok(entity) => {
                    check_GroupDEnumExamplePOption_b(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./check_GroupDEnumExamplePOption_b.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupD.EnumExampleP.Option_c.prot.bin")) {
        Ok(buf) => {
            match GroupD::EnumExampleP::decode(&buf) {
                Ok(entity) => {
                    check_GroupDEnumExamplePOption_c(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./check_GroupDEnumExamplePOption_c.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupD.EnumExampleP.Option_d.prot.bin")) {
        Ok(buf) => {
            match GroupD::EnumExampleP::decode(&buf) {
                Ok(entity) => {
                    check_GroupDEnumExamplePOption_d(entity);
                    println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./check_GroupDEnumExamplePOption_d.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }

    match read_file(ts_bin.join("./buffer.prot.bin")) {
        Ok(buf) => {
            let mut buffer = Buffer::new();
            if let Err(e) = buffer.chunk(&buf) {
                panic!("Fail to write data into buffer due error: {:?}", e);
            }
            let mut count = 0;
            let mut done = 0;
            loop {
                let msg = buffer.next();
                if let Some(msg) = msg {
                    count += 1;
                    match msg.msg {
                        AvailableMessages::EnumExampleA(entity) => match entity {
                            EnumExampleA::Option_a(entity) => {
                                check_EnumExampleA_a(EnumExampleA::Option_a(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleA::Option_a is OK");
                                done += 1;
                            },
                            EnumExampleA::Option_b(entity) => {
                                check_EnumExampleA_b(EnumExampleA::Option_b(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleA::Option_b is OK");
                                done += 1;
                            },
                            _ => {},
                        },
                        AvailableMessages::EnumExampleB(entity) => match entity {
                            EnumExampleB::Option_str(entity) => {
                                check_EnumExampleB_str(EnumExampleB::Option_str(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_str is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_u8(entity) => {
                                check_EnumExampleB_u8(EnumExampleB::Option_u8(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_u8 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_u16(entity) => {
                                check_EnumExampleB_u16(EnumExampleB::Option_u16(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_u16 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_u32(entity) => {
                                check_EnumExampleB_u32(EnumExampleB::Option_u32(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_u32 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_u64(entity) => {
                                check_EnumExampleB_u64(EnumExampleB::Option_u64(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_u64 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_i8(entity) => {
                                check_EnumExampleB_i8(EnumExampleB::Option_i8(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_i8 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_i16(entity) => {
                                check_EnumExampleB_i16(EnumExampleB::Option_i16(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_i16 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_i32(entity) => {
                                check_EnumExampleB_i32(EnumExampleB::Option_i32(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_i32 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_i64(entity) => {
                                check_EnumExampleB_i64(EnumExampleB::Option_i64(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_i64 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_f32(entity) => {
                                check_EnumExampleB_f32(EnumExampleB::Option_f32(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_f32 is OK");
                                done += 1;
                            },
                            EnumExampleB::Option_f64(entity) => {
                                check_EnumExampleB_f64(EnumExampleB::Option_f64(entity));
                                println!("[OK]\tPackage AvailableMessages::EnumExampleB::Option_f64 is OK");
                                done += 1;
                            },
                            _ => {}
                        }
                        AvailableMessages::StructExampleA(entity) => {
                            check_StructExampleA(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleA is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleB(entity) => {
                            check_StructExampleB(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleB is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleC(entity) => {
                            check_StructExampleC(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleC is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleD(entity) => {
                            check_StructExampleD(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleD is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleE(entity) => {
                            check_StructExampleE(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleE is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleF(entity) => {
                            check_StructExampleF(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleF is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleG(entity) => {
                            check_StructExampleG(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleG is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleJ(entity) => {
                            check_StructExampleJ(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleJ is OK");
                            done += 1;
                        },
                        AvailableMessages::StructExampleEmpty(entity) => {
                            check_StructExampleEmpty(entity);
                            println!("[OK]\tPackage AvailableMessages::StructExampleEmpty is OK");
                            done += 1;
                        },
                        AvailableMessages::GroupA(entity) => match entity {
                            GroupA::AvailableMessages::StructExampleA(entity) => {
                                check_GroupAStructExampleA(entity);
                                println!("[OK]\tPackage GroupA::AvailableMessages::StructExampleA is OK");
                                done += 1;
                            },
                            GroupA::AvailableMessages::StructExampleB(entity) => {
                                check_GroupAStructExampleB(entity);
                                println!("[OK]\tPackage GroupA::AvailableMessages::StructExampleB is OK");
                                done += 1;
                            },
                            _ => {}
                        },
                        AvailableMessages::GroupB(entity) => match entity {
                            GroupB::AvailableMessages::StructExampleA(entity) => {
                                check_GroupBStructExampleA(entity);
                                println!("[OK]\tPackage GroupB::AvailableMessages::StructExampleA is OK");
                                done += 1;
                            },
                            GroupB::AvailableMessages::GroupC(entity) => match entity {
                                GroupB::GroupC::AvailableMessages::StructExampleA(entity) => {
                                    check_GroupCStructExampleA(entity);
                                    println!("[OK]\tPackage GroupB::GroupC::AvailableMessages::StructExampleA is OK");
                                    done += 1;
                                },
                                GroupB::GroupC::AvailableMessages::StructExampleB(entity) => {
                                    check_GroupCStructExampleB(entity);
                                    println!("[OK]\tPackage GroupB::GroupC::AvailableMessages::StructExampleB is OK");
                                    done += 1;
                                }
                            },
                        },
                        AvailableMessages::GroupD(entity) => match entity {
                            GroupD::AvailableMessages::StructExampleP(entity) => {
                                check_GroupDStructExampleP(entity);
                                println!("[OK]\tPackage GroupD::AvailableMessages::StructExampleP is OK");
                                done += 1;
                            },
                            GroupD::AvailableMessages::EnumExampleP(entity) => match entity {
                                GroupD::EnumExampleP::Option_a(entity) => {
                                    check_GroupDEnumExamplePOption_a(GroupD::EnumExampleP::Option_a(entity));
                                    println!("[OK]\tPackage GroupD::AvailableMessages::EnumExampleP.Option_a is OK");
                                    done += 1;
                                },
                                GroupD::EnumExampleP::Option_b(entity) => {
                                    check_GroupDEnumExamplePOption_b(GroupD::EnumExampleP::Option_b(entity));
                                    println!("[OK]\tPackage GroupD::AvailableMessages::EnumExampleP.Option_b is OK");
                                    done += 1;
                                },
                                GroupD::EnumExampleP::Option_c(entity) => {
                                    check_GroupDEnumExamplePOption_c(GroupD::EnumExampleP::Option_c(entity));
                                    println!("[OK]\tPackage GroupD::AvailableMessages::EnumExampleP.Option_c is OK");
                                    done += 1;
                                },
                                GroupD::EnumExampleP::Option_d(entity) => {
                                    check_GroupDEnumExamplePOption_d(GroupD::EnumExampleP::Option_d(entity));
                                    println!("[OK]\tPackage GroupD::AvailableMessages::EnumExampleP.Option_d is OK");
                                    done += 1;
                                },
                                _ => {},
                            }
                        }
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            println!("[OK]\t[RS]: File {:?} has beed read.", ts_bin.join("./buffer.prot.bin"));
            if buffer.pending() != 0 || buffer.len() != 0 || count != 32 || count != done {
                panic!("Fail to read buffer correctly: \n- buffer.pending(): {}\n- buffer.len(): {}\n- count: {}", buffer.pending(), buffer.len(), count);
            }
            println!("[OK]\tPackages: {}; done: {}", count, done);
        },
        Err(e) => panic!(e),
    }
    Ok(())
}