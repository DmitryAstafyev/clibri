use super::{
    beacons, identification, pack, producer::Control, protocol, Context, EmitterError,
    ProducerError, scope::Scope,
};
use clibri::server;

pub async fn emit<E: server::Error, C: server::Control<E>>(
    identification: &identification::Identification,
    beacon: &protocol::BeaconA,
    sequence: u32,
    filter: &identification::Filter<'_>,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    let mut scope: Scope<'_, E, C> = Scope::new(context, control, identification, filter);
    beacons::beacona::emit(beacon, &mut scope)
        .await
        .map_err(EmitterError::Emitting)?;
    let mut response = protocol::InternalServiceGroup::BeaconConfirmation { error: None };
    let buffer = pack(&sequence, &identification.uuid(), &mut response)?;
    control
        .send(buffer, Some(identification.uuid()))
        .await
        .map_err(|e: ProducerError<E>| EmitterError::Emitting(e.to_string()))?;
    scope.call().await;
    Ok(())
}