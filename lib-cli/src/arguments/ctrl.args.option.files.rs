use super::helpers;
use super::protocol::{ Parser as ProtocolParser };
use super::protocol::store::{ Store };
use super::workflow::{ Parser as WorkflowParser };
use super::render::rust::RustRender;
use super::render::typescript::TypescriptRender;
use super::render::Render;
use super::workflow_render::{
    render as render_workflow,
    ProtocolRefs,
};
use super::{CtrlArg, EArgumentsNames, EArgumentsValues};
use std::collections::HashMap;
use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod keys {
    pub const SOURCE: &str = "--source";
    pub const SRC: &str = "--src";
    pub const S: &str = "-s";
    pub const DESTINATION_RS: &str = "--destination-rs";
    pub const DEST_RS: &str = "--dest-rs";
    pub const RS: &str = "-rs";
    pub const DESTINATION_TS: &str = "--destination-ts";
    pub const DEST_TS: &str = "--dest-ts";
    pub const TS: &str = "-ts";
    pub const WORKFLOW: &str = "--workflow";
    pub const WF: &str = "-wf";
}

pub struct ArgsOptionFiles {
    src: Option<PathBuf>,
    workflow: Option<PathBuf>,
    dest_rs: Option<PathBuf>,
    dest_ts: Option<PathBuf>,
    errs: Vec<String>,
}

impl ArgsOptionFiles {

    fn new() -> Self {
        Self {
            src: None,
            workflow: None,
            dest_rs: None,
            dest_ts: None,
            errs: vec![],
        }
    }

    fn set_src(&mut self, path: PathBuf) {
        if !path.exists() {
            self.set_err(format!(
                "Source file doesn't exist. Path: {}",
                path.as_path().display().to_string()
            ));
        } else {
            self.src = Some(path);
        }
    }

    fn set_workflow(&mut self, path: PathBuf) {
        if !path.exists() {
            self.set_err(format!(
                "Source file doesn't exist. Path: {}",
                path.as_path().display().to_string()
            ));
        } else {
            self.workflow = Some(path);
        }
    }

    fn set_dest_rs(&mut self, path: PathBuf, overwrite: bool) {
        if !overwrite && path.exists() {
            self.set_err(format!("Rust destination file already exists. Use key --overwrite (--ow or -o) to overwrite destination file. File: {}",
                path.as_path().display().to_string(),
            ));
        } else {
            self.dest_rs = Some(path);
        }
    }


    fn set_dest_ts(&mut self, path: PathBuf, overwrite: bool) {
        if !overwrite && path.exists() {
            self.set_err(format!("Typescript destination file already exists. Use key --overwrite (--ow or -o) to overwrite destination file. File: {}",
                path.as_path().display().to_string(),
            ));
        } else {
            self.dest_ts = Some(path);
        }
    }

    fn set_err(&mut self, err: String) {
        self.errs.push(err);
    }

    fn has_errs(&self) -> bool {
        !self.errs.is_empty()
    }

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

    fn write(&self, dest: PathBuf, protocol_store: &mut Store, render: impl Render) -> Result<(), String> {
        let t_render = Instant::now();
        let content: String = render.render(protocol_store);
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
        let mut options = ArgsOptionFiles::new();
        if let Some(src_index) = args
            .iter()
            .position(|arg| arg == keys::SOURCE || arg == keys::SRC || arg == keys::S)
        {
            if let Some(arg_str_src) = args.get(src_index + 1) {
                options.set_src(Path::new(pwd).join(arg_str_src))
            }
        }
        if let Some(wf_index) = args
            .iter()
            .position(|arg| arg == keys::WORKFLOW || arg == keys::WF)
        {
            if let Some(arg_str_wf) = args.get(wf_index + 1) {
                options.set_workflow(Path::new(pwd).join(arg_str_wf));
            }
        }
        let mut overwrite: bool = false;
        if let Some(param_box) = ctrls.get(&EArgumentsNames::OptionOverwrite) {
            if let EArgumentsValues::OptionOverwrite(ow) = param_box.as_ref().value() {
                overwrite = ow;
            }
        }
        if let Some(dest_index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_RS || arg == keys::DEST_RS || arg == keys::RS)
        {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                options.set_dest_rs(Path::new(pwd).join(arg_str_dest), overwrite);
            }
        }
        if let Some(dest_index) = args
            .iter()
            .position(|arg| arg == keys::DESTINATION_TS || arg == keys::DEST_TS || arg == keys::TS)
        {
            if let Some(arg_str_dest) = args.get(dest_index + 1) {
                options.set_dest_ts(Path::new(pwd).join(arg_str_dest), overwrite);
            }
        }
        options
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::OptionFiles
    }

    fn value(&self) -> EArgumentsValues {
        let mut src = self.src.clone();
        let mut dest_rs = self.dest_rs.clone();
        let mut dest_ts = self.dest_ts.clone();
        if let (Some(src_path_buf), Some(dest_rs_path_buf), Some(dest_ts_path_buf)) =
            (src.take(), dest_rs.take(), dest_ts.take())
        {
            EArgumentsValues::Files((src_path_buf, dest_rs_path_buf, dest_ts_path_buf))
        } else {
            EArgumentsValues::Empty(())
        }
    }

    fn get_err(&self) -> Option<String> {
        if self.has_errs() {
            Some(self.errs.join(""))
        } else {
            None
        }
    }

    fn is_action_available(&self) -> bool {
        !self.has_errs()
    }

    fn action(
        &self,
        ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>,
    ) -> Result<(), String> {
        if let Some(src) = self.src.clone() {
            let t_parsing = Instant::now();
            let overwrite: bool = self.get_overwrite_flag(ctrls);
            let embedded: bool = self.get_embedded_flag(ctrls);
            let mut protocol: ProtocolParser = ProtocolParser::new(src.clone());
            match protocol.parse() {
                Ok(mut protocol_store) => {
                    println!(
                        "[OK][{}ms] parsed {:?}",
                        t_parsing.elapsed().as_millis(),
                        src
                    );
                    if let Some(workflow_path) = self.workflow.as_ref() {
                        // TODO: remove workflow dest folder
                        let mut workflow: WorkflowParser = WorkflowParser::new(workflow_path.to_owned());
                        match workflow.parse(&mut protocol_store) {
                            Ok(workflow_store) => {
                                let protocol_refs: ProtocolRefs = ProtocolRefs {
                                    typescript: None,
                                    rust: None,
                                };
                                
                                if let Err(err) = render_workflow(
                                    protocol_refs, 
                                    None,
                                    None,
                                    workflow_store,
                                    &protocol_store
                                ) {
                                    return Err(err);
                                }
                            },
                            Err(err) => {
                                return Err(err);
                            }
                        };
                    }
                    if let Some(dest) = self.dest_rs.clone() {
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
                        if let Err(e) = self.write(dest, &mut protocol_store, RustRender::new(embedded, 0)) {
                            return Err(e);
                        }
                    }
                    if let Some(dest) = self.dest_ts.clone() {
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
                        if let Err(e) = self.write(dest, &mut protocol_store, TypescriptRender::new(embedded, 0)) {
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
        format!("{}\n{}\n{}\n{}",
            format!("{}{}",
                helpers::output::keys(&format!("{} ({}, {})", keys::SOURCE, keys::SRC, keys::S)),
                helpers::output::desk("[required] path to source file. Protocol file with description messages."),
            ),
            format!("{}{}",
                helpers::output::keys(&format!("{} ({})", keys::WORKFLOW, keys::WORKFLOW)),
                helpers::output::desk("path to workflow file. Description of communication between procuder & consumer."),
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
            .position(|arg| arg == keys::WORKFLOW || arg == keys::WF)
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
