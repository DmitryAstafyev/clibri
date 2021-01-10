use super::context::{ Context, Encodable };

pub type BroadcastHandler<Request, Broadcast: Encodable, Identification> =
    dyn (Fn(Request, &mut dyn Context<Identification>) -> Result<(Broadcast, Identification), String>) + Send + Sync;

pub enum BroadcastObserverErrors {
    AlreadySubscribed,
    AlreadyUnsubscrided,
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    NoHandlerForRequest,
}

pub trait BroadcastObserver<Request: Clone, Broadcast: Encodable, Identification> {
    fn subscribe(
        &mut self,
        hanlder: &'static BroadcastHandler<Request, Broadcast, Identification>,
    ) -> Result<(), BroadcastObserverErrors>;
    fn unsubscribe(&mut self) -> Result<(), BroadcastObserverErrors>;
    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), BroadcastObserverErrors>;
}

pub struct Observer<Request: Clone, Broadcast: Encodable, Identification>
{
    handler: Option<Box<BroadcastHandler<Request, Broadcast, Identification>>>,
}

impl<Request: Clone, Broadcast: Encodable, Identification>
    Observer<Request, Broadcast, Identification>
{
    pub fn new() -> Self {
        Observer {
            handler: None,
        }
    }
}

impl<Request: Clone, Broadcast: Encodable, Identification>
    BroadcastObserver<Request, Broadcast, Identification>
    for Observer<Request, Broadcast, Identification>
{
    fn subscribe(
        &mut self,
        hanlder: &'static BroadcastHandler<Request, Broadcast, Identification>,
    ) -> Result<(), BroadcastObserverErrors> {
        if self.handler.is_some() {
            Err(BroadcastObserverErrors::AlreadySubscribed)
        } else {
            self.handler = Some(Box::new(hanlder));
            Ok(())
        }
    }

    fn unsubscribe(&mut self) -> Result<(), BroadcastObserverErrors> {
        if self.handler.is_none() {
            Err(BroadcastObserverErrors::AlreadyUnsubscrided)
        } else {
            self.handler = None;
            Ok(())
        }
    }

    fn emit(
        &mut self,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), BroadcastObserverErrors> {
        if let Some(handler) = &self.handler {
            match handler(request, cx) {
                Ok((mut broadcast, identification)) => {
                    match broadcast.abduct() {
                        Ok(buffer) => if let Err(e) = cx.send_to(identification, buffer) {
                            Err(BroadcastObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        },
                        Err(e) => Err(BroadcastObserverErrors::EncodingResponseError(e)),
                    }
                }
                Err(e) => Err(BroadcastObserverErrors::GettingResponseError(e)),
            }
        } else {
            Err(BroadcastObserverErrors::NoHandlerForRequest)
        }
    }
}
