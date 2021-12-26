#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

mod connection;
mod consumer;
mod stat;
mod test;

use connection::run;
use console::{style, Term};
use futures::future::join_all;
use stat::{Alias, Stat, StatEvent};
use std::env;
use tokio::{
    join,
    runtime::Runtime,
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_util::sync::CancellationToken;

#[macro_export]
macro_rules! stop {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        std::process::exit(1);
    }}
}

#[derive(Debug)]
enum Port {
    Single,
    Multiple,
}

const DEFAULT_CONNECTIONS: usize = 1000;
const DEFAULT_TIMEOUT: u64 = 20000;
const DEFAULT_THREADS: usize = 1;

struct Configuration {
    pub port: Port,
    pub connections: usize,
    pub threads: usize,
    pub timeout: u64,
}

impl Configuration {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        Self {
            port: if args.iter().any(|a| a.to_lowercase() == "--multiple") {
                Port::Multiple
            } else {
                Port::Single
            },
            threads: if let Some(arg) = args.iter().find(|a| a.to_lowercase().contains("--threads"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    parts[1].parse::<usize>().unwrap_or(1)
                } else {
                    DEFAULT_THREADS
                }
            } else {
                DEFAULT_THREADS
            },
            timeout: if let Some(arg) = args.iter().find(|a| a.to_lowercase().contains("--timeout"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    parts[1].parse::<u64>().unwrap_or(1)
                } else {
                    DEFAULT_TIMEOUT
                }
            } else {
                DEFAULT_TIMEOUT
            },
            connections: if let Some(arg) = args
                .iter()
                .find(|a| a.to_lowercase().contains("--connections"))
            {
                let parts: Vec<&str> = arg.split('=').collect();
                if parts.len() == 2 {
                    match parts[1].parse::<usize>() {
                        Ok(connections) => connections,
                        Err(_) => DEFAULT_CONNECTIONS,
                    }
                } else {
                    DEFAULT_CONNECTIONS
                }
            } else {
                DEFAULT_CONNECTIONS
            },
        }
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "- port: {:?}\n- connections: {}\n- threads: {}\n",
                self.port, self.connections, self.threads
            )
        )
    }
}

async fn single_thread(configuration: Configuration) -> Result<(), String> {
    let (tx_stat, mut rx_stat): (UnboundedSender<StatEvent>, UnboundedReceiver<StatEvent>) =
        unbounded_channel();
    let mut jobs = vec![];
    println!("{} creating consumers jobs", style("[test]").bold().dim(),);
    for _ in 0..configuration.connections {
        jobs.push(run(
            "127.0.0.1:8080",
            configuration.timeout,
            tx_stat.clone(),
            false,
            matches!(configuration.port, Port::Multiple),
        ));
    }
    drop(tx_stat);
    task::spawn(async move {
        let mut stat = Stat::new(configuration.connections, false);
        while let Some(event) = rx_stat.recv().await {
            stat.apply(event);
        }
        println!(
            "\n{} all consumers did all jobs",
            style("[test]").bold().dim(),
        );
        println!("{}", stat);
        let errors = stat.get_errors();
        if !errors.is_empty() {
            for error in errors {
                eprintln!("{}", error);
            }
            stop!("");
        }
    });
    println!("{} starting consumers jobs", style("[test]").bold().dim(),);
    let results = join_all(jobs).await;
    for result in results {
        if let Err(err) = result {
            stop!("Failed with: {}", err);
        }
    }
    Ok(())
}

async fn multiple_threads(configuration: Configuration) -> Result<(), String> {
    let (tx_thread, mut rx_thread): (UnboundedSender<Stat>, UnboundedReceiver<Stat>) =
        unbounded_channel();
    let (tx_progress, mut rx_progress): (
        UnboundedSender<(usize, usize, usize)>,
        UnboundedReceiver<(usize, usize, usize)>,
    ) = unbounded_channel();
    let mut rx_next_holder: Option<oneshot::Receiver<()>> = None;
    let tx_progress_thread = tx_progress.clone();
    let timeout = configuration.timeout;
    for index in 0..configuration.threads {
        let connections = configuration.connections;
        let distributor = matches!(configuration.port, Port::Multiple);
        let tx_thread = tx_thread.clone();
        let tx_progress = tx_progress.clone();
        let (tx_next, rx_next): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
        let next = rx_next_holder.take();
        let mut tx_next = if index < configuration.connections - 1 {
            Some(tx_next)
        } else {
            None
        };
        rx_next_holder = if index < configuration.connections - 1 {
            Some(rx_next)
        } else {
            None
        };
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(rx_next) = next {
                    if rx_next.await.is_err() {
                        stop!("Fail to get start signal from previos thread");
                    }
                }
                let (tx_stat, mut rx_stat): (
                    UnboundedSender<StatEvent>,
                    UnboundedReceiver<StatEvent>,
                ) = unbounded_channel();
                let mut jobs = vec![];
                for _ in 0..connections {
                    jobs.push(run(
                        "127.0.0.1:8080",
                        timeout,
                        tx_stat.clone(),
                        false,
                        distributor,
                    ));
                }
                drop(tx_stat);
                let (tx_stat_done, rx_stat_done): (oneshot::Sender<Stat>, oneshot::Receiver<Stat>) =
                    oneshot::channel();
                task::spawn(async move {
                    let mut stat = Stat::new(connections, true);
                    while let Some(event) = rx_stat.recv().await {
                        let mut connected: usize = 0;
                        let mut done: usize = 0;
                        match &event {
                            StatEvent::Inc(alias) => {
                                if matches!(alias, Alias::Connected) {
                                    connected = 1;
                                }
                            }
                            StatEvent::ConsumerDone => {
                                done = 1;
                            }
                        }
                        stat.apply(event);
                        if tx_progress.send((connected, done, 0)).is_err() {
                            stop!("Fail to send test-progress information");
                        }
                        if stat.connected == connections {
                            if let Some(tx_next) = tx_next.take() {
                                if tx_next.send(()).is_err() {
                                    stop!("Fail to send start signal into next thread");
                                }
                            }
                        }
                    }
                    println!("{}", stat);
                    let errors = stat.get_errors();
                    if !errors.is_empty() {
                        for error in errors {
                            eprintln!("{}", error);
                        }
                        stop!("");
                    }
                    if tx_stat_done.send(stat).is_err() {
                        stop!("Fail to send stat");
                    }
                });
                let results = join_all(jobs).await;
                for result in results {
                    if let Err(err) = result {
                        stop!("Failed with: {}", err);
                    }
                }
                let stat = match rx_stat_done.await {
                    Ok(stat) => stat,
                    Err(_) => {
                        stop!("Fail to get stat");
                    }
                };
                if tx_thread.send(stat).is_err() {
                    stop!("Fail to report thread finish");
                }
            });
        });
    }
    select! {
        _ = async {
            let mut threads_done: usize = 0;
            while let Some(stat) = rx_thread.recv().await {
                threads_done += 1;
                if tx_progress.send((0, 0, 1)).is_err() {
                    stop!("Fail to send test-progress information");
                }
                if threads_done == configuration.threads {
                    break;
                }
            }
            println!("{} all threads are done", style("[test]").bold().dim(),);
        } => (),
        _ = async {
            let total_connected: usize = configuration.connections * configuration.threads;
            let total_done_operations: usize = total_connected * 52;
            let mut sum_connected: usize = 0;
            let mut sum_done_operations: usize = 0;
            let mut sum_done: usize = 0;
            let mut sum_threads: usize = 0;
            let term = Term::stdout();
            while let Some((connected, done, thread)) = rx_progress.recv().await {
                if thread == 0 {
                    sum_connected += connected;
                    sum_done_operations += 1;
                    sum_done += done;
                } else {
                    sum_threads += thread;
                }
                println!(
                    "{} {}/{} ({}%) threads done",
                    style("[test]").bold().dim(),
                    sum_threads,
                    configuration.threads,
                    (sum_threads * 100) / configuration.threads
                );
                println!(
                    "{} {}/{} ({}%) clients connected",
                    style("[test]").bold().dim(),
                    sum_connected,
                    total_connected,
                    (sum_connected * 100) / total_connected
                );
                println!(
                    "{} {} / {} ({}%) operations done",
                    style("[test]").bold().dim(),
                    sum_done_operations,
                    total_done_operations,
                    (sum_done_operations * 100) / total_done_operations
                );
                println!(
                    "{} {} / {} ({}%) clients done job",
                    style("[test]").bold().dim(),
                    sum_done,
                    total_connected,
                    (sum_done * 100) / total_connected
                );
                if let Err(err) = term.move_cursor_up(4) {
                    eprintln!("Fail to manipulate console: {}", err);
                }
            }
        } => ()
    };
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), String> {
    let configuration = Configuration::new();
    let distributor = matches!(configuration.port, Port::Multiple);
    let timeout = configuration.timeout;
    println!("Next configuration would be used:\n{}", configuration);
    match configuration.threads {
        1 => single_thread(configuration).await?,
        _ => multiple_threads(configuration).await?,
    };
    println!(
        "{} creating consumer to shutdown server",
        style("[test]").bold().dim(),
    );
    let (tx_stat, mut rx_stat): (UnboundedSender<StatEvent>, UnboundedReceiver<StatEvent>) =
        unbounded_channel();
    let done = CancellationToken::new();
    join!(
        async move {
            println!("{} starting consumer", style("[test]").bold().dim(),);
            if let Err(err) = run("127.0.0.1:8080", timeout, tx_stat, true, distributor).await {
                stop!("{}", err);
            }
            done.cancel();
        },
        async {
            let mut stat = Stat::new(1, true);
            while let Some(event) = rx_stat.recv().await {
                stat.apply(event);
            }
            println!("\n{} consumers did all jobs", style("[test]").bold().dim(),);
        }
    );
    println!("{} server is down", style("[test]").bold().dim(),);
    Ok(())
}
