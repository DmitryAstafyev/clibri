use std::time::{SystemTime, UNIX_EPOCH};

pub enum LogLevel {
    Error,  // 1
    Warn,   // 2
    Debug,  // 3
    Info,   // 4
    Verb    // 5
}

pub trait Logger {

    fn warn(&self, str: &str) -> () {
        self.log(str, LogLevel::Warn);
    }

    fn err(&self, str: &str) -> () {
        self.log(str, LogLevel::Error);
    }

    fn debug(&self, str: &str) -> () {
        self.log(str, LogLevel::Debug);
    }

    fn info(&self, str: &str) -> () {
        self.log(str, LogLevel::Debug);
    }

    fn verb(&self, str: &str) -> () {
        self.log(str, LogLevel::Verb);
    }

    fn log(&self, str: &str, level: LogLevel) -> () {
        let signature = format!("[{}\t][{}\t][{}\t]", self.get_ms(), match level {
            LogLevel::Error => "ERR",
            LogLevel::Warn  => "WARN",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info  => "INFO",
            LogLevel::Verb  => "VERB",
        }, self.get_alias());
        if match level {
            LogLevel::Error => true,
            LogLevel::Warn  => if self.get_level() <= 2 { true } else { false },
            LogLevel::Debug => if self.get_level() <= 3 { true } else { false },
            LogLevel::Info  => if self.get_level() <= 4 { true } else { false },
            LogLevel::Verb  => if self.get_level() <= 5 { true } else { false }
        } {
            println!("{}: {}", signature, str);
        }
    }

    fn set_level(&mut self, level: LogLevel) -> ();

    fn get_level(&self) -> u8;

    fn set_alias(&mut self, alias: String) -> ();

    fn get_alias(&self) -> &str;

    fn get_ms(&self) -> u128;

}

pub struct DefaultLogger {
    alias: String,
    level: u8,
    created: u128,
}

impl DefaultLogger {

    pub fn new(alias: String, level: u8) -> Self {
        let created: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v.as_millis(),
            Err(_) => 0,
        };
        DefaultLogger { alias: alias, level: level, created: created }
    }
}

impl Logger for DefaultLogger {
    fn set_level(&mut self, level: LogLevel) -> () {
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

    fn set_alias(&mut self, alias: String) -> () {
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


