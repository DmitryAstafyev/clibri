#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{OpenOptions, remove_file, create_dir};
use std::path::{PathBuf};
use std::io::prelude::*;

fn write_file(dest: PathBuf, buf: &Vec<u8>) -> Result<(), String> {
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
    let mut buffer: Vec<u8> = vec![];
    let mut usecase = EnumExampleA::Option_a("Option_a".to_owned());
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleA.a.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleA.a.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleA::Option_b("Option_b".to_owned());
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleA.b.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleA.b.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_str("Option_str".to_owned());
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.str.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.str.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_u8(8);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.u8.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.u8.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_u16(16);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.u16.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.u16.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_u32(32);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.u32.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.u32.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_u64(64);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.u64.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.u64.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_i8(-8);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.i8.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.i8.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_i16(-16);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.i16.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.i16.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_i32(-32);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.i32.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.i32.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_i64(-64);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.i64.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.i64.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_f32(0.02);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.f32.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.f32.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = EnumExampleB::Option_f64(0.02);
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./EnumExampleB.f64.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./EnumExampleB.f64.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleA {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleA.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleB {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleB.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleC {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleC.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleC.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleD {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleD.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleD.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleE {
        field_a: EnumExampleA::Option_a(String::from("Option_a")),
        field_b: EnumExampleB::Option_u8(1),
        field_c: EnumExampleC::Option_u8(vec![1]),
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleE.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleE.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleF {
        field_a: None,
        field_b: None,
        field_c: Some(EnumExampleC::Option_u8(vec![1])),
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleF.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleF.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleG {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleG.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleG.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = StructExampleJ {
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
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./StructExampleJ.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./StructExampleJ.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = GroupA::StructExampleA {
        field_u8: 1,
        field_u16: 2,
        opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./GroupAStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./GroupAStructExampleA.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = GroupA::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupA::StructExampleA {
            field_u8: 1,
            field_u16: 2,
            opt: GroupA::EnumExampleA::Option_a(String::from("Option_a")),
        },
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./GroupAStructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./GroupAStructExampleB.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = GroupB::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./GroupBStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./GroupBStructExampleA.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = GroupB::GroupC::StructExampleA {
        field_u8: 1,
        field_u16: 2,
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./GroupCStructExampleA.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./GroupCStructExampleA.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    let mut usecase = GroupB::GroupC::StructExampleB {
        field_u8: 1,
        field_u16: 2,
        strct: GroupB::GroupC::StructExampleA {
            field_u8: 1,
            field_u16: 2,
        }
    };
    if let Ok(buf) = usecase.encode() {
        if let Err(e) = write_file(root.join("./GroupCStructExampleB.prot.bin"), &buf) {
            panic!(e);
        }
        println!("[RS]: File {:?} has beed written.", root.join("./GroupCStructExampleB.prot.bin"));
    }
    buffer.append(&mut usecase.pack().unwrap());
    if let Err(e) = write_file(root.join("./buffer.prot.bin"), &buffer) {
        panic!(e);
    }
    println!("[RS]: File {:?} has beed written.", root.join("./buffer.prot.bin"));
    Ok(())
}