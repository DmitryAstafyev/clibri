
use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

type BroadcastEventsUserConnected = (Vec<Uuid>, protocol::Events::UserConnected);
type BroadcastEventsMessage = (Vec<Uuid>, protocol::Events::Message);

pub enum Response {    
    Accept(
    	(
    		protocol::UserLogin::Accepted,
            BroadcastEventsUserConnected,
            BroadcastEventsMessage,
    	)
    ),
    Deny(protocol::UserLogin::Denied),
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::UserLogin::Request,
    control: &Control<E, C>,
) -> Result<Response, protocol::UserLogin::Err> {
    panic!("Handler for protocol::UserLogin::Request isn't implemented");
}
