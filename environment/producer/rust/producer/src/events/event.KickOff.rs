use super::{Protocol::{PackingStruct, StructEncode}, consumer_identification::Filter};
use super::Control;
use super::Protocol;
use super::tools;

use tokio::{
    select,
    sync::mpsc::{
        unbounded_channel,
        UnboundedReceiver,
        UnboundedSender,
        error::SendError
    },
    sync::oneshot::{
        channel,
        Receiver,
        Sender
    },
    task::{
        spawn,
        JoinHandle
    },
    runtime::Runtime,
};
use fiber::{
    logger::Logger,
    server::{
        control::Control as ServerControl,
        events::Events,
        interface::Interface
    }
};
use uuid::Uuid;

pub struct Event {
    field_a: u32,
}

pub enum Broadcast {
    Message(Protocol::Events::Message),
    User(Protocol::Events::UserConnected),
}

#[allow(unused_variables)]
pub trait Observer {
    fn handler<UCX: 'static + Sync + Send + Clone>(
        event: Event,
        ucx: UCX,
        control: Control,
    ) -> Option<Vec<(Filter, Broadcast)>> {
        panic!("hanlder method for Connected isn't implemented");
    }
}

#[derive(Clone)]
pub struct ObserverEvent {
    
}

impl ObserverEvent {

    pub async fn listen<UCX: 'static + Sync + Send + Clone>(
        ucx: UCX,
        control: Control,
        mut rx_event: UnboundedReceiver<Event>,
    ) {
        while let Some(event) = rx_event.recv().await {
            if let Some(mut messages) = Self::handler(
                event,
                ucx.clone(),
                control.clone()
            ) {
                loop {
                    if messages.is_empty() {
                        break;
                    }
                    let (filter, message) = messages.remove(0);
                    match match message {
                        Broadcast::Message(mut msg) => msg.pack(0, None),
                        Broadcast::User(mut msg) => msg.pack(0, None),
                    } {
                        Ok(buffer) => {
                            if let Err(err) = control.send(filter, buffer) {
                                tools::logger.err(&format!("[event: Event] fail to send message due error: {}", err));
                            }
                        },
                        Err(err) => {
                            tools::logger.err(&format!("[event: Event] fail to get a buffer due error: {}", err));
                        },
                    }
                }
            }
        }
    }
}

impl Observer for ObserverEvent {}
