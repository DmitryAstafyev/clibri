use super::{Consumer, Context};

pub async fn handler<E: std::error::Error>(context: &mut Context, consumer: &mut Consumer<E>) {}
