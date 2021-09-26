use super::{events, identification, producer::Control, Context, EmitterError};

pub async fn emit<E: std::error::Error>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    control: &Control,
) -> Result<(), EmitterError> {
    events::connected::emit::<E>(identification, filter, context, control)
        .await
        .map_err(EmitterError::Emitting)
}
