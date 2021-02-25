use super::consumer_identification::EFilterMatchCondition;
use super::consumer::{Consumer};
use super::Broadcasting;
use fiber::server::context::ConnectionContext;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type TBroadcastHandler = &'static (dyn Fn(HashMap<String, String>, EFilterMatchCondition, Broadcasting) -> Result<(), String>
              + Send
              + Sync);

pub trait EventUserConnected<UCX: Send + Sync, E, CX> where CX: ConnectionContext + Send + Sync, {
    fn emitter(&self) -> Option<Sender<E>>;

    fn listen(&mut self, ucx: Arc<RwLock<UCX>>, consumers: Arc<RwLock<HashMap<Uuid, Consumer<CX>>>>);
}
