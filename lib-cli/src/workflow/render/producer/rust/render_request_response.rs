use super::{
    helpers, helpers::render as tools, workflow::broadcast::Broadcast, workflow::request::Request,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_CONCLUSION: &str = r#"
use super::{identification, producer::Control, protocol, Context};
use fiber::server;
use uuid::Uuid;

[[broadcast_types]]
pub enum Response {[[response_declaration]]
}

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::[[request]],
    control: &Control<E, C>,
) -> Result<Response, protocol::[[error]]> {
    panic!("Handler for protocol::[[request]] isn't implemented");
}
"#;
    pub const MODULE_NO_CONCLUSION: &str = r#"
use super::{identification, producer::Control, protocol, Context};
use fiber::server;

#[allow(unused_variables)]
pub async fn response<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::[[request]],
    control: &Control<E, C>,
) -> Result<protocol::[[response]], protocol::[[error]]> {
    panic!("Handler for protocol::[[request]] isn't implemented");
}
"#;
}

pub struct Render {}

impl Default for Render {
    fn default() -> Self {
        Self::new()
    }
}

impl Render {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, base: &Path, request: &Request) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, request)?;
        let request_ref = request.get_request()?;
        let mut output: String = if request.actions.len() > 1 {
            let mut output = templates::MODULE_WITH_CONCLUSION.to_owned();
            output = output.replace("[[broadcast_types]]", &self.get_broadcast_types(&request)?);
            output = output.replace(
                "[[response_declaration]]",
                &self.get_response_declaration(&request)?,
            );
            output
        } else {
            let mut output = templates::MODULE_NO_CONCLUSION.to_owned();
            output = output.replace(
                "[[response]]",
                &self.into_rust_path(&request.get_response()?),
            );
            output
        };
        output = output.replace("[[request]]", &self.into_rust_path(&request_ref));
        if let Some(error_ref) = request.error.as_ref() {
            output = output.replace("[[error]]", &self.into_rust_path(error_ref));
        }
        helpers::fs::write(dest, output, true)
    }

    fn get_response_declaration(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for (pos, action) in request.actions.iter().enumerate() {
            if action.broadcast.is_empty() {
                output = format!(
                    "{}{}(protocol::{}),{}",
                    output,
                    action.get_conclusion()?,
                    self.into_rust_path(&action.get_response()?),
                    if pos == request.actions.len() - 1 {
                        ""
                    } else {
                        "\n"
                    }
                );
            } else {
                let mut brodcast_output = String::new();
                for (pos, broadcast) in action.broadcast.iter().enumerate() {
                    brodcast_output = format!(
                        "{}{},{}",
                        brodcast_output,
                        self.get_broadcast_type_name(broadcast),
                        if pos == action.broadcast.len() - 1 {
                            ""
                        } else {
                            "\n"
                        }
                    );
                }
                output = format!(
                    "{}\n{}(\n\t(\n\t\tprotocol::{},\n{}\n\t)\n),{}",
                    output,
                    action.get_conclusion()?,
                    self.into_rust_path(&action.get_response()?),
                    tools::inject_tabs(2, brodcast_output),
                    if pos == request.actions.len() - 1 {
                        ""
                    } else {
                        "\n"
                    }
                );
            }
        }
        Ok(tools::inject_tabs(1, output))
    }

    fn get_broadcast_types(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for action in request.actions.iter() {
            if !action.broadcast.is_empty() {
                for broadcast in action.broadcast.iter() {
                    if broadcast.optional {
                        output = format!(
                            "{}type {} = Option<(Vec<Uuid>, protocol::{})>;\n",
                            output,
                            self.get_broadcast_type_name(broadcast),
                            self.into_rust_path(&broadcast.reference),
                        );
                    } else {
                        output = format!(
                            "{}type {} = (Vec<Uuid>, protocol::{});\n",
                            output,
                            self.get_broadcast_type_name(broadcast),
                            self.into_rust_path(&broadcast.reference),
                        );
                    }
                }
            }
        }
        Ok(output)
    }

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }

    fn get_broadcast_type_name(&self, broadcast: &Broadcast) -> String {
        format!("Broadcast{}", broadcast.reference.replace(".", ""))
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
        let dest = base.join("responses");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        let request = request.get_request()?;
        Ok(dest.join(format!("{}.rs", request.to_lowercase().replace(".", "_"))))
    }
}
