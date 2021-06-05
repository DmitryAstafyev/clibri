use std::{
    fs::{
        OpenOptions,
        remove_file,
    },
    path::{
        PathBuf,
    }
};
use std::io::prelude::*;

pub fn write(filename: PathBuf, content: String, overwrite: bool) -> Result<(), String> {
    if filename.exists() && overwrite {
        if let Err(e) = remove_file(filename.clone()) {
            return Err(e.to_string());
        }
    } else if filename.exists() && !overwrite {
        return Err(format!("File {} exists", filename.to_string_lossy()));
    }
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
    {
        Ok(mut file) => if let Err(e) = file.write_all(content.as_bytes()) {
            Err(e.to_string())
        } else {
            Ok(())
        }
        Err(e) => Err(e.to_string())
    }
} 