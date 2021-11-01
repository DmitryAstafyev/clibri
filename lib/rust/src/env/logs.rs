use super::vars;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::Once;

pub mod targets {
    pub static SERVER: &str = "fiber::wsserver";
    pub static CLIENT: &str = "fiber::wsclient";
    pub static PRODUCER: &str = "fiber::producer";
    pub static CONSUMER: &str = "fiber::consumer";
}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        let level = vars::log_level();
        if let Some(debug_mode) = vars::debug_mode() {
            if debug_mode {
                let stdout = ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d} [{l}] [{t}] {m}{n}")))
                    .build();
                match Config::builder()
                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                    .logger(Logger::builder().build(
                        targets::SERVER,
                        if let Some(level) = level.as_ref() {
                            *level
                        } else {
                            LevelFilter::Trace
                        },
                    ))
                    .logger(Logger::builder().build(
                        targets::PRODUCER,
                        if let Some(level) = level.as_ref() {
                            *level
                        } else {
                            LevelFilter::Trace
                        },
                    ))
                    .logger(Logger::builder().build(
                        targets::CONSUMER,
                        if let Some(level) = level.as_ref() {
                            *level
                        } else {
                            LevelFilter::Trace
                        },
                    ))
                    .logger(Logger::builder().build(
                        targets::CLIENT,
                        if let Some(level) = level.as_ref() {
                            *level
                        } else {
                            LevelFilter::Trace
                        },
                    ))
                    .build(
                        Root::builder()
                            .appender("stdout")
                            .build(vars::root_log_level()),
                    ) {
                    Ok(config) => {
                        if let Err(e) = log4rs::init_config(config) {
                            eprintln!("Fiber: fail to init log4rs. Error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Fiber: fail to build config for log4rs. Error: {}", e);
                    }
                };
            }
        } else if let Some(level) = level.as_ref() {
            let stdout = ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} [{l}] [{t}] {m}{n}")))
                .build();
            match Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .logger(Logger::builder().build(targets::SERVER, *level))
                .logger(Logger::builder().build(targets::PRODUCER, *level))
                .logger(Logger::builder().build(targets::CONSUMER, *level))
                .logger(Logger::builder().build(targets::CLIENT, *level))
                .build(
                    Root::builder()
                        .appender("stdout")
                        .build(vars::root_log_level()),
                ) {
                Ok(config) => {
                    if let Err(e) = log4rs::init_config(config) {
                        eprintln!("Fiber: fail to init log4rs. Error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Fiber: fail to build config for log4rs. Error: {}", e);
                }
            };
        }
    });
}
