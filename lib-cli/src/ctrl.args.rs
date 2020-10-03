use std::env;
use std::path::{ PathBuf, Path };
use std::collections::{ HashMap };

#[path = "./arguments/ctrl.args.src.rs"]
pub mod arg_src;
#[path = "./arguments/ctrl.args.option.overwrite.rs"]
pub mod arg_option_overwrite;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EArgumentsNames {
    Files,
    OptionOverwrite,
}

pub enum EArgumentsValues {
    Files((PathBuf, PathBuf)),
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
            Err(e) => panic!(format!("Fail to detect pwd folder: {}", e)),
        };
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        let mut ctrls: HashMap<EArgumentsNames, Box<dyn CtrlArg>> = HashMap::new();
        ctrls.insert(
            EArgumentsNames::OptionOverwrite, 
            Box::new(arg_option_overwrite::ArgsOptionOverwrite::new(&pwd, args.clone(), &ctrls))
        );
        ctrls.insert(
            EArgumentsNames::Files, 
            Box::new(arg_src::ArgsSrcDest::new(&pwd, args.clone(), &ctrls))
        );
        CtrlArgs { _ctrls: ctrls }
    }



}
