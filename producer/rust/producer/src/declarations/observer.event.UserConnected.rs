use super::consumer::Consumer;
use super::consumer_identification::EFilterMatchCondition;
use super::{broadcasting, Broadcasting};
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

pub type TBroadcastHandler = &'static (dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>
              + Send
              + Sync);

pub trait EventsController<UCX: Send + Sync> {
    fn connected(
        event: &Event,
        ucx: Arc<RwLock<UCX>>,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("connected method isn't implemented"))
    }

    fn emitter(&self) -> Option<Sender<Event>>;

    fn listen(
        &mut self,
        ucx: Arc<RwLock<UCX>>,
        consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
    );
}

#[derive(Clone)]
pub struct Observer {
    sender: Option<Sender<Event>>,
}

impl Observer {
    pub fn new() -> Self {
        Observer { sender: None }
    }
}

impl<UCX: Send + Sync> EventsController<UCX> for Observer {
    fn emitter(&self) -> Option<Sender<Event>> {
        self.sender.clone()
    }

    fn listen(
        &mut self,
        ucx: Arc<RwLock<UCX>>,
        consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
    ) {
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        self.sender = Some(sender);
        
        /*
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
                        if let Err(e) = Self::connected(&event, ucx.clone(), &broadcast) {
                            println!("Fail to call connected handler for event due error: {}", e);
                        }
                    }
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                }
            }
        });
        */
    }
}
