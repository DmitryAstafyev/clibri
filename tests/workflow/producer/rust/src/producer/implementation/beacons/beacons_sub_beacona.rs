use super::{
    beacons, identification, pack, producer::Control, protocol, Context, EmitterError,
    ProducerError,
};
use clibri::server;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    beacon: &protocol::Beacons::Sub::BeaconA,
    sequence: u32,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    beacons::beacons_sub_beacona::emit::<E, C>(identification, beacon, filter, context, control)
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