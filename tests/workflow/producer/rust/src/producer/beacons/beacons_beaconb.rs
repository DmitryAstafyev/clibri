use super::{identification, producer::Control, protocol, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    beacon: &protocol::Beacons::BeaconB,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    if !samples::beacons::beacon_b::equal(beacon.clone()) {
        stop!("BeaconA isn't equal to sample");
    }
    context.inc_stat(identification.uuid(), Alias::BeaconsBeaconB);
    context.check_beacons(identification.uuid(), control).await;
    Ok(())
}
