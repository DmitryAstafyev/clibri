use super::{Consumer, Context};
use clibri::client;

pub async fn handler<E: client::Error>(context: &mut Context, consumer: Consumer<E>) {
    context.shutdown();
    println!("you are disconnected");
}
