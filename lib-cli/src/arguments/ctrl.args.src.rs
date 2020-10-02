use std::env;
use std::path::{ PathBuf, Path };
use super::{ CtrlArg };

pub struct ArgsSrcDest {
    _src: Option<PathBuf>,
    _dest: Option<PathBuf>,
    _err: Option<String>,
}

impl CtrlArg for ArgsSrcDest {

    fn new(pwd: &Path, args: Vec<String>) -> Self {
        let mut src: Option<PathBuf> = None;
        let mut dest: Option<PathBuf> = None;
        let mut err: Option<String> = None;
        if args.len() == 2 {
            if let (Some(arg_str_src), Some(arg_str_dest)) = (args.get(0), args.get(1)) {
                src = Some(Path::new(pwd).join(arg_str_src));
                dest = Some(Path::new(pwd).join(arg_str_dest));
            }
        } else if let (
            Some(src_index),
            Some(dest_index)
        ) = (
            args.iter().position(|arg| arg == "--source" || arg == "--src" || arg == "-s"),
            args.iter().position(|arg| arg == "--destination" || arg == "--dest" || arg == "-d")
        ) {
            if let (Some(arg_str_src), Some(arg_str_dest)) = (args.get(src_index), args.get(dest_index)) {
                src = Some(Path::new(pwd).join(arg_str_src));
                dest = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if src.is_none() {
            err = Some("Source filename has to be defined. Use key --source (--src or -s) to set source file".to_string());
        } else if dest.is_none() {
            // Rename source
        } else if let (Some(src_path_buf), Some(dest_path_buf)) = (src.take(), dest.take()) {
            if !src_path_buf.exists() {
                err = Some(format!("Source file doesn't exist. Path: {}", src_path_buf.as_path().display().to_string()));
            }
        }
        ArgsSrcDest { _src: src, _dest: dest, _err: err }
    }

}