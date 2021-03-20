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
        let signature = format!("[{}\t][{}][{}]", match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => format!("{}ms", d.as_millis()),
            Err(_) => "n/d".to_string(),
        }, match level {
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

}
