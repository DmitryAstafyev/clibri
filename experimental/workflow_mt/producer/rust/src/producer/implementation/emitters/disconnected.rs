use super::{
    broadcast, events, hub, identification, pack, producer::Control, Context, EmitterError,
};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::disconnected::emit::<E, C>(identification, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
