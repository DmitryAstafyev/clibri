#[macro_use]
extern crate lazy_static;

#[path = "./transport/server/index.rs"]
pub mod server;

#[path = "./tools/logger.rs"]
pub mod logger;


pub mod tools {
    use crate::logger::{ GlobalLoggerSettings };
    use std::sync::{ Mutex };

    lazy_static! {
        pub static ref LOGGER_SETTINGS: Mutex<GlobalLoggerSettings> = Mutex::new(GlobalLoggerSettings::new());
    }

}