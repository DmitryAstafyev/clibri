use super::{
    helpers,
    helpers::{
        render as tools,
    },
    workflow::{
        request::{
            Request
        }
    }
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
use super::consumer::Cx;
use super::consumer_identification::Filter;
use super::RequestObserverErrors;
use super::protocol::PackingStruct;
use super::Protocol;
[[conclutions_enum]]
#[allow(unused_variables)]
pub trait Observer {
    fn [[conclutions_caller_name]]<UCX: 'static + Sync + Send + Clone>(
        request: &Protocol::[[request_ref]],
        cx: &Cx,
        ucx: UCX,
    ) -> Result<[[conclutions_caller_ok]], Protocol::[[request_err_ref]]> {
        panic!("[[conclutions_caller_name]] method isn't implemented");
    }[[conclutions_methods]]
}

#[derive(Clone)]
pub struct ObserverRequest {}

impl ObserverRequest {
    pub fn new() -> Self {
        ObserverRequest {}
    }

    pub fn emit<UCX: 'static + Sync + Send + Clone>(
        &self,
        cx: &Cx,
        ucx: UCX,
        sequence: u32,
        request: Protocol::[[request_ref]],
        broadcast: &dyn Fn(Filter, Vec<u8>) -> Result<(), String>,
    ) -> Result<(), RequestObserverErrors> {
        match Self::[[conclutions_caller_name]](&request, cx, ucx.clone()) {[[conclutions_processing]]
            Err(mut error) => match error.pack(sequence, Some(cx.uuid().to_string())) {
                Ok(buffer) => if let Err(e) = cx.send(buffer) {
                    Err(RequestObserverErrors::ResponsingError(e))
                } else {
                    Ok(())
                }
                Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
            },
        }
    }
}

impl Observer for ObserverRequest {}
    "#;
    pub const CONCLUSION_ENUM: &str = r#"
#[derive(Debug, Clone)]
pub enum Conclusion {
[[enum]]
}
    "#;
    pub const CONCLUSION_METHOD: &str = r#"
fn [[name]]<UCX: 'static + Sync + Send + Clone>(
    cx: &Cx,
    ucx: UCX,
    request: &Protocol::[[request_ref]],
) -> Result<
    [[broadcast]],
    String
> {
    Err(String::from("[[name]] method isn't implemented"))
}"#;
    pub const EMITTER_SINGLE: &str = r#"
Ok(mut response) => match response.pack(sequence, Some(cx.uuid().to_string())) {
    Ok(buffer) => if let Err(e) = cx.send(buffer) {
        Err(RequestObserverErrors::ResponsingError(e))
    } else {
        Ok(())
    }
    Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
}"#;
    pub const EMITTER_MULTIPLE: &str = r#"
Ok(conclusion) => match conclusion {[[conclutions]]
}"#;
    pub const EMITTER_CONCLUSION: &str = r#"
Conclusion::[[conclusion_name]](mut response) => match Self::[[conclusion_name]](cx, ucx.clone(), &request) {
    Ok([[broadcasts_refs]]) => match response.pack(sequence, Some(cx.uuid().to_string())) {
        Ok(buffer) => if let Err(e) = cx.send(buffer) {
            Err(RequestObserverErrors::ResponsingError(e))
        } else {[[broadcasts_impls]]
            Ok(())
        }
        Err(e) => Err(RequestObserverErrors::EncodingResponseError(e)),
    }
    Err(error) => Err(RequestObserverErrors::AfterConclusionError(error)),
}"#;
    pub const EMITTER_BROADCAST: &str = r#"
let (filter, mut msg) = [[broadcast_msg_name]];[[broadcast_sender]]"#;
    pub const EMITTER_BROADCAST_OPT: &str = r#"
if let Some((filter, mut msg)) = [[broadcast_msg_name]] {[[broadcast_sender]]
}"#;
    pub const EMITTER_BROADCAST_SENDER: &str = r#"
match msg.pack(0, Some(cx.uuid().to_string())) {
    Ok(buffer) => if let Err(e) = broadcast(filter, buffer) {
        return Err(RequestObserverErrors::BroadcastingError(e));
    }
    Err(e) => {
        return Err(RequestObserverErrors::EncodingResponseError(e));
    }
};"#;

}

pub struct RenderRequest {
}

impl Default for RenderRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderRequest {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        base: &Path,
        request: &Request
    ) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, request)?;
        let request_ref = request.get_request()?;
        let mut output: String = templates::MODULE.to_owned();
        if request.actions.len() > 1 {
            output = output.replace("[[conclutions_enum]]", &format!("\n{}\n", self.get_conclusion_enum(request)?));
            output = output.replace("[[conclutions_methods]]", &format!("\n{}\n", self.get_conclusion_methods(request)?));
            output = output.replace("[[conclutions_caller_name]]", "conclusion");
            output = output.replace("[[conclutions_caller_ok]]", "Conclusion");
        } else {
            output = output.replace("[[conclutions_enum]]", "");
            output = output.replace("[[conclutions_methods]]", "");
            output = output.replace("[[conclutions_caller_name]]", "response");
            output = output.replace("[[conclutions_caller_ok]]", &format!("Protocol::{}", self.into_rust_path(&request.get_response()?)));
        }
        output = output.replace("[[request_ref]]", &self.into_rust_path(&request_ref));
        if let Some(error_ref) = request.error.as_ref() {
            output = output.replace("[[request_err_ref]]", &self.into_rust_path(error_ref));
        }
        output = output.replace("[[conclutions_processing]]", &self.get_proccessing_emitter(request)?);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
        let dest = base.join("observers");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        let request = request.get_request()?;
        Ok(dest.join(format!("{}.rs", request.to_lowercase().replace(".", "_"))))
    }

    fn get_conclusion_enum(&self, request: &Request) -> Result<String, String> {
        let mut output: String = templates::CONCLUSION_ENUM.to_owned();
        let mut conclusions: String = String::new();
        for (pos, action) in request.actions.iter().enumerate() {
            let name = if let Some(name) = action.conclusion.as_ref() {
                name
            } else {
                return Err(String::from("Action doesn't have bound conclusion name"));
            };
            let reference = if let Some(reference) = action.response.as_ref() {
                self.into_rust_path(reference)
            } else {
                return Err(String::from("Action doesn't have bound response reference"));
            };
            conclusions = format!("{}{}{}(Protocol::{})", conclusions, tools::tabs(1), name, reference);
            if pos < request.actions.len() - 1 {
                conclusions = format!("{},\n", conclusions);
            }
        }
        output = output.replace("[[enum]]", &conclusions);
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

    fn get_proccessing_emitter(&self, request: &Request) -> Result<String, String> {
        let mut output: String;
        if request.actions.len() == 1 {
            output = tools::inject_tabs(3, templates::EMITTER_SINGLE.to_string());
        } else {
            output = templates::EMITTER_MULTIPLE.to_string();
            let mut conclusions: String = String::new();
            for (pos, action) in request.actions.iter().enumerate() {
                let mut conclusion: String = templates::EMITTER_CONCLUSION.to_string();
                conclusion = conclusion.replace("[[conclusion_name]]", &action.get_conclusion()?);
                if action.broadcast.is_empty() {
                    conclusion = conclusion
                        .replace("[[broadcasts_refs]]", "_")
                        .replace("[[broadcasts_impls]]", "");
                } else {
                    let mut broadcasts: String = String::new();
                    let mut broadcasts_refs: String = String::new();
                    for (pos, broadcast) in action.broadcast.iter().enumerate() {
                        broadcasts_refs = format!("{}{}", broadcasts_refs, self.get_broadcast_var_name(&broadcast.reference));
                        if pos < action.broadcast.len() - 1 {
                            broadcasts_refs = format!("{},\n", broadcasts_refs);
                        }
                        if broadcast.optional {
                            broadcasts = format!("{}{}",
                                broadcasts,
                                templates::EMITTER_BROADCAST_OPT
                                    .replace("[[broadcast_msg_name]]", &self.get_broadcast_var_name(&broadcast.reference))
                                    .replace("[[broadcast_sender]]", &tools::inject_tabs(1, String::from(templates::EMITTER_BROADCAST_SENDER))),
                            );
                        } else {
                            broadcasts = format!("{}{}",
                                broadcasts,
                                templates::EMITTER_BROADCAST
                                    .replace("[[broadcast_msg_name]]", &self.get_broadcast_var_name(&broadcast.reference))
                                    .replace("[[broadcast_sender]]", templates::EMITTER_BROADCAST_SENDER),
                            );
                        }
                    }
                    if action.broadcast.len() > 1 {
                        broadcasts_refs = format!("(\n{}\n{})", &tools::inject_tabs(2, broadcasts_refs), tools::tabs(1));
                    }
                    conclusion = conclusion
                        .replace("[[broadcasts_refs]]", &broadcasts_refs)
                        .replace("[[broadcasts_impls]]", &tools::inject_tabs(3, broadcasts));
                }
                conclusions = format!("{}{}", conclusions, conclusion);
                if pos < request.actions.len() - 1 {
                    //conclusions = format!("{},\n", conclusions);
                }
            }
            output = tools::inject_tabs(3, output.replace("[[conclutions]]", &tools::inject_tabs(1, conclusions)));
        }
        Ok(output)
    }

    fn get_broadcast_var_name(&self, reference: &str) -> String {
        reference.to_lowercase().replace(".", "_")
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }

}

