use super::{events, identification, producer::Control, Context, EmitterError};
use uuid::Uuid;

pub async fn emit<E: std::error::Error>(
    uuid: Uuid,
    context: &mut Context,
    filter: identification::Filter,
    control: &Control,
) -> Result<(), EmitterError> {
    events::connected::emit::<E>(uuid, context, filter, control)
        .await
        .map_err(EmitterError::Emitting)
}
