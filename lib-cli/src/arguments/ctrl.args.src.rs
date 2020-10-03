use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };

pub struct ArgsSrcDest {
    _src: Option<PathBuf>,
    _dest_rs: Option<PathBuf>,
    _dest_ts: Option<PathBuf>,
    _err: Option<String>,
}

impl CtrlArg for ArgsSrcDest {

    fn new(pwd: &Path, args: Vec<String>, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        let mut src: Option<PathBuf> = None;
        let mut dest_rs: Option<PathBuf> = None;
        let mut dest_ts: Option<PathBuf> = None;
        let mut err: Option<String> = None;
        if let Some(src_index) = args.iter().position(|arg| arg == "--source" || arg == "--src" || arg == "-s") {
            if let Some(arg_str_src) = args.get(src_index + 1) {
                src = Some(Path::new(pwd).join(arg_str_src));
            }
        }
        if let Some(dest_index) = args.iter().position(|arg| arg == "--destination-rs" || arg == "--dest-rs" || arg == "-rs") {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                dest_rs = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if let Some(dest_index) = args.iter().position(|arg| arg == "--destination-ts" || arg == "--dest-ts" || arg == "-ts") {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                dest_ts = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if src.is_none() {
            err = Some("Source filename has to be defined. Use key --source (--src or -s) to set source file".to_string());
        }
        if src.is_some() && dest_rs.is_none() {
            if let Some(src_path_buf) = src.clone().take() {
                dest_rs = Some(Path::new(&src_path_buf).with_extension("rs"));
            }
        }
        if src.is_some() && dest_ts.is_none() {
            if let Some(src_path_buf) = src.clone().take() {
                dest_ts = Some(Path::new(&src_path_buf).with_extension("ts"));
            }
        }
        if let (Some(src_path_buf), Some(dest_rs_path_buf), Some(dest_ts_path_buf)) = (src.clone().take(), dest_rs.clone().take(), dest_ts.clone().take()) {
            if !src_path_buf.exists() {
                err = Some(format!("Source file doesn't exist. Path: {}", src_path_buf.as_path().display().to_string()));
            }
            let mut overwrite: bool = false;
            if let Some(param_box) = ctrls.get(&EArgumentsNames::OptionOverwrite) {
                if let EArgumentsValues::OptionOverwrite(ow) = param_box.as_ref().value() {
                    overwrite = ow;
                }
            }
            if !overwrite && (dest_rs_path_buf.exists() || dest_ts_path_buf.exists()) {
                err = Some(format!("File(s) already exist. Use key --overwrite (--ow or -o) to overwrite destination file. Files: \n{}\n{}",
                    dest_rs_path_buf.as_path().display().to_string(),
                    dest_ts_path_buf.as_path().display().to_string(),
                ));
            }            
        }
        ArgsSrcDest { _src: src, _dest_rs: dest_rs, _dest_ts: dest_ts, _err: err }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::Files
    }

    fn value(&self) -> EArgumentsValues {
        let mut src = self._src.clone();
        let mut dest_rs = self._dest_rs.clone();
        let mut dest_ts = self._dest_ts.clone();
        if let (Some(src_path_buf), Some(dest_rs_path_buf), Some(dest_ts_path_buf)) = (src.take(), dest_rs.take(), dest_ts.take()) {
            EArgumentsValues::Files((src_path_buf, dest_rs_path_buf, dest_ts_path_buf))
        } else {
            EArgumentsValues::Empty(())
        }
    }

    fn get_err(&self) -> Option<String> {
        self._err.clone()
    }

}