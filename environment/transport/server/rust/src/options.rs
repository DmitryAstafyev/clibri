use std::net::SocketAddr;
use std::ops::Range;

#[derive(Clone, Debug)]
pub enum Ports {
    List(Vec<u16>),
    Range(Range<u16>),
}

#[derive(Clone, Debug)]
pub struct Distributor {
    pub addr: String,
    pub ports: Ports,
    pub distributor: SocketAddr,
    pub connections_per_port: u32,
}

#[derive(Clone, Debug)]
pub enum Listener {
    Direct(SocketAddr),
    Distributor(Distributor),
}

#[derive(Clone, Debug)]
pub struct Options {
    pub listener: Listener,
}
