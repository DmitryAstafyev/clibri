#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{OpenOptions, remove_file, create_dir};
use std::path::{Path, PathBuf};
use std::io::prelude::*;

fn writeFile(dest: PathBuf, buf: &Vec<u8>) -> Result<(), String> {
    if dest.exists() {
        if let Err(err) = remove_file(dest.clone()) {
            return Err(format!("Fail to remove file {:?} due error: {}", dest, err));
        }
    }
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest.clone())
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(buf) {
                return Err(e.to_string());
            }
            Ok(())
        }
        Err(e) => Err(e.to_string())
    }
} 

pub fn get_root_dir() -> Result<PathBuf, String> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dest) = exe.as_path().parent() {
            let dest = dest.join("../../binary");
            if !dest.exists() {
                if let Err(e) = create_dir(dest.clone()) {
                    return Err(format!("{}", e));
                }
            }
            Ok(dest)
        } else {
            Err("Fail to find exe-path".to_string())
        }
    } else {
        Err("Fail to find exe-path".to_string())
    }
}

pub fn write() -> Result<(), String> {
    let root = match get_root_dir() {
        Ok(root) => root,
        Err(e) => panic!(e),
    };
    if let Ok(buf) = (StructExampleA {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleB {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleC {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleC.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleD {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleD.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleE {
        field_a: EnumExampleA::Option_a(String::from("Option_a")),
        field_b: EnumExampleB::Option_u8(1),
        field_c: EnumExampleC::Option_u8(vec![1]),
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleE.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleF {
        field_a: None,
        field_b: None,
        field_c: Some(EnumExampleC::Option_u8(vec![1])),
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleF.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleG {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleG.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (StructExampleJ {
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
    }.encode()) {
        if let Err(e) = writeFile(root.join("./StructExampleJ.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (GroupA::StructExampleA {
        field_u8: 1,
        field_u16: 2,
        opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
    }.encode()) {
        if let Err(e) = writeFile(root.join("./GroupAStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (GroupA::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupA::StructExampleA {
            field_u8: 1,
            field_u16: 2,
            opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
        },
    }.encode()) {
        if let Err(e) = writeFile(root.join("./GroupAStructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (GroupB::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    }.encode()) {
        if let Err(e) = writeFile(root.join("./GroupBStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (GroupB::GroupC::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    }.encode()) {
        if let Err(e) = writeFile(root.join("./GroupCStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
    }
    if let Ok(buf) = (GroupB::GroupC::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupB::GroupC::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        }
    }.encode()) {
        if let Err(e) = writeFile(root.join("./GroupCStructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
    }
    Ok(())
}