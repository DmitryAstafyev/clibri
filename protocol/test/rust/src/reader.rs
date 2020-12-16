#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{File, create_dir};
use std::path::{Path, PathBuf};
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

pub fn read() -> Result<(), String> {
    let ts_bin = match get_ts_bin_dir() {
        Ok(root) => root,
        Err(e) => panic!(e),
    };
    match read_file(ts_bin.join("./StructExampleA.prot.bin")) {
        Ok(buf) => {
            match StructExampleA::decode(&buf) {
                Ok(entity) => {
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
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleA.prot.bin"));
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
                    };
                    if entity != src {
                        panic!("StructExampleB: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleB.prot.bin"));
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
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleC.prot.bin"));
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
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleD.prot.bin"));
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
                    let src = StructExampleE {
                        field_a: EnumExampleA::Option_a(String::from("Option_a")),
                        field_b: EnumExampleB::Option_u8(1),
                        field_c: EnumExampleC::Option_u8(vec![1]),
                    };
                    if entity != src {
                        panic!("StructExampleE: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleE.prot.bin"));
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
                    let src = StructExampleF {
                        field_a: None,
                        field_b: None,
                        field_c: Some(EnumExampleC::Option_u8(vec![1])),
                    };
                    if entity != src {
                        panic!("StructExampleF: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleF.prot.bin"));
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
                        },
                    };
                    if entity != src {
                        panic!("StructExampleG: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleG.prot.bin"));
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
                    };
                    if entity != src {
                        panic!("StructExampleJ: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./StructExampleJ.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    match read_file(ts_bin.join("./GroupAStructExampleA.prot.bin")) {
        Ok(buf) => {
            match GroupA::StructExampleA::decode(&buf) {
                Ok(entity) => {
                    let src = GroupA::StructExampleA {
                        field_u8: 1,
                        field_u16: 2,
                        opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
                    };
                    if entity != src {
                        panic!("GroupA::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./GroupAStructExampleA.prot.bin"));
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
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./GroupAStructExampleB.prot.bin"));
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
                    let src = GroupB::StructExampleA {
                        field_u8: 1,
                        field_u16: 2,
                    };
                    if entity != src {
                        panic!("GroupB::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./GroupBStructExampleA.prot.bin"));
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
                    let src = GroupB::GroupC::StructExampleA {
                        field_u8: 1,
                        field_u16: 2,
                    };
                    if entity != src {
                        panic!("GroupB::GroupC::StructExampleA: failed: \n\t{:?}\n\t{:?})", entity, src)
                    }
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./GroupCStructExampleA.prot.bin"));
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
                    println!("[RS]: File {:?} has beed read.", ts_bin.join("./GroupCStructExampleB.prot.bin"));
                },
                Err(e) => panic!(e)
            }
            
        },
        Err(e) => panic!(e),
    }
    Ok(())
}