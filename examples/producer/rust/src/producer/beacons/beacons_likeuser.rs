use super::{identification, producer::Control, protocol, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::LikeUser,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    println!("Handler for protocol::Beacons::LikeUser isn't implemented");
    Ok(())
}