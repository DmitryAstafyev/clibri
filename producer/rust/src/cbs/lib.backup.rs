use std::collections::{ HashMap };
use uuid::Uuid;

pub struct Identification {
    uuid: Option<String>,
    id: Option<u64>,
}

pub trait Connection {
    
    fn send(&mut self) -> Result<(), String> {
        Ok(())
    }

}

pub trait Context {

    fn connection(&mut self) -> Option<&'static mut dyn Connection> {
        None
    }

    fn connections(&mut self, ident: Identification) -> Option<Vec<&'static mut dyn Connection>> {
        None
    }

}

pub struct CX {

}

impl Context for CX {

}

pub trait RequestObserver<Request, Response, LifeCircle> {

    fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Response>) -> Result<(), String>;
    fn unsubscribe() -> Result<(), String>;
    fn emit(&mut self, cx: &mut dyn Context, request: Request) -> Result<(), String>;
    fn lifecircle() -> LifeCircle;

}

type RequestHandler<Request, Response> = dyn Fn(Request, &mut dyn Context) -> Response;
type RequestLifeCirclyHandler<Request> = dyn Fn(Request);

pub struct Observer<Request: Copy, Response: Copy, C, L> where C: Context + 'static {
    handler: Option<Box<RequestHandler<Request, Response, C>>>,
    lifecirle: L,
}

impl<Request: Copy, Response: Copy, C, L> Observer<Request, Response, C, L>  where C: Context {

    pub fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Response, C>) -> Result<String, String> {
        self.handler = Some(Box::new(hanlder));
        Ok("".to_string())
    }

    pub fn unsubscribe() -> Result<(), String> {
        Ok(())
    }

    pub fn emit(&mut self, cx: &mut C, request: Request) -> Result<(), String> {
        if let Some(handler) = &self.handler {
            let response = handler(request, cx);
            if let Some(conn) = cx.connection() {
                if let Err(e) = conn.send(/*response*/) {
                    println!("{}", e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AuthUserRequest {

}

pub enum AuthUserRequestLifeCircleEvents {
    Notify,
    Register,
}

pub struct AuthUserRequestLifeCircleNotify {
    handlers: HashMap<String, Box<RequestLifeCirclyHandler<AuthUserRequest>>>
}

impl AuthUserRequestLifeCircleNotify {
    pub fn subscribe(&mut self, hanlder: &'static RequestLifeCirclyHandler<AuthUserRequest>) -> Result<String, String> {
        self.handlers.insert("".to_string(), Box::new(hanlder));
        Ok("".to_string())
    }

    pub fn unsubscribe() -> Result<(), String> {
        Ok(())
    }

    pub fn emit(&mut self, request: AuthUserRequest) -> Result<(), String> {
        Ok(())
    }
}

pub struct AuthUserRequestLifeCircleRegister {
    handlers: HashMap<String, Box<RequestLifeCirclyHandler<AuthUserRequest>>>
}

impl AuthUserRequestLifeCircleRegister {
    pub fn subscribe(&mut self, hanlder: &'static RequestLifeCirclyHandler<AuthUserRequest>) -> Result<String, String> {
        self.handlers.insert("".to_string(), Box::new(hanlder));
        Ok("".to_string())
    }

    pub fn unsubscribe() -> Result<(), String> {
        Ok(())
    }

    pub fn emit(&mut self, request: AuthUserRequest) -> Result<(), String> {
        Ok(())
    }
}


pub struct AuthUserRequestLifeCircle {
    notify: AuthUserRequestLifeCircleNotify,
    register: AuthUserRequestLifeCircleRegister,
}

#[derive(Debug, Copy, Clone)]
pub struct AuthUserResponse {

}

#[derive(Debug, Copy, Clone)]
pub struct NewUserOnline {

}

pub struct Producer {
    // Listening incoming requests
    pub AuthUserRequest: Observer<AuthUserRequest, AuthUserResponse, CX, AuthUserRequestLifeCircle>,

}

impl Producer {

    pub fn new() -> Result<Self, String> {
        Producer {
            AuthUserRequest: Observer {

            }
        }
    }

    // Broadcasting
    pub fn NewUserOnline(ident: Identification, event: NewUserOnline) -> Result<(), String> {
        Ok(())
    }

}


#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        
        assert_eq!(true, false);
    }
}
