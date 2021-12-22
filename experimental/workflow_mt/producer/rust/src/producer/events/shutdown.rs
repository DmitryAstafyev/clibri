use super::{producer::Control, Context};
use crate::stop;
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    let summary = context.shutdown();
    Ok(())
}
