
use super::{
    Protocol,
};
pub enum Broadcast {    
    EventsUserDisconnected(Protocol::Events::UserDisconnected),
    EventsMessage(Protocol::Events::Message),
}
