#[path = "../producer/src/lib.rs"]
pub mod producer;

use fiber_transport_server::server::Server;
use fiber_transport_server::server::*;
use fiber::server::server::Server as ServerTrait;
use fiber::server::server::*;
use producer::*;
use std::sync::mpsc::Receiver;
use std::thread::spawn;
use std::sync::{Arc, Mutex, RwLock};
use producer::UserSingInObserver::{Observer as UserSingInObserver, ObserverRequest as UserSingInObserverRequest};
use producer::UserJoinObserver::{Observer as UserJoinObserver, ObserverRequest as UserJoinObserverRequest};

#[derive(Clone)]
struct CustomContext {}

impl CustomContext {}

struct ProducerInstance {}

// impl<Server: 'static + ServerTrait, CustomContext: 'static + Clone + Sync + Send> Producer<Server, CustomContext> for ProducerInstance {}
impl Producer<Server, Arc<RwLock<CustomContext>>> for ProducerInstance {

}
/*
impl UserSingInObserver<Arc<RwLock<CustomContext>>>
    for UserSingInObserverRequest
{
}
*/

impl UserJoinObserver<Arc<RwLock<CustomContext>>>
    for UserJoinObserverRequest
{
    fn conclusion(
        request: producer::protocol::UserJoin::Request,
        cx: &dyn producer::consumer_context::Context,
        ucx: Arc<RwLock<CustomContext>>,
    ) -> Result<producer::UserJoinObserver::Conclusion, String> {
        Err(String::from("conclusion method isn't implemented"))
    }
}

fn main() {
    spawn(move || {
        let server: Server = Server::new(String::from("127.0.0.1:8080"));
        let ucx = CustomContext {};
        // let mut producer: ProducerInstance = ProducerInstance {};
        let feedback = match ProducerInstance::listen(server, Arc::new(RwLock::new(ucx)), None) {
            Ok(feedback) => feedback,
            Err(e) => panic!(e),
        };
    });
}
