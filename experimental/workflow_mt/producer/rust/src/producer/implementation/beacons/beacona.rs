use super::{
    beacons, hub, identification, pack, producer::Control, protocol, Context, EmitterError,
    ProducerError,
};
use clibri::server;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    beacon: &protocol::BeaconA,
    sequence: u32,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    beacons::beacona::emit::<E, C>(identification, beacon, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)?;
    let mut response = protocol::InternalServiceGroup::BeaconConfirmation { error: None };
    let buffer = pack(&sequence, &identification.uuid(), &mut response)?;
    control
        .send(buffer, Some(identification.uuid()))
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Emitting(e.to_string()))?;
    Ok(())
}
