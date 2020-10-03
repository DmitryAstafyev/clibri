#[path = "./ctrl.args.rs"]
pub mod ctrlargs;

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
