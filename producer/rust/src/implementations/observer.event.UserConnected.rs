use super::consumer_identification::EFilterMatchCondition;
use super::observer::{EventObserverErrors};
use super::DeclEventUserConnected::{EventUserConnected};
use super::{Broadcasting, UserCustomContext};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

type TEventHandler = &'static (dyn (Fn(
    Arc<RwLock<UserCustomContext>>,
    &dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>,
) -> Result<(), String>)
              + Send
              + Sync);

#[derive(Clone)]
pub struct EventObserver {
    subscriptions: HashMap<Uuid, TEventHandler>,
}

impl EventObserver {
    pub fn new() -> Self {
        EventObserver {
            subscriptions: HashMap::new(),
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

impl EventUserConnected<UserCustomContext> for EventObserver {

    fn emit(
        &mut self,
        ucx: Arc<RwLock<UserCustomContext>>,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), EventObserverErrors> {
        for (uuid, handler) in self.subscriptions.iter() {
            if let Err(e) = handler(ucx.clone(), broadcast) {
                println!("Fail to call handler ({}) for event due error: {}", uuid, e);
            }
        }
        Ok(())
    }
}

