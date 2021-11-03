use super::{Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(context: &mut Context, consumer: Consumer<E>) {
    context.shutdown();
}
