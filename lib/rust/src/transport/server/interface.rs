use super::events::{ Events };
use super::control::{ Control };
use uuid::Uuid;
use tokio::{
    sync::{
        mpsc::{
            UnboundedSender,
            UnboundedReceiver
        }
    }
};
pub trait Interface: Send {

    fn listen(
        &mut self,
        channel: UnboundedSender<Events>,
        messages: UnboundedReceiver<(Vec<u8>, Option<Uuid>)>,
        controll: Option<UnboundedReceiver<Control>>,
    ) -> Result<(), String>;

}
