use super::{events, hub, identification, producer::Control, Context, EmitterError};
use clibri::server;
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    identification: &identification::Identification,
    filter: hub::filter::Filter,
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::connected::emit::<E, C>(identification, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
