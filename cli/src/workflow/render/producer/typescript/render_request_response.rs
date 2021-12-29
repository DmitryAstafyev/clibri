use super::{helpers, helpers::render as tools, workflow::request::Request};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import { Response } from "../implementation/responses/[[module]]";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";

export function response(request: Protocol.[[request]], scope: Scope): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.[[request]] isn't implemented.`)
    );
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

    pub fn render(&self, base: &Path, request: &Request) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, request)?;
        if dest.exists() {
            println!("[SKIP]: {}", dest.to_string_lossy());
            return Ok(());
        }
        let request_ref = request.get_request()?;
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[request]]", &request_ref);
        output = output.replace("[[module]]", &tools::into_ts_path(&request_ref));
        helpers::fs::write(dest, output, true)
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
        Ok(dest.join(format!("{}.ts", request.to_lowercase())))
    }
}
