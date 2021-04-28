use super::control::Control;
use super::events::Events;
use futures::Future;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

pub trait Interface: Send {
    type Output: Future<Output = Result<(), String>>;

    fn listen(
        &mut self,
        channel: UnboundedSender<Events>,
        messages: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        controll: Option<UnboundedReceiver<Control>>,
    ) -> Self::Output;
}
