use super::consumer::{ Cx };
use super::protocol::{ PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::Protocol;
use futures::{
    Future,
};
#[derive(Debug, Clone)]
pub enum Conclusion {
    Accept(Protocol::Message::Accepted),
    Deny(Protocol::Message::Denied),
}

pub struct AcceptBroadcasting {
    pub Message: (Filter, Protocol::Events::Message),
}

#[allow(unused_variables)]
pub trait Observer
{

    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::Message::Request,
        cx: &Cx,
        ucx: UCX,
    ) -> Result<Conclusion, Protocol::Message::Err> {
        panic!("conclusion method isn't implemented")
    }

    fn Accept<UCX: 'static + Sync + Send + Clone>(
        cx: &Cx,
        ucx: UCX,
        request: Protocol::Message::Request,
    ) -> Result<AcceptBroadcasting, String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn Deny<UCX: 'static + Sync + Send + Clone>(
        cx: &Cx,
        ucx: UCX,
        request: Protocol::Message::Request,
    ) -> Result<(), String> {
        Err(String::from("deny method isn't implemented"))
    }

}

#[derive(Clone)]
pub struct ObserverRequest {}

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }

    pub async fn emit<UCX: 'static + Sync + Send + Clone, F: Future<Output = Result<(), String>>>(
        &self,
        cx: &Cx,
        ucx: UCX,
        sequence: u32,
        request: Protocol::Message::Request,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> F,
    ) -> Result<(), RequestObserverErrors>  {
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
            Ok(conclusion) => match conclusion {
                Conclusion::Accept(mut response) => {
                    match Self::Accept(cx, ucx.clone(), request.clone()) {
                        Ok(mut msgs) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                            Ok(buffer) => if let Err(e) = cx.send(buffer).await {
                                Err(RequestObserverErrors::ResponsingError(e))
                            } else {
                                match msgs.Message.1.pack(0, Some(cx.uuid().to_string())) {
                                    Ok(buffer) => if let Err(e) = broadcast(msgs.Message.0, buffer).await {
                                        return Err(RequestObserverErrors::BroadcastingError(e));
                                    },
                                    Err(e) => {
                                        return Err(RequestObserverErrors::EncodingResponseError(e));
                                    },
                                };
                                Ok(())
                            },
                            Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                        },
                        Err(error) => Err(RequestObserverErrors::AfterConclusionError(error))
                    }
                   
                },
                Conclusion::Deny(mut response) => {
                    match Self::Deny(cx, ucx, request) {
                        Ok(_) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                            Ok(buffer) => if let Err(e) = cx.send(buffer).await {
                                Err(RequestObserverErrors::ResponsingError(e))
                            } else {
                                Ok(())
                            },
                            Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                        },
                        Err(error) => Err(RequestObserverErrors::AfterConclusionError(error))
                    }
                },
            },
            Err(mut error) => {
                match error.pack(sequence, Some(cx.uuid().to_string())) {
                    Ok(buffer) => if let Err(e) = cx.send(buffer).await {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else {
                        Ok(())
                    },
                    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                }
            }
        }
    }
}

impl Observer for ObserverRequest { }
