#[path = "./request.observer.rs"]
pub mod observer;

#[path = "./context.rs"]
pub mod context;

use observer::*;
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

pub struct LifeCircle {}

pub struct Producer {

    pub UserLogin: Observer<UserLoginRequest, LifeCircle, Identification>,
    
}







#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        
        assert_eq!(true, false);
    }
}
