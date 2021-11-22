use std::collections::HashMap;
use uuid::Uuid;
use super::{implementation::{protocol, producer::{Control}}};
use clibri::server;

pub struct Requests {
    structa: HashMap<Uuid, u8>,
    structc: HashMap<Uuid, u8>,
    structd: HashMap<Uuid, u8>,
    structf: HashMap<Uuid, u8>,
    groupb_groupc_structa: HashMap<Uuid, u8>,
    groupa_structb: HashMap<Uuid, u8>,
    groupa_structa: HashMap<Uuid, u8>,
    groupb_structa: HashMap<Uuid, u8>,
    groupb_groupc_structb: HashMap<Uuid, u8>,
    structempty: HashMap<Uuid, u8>,
}

impl Default for Requests {
    fn default() -> Self {
        Self::new()
    }
}

impl Requests {
    pub fn new() -> Self {
        Self {
            structa: HashMap::new(),
            structc: HashMap::new(),
            structd: HashMap::new(),
            structf: HashMap::new(),
            groupb_groupc_structa: HashMap::new(),
            groupa_structa: HashMap::new(),
            groupa_structb: HashMap::new(),
            groupb_structa: HashMap::new(),
            groupb_groupc_structb: HashMap::new(),
            structempty: HashMap::new(),
        }
    }
    pub fn structa(&mut self, uuid: Uuid) -> u8 {
        *self.structa.entry(uuid).or_insert(0) += 1;
        self.structa[&uuid]
    }
    pub fn structc(&mut self, uuid: Uuid) -> u8 {
        *self.structc.entry(uuid).or_insert(0) += 1;
        self.structc[&uuid]
    }
    pub fn structd(&mut self, uuid: Uuid) -> u8 {
        *self.structd.entry(uuid).or_insert(0) += 1;
        self.structd[&uuid]
    }
    pub fn structf(&mut self, uuid: Uuid) -> u8 {
        *self.structf.entry(uuid).or_insert(0) += 1;
        self.structf[&uuid]
    }
    pub fn groupb_groupc_structa(&mut self, uuid: Uuid) -> u8 {
        *self.groupb_groupc_structa.entry(uuid).or_insert(0) += 1;
        self.groupb_groupc_structa[&uuid]
    }
    pub fn groupa_structa(&mut self, uuid: Uuid) -> u8 {
        *self.groupa_structa.entry(uuid).or_insert(0) += 1;
        self.groupa_structa[&uuid]
    }
    pub fn groupa_structb(&mut self, uuid: Uuid) -> u8 {
        *self.groupa_structb.entry(uuid).or_insert(0) += 1;
        self.groupa_structb[&uuid]
    }
    pub fn groupb_structa(&mut self, uuid: Uuid) -> u8 {
        *self.groupb_structa.entry(uuid).or_insert(0) += 1;
        self.groupb_structa[&uuid]
    }
    pub fn groupb_groupc_structb(&mut self, uuid: Uuid) -> u8 {
        *self.groupb_groupc_structb.entry(uuid).or_insert(0) += 1;
        self.groupb_groupc_structb[&uuid]
    }
    pub fn structempty(&mut self, uuid: Uuid) -> u8 {
        *self.structempty.entry(uuid).or_insert(0) += 1;
        self.structempty[&uuid]
    }
}

pub struct Beacons {
    beacona: HashMap<Uuid, u8>,
    beacons_beacona: HashMap<Uuid, u8>,
    beacons_beaconb: HashMap<Uuid, u8>,
    beacons_sub_beacona: HashMap<Uuid, u8>,
}

impl Default for Beacons {
    fn default() -> Self {
        Self::new()
    }
}

impl Beacons {
    pub fn new() -> Self {
        Self {
            beacona: HashMap::new(),
            beacons_beacona: HashMap::new(),
            beacons_beaconb: HashMap::new(),
            beacons_sub_beacona: HashMap::new(),
        }
    }
    pub fn beacona(&mut self, uuid: Uuid) -> u8 {
        *self.beacona.entry(uuid).or_insert(0) += 1;
        self.beacona[&uuid]
    }
    pub fn beacons_beacona(&mut self, uuid: Uuid) -> u8 {
        *self.beacons_beacona.entry(uuid).or_insert(0) += 1;
        self.beacons_beacona[&uuid]
    }
    pub fn beacons_beaconb(&mut self, uuid: Uuid) -> u8 {
        *self.beacons_beaconb.entry(uuid).or_insert(0) += 1;
        self.beacons_beaconb[&uuid]
    }
    pub fn beacons_sub_beacona(&mut self, uuid: Uuid) -> u8 {
        *self.beacons_sub_beacona.entry(uuid).or_insert(0) += 1;
        self.beacons_sub_beacona[&uuid]
    }
    pub async fn check<E: std::error::Error, C: server::Control<E> + Send + Clone>(&self, uuid: Uuid, control: &Control<E, C>) {
        let mut sum: u8 = 0;
        if let Some(count) = self.beacona.get(&uuid) {
            sum += count;
        }
        if let Some(count) = self.beacons_beacona.get(&uuid) {
            sum += count;
        }
        if let Some(count) = self.beacons_beaconb.get(&uuid) {
            sum += count;
        }
        if let Some(count) = self.beacons_sub_beacona.get(&uuid) {
            sum += count;
        }
        if sum == 4 {
            if let Err(err) = control.events.finishconsumertest(protocol::FinishConsumerTest { uuid: uuid.to_string() }).await {
                panic!("Sending FinishConsumerTest error: {}", err);
            }
        }
    }
}

pub struct Context {
    pub requests: Requests,
    pub beacons: Beacons,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
            beacons: Beacons::new(),
        }
    }

    
}
