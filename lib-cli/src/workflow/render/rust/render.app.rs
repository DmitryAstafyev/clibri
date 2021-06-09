use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        store::{
            Store as WorkflowStore
        },
        request::Request,
    },
    Protocol,
};
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    }
};

mod templates {
    pub const MODULE: &str = r#"
#[path = "../producer/producer.rs"]
pub mod producer;

use fiber::{
    logger::{
        LogLevel,
    },
};

use producer::{
[[requests_imports]]
[[events_imports]]
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

#[allow(non_upper_case_globals)]
pub mod tools {
    use fiber::logger::DefaultLogger;

    lazy_static! {
        pub static ref logger: DefaultLogger = DefaultLogger::new("Producer".to_owned(), Some(5));
    }
}

#[derive(Clone)]
struct CustomContext {}

impl CustomContext {}

type WrappedCustomContext = Arc<RwLock<CustomContext>>;

[[requests]]

impl ConnectedEvent {
    fn handler<WrappedCustomContext>(
        _uuid: Uuid,
        _ucx: WrappedCustomContext,
        _broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> () {
        // Implementation
    }
}

impl DisconnectedEvent {
    fn handler<WrappedCustomContext>(
        uuid: Uuid,
        _ucx: WrappedCustomContext,
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> () {
        // Implementation
    }
}

[[events]]

#[allow(non_snake_case)]
impl producer::ProducerEventsHolder {

    fn Connected(uuid: Uuid) {
        println!("=========> {} has been connected!", uuid);
    }

}

fn main() {
    match fiber::tools::LOGGER_SETTINGS.lock() {
        Ok(mut settings) => settings.set_level(LogLevel::Verb),
        Err(e) => println!("Fail set log level due error: {}", e),
    };
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
"#;
    pub const REQUEST_IMPL: &str = 
r#"#[allow(unused_variables)]
#[allow(non_snake_case)]
impl [[struct_name]]Observer {
    fn [[conclutions_caller_name]]<WrappedCustomContext>(
        request: &Protocol::[[request_ref]],
        cx: &Cx,
        ucx: WrappedCustomContext,
    ) -> Result<[[conclutions_caller_ok]], Protocol::[[request_err_ref]]> {
        panic!("[[conclutions_caller_name]] method isn't implemented");
    }[[conclutions_methods]]
}"#;
    pub const REQUEST_IMPORT: &str =
r#"[[module]]::{
    ObserverRequest as [[struct_name]]Observer,
},"#;
    pub const EVENT_IMPORT: &str =
r#"[[module]]::{
    ObserverEvent as [[struct_name]]Event,
},"#;
    pub const EVENT_HANDLER: &str =
r#"impl [[struct_name]]Event {
    fn handler<WrappedCustomContext>(
        event: [[reference]],
        ucx: WrappedCustomContext,
        control: Control,
    ) -> Option<Vec<(Filter, [[struct_name]]Event::Broadcast)>> {
        // Implementation       
    }
}"#;
    pub const CONCLUSION_METHOD: &str = r#"
fn [[name]]<WrappedCustomContext>(
    cx: &Cx,
    ucx: WrappedCustomContext,
    request: &Protocol::[[request_ref]],
) -> Result<
    [[broadcast]],
    String
> {
    Err(String::from("[[name]] method isn't implemented"))
}"#;
}

pub struct RenderApp {
}

impl Default for RenderApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderApp {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        store: &WorkflowStore,
        _protocol: &Protocol,
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[requests_imports]]", &tools::inject_tabs(1, self.requests_imports(store)?));
        output = output.replace("[[events_imports]]", &tools::inject_tabs(1, self.events_imports(store)?));
        output = output.replace("[[requests]]", &self.requests(store)?);
        output = output.replace("[[events]]", &self.events(store)?);
        helpers::fs::write(dest, output, true)
    }

    fn requests_imports(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::REQUEST_IMPORT
                    .replace("[[module]]", &request.as_mod_name()?)
                    .replace("[[struct_name]]", &request.as_struct_name()?)
            );
            if pos < store.requests.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn requests(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, request) in store.requests.iter().enumerate() {
            let mut out: String = templates::REQUEST_IMPL.to_owned();
            if request.actions.len() > 1 {
                out = out.replace("[[conclutions_methods]]", &format!("\n{}\n", self.get_conclusion_methods(request)?));
                out = out.replace("[[conclutions_caller_name]]", "conclusion");
                out = out.replace("[[conclutions_caller_ok]]", &format!("{}Observer::Conclusion", &request.as_struct_name()?));
            } else {
                out = out.replace("[[conclutions_methods]]", "");
                out = out.replace("[[conclutions_caller_name]]", "response");
                out = out.replace("[[conclutions_caller_ok]]", &format!("Protocol::{}", self.into_rust_path(&request.get_response()?)));
            }
            out = out.replace("[[request_ref]]", &self.into_rust_path(&request.get_request()?));
            out = out.replace("[[struct_name]]", &request.as_struct_name()?);
            if let Some(error_ref) = request.error.as_ref() {
                out = out.replace("[[request_err_ref]]", &self.into_rust_path(error_ref));
            }
            if pos < store.requests.len() - 1 {
                out = format!("{}\n", out);
            }
            output = format!("{}{}", output, out);
        }
        Ok(output)
    }

    fn events_imports(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            output = format!("{}{}",
                output,
                templates::EVENT_IMPORT
                    .replace("[[module]]", &event.as_mod_name()?)
                    .replace("[[struct_name]]", &event.as_struct_name()?)
            );
            if pos < store.events.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(output)
    }

    fn events(&self, store: &WorkflowStore) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, event) in store.events.iter().enumerate() {
            let mut out: String = templates::EVENT_HANDLER.to_owned();
            out = out.replace("[[reference]]", &self.into_rust_path(&event.get_reference()?));
            out = out.replace("[[struct_name]]", &event.as_struct_name()?);
            if pos < store.requests.len() - 1 {
                out = format!("{}\n", out);
            }
            output = format!("{}{}", output, out);
        }
        Ok(output)
    }

    fn get_conclusion_methods(&self, request: &Request) -> Result<String, String> {
        let mut output: String = String::new();
        for (pos, action) in request.actions.iter().enumerate() {
            let mut conclusion: String = templates::CONCLUSION_METHOD.to_owned();
            if let Some(name) = action.conclusion.as_ref() {
                conclusion = conclusion.replace("[[name]]", name);
            } else {
                return Err(String::from("Action doesn't have bound conclusion name"));
            }
            if let Some(request_ref) = request.request.as_ref() {
                conclusion = conclusion.replace("[[request_ref]]", &self.into_rust_path(request_ref));
            } else {
                return Err(String::from("Fail to find reference to object/struct of request"));
            }
            let mut broadcasts: String = if action.broadcast.is_empty() {
                String::from("()")
            } else {
                String::new()
            };
            let tabs = if action.broadcast.len() == 1 {
                tools::tabs(0)
            } else {
                tools::tabs(2)
            };
            for (pos, broadcast) in action.broadcast.iter().enumerate() {
                if broadcast.optional {
                    broadcasts = format!("{}{}Option<(Filter, Protocol::{})>", broadcasts, tabs, self.into_rust_path(&(broadcast.reference)));
                } else {
                    broadcasts = format!("{}{}(Filter, Protocol::{})", broadcasts, tabs, self.into_rust_path(&(broadcast.reference)));
                }
                if pos < action.broadcast.len() - 1 {
                    broadcasts = format!("{},\n", broadcasts);
                }
            }
            if action.broadcast.len() > 1 {
                broadcasts = format!("(\n{}\n{})", broadcasts, tools::tabs(1));
            }
            conclusion = conclusion.replace("[[broadcast]]", &broadcasts);
            output = format!("{}{}", output, conclusion);
            if pos < request.actions.len() - 1 {
                output = format!("{}\n", output);
            }
        }
        Ok(tools::inject_tabs(1, output))
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        Ok(base.join("app.rs"))
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }

}

