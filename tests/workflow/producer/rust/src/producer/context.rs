use super::implementation::{producer::Control, protocol};
use crate::stat::{Alias, Stat, StatEvent};
use clibri::server;
use std::collections::HashMap;
use uuid::Uuid;
pub struct Context {
    pub stats: HashMap<Uuid, Stat>,
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
        }
    }

    pub fn add_stat(&mut self, uuid: Uuid) {
        self.stats.insert(uuid, Stat::new());
    }

    pub fn inc_stat(&mut self, uuid: Uuid, alias: Alias) {
        if self.stats.contains_key(&uuid) {
            self.stats
                .entry(uuid)
                .or_insert(Stat::new())
                .apply(StatEvent::Inc(alias));
        }
    }

    pub fn get_index(&mut self, uuid: Uuid, alias: Alias) -> usize {
        if self.stats.contains_key(&uuid) {
            self.stats
                .entry(uuid)
                .or_insert(Stat::new())
                .get_index(alias)
        } else {
            0
        }
    }

    pub async fn check_beacons<E: server::Error, C: server::Control<E> + Send + Clone>(
        &self,
        uuid: Uuid,
        control: &Control<E, C>,
    ) {
        let count = self.stats[&uuid].get_beacons_count();
        if count == 4 {
            if let Err(err) = control
                .events
                .finishconsumertest(protocol::FinishConsumerTest {
                    uuid: uuid.to_string(),
                })
                .await
            {
                panic!("Sending FinishConsumerTest error: {}", err);
            }
        } else if count > 4 {
            panic!("Too many beacons has been gotten");
        }
    }

    pub fn summary(&self) -> Stat {
        let mut summary = Stat::new();
        summary.drop();
        for (_, stat) in &self.stats {
            summary.merge(&stat);
        }
        summary
    }
}
