#[path = "./request.observer.rs"]
pub mod request_observer;

#[path = "./event.observer.rs"]
pub mod event_observer;

#[path = "./events.holder.rs"]
pub mod events_holder;

#[path = "./context.rs"]
pub mod context;

use request_observer::{ Observer as RequestObserver};
use event_observer::{ Observer as EventObserver};
use context::*;

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

pub struct UserLoginEvents {
    pub success: EventObserver<UserLoginRequest, Identification>,
    pub fail: EventObserver<UserLoginRequest, Identification>,
}

pub struct Producer {

    pub UserLogin: RequestObserver<UserLoginRequest, Identification>,
    
}

impl Producer {
    pub fn new() -> Result<Self, String> {
        Ok(Producer {
            UserLogin: UserLogin {
                events:        
            }
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
