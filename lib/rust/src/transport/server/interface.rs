use super::control::Control;
use super::events::Events;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;
use async_trait::async_trait;

pub type Sending = (Vec<u8>, Option<Uuid>);

#[async_trait]
pub trait Interface<E: std::error::Error>: Send {

    async fn listen(
        &mut self,
    ) -> Result<(), E>;
    fn observer(&mut self) -> Result<UnboundedReceiver<Events<E>>, E>;
    fn sender(&self) -> UnboundedSender<Sending>;
    fn control(&self) -> UnboundedSender<Control>;

}
