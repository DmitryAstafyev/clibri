use super::{identification, producer::Control, protocol, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::Sub::BeaconA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    if !samples::beacons::sub::beacon_a::equal(beacon.clone()) {
        stop!("BeaconA isn't equal to sample");
    }
    context.inc_stat(identification.uuid(), Alias::BeaconsSubBeaconA);
    context.check_beacons(identification.uuid(), control).await;
    Ok(())
}
