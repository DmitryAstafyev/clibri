use super::{events, producer::Control, Context, EmitterError};
use clibri::server;

pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::ready::emit(context, control)
        .await
        .map_err(EmitterError::Emitting)
}
