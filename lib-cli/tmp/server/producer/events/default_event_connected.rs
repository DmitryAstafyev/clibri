use super::{
    Filter,
    Broadcast,
    Protocol::PackingStruct,
};

use uuid::Uuid;

#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        uuid: Uuid,
        ucx: UCX,
        broadcast: &dyn Fn(Filter, Broadcast) -> Result<(), String>,
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
        Self::handler(uuid, ucx, &(|filter: Filter, message: Broadcast| {
            broadcast(filter, match message {                
                Broadcast::EventsUserDisconnected(mut msg) => msg.pack(0, None)?,
                Broadcast::EventsMessage(mut msg) => msg.pack(0, None)?,Other
            })
        }));
    }
}

impl Observer for ObserverEvent {}
