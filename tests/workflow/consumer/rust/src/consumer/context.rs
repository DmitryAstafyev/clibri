use super::implementation::{controller, protocol, Consumer};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct BroadcastReport {
    pub groupa_structa: u8,
    pub groupa_structb: u8,
    pub groupb_groupc_structa: u8,
    pub groupb_groupc_structb: u8,
    pub groupb_structa: u8,
    pub structa: u8,
    pub structb: u8,
    pub structc: u8,
    pub structd: u8,
    pub structf: u8,
    pub structj: u8,
    pub received: CancellationToken,
}

impl BroadcastReport {
    pub fn valid(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn check(&self) {
        let ready = if self.groupa_structa == 0 {
            false
        } else if self.groupa_structb == 0 {
            false
        } else if self.groupb_groupc_structa == 0 {
            false
        } else if self.groupb_groupc_structb == 0 {
            false
        } else if self.groupb_structa == 0 {
            false
        } else if self.structa == 0 {
            false
        } else if self.structb == 0 {
            false
        } else if self.structc == 0 {
            false
        } else if self.structd == 0 {
            false
        } else if self.structf == 0 {
            false
        } else if self.structj == 0 {
            false
        } else {
            true
        };
        if ready {
            self.received.cancel();
        }
    }
}

pub struct Context {
    pub connected: CancellationToken,
    pub finish: CancellationToken,
    pub broadcast: BroadcastReport,
}

impl Context {
    pub fn new() -> Self {
        Context {
            connected: CancellationToken::new(),
            finish: CancellationToken::new(),
            broadcast: BroadcastReport {
                groupa_structa: 0,
                groupa_structb: 0,
                groupb_groupc_structa: 0,
                groupb_groupc_structb: 0,
                groupb_structa: 0,
                structa: 0,
                structb: 0,
                structc: 0,
                structd: 0,
                structf: 0,
                structj: 0,
                received: CancellationToken::new(),
            }
        }
    }
}
