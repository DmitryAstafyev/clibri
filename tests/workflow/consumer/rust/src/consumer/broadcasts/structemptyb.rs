use super::{protocol, Consumer, Context};
use clibri::client;
use crate::test::samples;

pub async fn handler<E: client::Error>(
    event: protocol::StructEmptyB,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.broadcast.check();
    if let Err(err) = context.broadcast.valid() {
        panic!("Broadcast error: {}", err);
    }
    if let Err(err) = consumer.beacon_beacona(samples::beacon_a::get()).await {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer.beacon_beacons_beacona(samples::beacons::beacon_a::get()).await {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer.beacon_beacons_beaconb(samples::beacons::beacon_b::get()).await {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer.beacon_beacons_sub_beacona(samples::beacons::sub::beacon_a::get()).await {
        panic!("Beacon sending error: {}", err);
    }
}