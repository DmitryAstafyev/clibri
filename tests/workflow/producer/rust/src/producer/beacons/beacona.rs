use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    beacon: &protocol::BeaconA,
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    if !samples::beacon_a::equal(beacon.clone()) {
        stop!("BeaconA isn't equal to sample");
    }
    scope
        .context
        .inc_stat(scope.identification.uuid(), Alias::BeaconA);
    scope
        .context
        .check_beacons(scope.identification.uuid(), scope.control)
        .await;
    Ok(())
}
