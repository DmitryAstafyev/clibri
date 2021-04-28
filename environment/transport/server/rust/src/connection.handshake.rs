use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};

pub trait Handshake {
    fn accept(_req: &Request, response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }
}
