use super::{identification, producer::Control, Context, ProducerError};
use crate::{stat::Alias, stop};
use clibri::server;
use uuid::Uuid;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E>>(
    error: ProducerError<E>,
    uuid: Option<Uuid>,
    context: &mut Context,
    identification: Option<&identification::Identification>,
    control: &Control<E, C>,
) -> Result<(), String> {
    if let Some(uuid) = uuid {
        context.inc_stat(uuid, Alias::Error);
        stop!("Consumer error: {:?}", error);
    } else {
        stop!("Producer error: {:?}", error);
    }
}
