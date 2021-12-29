use super::{identification, producer::Control, protocol, scope::Scope, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    beacon: &protocol::Beacons::LikeUser,
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    println!("Handler for protocol::Beacons::LikeUser isn't implemented");
    Ok(())
}
