use super::helpers;
use super::parser::{ Parser };
use super::parser::store::{ Store };
use super::render::rust::RustRender;
use super::render::typescript::TypescriptRender;
use super::render::Render;
use super::{CtrlArg, EArgumentsNames, EArgumentsValues};
use std::collections::HashMap;
use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod keys {
    pub const SOURCE: &str = "--source";
    pub const SRC: &str = "--src";
    pub const S: &str = "--s";
    pub const DESTINATION_RS: &str = "--destination-rs";
    pub const DEST_RS: &str = "--dest-rs";
    pub const RS: &str = "-rs";
    pub const DESTINATION_TS: &str = "--destination-ts";
    pub const DEST_TS: &str = "--dest-ts";
    pub const TS: &str = "-ts";
}

pub struct ArgsOptionFiles {
    _src: Option<PathBuf>,
    _dest_rs: Option<PathBuf>,
    _dest_ts: Option<PathBuf>,
    _err: Option<String>,
}

impl ArgsOptionFiles {

    fn get_overwrite_flag(&self, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> bool {
        if let Some(arg) = ctrls.get(&EArgumentsNames::OptionOverwrite) {
            if let EArgumentsValues::OptionOverwrite(overwrite) = arg.value() {
                overwrite
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_embedded_flag(&self, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> bool {
        if let Some(arg) = ctrls.get(&EArgumentsNames::OptionEmbedded) {
            if let EArgumentsValues::OptionEmbedded(embedded) = arg.value() {
                embedded
            } else {
                false
            }
        } else {
            false
        }
    }

    fn write(&self, dest: PathBuf, store: Store, render: impl Render) -> Result<(), String> {
        let t_render = Instant::now();
        let content: String = render.render(store);
        match OpenOptions::new()
            .write(true)
            .create(true)
            .open(dest.clone())
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    return Err(e.to_string());
                }
                println!(
                    "[OK][{}ms] saved {:?}",
                    t_render.elapsed().as_millis(),
                    dest
                );
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    } 

}

impl CtrlArg for ArgsOptionFiles {

    fn new(
        pwd: &Path,
        args: Vec<String>,
        ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>,
    ) -> Self {
        let mut src: Option<PathBuf> = None;
        let mut dest_rs: Option<PathBuf> = None;
        let mut dest_ts: Option<PathBuf> = None;
        let mut err: Option<String> = None;
        if let Some(src_index) = args
            .iter()
            .position(|arg| arg == keys::SOURCE || arg == keys::SRC || arg == keys::S)
        {
            if let Some(arg_str_src) = args.get(src_index + 1) {
                src = Some(Path::new(pwd).join(arg_str_src));
            }
        }
        if let Some(dest_index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_RS || arg == keys::DEST_RS || arg == keys::RS)
        {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                dest_rs = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if let Some(dest_index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_TS || arg == keys::DEST_TS || arg == keys::TS)
        {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                dest_ts = Some(Path::new(pwd).join(arg_str_dest));
            }
        }
        if src.is_none() {
            err = Some("Source filename has to be defined. Use key --source (--src or -s) to set source file".to_string());
        }
        if let (Some(src_path_buf), Some(dest_rs_path_buf), Some(dest_ts_path_buf)) = (
            src.clone().take(),
            dest_rs.clone().take(),
            dest_ts.clone().take(),
        ) {
            if !src_path_buf.exists() {
                err = Some(format!(
                    "Source file doesn't exist. Path: {}",
                    src_path_buf.as_path().display().to_string()
                ));
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
        ArgsOptionFiles {
            _src: src,
            _dest_rs: dest_rs,
            _dest_ts: dest_ts,
            _err: err,
        }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::OptionFiles
    }

    fn value(&self) -> EArgumentsValues {
        let mut src = self._src.clone();
        let mut dest_rs = self._dest_rs.clone();
        let mut dest_ts = self._dest_ts.clone();
        if let (Some(src_path_buf), Some(dest_rs_path_buf), Some(dest_ts_path_buf)) =
            (src.take(), dest_rs.take(), dest_ts.take())
        {
            EArgumentsValues::Files((src_path_buf, dest_rs_path_buf, dest_ts_path_buf))
        } else {
            EArgumentsValues::Empty(())
        }
    }

    fn get_err(&self) -> Option<String> {
        self._err.clone()
    }

    fn is_action_available(&self) -> bool {
        self._err.is_none()
    }

    fn action(
        &self,
        ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>,
    ) -> Result<(), String> {
        if let Some(src) = self._src.clone() {
            let t_parsing = Instant::now();
            let overwrite: bool = self.get_overwrite_flag(ctrls);
            let embedded: bool = self.get_embedded_flag(ctrls);
            let mut parser: Parser = Parser::new(src.clone());
            match parser.parse() {
                Ok(store) => {
                    println!(
                        "[OK][{}ms] parsed {:?}",
                        t_parsing.elapsed().as_millis(),
                        src
                    );
                    if let Some(dest) = self._dest_rs.clone() {
                        if dest.exists() && !overwrite {
                            return Err(format!("File {:?} exists. Use key \"overwrite\" to overwrite file. -h to get more info", dest));
                        } else if dest.exists() {
                            println!(
                                "[INFO] {:?} will be overwritten",
                                dest
                            );
                            if let Err(err) = remove_file(dest.clone()) {
                                return Err(format!("Fail to remove file {:?} due error: {}", dest, err));
                            } else {
                                println!(
                                    "[INFO] {:?} clean",
                                    dest
                                );
                            }
                        }
                        if let Err(e) = self.write(dest, store.clone(), RustRender::new(embedded, 0)) {
                            return Err(e);
                        }
                    }
                    if let Some(dest) = self._dest_ts.clone() {
                        if dest.exists() && !overwrite {
                            return Err(format!("File {:?} exists. Use key \"overwrite\" to overwrite file. -h to get more info", dest));
                        } else if dest.exists() {
                            println!(
                                "[INFO] {:?} will be overwritten",
                                dest
                            );
                            if let Err(err) = remove_file(dest.clone()) {
                                return Err(format!("Fail to remove file {:?} due error: {}", dest, err));
                            } else {
                                println!(
                                    "[INFO] {:?} clean",
                                    dest
                                );
                            }
                        }
                        if let Err(e) = self.write(dest, store, TypescriptRender::new(embedded, 0)) {
                            return Err(e);
                        }
                    }
                    Ok(())
                }
                Err(errs) => Err(errs.join("\n")),
            }
        } else {
            Err(String::from("protocol file isn't defined"))
        }
    }

    fn get_help(&self) -> String {
        format!("{}\n{}\n{}",
            format!("{}{}",
                helpers::output::keys(&format!("{} ({}, {})", keys::SOURCE, keys::SRC, keys::S)),
                helpers::output::desk("[required] path to source file. Protocol file with description messages."),
            ),
            format!("{}{}",
                helpers::output::keys(&format!("{} ({}, {})", keys::DESTINATION_RS, keys::DEST_RS, keys::RS)),
                helpers::output::desk("path to destination rs (rust) file. If value isn't defined, would be used path and name of source file"),
            ),
            format!("{}{}",
                helpers::output::keys(&format!("{} ({}, {})", keys::DESTINATION_TS, keys::DEST_TS, keys::TS)),
                helpers::output::desk("path to destination ts (typescript) file. If value isn't defined, would be used path and name of source file"),
            )
        )
    }

}

pub fn get_cleaner() -> impl Fn(Vec<String>) -> Vec<String> {
    move |mut args: Vec<String>| {
        if let Some(index) = args
            .iter()
            .position(|arg| arg == keys::SOURCE || arg == keys::SRC || arg == keys::S)
        {
            match args.get(index + 1) {
                Some(_) => {
                    args.remove(index + 1);
                    args.remove(index);
                }
                None => {
                    args.remove(index);
                }
            }
        }
        if let Some(index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_RS || arg == keys::DEST_RS || arg == keys::RS)
        {
            match args.get(index + 1) {
                Some(_) => {
                    args.remove(index + 1);
                    args.remove(index);
                }
                None => {
                    args.remove(index);
                }
            }
        }
        if let Some(index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_TS || arg == keys::DEST_TS || arg == keys::TS)
        {
            match args.get(index + 1) {
                Some(_) => {
                    args.remove(index + 1);
                    args.remove(index);
                }
                None => {
                    args.remove(index);
                }
            }
        }
        args
    }
}
