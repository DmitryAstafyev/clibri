use super::context::{ Context };

pub trait Response {

    fn get_buffer(&mut self) -> Vec<u8>;

}

pub type RequestHandler<Request, Identification> = dyn Fn(Request, &mut dyn Context<Identification>) -> Result<Vec<u8>, String>;

pub enum RequestObserverErrors {
    AlreadySubscribed,
    AlreadyUnsubscrided,
    NoConnectionTpResponse,
    ResponsingError(String),
    GettingResponseError(String),
    NoHandlerForRequest,
}

pub trait RequestObserver<Request: Clone, LifeCircle, Identification> {

    fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Identification>) -> Result<(), RequestObserverErrors>;
    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors>;
    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), RequestObserverErrors>;
    fn lifecircle(&self) -> &LifeCircle;

}

pub struct Observer<Request: Clone, LifeCircle, Identification> {
    handler: Option<Box<RequestHandler<Request, Identification>>>,
    circle: LifeCircle,
}

impl<Request: Clone, LifeCircle, Identification> RequestObserver<Request, LifeCircle, Identification> for  Observer<Request, LifeCircle, Identification> {

    fn subscribe(&mut self, hanlder: &'static RequestHandler<Request, Identification>) -> Result<(), RequestObserverErrors> {
        if self.handler.is_some() {
            Err(RequestObserverErrors::AlreadySubscribed)
        } else {
            self.handler = Some(Box::new(hanlder));
            Ok(())    
        }
    }

    fn unsubscribe(&mut self) -> Result<(), RequestObserverErrors> {
        if self.handler.is_none() {
            Err(RequestObserverErrors::AlreadyUnsubscrided)
        } else {
            self.handler = None;
            Ok(())
        }
    }

    fn emit(&mut self, cx: &mut dyn Context<Identification>, request: Request) -> Result<(), RequestObserverErrors> {
        if let Some(handler) = &self.handler {
            match handler(request, cx) {
                Ok(buffer) => {
                    if let Some(conn) = cx.connection() {
                        if let Err(e) = conn.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        }
                    } else {
                        Err(RequestObserverErrors::NoConnectionTpResponse)
                    }
                },
                Err(e) => Err(RequestObserverErrors::GettingResponseError(e)),
            }
        } else {
            Err(RequestObserverErrors::NoHandlerForRequest)
        }
    }

    fn lifecircle(&self) -> &LifeCircle {
        &self.circle
    }

}