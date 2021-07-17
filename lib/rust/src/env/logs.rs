use super::{vars};
use log::{LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root, Logger};
use std::sync::Once;

pub mod targets {
    pub static SERVER: &str = "fiber::wsserver";
    pub static PRODUCER: &str = "fiber::producer";
}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        if let Some(debug_mode) = vars::debug_mode() {
            if debug_mode {
                let stdout = ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d} [{l}] [{t}] {m}{n}"))).build();
                match Config::builder()
                    .appender(Appender::builder()
                        .build("stdout", Box::new(stdout))
                    )
                    .logger(Logger::builder()
                        .build(targets::SERVER, LevelFilter::Trace)
                    )
                    .logger(Logger::builder()
                        .build(targets::PRODUCER, LevelFilter::Trace)
                    )
                    .build(Root::builder()
                        .appender("stdout")
                        .build(LevelFilter::Warn)
                    )
                {
                    Ok(config) => {
                        if let Err(e) = log4rs::init_config(config) {
                            eprintln!("Fiber: fail to init log4rs. Error: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Fiber: fail to build config for log4rs. Error: {}", e);
                    }
                };
            }
        }
    });
}
