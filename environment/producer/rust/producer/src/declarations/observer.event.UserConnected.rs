use super::consumer::Consumer;
use super::consumer_identification::EFilterMatchCondition;
use super::logger::Logger;
use super::{broadcasting, Broadcasting,ProducerEvents};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread::spawn;

use uuid::Uuid;

pub struct Event {
    pub prop1: String,
    pub prop2: u64,
}

#[allow(unused_variables)]
pub trait Controller {
    fn connected<UCX: 'static + Sync + Send + Clone>(
        event: &Event,
        ucx: UCX,
        broadcasting: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Err(String::from("connected handler isn't implemented"))
    }

    fn listen<UCX: 'static + Sync + Send + Clone>(
        &mut self,
        ucx: UCX,
        consumers: Arc<RwLock<HashMap<Uuid, Consumer>>>,
        logger: &'static (dyn Logger + Send + Sync),
        feedback: Sender<ProducerEvents<UCX>>,
    ) -> Result<Sender<Event>, String> {
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        spawn(move || {
            loop {
                match receiver.recv() {
                    Ok(event) => {
                        let broadcast =
                            |filter: HashMap<String, String>,
                             condition: EFilterMatchCondition,
                             broadcast: Broadcasting| {
                                broadcasting(consumers.clone(), filter, condition, broadcast)
                            };
                        if let Err(e) = Self::connected(&event, ucx.clone(), &broadcast) {
                            if let Err(e) = feedback.send(ProducerEvents::EventError(e)) {
                                logger.err(&format!("Fail to call connected handler for event due error: {}", e));
                            }
                            break;
                        }
                    },
                    Err(e) => {
                        if let Err(e) = feedback.send(ProducerEvents::EventChannelError(e.to_string())) {
                            logger.err(&format!("Fail receive event due error: {}", e));
                        }
                        break;
                    }
                }
            }
        });
        Ok(sender)
    }
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

