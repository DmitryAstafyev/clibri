use super::{helpers, stop};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[path = "./arguments/args_option_embedded.rs"]
pub mod arg_option_embedded;
#[path = "./arguments/args_option_files.rs"]
pub mod arg_option_files;
#[path = "./arguments/args_option_help.rs"]
pub mod arg_option_help;
#[path = "./arguments/args_option_overwrite.rs"]
pub mod arg_option_overwrite;
#[path = "./protocol/parser.rs"]
pub mod protocol;
#[path = "./protocol/render/render.rs"]
pub mod render;
#[path = "./workflow/parser.rs"]
pub mod workflow;
#[path = "./workflow/render/render.rs"]
pub mod workflow_render;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EArgumentsNames {
    OptionFiles,
    OptionOverwrite,
    OptionEmbedded,
    OptionHelp,
}

pub enum EArgumentsValues {
    Files((PathBuf, PathBuf, PathBuf)),
    OptionOverwrite(bool),
    OptionEmbedded(bool),
    Empty(()),
}
pub trait CtrlArg {
    fn new(
        pwd: &Path,
        args: Vec<String>,
        ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>,
    ) -> Self
    where
        Self: Sized;
    fn name(&self) -> EArgumentsNames;
    fn value(&self) -> EArgumentsValues;
    fn get_err(&self) -> Option<String>;
    fn action(
        &self,
        ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>,
    ) -> Result<(), String>;
    fn is_action_available(&self) -> bool;
    fn get_help(&self) -> String;
}

pub struct CtrlArgs {
    _ctrls: HashMap<EArgumentsNames, Box<dyn CtrlArg>>,
}

pub type TCleaner = Box<dyn Fn(Vec<String>) -> Vec<String>>;

#[allow(clippy::new_without_default)]
impl CtrlArgs {
    pub fn new() -> Self {
        let pwd: PathBuf = match env::current_dir() {
            Ok(pwd) => pwd,
            Err(e) => {
                println!("Fail to detect pwd folder: {}", e);
                std::process::exit(0);
            }
        };
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        let mut ctrls: HashMap<EArgumentsNames, Box<dyn CtrlArg>> = HashMap::new();
        ctrls.insert(
            EArgumentsNames::OptionHelp,
            Box::new(arg_option_help::ArgsOptionHelp::new(
                &pwd,
                args.clone(),
                &ctrls,
            )),
        );
        ctrls.insert(
            EArgumentsNames::OptionOverwrite,
            Box::new(arg_option_overwrite::ArgsOptionOverwrite::new(
                &pwd,
                args.clone(),
                &ctrls,
            )),
        );
        ctrls.insert(
            EArgumentsNames::OptionEmbedded,
            Box::new(arg_option_embedded::ArgsOptionEmbedded::new(
                &pwd,
                args.clone(),
                &ctrls,
            )),
        );
        ctrls.insert(
            EArgumentsNames::OptionFiles,
            Box::new(arg_option_files::ArgsOptionFiles::new(&pwd, args, &ctrls)),
        );
        CtrlArgs { _ctrls: ctrls }
    }

    pub fn has_errors(&self) -> bool {
        let mut errors: bool = false;
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        let unknown = Self::get_unknown_args(args);
        if !unknown.is_empty() {
            println!("Unknown keys/arguments: \n\t- {}", unknown.join("\n\t- "));
            errors = true;
        } else {
            if let Some(ctrl) = self._ctrls.get(&EArgumentsNames::OptionHelp) {
                if ctrl.as_ref().is_action_available() {
                    return false;
                }
            }
            for ctrl in self._ctrls.values() {
                if let Some(err) = ctrl.as_ref().get_err() {
                    errors = true;
                    println!("{}", err);
                }
            }
        }
        errors
    }

    pub fn actions(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = vec![];
        for ctrl in self._ctrls.values() {
            if let Err(e) = ctrl.as_ref().action(&self._ctrls) {
                errors.push(e)
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn print(&self) {
        for ctrl in self._ctrls.values() {
            if let Some(err) = ctrl.as_ref().get_err() {
                println!("Error: {}", err);
            }
            match ctrl.as_ref().value() {
                EArgumentsValues::OptionOverwrite(ow) => {
                    println!("{:?} = {}", EArgumentsNames::OptionOverwrite, ow)
                }
                EArgumentsValues::OptionEmbedded(em) => {
                    println!("{:?} = {}", EArgumentsNames::OptionEmbedded, em)
                }
                EArgumentsValues::Files((src, dest_rs, dest_ts)) => {
                    println!(
                        "{:?}: src = {}",
                        EArgumentsNames::OptionFiles,
                        src.as_path().display().to_string()
                    );
                    println!(
                        "{:?}: dest_rs = {}",
                        EArgumentsNames::OptionFiles,
                        dest_rs.as_path().display().to_string()
                    );
                    println!(
                        "{:?}: dest_ts = {}",
                        EArgumentsNames::OptionFiles,
                        dest_ts.as_path().display().to_string()
                    );
                }
                _ => println!("Empty value has been found"),
            }
        }
    }

    pub fn get_unknown_args(mut args: Vec<String>) -> Vec<String> {
        let cleaners: Vec<TCleaner> = vec![
            Box::new(arg_option_help::get_cleaner()),
            Box::new(arg_option_embedded::get_cleaner()),
            Box::new(arg_option_overwrite::get_cleaner()),
            Box::new(arg_option_files::get_cleaner()),
        ];
        for cleaner in cleaners {
            args = cleaner.as_ref()(args);
        }
        args
    }
}
