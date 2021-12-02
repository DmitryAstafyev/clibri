use super::{protocol, Consumer, Context};
use crate::stat::Alias;
use crate::test::samples;
use clibri::client;

pub async fn handler<E: client::Error>(
    event: protocol::TriggerBeacons,
    context: &mut Context,
    consumer: Consumer<E>,
) {
    context.inc_stat(Alias::TriggerBeacons);
    if let Err(err) = consumer.beacon_beacona(samples::beacon_a::get()).await {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer
        .beacon_beacons_beacona(samples::beacons::beacon_a::get())
        .await
    {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer
        .beacon_beacons_beaconb(samples::beacons::beacon_b::get())
        .await
    {
        panic!("Beacon sending error: {}", err);
    }
    if let Err(err) = consumer
        .beacon_beacons_sub_beacona(samples::beacons::sub::beacon_a::get())
        .await
    {
        panic!("Beacon sending error: {}", err);
    }
}
