use super::{Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(context: &mut Context, consumer: Consumer<E>) {
    println!("handler for event disconnected isn't implemented");
}
