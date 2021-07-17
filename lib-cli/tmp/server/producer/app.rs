
#[path = "../producer/producer.rs"]
pub mod producer;

use producer::{
    userlogin_request,
    userlogin_request::{
        ObserverRequest as UserLoginRequestObserver,
    },
    users_request::{
        ObserverRequest as UsersRequestObserver,
    },
    message_request,
    message_request::{
        ObserverRequest as MessageRequestObserver,
    },
    messages_request::{
        ObserverRequest as MessagesRequestObserver,
    },
    serverevents_userkickoff,
    serverevents_userkickoff::{
        ObserverEvent as ServerEventsUserKickOffEvent,
    },
    default_connected_event::{
        ObserverEvent as ConnectedEvent,
    },
    default_disconnected_event::{
        ObserverEvent as DisconnectedEvent,
    },
    consumer_identification::Filter,
    protocol as Protocol,
    consumer::Cx,
    Control,
};
use std::sync::{
    Arc,
    RwLock
};
use uuid::Uuid;
use tokio::{
    join,
    runtime::Runtime,
};
use log::{debug, error, info, warn};

#[derive(Clone)]
struct CustomContext {}

impl CustomContext {}

type WrappedCustomContext = Arc<RwLock<CustomContext>>;

#[allow(unused_variables)]
#[allow(non_snake_case)]
impl UserLoginRequestObserver {
    fn conclusion<WrappedCustomContext>(
        request: &Protocol::UserLogin::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<userlogin_request::Conclusion, Protocol::UserLogin::Err> {
        panic!("conclusion method isn't implemented");
    }
    
    fn Accept<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (
            (Filter, Protocol::Events::UserConnected),
            (Filter, Protocol::Events::Message)
        ),
        String
    > {
        Err(String::from("Accept method isn't implemented"))
    }
    
    fn Deny<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::UserLogin::Request,
    ) -> Result<
        (),
        String
    > {
        Err(String::from("Deny method isn't implemented"))
    }

}
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl UsersRequestObserver {
    fn response<WrappedCustomContext>(
        request: &Protocol::Users::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<Protocol::Users::Response, Protocol::Users::Err> {
        panic!("response method isn't implemented");
    }
}
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl MessageRequestObserver {
    fn conclusion<WrappedCustomContext>(
        request: &Protocol::Message::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<message_request::Conclusion, Protocol::Message::Err> {
        panic!("conclusion method isn't implemented");
    }
    
    fn Accept<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::Message::Request,
    ) -> Result<
        (Filter, Protocol::Events::Message),
        String
    > {
        Err(String::from("Accept method isn't implemented"))
    }
    
    fn Deny<WrappedCustomContext>(
        cx: &Cx,
        ucx: WrappedCustomContext,
        request: &Protocol::Message::Request,
    ) -> Result<
        (),
        String
    > {
        Err(String::from("Deny method isn't implemented"))
    }

}
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl MessagesRequestObserver {
    fn response<WrappedCustomContext>(
        request: &Protocol::Messages::Request,
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<Protocol::Messages::Response, Protocol::Messages::Err> {
        panic!("response method isn't implemented");
    }
}

impl ConnectedEvent {
    fn handler<WrappedCustomContext>(
        _uuid: Uuid,
        _ucx: WrappedCustomContext,
        _broadcast: &dyn Fn(Filter, producer::broadcast::Broadcast) -> Result<(), String>,
    ) -> () {
        // Implementation
    }
}

impl DisconnectedEvent {
    fn handler<WrappedCustomContext>(
        uuid: Uuid,
        _ucx: WrappedCustomContext,
        broadcast: &dyn Fn(Filter, producer::broadcast::Broadcast) -> Result<(), String>,
    ) -> () {
        // Implementation
    }
}

impl ServerEventsUserKickOffEvent {
    fn handler<WrappedCustomContext>(
        event: &Protocol::ServerEvents::UserKickOff,
        ucx: WrappedCustomContext,
        control: Control,
    ) -> Option<Vec<(Filter, serverevents_userkickoff::Broadcast)>> {
        // Implementation
        None      
    }
}


#[allow(non_snake_case)]
impl producer::ProducerEventsHolder {

    fn Connected(uuid: Uuid) {
        println!("=========> {} has been connected!", uuid);
    }

}

fn main() {
    let server: Server = Server::new(String::from("127.0.0.1:8080"));
    let ucx = CustomContext {};
    // producer::init_and_start(server, ucx, None);
    let rt  = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            panic!(e);
        },
    };
    rt.block_on( async move {
        let (thread, control) = producer::init(server, ucx);
        let kickoff_task = async move {
            tokio::time::sleep(std::time::Duration::from_millis(20000)).await;
            control.events.KickOffEvent.send(producer::KickOffEvent::Event {
                reason: String::from("Test")
            });
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        };
        join!(
            thread,
            kickoff_task,
        );
    });
}
