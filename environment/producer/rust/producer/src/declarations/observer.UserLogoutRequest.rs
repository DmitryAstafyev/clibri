use super::consumer_context::{ Context };
use super::protocol::{ StructEncode, PackingStruct };
use super::observer::{ RequestObserverErrors };
use super::consumer_identification::Filter;
use super::{ Broadcasting };
use super::Protocol;

#[derive(Debug, Clone)]
pub enum Conclusion {
    Done(Protocol::UserLogout::Done),
}

#[allow(unused_variables)]
pub trait Observer
{

    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::UserLogout::Request,
        cx: &dyn Context,
        ucx: UCX,
        error: &dyn Fn(Protocol::UserLogout::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<Conclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

    fn done<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserLogout::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::UserLogout::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("done method isn't implemented"))
    }

    fn broadcast<UCX: 'static + Sync + Send + Clone>(
        cx: &dyn Context,
        ucx: UCX,
        request: Protocol::UserLogout::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
        error: &dyn Fn(Protocol::UserLogout::Err) -> Result<(), RequestObserverErrors>,
    ) -> Result<(), String> {
        Err(String::from("broadcast method isn't implemented"))
    }

    fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &dyn Context,
        ucx: UCX,
        sequence: u32,
        request: Protocol::UserLogout::Request,
        broadcast: &dyn Fn(Filter, Broadcasting) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        let error = |mut error: Protocol::UserLogout::Err| {
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
                Conclusion::Done(mut response) => {
                    if let Err(e) = Self::done(cx, ucx.clone(), request.clone(), broadcast, &error) {
                        return Err(RequestObserverErrors::ErrorOnEventsEmit(e));
                    }
                    if let Err(e) = Self::broadcast(cx, ucx.clone(), request, broadcast, &error) {
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
