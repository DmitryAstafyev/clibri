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
    created: u128,
    last: u128,
}

#[allow(clippy::new_without_default)]
impl GlobalLoggerSettings {

    pub fn new() -> Self {
        let current = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis(),
            Err(_) => 0,
        };
        GlobalLoggerSettings { level: 0, created: current, last: current }
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

    pub fn get_pass_time(&mut self) -> u128 {
        let prev = self.last;
        self.last = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis(),
            Err(_) => 0,
        };
        self.last - prev
    }

    pub fn get_gen_time(&mut self) -> u128 {
        let cur = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis(),
            Err(_) => 0,
        };
        cur - self.created
    }

}

pub trait Logger {

    fn warn(&self, str: &str) -> String {
        self.log(str, LogLevel::Warn)
    }

    fn err(&self, str: &str) -> String {
        self.log(str, LogLevel::Error)
    }

    fn debug(&self, str: &str) -> String {
        self.log(str, LogLevel::Debug)
    }

    fn info(&self, str: &str) -> String {
        self.log(str, LogLevel::Debug)
    }

    fn verb(&self, str: &str) -> String {
        self.log(str, LogLevel::Verb)
    }

    fn log(&self, str: &str, level: LogLevel) -> String {
        let (g_level, gen_ms, pass_ms): (u8, u128, u128) = match tools::LOGGER_SETTINGS.lock() {
            Ok(mut settings) => (settings.get_level(), settings.get_gen_time(), settings.get_pass_time()),
            Err(_) => (0u8, 0u128, 0u128),
        };
        let signature = format!("[{} +{}ms\t][{}\t][{}\t]", gen_ms, pass_ms, match level {
            LogLevel::Error => "ERR",
            LogLevel::Warn  => "WARN",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info  => "INFO",
            LogLevel::Verb  => "VERB",
        }, self.get_alias());
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
        str.to_string()
    }

    fn set_level(&mut self, level: LogLevel);

    fn get_level(&self) -> u8;

    fn set_alias(&mut self, alias: String);

    fn get_alias(&self) -> &str;

}

pub struct DefaultLogger {
    alias: String,
    level: u8,
}

impl DefaultLogger {

    pub fn new(alias: String, level: Option<u8>) -> Self {
        let mut g_level = 0;
        let level = if let Some(l) = level {
            match tools::LOGGER_SETTINGS.lock() {
                Ok(settings) => {
                    g_level = settings.get_level();
                    if g_level != 0 {
                        g_level
                    } else if l < settings.get_min_level() || l > settings.get_max_level() {
                        settings.get_default_level()
                    } else {
                        l
                    }
                },
                Err(_) => {
                    l
                }
            }
        } else {
            1
        };
        let logger = DefaultLogger { alias: alias.clone(), level };
        logger.verb(&format!("Created logger {}. {} log level: {}", if g_level == 0 { "Default" } else { "Using global "}, alias, logger.get_level()));
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

}


