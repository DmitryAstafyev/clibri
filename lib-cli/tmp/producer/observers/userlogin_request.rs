
use super::consumer::Cx;
use super::consumer_identification::Filter;
use super::RequestObserverErrors;
use super::protocol::PackingStruct;
use super::Protocol;


#[derive(Debug, Clone)]
pub enum Conclusion {
    Accept(Protocol::UserLogin::Accepted),
    Deny(Protocol::UserLogin::Denied)
}
    

#[allow(unused_variables)]
pub trait Observer {
    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: &Protocol::UserLogin::Request,
        cx: &Cx,
        ucx: UCX,
    ) -> Result<Conclusion, Protocol::UserLogin::Err> {
        panic!("conclusion method isn't implemented");
    }
    
    fn Accept<UCX: 'static + Sync + Send + Clone>(
        cx: &Cx,
        ucx: UCX,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (
            (Filter, Protocol::Events::UserConnected),
            (Filter, Protocol::Events::Message)
        ),
        String
    > {
        Err(String::from("Accept method isn't implemented"))
    }
    
    fn Deny<UCX: 'static + Sync + Send + Clone>(
        cx: &Cx,
        ucx: UCX,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (),
        String
    > {
        Err(String::from("Deny method isn't implemented"))
    }

}

#[derive(Clone)]
pub struct ObserverRequest {}

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }

    pub fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &Cx,
        ucx: UCX,
        sequence: u32,
        request: Protocol::UserLogin::Request,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(&request, cx, ucx.clone()) {            
            Ok(conclusion) => match conclusion {    
                Conclusion::Accept(mut response) => match Self::Accept(cx, ucx.clone(), &request) {
                    Ok((
                        events_userconnected,
                        events_message
                    )) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                        Ok(buffer) => if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {            
                            let (filter, mut msg) = events_userconnected;
                            match msg.pack(0, Some(cx.uuid().to_string())) {
                                Ok(buffer) => if let Err(e) = broadcast(filter, buffer) {
                                    return Err(RequestObserverErrors::BroadcastingError(e));
                                }
                                Err(e) => {
                                    return Err(RequestObserverErrors::EncodingResponseError(e));
                                }
                            };
                            let (filter, mut msg) = events_message;
                            match msg.pack(0, Some(cx.uuid().to_string())) {
                                Ok(buffer) => if let Err(e) = broadcast(filter, buffer) {
                                    return Err(RequestObserverErrors::BroadcastingError(e));
                                }
                                Err(e) => {
                                    return Err(RequestObserverErrors::EncodingResponseError(e));
                                }
                            };
                            Ok(())
                        }
                        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                    }
                    Err(error) => Err(RequestObserverErrors::AfterConclusionError(error)),
                }
                Conclusion::Deny(mut response) => match Self::Deny(cx, ucx.clone(), &request) {
                    Ok(_) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                        Ok(buffer) => if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        }
                        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                    }
                    Err(error) => Err(RequestObserverErrors::AfterConclusionError(error)),
                }
            }
            Err(mut error) => match error.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => if let Err(e) = cx.send(buffer) {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
        }
    }
}

impl Observer for ObserverRequest {}
    