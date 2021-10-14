use super::{events, producer::Control, Context, EmitterError};
use fiber::server;

pub async fn emit<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), EmitterError> {
    events::ready::emit(context, control)
        .await
        .map_err(EmitterError::Emitting)
}
