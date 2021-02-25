#[path = "../producer/src/lib.rs"]
pub mod producer;

use producer as Producer;
use std::sync::mpsc::Receiver;
use std::thread::spawn;
use std::sync::{Arc, RwLock};

#[allow(unused)]
impl Producer::ImplUserJoinRequest::ObserverRequestInterface for Producer::ImplUserJoinRequest::ObserverRequest {

    fn conclusion(
        request: Producer::protocol::UserJoin::Request,
        cx: &dyn Producer::consumer_context::Context,
        ucx: Arc<RwLock<Producer::Context>>,
    ) -> Result<Producer::DeclUserJoinRequest::UserJoinConclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }

}

#[allow(non_snake_case)]
mod UserJoin {
    use super::{Producer};
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
        ucx: Arc<RwLock<Producer::Context>>,
    ) -> Result<UserJoinConclusion, String> {
        Ok(UserJoinConclusion::Accept)
    }

    #[allow(unused)]
    pub fn response(
        request: Protocol::UserJoin::Request,
        cx: &dyn Context,
        ucx: Arc<RwLock<Producer::Context>>,
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
        ucx: Arc<RwLock<Producer::Context>>,
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
        ucx: Arc<RwLock<Producer::Context>>,
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
        ucx: Arc<RwLock<Producer::Context>>,
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
        let ucx = Producer::Context {};
        let (mut producer, _receiver): (
            Producer::Producer<Server, ServerConnectionContext>,
            Receiver<Producer::ProducerEvents>,
        ) = Producer::Producer::new(server, None);
        /*
        producer.UserJoin().conclusion(&UserJoin::conclusion);
        producer.UserJoin().broadcast(&UserJoin::broadcast);
        producer.UserJoin().accept(&UserJoin::accept);
        producer.UserJoin().deny(&UserJoin::deny);
        producer.UserJoin().response(&UserJoin::response);
        */
        if let Err(e) = producer.listen(ucx) {
            println!("{}", e);
        }
    });
}
