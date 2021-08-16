use log::LevelFilter;

pub fn debug_mode() -> Option<bool> {
    match std::env::var("FIBER_DEBUG_MODE") {
        Ok(value) => {
            if value.to_ascii_lowercase() == "true"
                || value.to_ascii_lowercase() == "on"
                || value.to_ascii_lowercase() == "1"
            {
                Some(true)
            } else {
                Some(false)
            }
        }
        Err(_) => None,
    }
}

mod levels {
    pub const ERROR: &str = "error";
    pub const WARN: &str = "warn";
    pub const INFO: &str = "info";
    pub const DEBUG: &str = "debug";
    pub const TRACE: &str = "trace";
}
pub fn log_level() -> Option<LevelFilter> {
    match std::env::var("FIBER_LOG_LEVEL") {
        Ok(value) => {
            if value.to_ascii_lowercase() == levels::ERROR {
                Some(LevelFilter::Error)
            } else if value.to_ascii_lowercase() == levels::WARN {
                Some(LevelFilter::Warn)
            } else if value.to_ascii_lowercase() == levels::DEBUG {
                Some(LevelFilter::Debug)
            } else if value.to_ascii_lowercase() == levels::INFO {
                Some(LevelFilter::Info)
            } else if value.to_ascii_lowercase() == levels::TRACE {
                Some(LevelFilter::Trace)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn root_log_level() -> LevelFilter {
    match std::env::var("FIBER_ROOT_LOG_LEVEL") {
        Ok(value) => {
            if value.to_ascii_lowercase() == levels::ERROR {
                LevelFilter::Error
            } else if value.to_ascii_lowercase() == levels::WARN {
                LevelFilter::Warn
            } else if value.to_ascii_lowercase() == levels::DEBUG {
                LevelFilter::Debug
            } else if value.to_ascii_lowercase() == levels::INFO {
                LevelFilter::Info
            } else if value.to_ascii_lowercase() == levels::TRACE {
                LevelFilter::Trace
            } else {
                LevelFilter::Warn
            }
        }
        Err(_) => LevelFilter::Warn,
    }
}
