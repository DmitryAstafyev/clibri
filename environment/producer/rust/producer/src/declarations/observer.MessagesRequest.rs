use super::consumer_context::{ Context };
use super::protocol::{ PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::{ Broadcasting };
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
        error: &dyn Fn(Protocol::Messages::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<Conclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

    fn response<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::Messages::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::Messages::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("accept method isn't implemented"))
    }

    fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &dyn Context,
        ucx: UCX,
        sequence: u32,
        request: Protocol::Messages::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        let error = |mut error: Protocol::Messages::Err| {
            match error.pack(sequence) {
                Ok(buffer) => if let Err(e) = cx.send(buffer) {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                },
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            }
        };
        match Self::conclusion(request.clone(), cx, ucx.clone(), &error) {
            Ok(conclusion) => match conclusion {
                Conclusion::Response(mut response) => {
                    if let Err(e) = Self::response(cx, ucx.clone(), request.clone(), broadcast, &error) {
                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                    }
                    match response.pack(sequence) {
                        Ok(buffer) => if let Err(e) = cx.send(buffer) {
                            Err(RequestObserverErrors::ResponsingError(e))
                        } else {
                            Ok(())
                        },
                        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
                    }
                },
            },
            Err(e) => Err(RequestObserverErrors::GettingConclusionError(e))
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
