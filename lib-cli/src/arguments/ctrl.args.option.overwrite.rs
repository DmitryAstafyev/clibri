use std::path::{ Path };
use std::collections::{ HashMap };
use super::{ CtrlArg, EArgumentsNames, EArgumentsValues };

pub struct ArgsOptionOverwrite {
    _overwrite: bool,
}

impl CtrlArg for ArgsOptionOverwrite {

    fn new(_pwd: &Path, args: Vec<String>, mut _ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self {
        let mut overwrite: bool = false;
        if let Some(_opt_index) = args.iter().position(|arg| arg == "--overwrite" || arg == "--ow" || arg == "-o") {
            overwrite = true;
        }
        ArgsOptionOverwrite { _overwrite: overwrite }
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

}