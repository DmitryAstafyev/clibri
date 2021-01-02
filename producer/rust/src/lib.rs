#[path = "./request.observer.rs"]
pub mod request_observer;

#[path = "./event.observer.rs"]
pub mod event_observer;

#[path = "./events.holder.rs"]
pub mod events_holder;

#[path = "./context.rs"]
pub mod context;

use request_observer::{ Observer as RequestObserver};
use event_observer::{ EventObserver as EventObserverTrait, Observer as EventObserver, EventObserverErrors};
use events_holder:: { EventsHolder };
use context::*;
use std::cmp::{ PartialEq, Eq };

/*
use std::collections::{ HashMap };
use uuid::Uuid;
*/
pub struct Identification {
    uuid: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserLoginRequest {
    pub login: String,
    pub email: String,
}


#[derive(Debug, Clone)]
pub struct UserLoginResponse {
    error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserLoginConclusion {
    Accept,
    Deny,
}

pub struct UserLoginEvents {
    pub accept: EventObserver<UserLoginRequest, Identification, UserLoginConclusion>,
    pub deny: EventObserver<UserLoginRequest, Identification, UserLoginConclusion>,
}

impl Default for UserLoginEvents {
    fn default() -> Self {
        Self::new()
    }
}

impl UserLoginEvents {

    pub fn new() -> Self {
        UserLoginEvents {
            accept: EventObserver::new(),
            deny: EventObserver::new(),
        }
    }

}

impl EventsHolder<UserLoginRequest, Identification, UserLoginConclusion> for UserLoginEvents {
    fn emit(
        &mut self,
        conclusion: UserLoginConclusion,
        cx: &mut dyn Context<Identification>,
        request: UserLoginRequest,
    ) -> Result<(), EventObserverErrors> {
        match conclusion {
            UserLoginConclusion::Accept => self.accept.emit(conclusion, cx, request),
            UserLoginConclusion::Deny => self.deny.emit(conclusion, cx, request),
        }
    }
}

#[allow(non_snake_case)]
pub struct Producer {

    pub UserLogin: RequestObserver<UserLoginRequest, Identification, UserLoginConclusion, UserLoginEvents>,
    
}

impl Producer {
    pub fn new() -> Result<Self, String> {
        Ok(Producer {
            UserLogin: RequestObserver::new(UserLoginEvents::new())
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
