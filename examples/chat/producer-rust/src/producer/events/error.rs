use super::{producer::Control, Context, ProducerError};
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: std::error::Error>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    control: &Control,
) -> Result<(), String> {
    Err(String::from("Event emitter \"error\" isn't implemented"))
}
