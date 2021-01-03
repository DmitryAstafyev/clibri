#[path = "./request.observer.rs"]
pub mod request_observer;

#[path = "./event.observer.rs"]
pub mod event_observer;

#[path = "./broadcast.observer.rs"]
pub mod broadcast_observer;

#[path = "./events.holder.rs"]
pub mod events_holder;

#[path = "./context.rs"]
pub mod context;

use request_observer::{ Observer as RequestObserver};
use broadcast_observer::{ BroadcastObserver as BroadcastObserverTrait, Observer as BroadcastObserver };
use event_observer::{ EventObserver as EventObserverTrait, Observer as EventObserver, EventObserverErrors};
use events_holder:: { EventsHolder };
use context::*;
use std::cmp::{ PartialEq, Eq };

/*
use std::collections::{ HashMap };
use uuid::Uuid;
*/
pub struct Identification {
    pub uuid: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserSingInRequest {
    pub login: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct UserSingInBroadcast {
    login: String,
}

impl Encodable for UserSingInBroadcast {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct UserSingInResponse {
    error: Option<String>,
}

impl Encodable for UserSingInResponse {
    fn abduct(&mut self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserSingInConclusion {
    Accept,
    Deny,
}

pub struct UserSingInEvents {
    pub accept: EventObserver<UserSingInRequest, Identification, UserSingInConclusion>,
    pub broadcast: BroadcastObserver<UserSingInRequest, UserSingInBroadcast, Identification>,
    pub deny: EventObserver<UserSingInRequest, Identification, UserSingInConclusion>,
}

impl Default for UserSingInEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl UserSingInEvents {

    pub fn new() -> Self {
        UserSingInEvents {
            accept: EventObserver::new(),
            broadcast: BroadcastObserver::new(),
            deny: EventObserver::new(),
        }
    }

}

impl EventsHolder<UserSingInRequest, Identification, UserSingInConclusion> for UserSingInEvents {
    fn emit(
        &mut self,
        conclusion: UserSingInConclusion,
        cx: &mut dyn Context<Identification>,
        request: UserSingInRequest,
    ) -> Result<(), EventObserverErrors> {
        match conclusion {
            UserSingInConclusion::Accept => {
                if let Err(e) = self.accept.emit(conclusion, cx, request.clone()) {
                    return Err(e);
                }
                if let Err(e) = self.broadcast.emit(cx, request) {
                    return Err(EventObserverErrors::ErrorOnBroadcasting(e));
                }
                Ok(())
            },
            UserSingInConclusion::Deny => self.deny.emit(conclusion, cx, request),
        }
    }
}

#[allow(non_snake_case)]
pub struct Producer {

    pub UserSingIn: RequestObserver<UserSingInRequest, UserSingInResponse, Identification, UserSingInConclusion, UserSingInEvents>,
    
}

impl Producer {
    pub fn new() -> Result<Self, String> {
        Ok(Producer {
            UserSingIn: RequestObserver::new(UserSingInEvents::new())
        })
    }
}






#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        
        assert_eq!(true, false);
    }
}
