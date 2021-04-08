use super::consumer_context::{ Context };
use super::protocol::{ PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::Protocol;

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
        cx: &dyn Context,
        ucx: UCX,
    ) -> Result<Conclusion, Protocol::Message::Err> {
        panic!("conclusion method isn't implemented")
    }

    fn Accept<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::Message::Request,
    ) -> Result<AcceptBroadcasting, String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn Deny<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::Message::Request,
    ) -> Result<(), String> {
        Err(String::from("deny method isn't implemented"))
    }

    fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &dyn Context,
        ucx: UCX,
        sequence: u32,
        request: Protocol::Message::Request,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
            Ok(conclusion) => match conclusion {
                Conclusion::Accept(mut response) => {
                    match Self::Accept(cx, ucx.clone(), request.clone()) {
                        Ok(mut msgs) => match response.pack(sequence) {
                            Ok(buffer) => if let Err(e) = cx.send(buffer) {
                                Err(RequestObserverErrors::ResponsingError(e))
                            } else {
                                match msgs.Message.1.pack(0) {
                                    Ok(buffer) => if let Err(e) = broadcast(msgs.Message.0, buffer) {
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
                        Ok(_) => match response.pack(sequence) {
                            Ok(buffer) => if let Err(e) = cx.send(buffer) {
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
                match error.pack(sequence) {
                    Ok(buffer) => if let Err(e) = cx.send(buffer) {
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

#[derive(Clone)]
pub struct ObserverRequest { }

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }
}
