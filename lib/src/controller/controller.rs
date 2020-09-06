use super::{ Request, Response, ErrorResponse };

#[allow(unused_variables)]
pub trait Controller: Send + Sync {

    fn handshake(&mut self, req: &Request, mut response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }

}