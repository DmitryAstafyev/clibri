use std::time::{SystemTime, UNIX_EPOCH};
use super::{ tools };

pub enum LogLevel {
    Error,  // 1
    Warn,   // 2
    Debug,  // 3
    Info,   // 4
    Verb    // 5
}

pub struct GlobalLoggerSettings {
    level: u8,
}

impl GlobalLoggerSettings {

    pub fn new() -> Self {
        GlobalLoggerSettings { level: 0 }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = match level {
            LogLevel::Error => 1,
            LogLevel::Warn => 2,
            LogLevel::Debug => 3,
            LogLevel::Info => 4,
            LogLevel::Verb => 5,
        };
    }

    pub fn get_max_level(&self) -> u8 {
        5
    }

    pub fn get_min_level(&self) -> u8 {
        1
    }

    pub fn get_default_level(&self) -> u8 {
        1
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

}

pub trait Logger {

    fn warn(&self, str: &str) {
        self.log(str, LogLevel::Warn);
    }

    fn err(&self, str: &str) -> () {
        self.log(str, LogLevel::Error);
    }

    fn debug(&self, str: &str) {
        self.log(str, LogLevel::Debug);
    }

    fn info(&self, str: &str){
        self.log(str, LogLevel::Debug);
    }

    fn verb(&self, str: &str) {
        self.log(str, LogLevel::Verb);
    }

    fn log(&self, str: &str, level: LogLevel) {
        let signature = format!("[{}\t][{}\t][{}\t]", self.get_ms(), match level {
            LogLevel::Error => "ERR",
            LogLevel::Warn  => "WARN",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info  => "INFO",
            LogLevel::Verb  => "VERB",
        }, self.get_alias());
        let g_level: u8 = match tools::LOGGER_SETTINGS.lock() {
            Ok(settings) => settings.get_level(),
            Err(_) => 0,
        };
        let c_level: u8 = if g_level == 0 { self.get_level() } else { g_level };
        if match level {
            LogLevel::Error => true,
            LogLevel::Warn  => { c_level >= 2 },
            LogLevel::Debug => { c_level >= 3 },
            LogLevel::Info  => { c_level >= 4 },
            LogLevel::Verb  => { c_level == 5 }
        } {
            println!("{}: {}", signature, str);
        }
    }

    fn set_level(&mut self, level: LogLevel);

    fn get_level(&self) -> u8;

    fn set_alias(&mut self, alias: String);

    fn get_alias(&self) -> &str;

    fn get_ms(&self) -> u128;

}

pub struct DefaultLogger {
    alias: String,
    level: u8,
    created: u128,
}

impl DefaultLogger {

    pub fn new(alias: String, level: Option<u8>) -> Self {
        let created: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis(),
            Err(_) => 0,
        };
        let level = if let Some(l) = level {
            match tools::LOGGER_SETTINGS.lock() {
                Ok(settings) => if l < settings.get_min_level() || l > settings.get_max_level() {
                    settings.get_default_level()
                } else {
                    l
                },
                Err(_) => {
                    l
                }
            }
        } else {
            1
        };
        let logger = DefaultLogger { alias: alias.clone(), level, created };
        logger.verb(&format!("Created logger {}. Default log level: {}", alias, logger.get_level()));
        logger
    }
}

impl Logger for DefaultLogger {
    fn set_level(&mut self, level: LogLevel) {
        self.level = match level {
            LogLevel::Error => 1,
            LogLevel::Warn => 2,
            LogLevel::Debug => 3,
            LogLevel::Info => 4,
            LogLevel::Verb => 5,
        };
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn set_alias(&mut self, alias: String) {
        self.alias = alias;
    }

    fn get_alias(&self) -> &str {
        &self.alias
    }

    fn get_ms(&self) -> u128 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis() - self.created,
            Err(_) => 0,
        }
    }
}


