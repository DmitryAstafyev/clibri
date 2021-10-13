use super::{
    helpers, helpers::render as tools, workflow::beacon::Broadcast, workflow::request::Request,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_CONCLUSION: &str = r#"
use super::{
    broadcast, identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use fiber::server;
use uuid::Uuid;

pub async fn process<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::[[request]],
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let mut broadcasting: Vec<(Vec<Uuid>, Vec<u8>)> = vec![];
    let buffer =
        match responses::[[response_mod]]::response(identification, filter, context, request, control).await {
            Ok(conclusion) => match conclusion {
[[conclusions]]
            },
            Err(mut error) => pack(&sequence, &uuid, &mut error)?,
        };
    control
        .send(buffer, Some(uuid))
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))?;
    for msg in broadcasting.iter_mut() {
        broadcast::<E, C>(msg, control).await?;
    }
    Ok(())
}    
"#;
    pub const CONCLUSION_WITH_BROADCAST: &str = r#"responses::[[response_mod]]::Response::[[conclusion]]((
    mut response,
[[broadcasts_declaration]],
)) => {
[[broadcasts_processing]]
    pack(&sequence, &uuid, &mut response)?
},"#;
    pub const CONCLUSION_WITHOUT_BROADCAST: &str = r#"responses::[[response_mod]]::Response::[[conclusion]](mut response) => {
    pack(&sequence, &uuid, &mut response)?
},"#;
    pub const MODULE_NO_CONCLUSION: &str = r#"
use super::{
    identification, pack, producer::Control, protocol, responses, Context, HandlerError,
    ProducerError,
};
use fiber::server;

pub async fn process<E: std::error::Error, C: server::Control<E> + Send + Clone>(
    identification: &mut identification::Identification,
    filter: &identification::Filter,
    context: &mut Context,
    request: &protocol::[[request]],
    sequence: u32,
    control: &Control<E, C>,
) -> Result<(), HandlerError> {
    let uuid = identification.uuid();
    let buffer = match responses::[[response_mod]]::response(
        identification,
        filter,
        context,
        request,
        control,
    )
    .await
    {
        Ok(mut response) => pack(&sequence, &uuid, &mut response)?,
        Err(mut error) => pack(&sequence, &uuid, &mut error)?,
    };
    control
        .send(buffer, Some(uuid))
        .await
        .map_err(|e: ProducerError<E>| HandlerError::Processing(e.to_string()))
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
            output = output.replace("[[response_mod]]", &self.get_response_mod_name(&request)?);
            output = output.replace("[[conclusions]]", &self.get_conclusions(&request)?);
            output
        } else {
            let mut output = templates::MODULE_NO_CONCLUSION.to_owned();
            output = output.replace("[[response_mod]]", &self.get_response_mod_name(&request)?);
            output
        };
        output = output.replace("[[request]]", &self.into_rust_path(&request_ref));
        helpers::fs::write(dest, output, true)
    }

    fn get_conclusions(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for (pos, action) in request.actions.iter().enumerate() {
            if action.broadcast.is_empty() {
                let mut out = templates::CONCLUSION_WITHOUT_BROADCAST.to_owned();
                out = out.replace("[[response_mod]]", &self.get_response_mod_name(&request)?);
                out = out.replace("[[conclusion]]", &action.get_conclusion()?);
                output = format!(
                    "{}{}{}",
                    output,
                    &tools::inject_tabs(3, out),
                    if pos == request.actions.len() - 1 {
                        ""
                    } else {
                        "\n"
                    }
                );
            } else {
                let mut out = templates::CONCLUSION_WITH_BROADCAST.to_owned();
                out = out.replace("[[response_mod]]", &self.get_response_mod_name(&request)?);
                out = out.replace("[[conclusion]]", &action.get_conclusion()?);
                let mut broadcasts_declaration = String::new();
                let mut broadcasts_processing = String::new();
                for (pos, broadcast) in action.broadcast.iter().enumerate() {
                    let broadcast_var_name = format!(
                        "broadcast_{}",
                        broadcast.reference.replace(".", "_").to_lowercase()
                    );
                    broadcasts_declaration = format!(
                        "{}\tmut {},{}",
                        broadcasts_declaration,
                        broadcast_var_name,
                        if pos == action.broadcast.len() - 1 {
                            ""
                        } else {
                            "\n"
                        }
                    );
                    if broadcast.optional {
                        broadcasts_processing = format!(
                            "{}{}{}",
                            broadcasts_processing,
                            r#"if let Some(mut [[var_name]]) = [[var_name]].take() {
    broadcasting.push((
        [[var_name]].0,
        pack(&0, &uuid, &mut [[var_name]].1)?,
    ));
}"#,
                            if pos == action.broadcast.len() - 1 {
                                ""
                            } else {
                                "\n"
                            }
                        );
                    } else {
                        broadcasts_processing = format!(
                            "{}{}{}",
                            broadcasts_processing,
                            r#"broadcasting.push((
    [[var_name]].0,
    pack(&0, &uuid, &mut [[var_name]].1)?,
));"#,
                            if pos == action.broadcast.len() - 1 {
                                ""
                            } else {
                                "\n"
                            }
                        );
                    }
                    broadcasts_processing =
                        broadcasts_processing.replace("[[var_name]]", &broadcast_var_name);
                }
                out = out.replace("[[broadcasts_declaration]]", &broadcasts_declaration);
                out = out.replace(
                    "[[broadcasts_processing]]",
                    &tools::inject_tabs(1, broadcasts_processing),
                );
                output = format!(
                    "{}{}{}",
                    output,
                    &tools::inject_tabs(3, out),
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

    fn into_rust_path(&self, input: &str) -> String {
        input.to_string().replace(".", "::")
    }

    fn get_response_mod_name(&self, request: &Request) -> Result<String, String> {
        Ok(request.get_request()?.to_lowercase().replace(".", "_"))
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("handlers");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
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
