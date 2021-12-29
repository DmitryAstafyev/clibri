use std::{env, process::Command, thread};

struct Configuration {
    pub consumer: String,
    pub producer: String,
    pub node: bool,
}

impl Configuration {
    pub fn new() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        Ok(Self {
            node: args.iter().any(|a| a.to_lowercase().contains("--node")),
            consumer: if let Some(arg) = args
                .iter()
                .find(|a| a.to_lowercase().contains("--consumer"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    parts[1].parse::<String>().map_err(|e| e.to_string())?
                } else {
                    return Err(String::from("--consumer isn't defined"));
                }
            } else {
                return Err(String::from("--consumer isn't defined"));
            },
            producer: if let Some(arg) = args
                .iter()
                .find(|a| a.to_lowercase().contains("--producer"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    parts[1].parse::<String>().map_err(|e| e.to_string())?
                } else {
                    return Err(String::from("--producer isn't defined"));
                }
            } else {
                return Err(String::from("--producer isn't defined"));
            },
        })
    }
}

fn main() {
    let configuration = match Configuration::new() {
        Ok(configuration) => configuration,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    let consumer_path = configuration.consumer;
    let producer_path = configuration.producer;
    let producer_node = configuration.node;
    let consumer_node = configuration.node;
    let producer_handler = thread::spawn(move || {
        Command::new(if producer_node { "node" } else { "sh" })
            .arg(producer_path)
            .status()
            .expect("failed to execute producer process")
    });
    let consumer_handler = thread::spawn(move || {
        Command::new(if consumer_node { "node" } else { "sh" })
            .arg(consumer_path)
            .status()
            .expect("failed to execute consumer process")
    });
    producer_handler
        .join()
        .expect("The producer thread has panicked");
    consumer_handler
        .join()
        .expect("The consumer thread has panicked");
}
