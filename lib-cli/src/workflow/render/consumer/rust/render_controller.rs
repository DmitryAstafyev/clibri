use super::{
    helpers, helpers::render as tools, workflow::beacon::Broadcast, workflow::request::Request,
    WorkflowStore,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"use super::{api::Api, error::ConsumerError, protocol, protocol::PackingStruct};
use fiber::client;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Consumer<E: client::Error> {
    api: Api<E>,
    shutdown: CancellationToken,
}[[request_enums]]
impl<E: client::Error> Consumer<E> {
    pub fn new(api: Api<E>) -> Self {
        let shutdown = api.get_shutdown_token();
        Consumer { api, shutdown }
    }[[beacons]]
[[requests]]
    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}"#;
    pub const BEACON: &str = r#"pub async fn [[name]](
    &mut self,
    mut beacon: protocol::[[request]],
) -> Result<(), ConsumerError<E>> {
    let sequence = self.api.sequence().await?;
    let uuid = self.api.uuid_as_string().await?;
    self.api
        .send(
            &beacon
                .pack(sequence, uuid)
                .map_err(ConsumerError::Protocol)?,
        )
        .await
}"#;
    pub const REQUEST: &str = r#"pub async fn [[name]](
    &mut self,
    mut request: protocol::[[request]],
) -> Result<[[response]], ConsumerError<E>> {
    let sequence = self.api.sequence().await?;
    let uuid = self.api.uuid_as_string().await?;
    let message = self
        .api
        .request(
            sequence,
            &request
                .pack(sequence, uuid)
                .map_err(ConsumerError::Protocol)?,
        )
        .await?;
    match message {[[responses]]
        _ => Err(ConsumerError::UnexpectedResponse(String::from(
            "for [[request]] has been gotten wrong response",
        ))),
    }
}"#;
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

    pub fn render(&self, base: &Path, store: &WorkflowStore) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace(
            "[[request_enums]]",
            &self.get_request_enums(&store.requests)?,
        );
        output = output.replace("[[requests]]", &self.get_requests(&store.requests)?);
        output = output.replace("[[beacons]]", &self.get_beacons(&store.beacons)?);
        helpers::fs::write(dest, output, true)
    }

    fn get_request_enums(&self, requests: &Vec<Request>) -> Result<String, String> {
        let mut output: String = String::new();
        for request in requests.iter() {
            let mut out: String = String::new();
            for action in request.actions.iter() {
                if action.get_conclusion().is_err() {
                    out = format!(
                        "{}Response(protocol::{}),\n",
                        out,
                        action.get_response()?.replace(".", "::"),
                    );
                } else {
                    out = format!(
                        "{}{}(protocol::{}),\n",
                        out,
                        action.get_conclusion()?,
                        action.get_response()?.replace(".", "::"),
                    );
                }
            }
            out = format!(
                "{}Err(protocol::{}),",
                out,
                request.get_err()?.replace(".", "::"),
            );
            output = format!(
                r#"{}
pub enum {} {{
{}
}}"#,
                output,
                &Render::get_request_response_type(request)?,
                tools::inject_tabs(1, out)
            );
        }
        Ok(output)
    }

    fn get_beacons(&self, beacons: &Vec<Broadcast>) -> Result<String, String> {
        let mut output: String = String::new();
        for beacon in beacons {
            output = format!(
                "{}\n{}",
                output,
                templates::BEACON
                    .replace("[[request]]", &beacon.reference.replace(".", "::"))
                    .replace(
                        "[[name]]",
                        &beacon.reference.replace(".", "_").to_lowercase()
                    )
            );
        }
        Ok(tools::inject_tabs(1, output))
    }

    fn get_requests(&self, requests: &Vec<Request>) -> Result<String, String> {
        let mut output: String = String::new();
        for request in requests {
            let mut out = String::from(templates::REQUEST);
            out = out.replace("[[response]]", &Render::get_request_response_type(request)?);
            out = out.replace("[[name]]", &Render::get_request_method_name(request)?);
            out = out.replace("[[request]]", &request.get_request()?.replace(".", "::"));
            let mut responses: String = String::new();
            for action in request.actions.iter() {
                if action.get_conclusion().is_err() {
                    responses = format!(
                        r#"{}{}{} =>
    Ok({}::Response(msg)),"#,
                        responses,
                        "\n",
                        Render::get_request_enum_reference(action.get_response()?)?,
                        Render::get_request_response_type(request)?,
                    );
                } else {
                    responses = format!(
                        r#"{}{}{} =>
    Ok({}::{}(msg)),"#,
                        responses,
                        "\n",
                        Render::get_request_enum_reference(action.get_response()?)?,
                        Render::get_request_response_type(request)?,
                        action.get_conclusion()?,
                    );
                }
            }
            responses = format!(
                r#"{}{}{} =>
    Ok({}::Err(msg)),"#,
                responses,
                "\n",
                Render::get_request_enum_reference(request.get_err()?)?,
                Render::get_request_response_type(request)?,
            );
            out = out.replace("[[responses]]", &tools::inject_tabs(2, responses));
            output = format!("{}\n{}", output, out);
        }
        Ok(tools::inject_tabs(1, output))
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("consumer");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("controller.rs"))
    }

    fn get_request_method_name(request: &Request) -> Result<String, String> {
        Ok(request.get_request()?.replace(".", "_").to_lowercase())
    }

    fn get_request_response_type(request: &Request) -> Result<String, String> {
        Ok(format!(
            "{}Response",
            request.get_request()?.replace(".", "")
        ))
    }

    pub fn get_request_enum_reference(request: String) -> Result<String, String> {
        let parts: Vec<String> = request
            .split('.')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| String::from(*v))
            .collect();
        let enum_ref: String = if parts.len() == 1 {
            format!(
                "protocol::AvailableMessages::{}(protocol::{}(msg))",
                parts[0], parts[0]
            )
        } else {
            //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(protocol::UserLogin::Request::AvailableMessages::Request(request))) => {
            //protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Request(request))
            let mut chain: String = String::from("");
            for (pos, part) in parts.iter().enumerate() {
                let mut step: String = String::from("protocol");
                for part in parts.iter().take(pos) {
                    step = format!("{}::{}", step, part);
                }
                step = format!("{}::AvailableMessages::{}(", step, part);
                chain = format!("{}{}", chain, step);
            }
            format!("{}msg{}", chain, ")".repeat(parts.len()))
        };
        Ok(enum_ref)
    }
}
