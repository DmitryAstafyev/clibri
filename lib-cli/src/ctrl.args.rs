use std::env;
use std::path::{ PathBuf, Path };

#[path = "./arguments/ctrl.args.src.rs"]
pub mod arg_src;

pub trait CtrlArg {

    fn new(pwd: &Path, args: Vec<String>) -> Self;

}

pub struct CtrlArgs {
    _src: String,
    _dest: String,
}

impl CtrlArgs {

    pub fn new() -> Result<(), String> {
        let pwd: PathBuf = match env::current_dir() {
            Ok(pwd) => pwd,
            Err(e) => return Err(e.to_string()),
        };
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        Ok(())
    }

}