use super::{Consumer, ConsumerError, Context};

pub async fn handler<E: std::error::Error + Clone>(
    error: Option<ConsumerError<E>>,
    context: &mut Context,
    consumer: Consumer<E>,
) {
}
