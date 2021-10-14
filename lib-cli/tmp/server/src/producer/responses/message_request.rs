
use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {    
    Accept(
    	(
    		protocol::Message::Accepted,
            BroadcastEventsMessage,
    	)
    ),
    Deny(protocol::Message::Denied),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::Message::Request,
    control: &Control<E, C>,
) -> Result<Response, protocol::Message::Err> {
    panic!("Handler for protocol::Message::Request isn't implemented");
}
