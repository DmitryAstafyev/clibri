use super::{broadcast, events, identification, pack, producer::Control, Context, EmitterError};
use clibri::server;
use uuid::Uuid;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::disconnected::emit::<E, C>(identification, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
