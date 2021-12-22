use super::{producer::Control, Context};
use clibri::server;
use console::style;

#[allow(unused_variables)]
pub async fn emit<E: server::Error, C: server::Control<E> + Send + Clone>(
    context: &Context,
    control: &Control<E, C>,
) -> Result<(), String> {
    println!("{} server is ready", style("[test]").bold().dim(),);
    Ok(())
}
