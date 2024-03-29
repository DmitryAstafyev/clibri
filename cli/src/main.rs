pub mod ctrlargs;
pub mod helpers;
#[path = "./protocol/parser.rs"]
pub mod protocol;
#[path = "./protocol/render/render.rs"]
pub mod render;
#[path = "./workflow/parser.rs"]
pub mod workflow;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        std::process::exit(1);
    }}
}

fn main() {
    let ctrl: ctrlargs::CtrlArgs = ctrlargs::CtrlArgs::new();
    if ctrl.has_errors() {
        std::process::exit(1);
    }
    if let Err(errors) = ctrl.actions() {
        println!("{}", errors.join("\n"))
    }
}
