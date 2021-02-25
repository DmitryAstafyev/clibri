use super::consumer::Consumer;
use super::consumer_identification::EFilterMatchCondition;
use super::DeclEventUserConnected::EventUserConnected;
use super::{broadcasting, Broadcasting};
use fiber::server::context::ConnectionContext;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use uuid::Uuid;

pub struct Event {
    pub prop1: String,
    pub prop2: u64,
}

pub trait EventObserverInterface<UCX: Send + Sync> {

    fn connected(
        &mut self,
        event: &Event,
        ucx: Arc<RwLock<UCX>>,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

}

#[derive(Clone)]
pub struct EventObserver {
    sender: Option<Sender<Event>>,
}

impl<UCX: Send + Sync> EventObserverInterface<UCX> for EventObserver {}

impl EventObserver {

    pub fn new() -> Self {
        EventObserver {
            sender: None,
        }
    }

}

impl<CX: 'static, UCX: Send + Sync> EventUserConnected<UCX, Event, CX>
    for EventObserver
where
    CX: ConnectionContext + Send + Sync,
    UCX: Send + Sync,
{
    fn emitter(&self) -> Option<Sender<Event>> {
        self.sender.clone()
    }

    fn listen(
        &mut self,
        ucx: Arc<RwLock<UCX>>,
        consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>,
    ) {
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        self.sender = Some(sender);
        let wrapped = Arc::new(RwLock::new(self));
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match receiver.try_recv() {
                    Ok(event) => {
                        let broadcast =
                            |filter: HashMap<String, String>,
                             condition: EFilterMatchCondition,
                             broadcast: Broadcasting| {
                                broadcasting(consumers.clone(), filter, condition, broadcast)
                            };
                            /*
                            match wrapped.write() {
                                Ok(s) => {
                                    if let Err(e) = s.connected(&event, ucx.clone(), &broadcast) {
                                        println!(
                                            "Fail to call connected handler for event due error: {}",
                                            e
                                        );
                                    }
                                },
                                Err(e) => {
    
                                },
                            }*/
                    }
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                }
            }
        });
    }
}
