use super::control::Control;
use super::events::Events;
use futures::Future;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use std::pin::Pin;
pub type Task<E> = Pin<Box<dyn Future<Output = Result<(), E>>>>;

pub trait Interface<E: std::error::Error>: Send {
    fn listen(
        &mut self,
        channel: UnboundedSender<Events<E>>,
        messages: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        controll: Option<UnboundedReceiver<Control>>,
    ) -> Result<Task<E>, E>;
}
