use std::path::{ Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };
use super::{ helpers };

mod keys {
    pub const EMBEDDED: &str = "--embedded";
    pub const EM: &str = "--em";
    pub const E: &str = "-e";
}

pub struct ArgsOptionEmbedded {
    _embedded: bool,
}

impl CtrlArg for ArgsOptionEmbedded {

    fn new(_pwd: &Path, args: Vec<String>, mut _ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        ArgsOptionEmbedded {
            _embedded: args.iter().any(|arg| arg == keys::EMBEDDED || arg == keys::EM || arg == keys::E)
        }
    }

    fn name(&self) -> EArgumentsNames {
        EArgumentsNames::OptionEmbedded
    }

    fn value(&self) -> EArgumentsValues {
        EArgumentsValues::OptionEmbedded(self._embedded)
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
            helpers::output::keys(&format!("{} ({}, {})", keys::EMBEDDED, keys::EM, keys::E)),
            helpers::output::desk("if key exist, some addition code will be included. It will make possible to use protocol independently on clibri. Default: true"),
        )
    }

}

pub fn get_cleaner() -> impl Fn(Vec<String>) -> Vec<String> {
    move |mut args: Vec<String>| {
        if let Some(index) = args.iter().position(|arg| arg == keys::EMBEDDED || arg == keys::EM || arg == keys::E) {
            args.remove(index);
        }
        args
    }
}
