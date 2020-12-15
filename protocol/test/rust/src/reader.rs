#[path = "./protocol.rs"]
pub mod protocol;

use protocol::*;
use std::fs::{OpenOptions, remove_file};
use std::path::{Path, PathBuf};
use std::io::prelude::*;

pub fn read() -> Result<(), String> {


}