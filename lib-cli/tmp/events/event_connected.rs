
use super::consumer_identification::Filter;

use uuid::Uuid;
#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        uuid: Uuid,
        ucx: UCX,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> () {
        panic!("hanlder method for Connected isn't implemented");
    }
}

#[derive(Clone)]
pub struct ObserverEvent {}

impl ObserverEvent {
    pub fn new() -> Self {
        ObserverEvent {}
    }

    pub fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        uuid: Uuid,
        ucx: UCX,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> () {
        Self::handler(uuid, ucx, broadcast);
    }
}

impl Observer for ObserverEvent {}
