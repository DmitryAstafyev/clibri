use super::consumer::Cx;
use super::consumer_identification::Filter;
use super::observer::RequestObserverErrors;
use super::protocol::PackingStruct;
use super::Protocol;

#[allow(unused_variables)]
pub trait Observer {
    fn conclusion<UCX: 'static + Sync + Send + Clone>(
        request: Protocol::Users::Request,
        cx: &Cx,
        ucx: UCX,
    ) -> Result<Protocol::Users::Response, Protocol::Users::Err> {
        panic!("conclusion method isn't implemented");
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
        request: Protocol::Users::Request,
        _broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        /*
        let error = |mut error: Protocol::Users::Err| {
            match error.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => if let Err(e) = cx.send(buffer).await {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                },
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            }
        };
        */
        match Self::conclusion(request.clone(), cx, ucx.clone()) {
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
