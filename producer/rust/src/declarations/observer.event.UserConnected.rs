use super::consumer_identification::EFilterMatchCondition;
use super::observer::{EventObserverErrors};
use super::{Broadcasting};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

//TODO: add event type
pub trait EventUserConnected<UCX: Send + Sync> {
    fn emit(
        &mut self,
        ucx: Arc<RwLock<UCX>>,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), EventObserverErrors>;
}
