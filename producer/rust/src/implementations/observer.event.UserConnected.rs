use super::consumer_identification::EFilterMatchCondition;
use super::observer::{EventObserverErrors};
use super::DeclEventUserConnected::{EventUserConnected, TBroadcastHandler};
use super::{Broadcasting, UserCustomContext, broadcasting};
use super::consumer::{Consumer};
use fiber::server::context::ConnectionContext;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

pub struct Event {
    pub prop1: String,
    pub prop2: u64,
}

type TEventHandler = &'static (dyn (Fn(
    &Event,
    Arc<RwLock<UserCustomContext>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);

#[derive(Clone)]
pub struct EventObserver {
    subscriptions: HashMap<Uuid, TEventHandler>,
    sender: Option<Sender<Event>>,
}

impl EventObserver {
    pub fn new() -> Self {
        EventObserver {
            subscriptions: HashMap::new(),
            sender: None,
        }
    }

    pub fn subscribe(&mut self, handler: TEventHandler) -> Uuid {
        let uuid: Uuid = Uuid::new_v4();
        self.subscriptions.insert(uuid.clone(), handler);
        uuid
    }

    pub fn unsubscribe(&mut self, uuid: Uuid) {
        self.subscriptions.remove(&uuid);
    }

}

impl<CX: 'static> EventUserConnected<UserCustomContext, Event, CX> for EventObserver where CX: ConnectionContext + Send + Sync, {

    fn emitter(&self) -> Option<Sender<Event>> {
        self.sender.clone()
    }

    //fn listen(&mut self, ucx: Arc<RwLock<UserCustomContext>>, broadcast: TBroadcastHandler) {
    fn listen(&mut self, ucx: Arc<RwLock<UserCustomContext>>, consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>) {
        let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        self.sender = Some(sender);
        let subscriptions = self.subscriptions.clone();
        spawn(move || {
            let timeout = Duration::from_millis(50);
            loop {
                match receiver.try_recv() {
                    Ok(event) => {
                        let broadcast = |filter: HashMap<String, String>, condition: EFilterMatchCondition, broadcast: Broadcasting| {
                            broadcasting(consumers.clone(), filter, condition, broadcast)
                        };
                        for (uuid, handler) in subscriptions.iter() {
                            if let Err(e) = handler(&event, ucx.clone(), &broadcast) {
                                println!("Fail to call handler ({}) for event due error: {}", uuid, e);
                            }
                        }
                    },
                    Err(_) => {
                        // No needs logs here;
                        thread::sleep(timeout);
                    }
                }
            }
        });
    }


}

