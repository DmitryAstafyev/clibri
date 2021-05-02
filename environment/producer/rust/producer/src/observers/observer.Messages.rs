use super::consumer::Cx;
use super::consumer_identification::Filter;
use super::observer::RequestObserverErrors;
use super::protocol::PackingStruct;
use super::Protocol;

#[allow(unused_variables)]
pub trait Observer {
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
        request: Protocol::Messages::Request,
        _broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::conclusion(request, cx, ucx) {
            Ok(mut response) => match response.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => {
                    if let Err(e) = cx.send(buffer) {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
            Err(mut error) => match error.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => {
                    if let Err(e) = cx.send(buffer) {
                        Err(RequestObserverErrors::ResponsingError(e))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
        }
    }
}

impl Observer for ObserverRequest {}
