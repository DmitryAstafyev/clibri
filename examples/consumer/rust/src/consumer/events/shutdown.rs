use super::{Consumer, ConsumerError, Context};
use clibri::client;

pub async fn handler<E: client::Error>(
    error: Option<ConsumerError<E>>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    println!("handler for event shutdown isn't implemented");
}
