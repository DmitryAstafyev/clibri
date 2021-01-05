use super::{ Request, Response, ErrorResponse, connection_context };
use connection_context::ConnectionContext;
use uuid::Uuid;

#[allow(unused_variables)]
pub trait Controller: Send + Sync {

    #[allow(unused_mut)]
    fn handshake(&mut self, req: &Request, mut response: Response) -> Result<Response, ErrorResponse> {
        Ok(response)
    }

    fn error(&mut self, uuid: Uuid, err: String) {

    }

    fn connected(&mut self, uuid: Uuid, cx: ConnectionContext) {

    }

    fn received(&mut self, uuid: Uuid, cx: ConnectionContext, buffer: Vec<u8>) {

    }

    fn disconnected(&mut self, uuid: Uuid, cx: ConnectionContext) {

    }
}