use crate::{
    stat::{Alias, StatEvent},
    stop,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

pub struct Context {
    pub connected: CancellationToken,
    pub finish: CancellationToken,
    pub broadcast_received: CancellationToken,
    pub disconnected: CancellationToken,
    broadcasts: usize,
    tx_stat: UnboundedSender<StatEvent>,
}

impl Context {
    pub fn new(tx_stat: UnboundedSender<StatEvent>) -> Self {
        Context {
            connected: CancellationToken::new(),
            finish: CancellationToken::new(),
            broadcast_received: CancellationToken::new(),
            disconnected: CancellationToken::new(),
            broadcasts: 0,
            tx_stat,
        }
    }

    pub fn inc_stat(&mut self, alias: Alias) {
        self.broadcasts += 1;
        if self.tx_stat.send(StatEvent::Inc(alias)).is_err() {
            stop!("Fail to send stat event");
        }
        if self.broadcasts >= 19 {
            self.broadcast_received.cancel();
        }
    }
}
