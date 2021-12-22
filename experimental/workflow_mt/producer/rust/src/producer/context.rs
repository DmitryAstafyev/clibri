use super::implementation::{producer::Control, protocol};
use crate::{
    stat::{Alias, Stat, StatEvent},
    stop,
};
use clibri::server;
use console::{style, Term};
use std::collections::HashMap;
use tokio::{
    join, select,
    sync::{
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct Summary {
    pub stats: HashMap<Uuid, Stat>,
    connected: usize,
    diconnected: usize,
    term: Term,
}

impl Default for Summary {
    fn default() -> Self {
        Self::new()
    }
}

impl Summary {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            connected: 0,
            diconnected: 0,
            term: Term::stdout(),
        }
    }

    pub fn add_stat(&mut self, uuid: Uuid) {
        self.stats.insert(uuid, Stat::new());
        self.connected += 1;
        self.report();
    }

    pub fn inc_stat(&mut self, uuid: Uuid, alias: Alias) {
        if self.stats.contains_key(&uuid) {
            self.stats
                .entry(uuid)
                .or_insert_with(Stat::new)
                .apply(StatEvent::Inc(alias.clone()));
            if let Alias::Disconnected = alias {
                self.diconnected += 1;
                self.report();
            }
        }
    }

    pub fn get_index(&mut self, uuid: Uuid, alias: Alias) -> usize {
        if self.stats.contains_key(&uuid) {
            self.stats
                .entry(uuid)
                .or_insert_with(Stat::new)
                .get_index(alias)
        } else {
            0
        }
    }

    pub fn is_all_beacons_gotten(&self, uuid: Uuid) -> bool {
        let count = self.stats[&uuid].get_beacons_count();
        if count > 4 {
            stop!("Too many beacons has been gotten");
        }
        count == 4
    }

    pub fn summary(&self) -> Stat {
        let mut summary = Stat::new();
        summary.drop();
        for stat in self.stats.values() {
            summary.merge(stat);
        }
        summary
    }

    fn report(&self) {
        println!(
            "{} {} / {} ({}%) connections done",
            style("[test]").bold().dim(),
            self.diconnected,
            self.connected,
            (self.diconnected * 100) / self.connected
        );
        if let Err(err) = self.term.move_cursor_up(1) {
            eprintln!("Fail to manipulate console: {}", err);
        }
    }
}

pub enum ContextAction {
    AddStat(Uuid, oneshot::Sender<()>),
    IncStat(Uuid, Alias, oneshot::Sender<()>),
    GetIndex(Uuid, Alias, oneshot::Sender<usize>),
    IsAllBeaconsGotten(Uuid, oneshot::Sender<bool>),
    RemoveStat(Uuid, oneshot::Sender<()>),
}

#[derive(Clone, Debug)]
pub struct Context {
    tx_context: UnboundedSender<ContextAction>,
    shutdown: CancellationToken,
}

impl Context {
    pub fn new() -> Self {
        let (tx_context, mut rx_context): (
            UnboundedSender<ContextAction>,
            UnboundedReceiver<ContextAction>,
        ) = unbounded_channel();
        let shutdown = CancellationToken::new();
        let cancel = shutdown.clone();
        task::spawn(async move {
            let mut summary = Summary::new();
            while let Some(action) = select! {
                action = rx_context.recv() => action,
                _ = cancel.cancelled() => None,
            } {
                match action {
                    ContextAction::AddStat(uuid, tx_response) => {
                        summary.add_stat(uuid);
                        if tx_response.send(()).is_err() {
                            stop!("Fail send response for ContextAction::AddStat");
                        }
                    }
                    ContextAction::IncStat(uuid, alias, tx_response) => {
                        summary.inc_stat(uuid, alias);
                        if tx_response.send(()).is_err() {
                            stop!("Fail send response for ContextAction::IncStat");
                        }
                    }
                    ContextAction::GetIndex(uuid, alias, tx_response) => {
                        if tx_response.send(summary.get_index(uuid, alias)).is_err() {
                            stop!("Fail send response for ContextAction::GetIndex");
                        }
                    }
                    ContextAction::IsAllBeaconsGotten(uuid, tx_response) => {
                        if tx_response
                            .send(summary.is_all_beacons_gotten(uuid))
                            .is_err()
                        {
                            stop!("Fail send response for ContextAction::IsAllBeaconsGotten");
                        }
                    }
                    ContextAction::RemoveStat(uuid, tx_response) => {
                        summary.stats.remove(&uuid);
                        if tx_response.send(()).is_err() {
                            stop!("Fail send response for ContextAction::RemoveStat");
                        }
                    }
                }
            }
            let report = summary.summary();
            println!("{}", report);
            let errors = report.get_errors();
            if !errors.is_empty() {
                for error in errors {
                    eprintln!("{}", error);
                }
                stop!("");
            }
        });
        Context {
            tx_context,
            shutdown,
        }
    }

    pub async fn add_stat(&self, uuid: Uuid) {
        let (tx_response, rx_response): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        if let Err(err) = self
            .tx_context
            .send(ContextAction::AddStat(uuid, tx_response))
        {
            stop!("{}", err);
        }
        if rx_response.await.is_err() {
            stop!("Fail to get response for ContextAction::AddStat");
        }
    }

    pub async fn inc_stat(&self, uuid: Uuid, alias: Alias) {
        let (tx_response, rx_response): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        if let Err(err) = self
            .tx_context
            .send(ContextAction::IncStat(uuid, alias, tx_response))
        {
            stop!("{}", err);
        }
        if rx_response.await.is_err() {
            stop!("Fail to get response for ContextAction::IncStat");
        }
    }

    pub async fn get_index(&self, uuid: Uuid, alias: Alias) -> usize {
        let (tx_response, rx_response): (oneshot::Sender<usize>, oneshot::Receiver<usize>) =
            oneshot::channel();
        if let Err(err) = self
            .tx_context
            .send(ContextAction::GetIndex(uuid, alias, tx_response))
        {
            stop!("{}", err);
        }
        match rx_response.await {
            Ok(index) => index,
            Err(_) => {
                stop!("Fail to get response for ContextAction::GetIndex");
            }
        }
    }

    pub async fn is_all_beacons_gotten(&self, uuid: Uuid) -> bool {
        let (tx_response, rx_response): (oneshot::Sender<bool>, oneshot::Receiver<bool>) =
            oneshot::channel();
        if let Err(err) = self
            .tx_context
            .send(ContextAction::IsAllBeaconsGotten(uuid, tx_response))
        {
            stop!("{}", err);
        }
        match rx_response.await {
            Ok(state) => state,
            Err(_) => {
                stop!("Fail to get response for ContextAction::IsAllBeaconsGotten");
            }
        }
    }

    pub async fn remove_stat(&self, uuid: Uuid) {
        let (tx_response, rx_response): (oneshot::Sender<()>, oneshot::Receiver<()>) =
            oneshot::channel();
        if let Err(err) = self
            .tx_context
            .send(ContextAction::RemoveStat(uuid, tx_response))
        {
            stop!("{}", err);
        }
        if rx_response.await.is_err() {
            stop!("Fail to get response for ContextAction::RemoveStat");
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }
}
