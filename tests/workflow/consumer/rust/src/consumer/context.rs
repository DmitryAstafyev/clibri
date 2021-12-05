use crate::stat::{Alias, StatEvent};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

pub struct Context {
    pub connected: CancellationToken,
    pub finish: CancellationToken,
    pub broadcast_received: CancellationToken,
    broadcasts: usize,
    tx_stat: UnboundedSender<StatEvent>,
}

impl Context {
    pub fn new(tx_stat: UnboundedSender<StatEvent>) -> Self {
        Context {
            connected: CancellationToken::new(),
            finish: CancellationToken::new(),
            broadcast_received: CancellationToken::new(),
            broadcasts: 0,
            tx_stat,
        }
    }

    pub fn inc_stat(&mut self, alias: Alias) {
        self.broadcasts += 1;
        self.tx_stat.send(StatEvent::Inc(alias));
        if self.broadcasts >= 19 {
            self.broadcast_received.cancel();
        }
    }
}
