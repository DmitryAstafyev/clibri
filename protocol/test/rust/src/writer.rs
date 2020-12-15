#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{OpenOptions, remove_file};
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

pub fn write() -> Result<(), String> {
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
        if let Err(e) = writeFile(Path::new("/Users/dmitry.astafyev/projects/fiber/protocol/test/rust/test.bin").to_path_buf(), &buf) {
            panic!(e);
        }
    }
    Ok(())
}