use std::path::{ Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };
use super:: { helpers };

mod keys {
    pub const OVERWRITE: &str = "--overwrite";
    pub const OW: &str = "--ow";
    pub const O: &str = "-o";
}

pub struct ArgsOptionOverwrite {
    _overwrite: bool,
}

impl CtrlArg for ArgsOptionOverwrite {

    fn new(_pwd: &Path, args: Vec<String>, mut _ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        ArgsOptionOverwrite {
            _overwrite: args.iter().any(|arg| arg == keys::OVERWRITE || arg == keys::OW || arg == keys::O)
        }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::OptionOverwrite
    }

    fn value(&self) -> EArgumentsValues {
        EArgumentsValues::OptionOverwrite(self._overwrite)
    }

    fn get_err(&self) -> Option<String> {
        None
    }

    fn is_action_available(&self) -> bool {
        false
    }

    fn action(&self, mut _ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Result<(), String> {
        Ok(())
    }

    fn get_help(&self) -> String {
        format!("{}{}",
            helpers::output::keys(&format!("{} ({}, {})", keys::OVERWRITE, keys::OW, keys::O)),
            helpers::output::desk("if key exist, destination files would be overwritten."),
        )
    }

}

pub fn get_cleaner() -> impl Fn(Vec<String>) -> Vec<String> {
    move |mut args: Vec<String>| {
        if let Some(index) = args.iter().position(|arg| arg == keys::OVERWRITE || arg == keys::OW || arg == keys::O) {
            args.remove(index);
        }
        args
    }
}
