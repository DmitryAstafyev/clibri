use super::{hub, identification, producer::Control, protocol, Context};
use crate::{stat::Alias, stop, test::samples};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    beacon: &protocol::Beacons::BeaconA,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    if !samples::beacons::beacon_a::equal(beacon.clone()) {
        stop!("BeaconA isn't equal to sample");
    }
    context
        .inc_stat(identification.uuid(), Alias::BeaconsBeaconA)
        .await;
    if context.is_all_beacons_gotten(identification.uuid()).await {
        if let Err(err) = control
            .events
            .finishconsumertest(protocol::FinishConsumerTest {
                uuid: identification.uuid().to_string(),
            })
            .await
        {
            stop!("Sending FinishConsumerTest error: {}", err);
        }
    }
    Ok(())
}
