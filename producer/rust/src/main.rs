#[path = "./producer/lib.rs"]
pub mod producer;

use producer as Producer;
use std::sync::mpsc::Receiver;
use std::thread::spawn;

#[derive(Debug, Clone)]
pub struct UserCustomContext {}

#[allow(non_snake_case)]
mod UserJoin {
    use super::{Producer, UserCustomContext};
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};
    use Producer::consumer_context::Context;
    use Producer::consumer_identification::EFilterMatchCondition;
    use Producer::protocol as Protocol;
    use Producer::Broadcasting;
    use Producer::DeclUserJoinRequest::UserJoinConclusion;
    #[allow(unused)]
    pub fn conclusion(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
    ) -> Result<UserJoinConclusion, String> {
        Ok(UserJoinConclusion::Accept)
    }

    #[allow(unused)]
    pub fn response(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        conclusion: UserJoinConclusion,
    ) -> Result<Protocol::UserJoin::Response, String> {
        Ok(Protocol::UserJoin::Response {
            error: None,
            uuid: String::from(""),
        })
    }

    #[allow(unused)]
    pub fn accept(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Ok(())
    }

    #[allow(unused)]
    pub fn deny(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Ok(())
    }

    #[allow(unused)]
    pub fn broadcast(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<UserCustomContext>>,
        broadcast: &dyn Fn(
            HashMap<String, String>,
            EFilterMatchCondition,
            Broadcasting,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        Ok(())
    }
}

fn main() {
    use fiber_transport_server::connection_context::ConnectionContext as ServerConnectionContext;
    use fiber_transport_server::server::Server;
    spawn(move || {
        let server: Server = Server::new(String::from("127.0.0.1:8080"));
        let ucx: UserCustomContext = UserCustomContext {};
        let (mut producer, _receiver): (
            Producer::Producer<Server, ServerConnectionContext, UserCustomContext>,
            Receiver<Producer::ProducerEvents<UserCustomContext>>,
        ) = Producer::Producer::new(server, None);
        producer.UserJoin().conclusion(&UserJoin::conclusion);
        producer.UserJoin().broadcast(&UserJoin::broadcast);
        producer.UserJoin().accept(&UserJoin::accept);
        producer.UserJoin().deny(&UserJoin::deny);
        producer.UserJoin().response(&UserJoin::response);
        if let Err(e) = producer.listen(ucx) {
            println!("{}", e);
        }
    });
}
