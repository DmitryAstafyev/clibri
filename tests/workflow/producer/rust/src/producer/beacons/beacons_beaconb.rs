use super::{identification, producer::Control, protocol, scope::Scope, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    beacon: &protocol::Beacons::BeaconB,
    scope: &mut Scope<'_, E, C>,
) -> Result<(), String> {
    if !samples::beacons::beacon_b::equal(beacon.clone()) {
        stop!("BeaconA isn't equal to sample");
    }
    scope
        .context
        .inc_stat(scope.identification.uuid(), Alias::BeaconsBeaconB);
    scope
        .context
        .check_beacons(scope.identification.uuid(), scope.control)
        .await;
    Ok(())
}
