use super::{Consumer, Context};

pub async fn handler<E: std::error::Error + Clone>(context: &mut Context, consumer: Consumer<E>) {}
