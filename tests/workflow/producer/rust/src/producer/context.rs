use super::implementation::{producer::Control, protocol};
use crate::{
    stat::{Alias, Stat, StatEvent},
    stop,
};
use clibri::server;
use console::{style, Term};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Context {
    pub stats: HashMap<Uuid, Stat>,
    connected: usize,
    diconnected: usize,
    term: Term,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
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
        //self.report();
    }

    pub fn inc_stat(&mut self, uuid: Uuid, alias: Alias) {
        if self.stats.contains_key(&uuid) {
            self.stats
                .entry(uuid)
                .or_insert_with(Stat::new)
                .apply(StatEvent::Inc(alias.clone()));
            if let Alias::Disconnected = alias {
                self.diconnected += 1;
                //self.report();
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

    pub async fn check_beacons<E: server::Error, C: server::Control<E>>(
        &self,
        uuid: Uuid,
        control: &Control<E, C>,
    ) {
        let count = self.stats[&uuid].get_beacons_count();
        if count > 4 {
            stop!("Too many beacons has been gotten");
        }
        if count == 4 {
            if let Err(err) = control
                .events
                .finishconsumertest(protocol::FinishConsumerTest {
                    uuid: uuid.to_string(),
                })
                .await
            {
                stop!("Sending FinishConsumerTest error: {}", err);
            }
        }
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
