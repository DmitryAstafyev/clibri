use tokio_tungstenite::{
    tungstenite::{
        handshake::server::{
            Request,
            Response,
            ErrorResponse
        }
    }
};

pub trait Handshake {

    fn accept(_req: &Request, response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }

}
