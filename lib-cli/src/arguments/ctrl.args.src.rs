use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };

pub struct ArgsSrcDest {
    _src: Option<PathBuf>,
    _dest: Option<PathBuf>,
    _err: Option<String>,
}

impl CtrlArg for ArgsSrcDest {

    fn new(pwd: &Path, args: Vec<String>, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        let mut src: Option<PathBuf> = None;
        let mut dest: Option<PathBuf> = None;
        let mut err: Option<String> = None;
        if let Some(src_index) = args.iter().position(|arg| arg == "--source" || arg == "--src" || arg == "-s") {
            if let Some(arg_str_src) = args.get(src_index + 1) {
                src = Some(Path::new(pwd).join(arg_str_src));
            }
        }
        if let Some(dest_index) = args.iter().position(|arg| arg == "--destination" || arg == "--dest" || arg == "-d") {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                dest = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if src.is_none() {
            err = Some("Source filename has to be defined. Use key --source (--src or -s) to set source file".to_string());
        } else if dest.is_none() {
            if let Some(src_path_buf) = src.clone().take() {
                dest = Some(src_path_buf);
            }
        }
        if let (Some(src_path_buf), Some(dest_path_buf)) = (src.clone().take(), dest.clone().take()) {
            if !src_path_buf.exists() {
                err = Some(format!("Source file doesn't exist. Path: {}", src_path_buf.as_path().display().to_string()));
            }
            let mut overwrite: bool = false;
            if let Some(param_box) = ctrls.get(&EArgumentsNames::OptionOverwrite) {
                if let EArgumentsValues::OptionOverwrite(ow) = param_box.as_ref().value() {
                    overwrite = ow;
                }
            }
            let dest_path_buf_rs = dest_path_buf.join(".rs");
            let dest_path_buf_ts = dest_path_buf.join(".ts");
            if !overwrite && (dest_path_buf_rs.exists() || dest_path_buf_ts.exists()) {
                err = Some(format!("File(s) already exist. Use key --overwrite (--ow or -o) to overwrite destination file. Files: \n{}\n{}",
                    dest_path_buf_rs.as_path().display().to_string(),
                    dest_path_buf_ts.as_path().display().to_string(),
                ));
            }            
        }
        ArgsSrcDest { _src: src, _dest: dest, _err: err }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::Files
    }

    fn value(&self) -> EArgumentsValues {
        let mut src = self._src.clone();
        let mut dest = self._dest.clone();
        if let (Some(src_path_buf), Some(dest_path_buf)) = (src.take(), dest.take()) {
            EArgumentsValues::Files((src_path_buf, dest_path_buf))
        } else {
            EArgumentsValues::Empty(())
        }
    }

    fn get_err(&self) -> Option<String> {
        self._err.clone()
    }

}