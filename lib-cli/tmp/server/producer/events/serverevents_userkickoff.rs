
use super::{
    Control,
    Protocol,
    Protocol::PackingStruct,
    consumer_identification::Filter
};
use tokio::{
    sync::mpsc::{
        UnboundedReceiver,
    }
};
use fiber::env::logs;
use log::{error};

pub enum Broadcast {
    EventsMessage(Protocol::Events::Message),
    EventsUserConnected(Protocol::Events::UserConnected),
}

#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        event: &Protocol::ServerEvents::UserKickOff,
        ucx: UCX,
        control: Control,
    ) -> Option<Vec<(Filter, Broadcast)>> {
        panic!("hanlder method for ServerEvents::UserKickOff isn't implemented");
    }
}

#[derive(Clone)]
pub struct ObserverEvent {
    
}

impl ObserverEvent {

    pub async fn listen<UCX: 'static + Sync + Send + Clone>(
        ucx: UCX,
        control: Control,
        mut rx_event: UnboundedReceiver<Protocol::ServerEvents::UserKickOff>,
    ) {
        while let Some(event) = rx_event.recv().await {
            if let Some(mut messages) = Self::handler(
                &event,
                ucx.clone(),
                control.clone()
            ) {
                loop {
                    if messages.is_empty() {
                        break;
                    }
                    let (filter, message) = messages.remove(0);
                    match match message {
                        Broadcast::EventsMessage(mut msg) => msg.pack(0, None),
                        Broadcast::EventsUserConnected(mut msg) => msg.pack(0, None),
                    } {
                        Ok(buffer) => if let Err(err) = control.send(filter, buffer) {
                            error!(target: logs::targets::PRODUCER, "[event: ServerEvents::UserKickOff] fail to send message due error: {}", err);
                        },
                        Err(err) => {
                            error!(target: logs::targets::PRODUCER, "[event: ServerEvents::UserKickOff] fail to get a buffer due error: {}", err);
                        },
                    }
                }
            }
        }
    }
}

impl Observer for ObserverEvent {}

