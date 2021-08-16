use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub enum ConnectionType {
    Direct(SocketAddr),
    Distributor(SocketAddr),
}

#[derive(Clone, Debug)]
pub struct Options {
    pub connection: ConnectionType,
}