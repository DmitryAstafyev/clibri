use super::{controller, protocol, Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(context: &mut Context, mut consumer: Consumer<E>) {
    context.connected.cancel();
}
