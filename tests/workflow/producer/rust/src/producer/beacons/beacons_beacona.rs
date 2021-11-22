use super::{identification, producer::Control, protocol, Context};
use clibri::server;
use crate::test::samples;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::BeaconA,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    if !samples::beacons::beacon_a::equal(beacon.clone()) {
        panic!("BeaconA isn't equal to sample");
    }
    context.beacons.beacons_beacona(identification.uuid());
    context.beacons.check(identification.uuid(), control).await;
    Ok(())
}