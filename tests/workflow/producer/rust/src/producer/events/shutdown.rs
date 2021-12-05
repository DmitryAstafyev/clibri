use super::{producer::Control, Context};
use clibri::server;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    context: &mut Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    println!("{}", context.summary());
    Ok(())
}
