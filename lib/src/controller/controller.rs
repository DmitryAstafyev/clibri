use super::{ Request, Response, ErrorResponse };

#[derive(Debug, Clone)]
pub enum Error {
    Session(String),
}

#[allow(unused_variables)]
pub trait Controller: Send + Sync {

    #[allow(unused_mut)]
    fn handshake(&mut self, req: &Request, mut response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }

    fn error(&mut self, err: Error) {

    }

}