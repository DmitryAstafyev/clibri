use super::{identification, producer::Control, protocol, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::BeaconA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    if !samples::beacons::beacon_a::equal(beacon.clone()) {
        panic!("BeaconA isn't equal to sample");
    }
    context.inc_stat(identification.uuid(), Alias::BeaconsBeaconA);
    context.check_beacons(identification.uuid(), control).await;
    Ok(())
}
