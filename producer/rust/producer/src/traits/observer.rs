#[derive(Debug)]
pub enum RequestObserverErrors {
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    BeforeResponseActionFail(String),
    ErrorOnEventsEmit(String),
    GettingConclusionError(String),
}

#[derive(Debug)]
pub enum EventObserverErrors {
    ResponsingError(String),
    GettingResponseError(String),
    EncodingResponseError(String),
    BeforeResponseActionFail(String),
    ErrorOnEventsEmit(String),
    GettingConclusionError(String),
}
