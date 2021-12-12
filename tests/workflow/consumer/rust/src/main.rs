#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod connection;
mod consumer;
mod stat;
mod test;

use connection::run;
use console::style;
use futures::future::join_all;
use stat::{Stat, StatEvent};
use tokio::{
    join, select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

const CONNECTIONS: usize = 10000;

#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx_stat, mut rx_stat): (UnboundedSender<StatEvent>, UnboundedReceiver<StatEvent>) =
        unbounded_channel();
    let mut jobs = vec![];
    println!("{} creating consumers jobs", style("[test]").bold().dim(),);
    for _ in 0..CONNECTIONS {
        jobs.push(run("127.0.0.1:8080", tx_stat.clone(), false));
    }
    drop(tx_stat);
    let done = CancellationToken::new();
    join!(
        async {
            println!("{} starting consumers jobs", style("[test]").bold().dim(),);
            join_all(jobs).await;
            done.cancel();
        },
        async {
            let mut stat = Stat::new(CONNECTIONS);
            while let Some(event) = rx_stat.recv().await {
                stat.apply(event);
            }
            println!(
                "\n{} all consumers did all jobs",
                style("[test]").bold().dim(),
            );
            println!("{}", stat);
        }
    );
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
            if let Err(err) = run("127.0.0.1:8080", tx_stat, true).await {
                eprintln!("{}", err);
            }
            done.cancel();
        },
        async {
            let mut stat = Stat::new(1);
            while let Some(event) = rx_stat.recv().await {
                stat.apply(event);
            }
            println!("\n{} consumers did all jobs", style("[test]").bold().dim(),);
        }
    );
    println!("{} server is down", style("[test]").bold().dim(),);
    Ok(())
}
