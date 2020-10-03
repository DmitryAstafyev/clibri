use std::env;
use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };

#[path = "./arguments/ctrl.args.src.rs"]
pub mod arg_option_src;
#[path = "./arguments/ctrl.args.option.overwrite.rs"]
pub mod arg_option_overwrite;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EArgumentsNames {
    Files,
    OptionOverwrite,
}

pub enum EArgumentsValues {
    Files((PathBuf, PathBuf, PathBuf)),
    OptionOverwrite(bool),
    Empty(()),
}
pub trait CtrlArg {

    fn new(pwd: &Path, args: Vec<String>, ctrls: &HashMap<EArgumentsNames, Box<dyn CtrlArg + 'static>>) -> Self where Self: Sized;
    fn name(&self) -> EArgumentsNames;
    fn value(&self) -> EArgumentsValues;
    fn get_err(&self) -> Option<String>;

}

pub struct CtrlArgs {
    _ctrls: HashMap<EArgumentsNames, Box<dyn CtrlArg>>,
}

#[allow(clippy::new_without_default)]
impl CtrlArgs {

    pub fn new() -> Self {
        let pwd: PathBuf = match env::current_dir() {
            Ok(pwd) => pwd,
            Err(e) => {
                println!("Fail to detect pwd folder: {}", e);
                std::process::exit(0);
            },
        };
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        let mut _args = arg_option_overwrite::clean(args.clone());
        _args = arg_option_src::clean(_args);
        if !_args.is_empty() {
            println!("Unknown keys: \n\t- {}", _args.join("\n\t- "));
            std::process::exit(0);
        }
        let mut ctrls: HashMap<EArgumentsNames, Box<dyn CtrlArg>> = HashMap::new();
        ctrls.insert(
            EArgumentsNames::OptionOverwrite, 
            Box::new(arg_option_overwrite::ArgsOptionOverwrite::new(&pwd, args.clone(), &ctrls))
        );
        ctrls.insert(
            EArgumentsNames::Files, 
            Box::new(arg_option_src::ArgsSrcDest::new(&pwd, args.clone(), &ctrls))
        );
        CtrlArgs { _ctrls: ctrls }
    }

    pub fn errors(&self) -> Result<(), ()> {
        let mut errors: bool = false;
        for ctrl in self._ctrls.values() {
            if let Some(err) = ctrl.as_ref().get_err() {
                errors = true;
                println!("{}", err);
            }
        }
        if errors {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn print(&self) -> Result<(), ()> {
        let mut errors: bool = false;
        for ctrl in self._ctrls.values() {
            if let Some(err) = ctrl.as_ref().get_err() {
                errors = true;
                println!("Error: {}", err);
            }
            match ctrl.as_ref().value() {
                EArgumentsValues::OptionOverwrite(ow) => println!("{:?} = {}", EArgumentsNames::OptionOverwrite, ow),
                EArgumentsValues::Files((src, dest_rs, dest_ts)) => {
                    println!("{:?}: src = {}", EArgumentsNames::Files, src.as_path().display().to_string());
                    println!("{:?}: dest_rs = {}", EArgumentsNames::Files, dest_rs.as_path().display().to_string());
                    println!("{:?}: dest_ts = {}", EArgumentsNames::Files, dest_ts.as_path().display().to_string());
                },
                _ => println!("Empty value has been found"),
            }
        }
        if errors {
            Err(())
        } else {
            Ok(())
        }
    }

}
