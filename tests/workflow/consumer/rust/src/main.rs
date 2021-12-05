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
use spinners::{Spinner, Spinners};
use stat::{Stat, StatEvent};
use tokio::{
    join, select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

const CONNECTIONS: usize = 100;

#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx_stat, mut rx_stat): (UnboundedSender<StatEvent>, UnboundedReceiver<StatEvent>) =
        unbounded_channel();
    let mut jobs = vec![];
    println!("{} creating consumers jobs", style("[test]").bold().dim(),);
    for _ in 0..CONNECTIONS {
        jobs.push(run("127.0.0.1:8080", tx_stat.clone()));
    }
    let done = CancellationToken::new();

    join!(
        async {
            println!("{} starting consumers jobs", style("[test]").bold().dim(),);
            join_all(jobs).await;
            done.cancel();
        },
        async {
            let spinner = Spinner::new(
                &Spinners::Dots9,
                format!("Waiting for {} consumers", CONNECTIONS),
            );
            let mut stat = Stat::new(CONNECTIONS);
            while let Some(event) = select! {
                event = rx_stat.recv() => event,
                _ = done.cancelled() => None
            } {
                stat.apply(event);
            }
            spinner.stop();
            println!(
                "\n{} all consumers did all jobs",
                style("[test]").bold().dim(),
            );
            println!("{}", stat);
        }
    );
    Ok(())
}
