use super::consumer_context::{ Context };
use super::protocol::{ PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::Protocol;

#[derive(Debug, Clone)]
pub enum Conclusion {
    Response(Protocol::Messages::Response),
}

#[allow(unused_variables)]
pub trait Observer
{

    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::Messages::Request,
        cx: &dyn Context,
        ucx: UCX,
    ) -> Result<Conclusion, Protocol::Messages::Err> {
        panic!("conclusion method isn't implemented");
    }

    fn Response<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::Messages::Request,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &dyn Context,
        ucx: UCX,
        sequence: u32,
        request: Protocol::Messages::Request,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
            Ok(conclusion) => match conclusion {
                Conclusion::Response(mut response) => {
                    match Self::Response(cx, ucx.clone(), request.clone()) {
                        Ok(_) => {
                            match response.pack(sequence) {
                                Ok(buffer) => if let Err(e) = cx.send(buffer) {
                                    Err(RequestObserverErrors::ResponsingError(e))
                                } else {
                                    Ok(())
                                },
                                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                            }
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
