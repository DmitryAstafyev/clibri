use std::path::{ Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };

mod keys {
    pub const HELP: &str = "--help";
    pub const H: &str = "--h";
}

pub struct ArgsOptionHelp {
    _requested: bool,
}

impl CtrlArg for ArgsOptionHelp {

    fn new(_pwd: &Path, args: Vec<String>, mut _ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        let mut requested: bool = false;
        if args.iter().any(|arg| arg == keys::HELP || arg == keys::H) {
            requested = true;
        }
        ArgsOptionHelp { _requested: requested }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::OptionOverwrite
    }

    fn value(&self) -> EArgumentsValues {
        EArgumentsValues::Empty(())
    }

    fn get_err(&self) -> Option<String> {
        None
    }

    fn action(&self, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Result<(), String> {
        if self._requested {
            for ctrl in ctrls.values() {
                println!("{}", ctrl.as_ref().get_help());
            }
        }
        Ok(())
    }

    fn get_help(&self) -> String {
        format!("\t{} ({})\t - shows this help.", keys::HELP, keys::H)
    }


}

pub fn get_cleaner() -> impl Fn(Vec<String>) -> Vec<String> {
    move |mut args: Vec<String>| {
        if let Some(index) = args.iter().position(|arg| arg == keys::HELP || arg == keys::H) {
            args.remove(index);
        }
        args
    }
}
