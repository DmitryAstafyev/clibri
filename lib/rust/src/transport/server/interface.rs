use super::control::Control;
use super::events::Events;
use futures::Future;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use std::pin::Pin;

pub type Task<E> = Pin<Box<dyn Future<Output = Result<(), E>> + Send>>;

pub type Sending = (Vec<u8>, Option<Uuid>);

pub trait Interface<E: std::error::Error>: Send {

    fn listen(
        &mut self,
    ) -> Result<Task<E>, E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Events<E>>, E>;
    fn sender(&self) -> UnboundedSender<Sending>;
    fn control(&self) -> UnboundedSender<Control>;

}
