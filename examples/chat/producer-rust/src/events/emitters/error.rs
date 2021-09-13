use super::EmitterError;
use crate::{events, producer::Control, Context, ProducerError};
use uuid::Uuid;

pub async fn emit(
    error: ProducerError,
    uuid: Option<Uuid>,
    context: &mut Context,
    control: &Control,
) -> Result<(), EmitterError> {
    events::error::emit(error, uuid, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
