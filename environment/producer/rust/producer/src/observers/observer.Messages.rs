use super::consumer::{ Cx };
use super::protocol::{ PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::Protocol;
use futures::{
    Future,
};
#[allow(unused_variables)]
pub trait Observer
{

    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::Messages::Request,
        cx: &Cx,
        ucx: UCX,
    ) -> Result<Protocol::Messages::Response, Protocol::Messages::Err> {
        panic!("conclusion method isn't implemented");
    }

    fn Response<UCX: 'static + Sync + Send + Clone>(
        cx: &Cx,
        ucx: UCX,
        request: Protocol::Messages::Request,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

}

#[derive(Clone)]
pub struct ObserverRequest { }

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }

    pub async fn emit<UCX: 'static + Sync + Send + Clone, F: Future<Output = Result<(), String>>>(
        &self,
        cx: &Cx,
        ucx: UCX,
        sequence: u32,
        request: Protocol::Messages::Request,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> F,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
            Ok(mut response) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => if let Err(e) = cx.send(buffer).await {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                },
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
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

