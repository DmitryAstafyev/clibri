#[macro_use]
extern crate lazy_static;
use std::env;

pub mod reader;
pub mod writer;

use reader::read;
use writer::write;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        std::process::exit(1);
    }}
}

#[allow(non_upper_case_globals)]
pub mod state {
    use std::sync::Mutex;

    pub struct TestState {
        pub middleware: bool,
    }

    lazy_static! {
        pub static ref state: Mutex<TestState> = Mutex::new(TestState { middleware: false });
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args[0] == "write" {
        match write() {
            Ok(_) => {}
            Err(e) => stop!("{}", e),
        };
        match state::state.lock() {
            Ok(mut state) => {
                state.middleware = true;
            }
            Err(e) => {
                stop!("Fail get state due error {}", e);
            }
        };
        match write() {
            Ok(_) => {}
            Err(e) => stop!("{}", e),
        };
    } else if args[0] == "read" {
        match read() {
            Ok(_) => {}
            Err(e) => stop!("{}", e),
        };
        match state::state.lock() {
            Ok(mut state) => {
                state.middleware = true;
            }
            Err(e) => {
                stop!("Fail get state due error {}", e);
            }
        };
        match read() {
            Ok(_) => {}
            Err(e) => stop!("{}", e),
        };
    }
}
