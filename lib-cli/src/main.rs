#[path = "./ctrl.args.rs"]
pub mod ctrlargs;

#[path = "./helpers/helpers.rs"]
pub mod helpers;

#[path = "./protocol/parser.rs"]
pub mod protocol;

#[path = "./workflow/parser.rs"]
pub mod workflow;

#[path = "./render/render.rs"]
pub mod render;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        //eprint!($crate::fmt::format($crate::__export::format_args!($($arg)*)));
        std::process::exit(1);
    }}
}

fn main() {
    let ctrl: ctrlargs::CtrlArgs = ctrlargs::CtrlArgs::new();
    match ctrl.errors() {
        Ok(_) => {},
        Err(_) => std::process::exit(1),
    }
    if let Err(errors) = ctrl.actions() {
        println!("{}", errors.join("\n"))
    }
}
