use super::{identification, producer::Control, protocol, Context, scope::Scope};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    beacon: &protocol::Beacons::LikeMessage,
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    println!("Handler for protocol::Beacons::LikeMessage isn't implemented");
    Ok(())
}