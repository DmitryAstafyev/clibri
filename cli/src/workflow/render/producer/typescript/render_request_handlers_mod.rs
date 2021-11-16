use super::{helpers, workflow::request::Request};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import * as Protocol from "../../implementation/protocol";
import { Producer } from "../index";

[[handlers]]
export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { Protocol };

export function broadcastAll(
    producer: Producer,
    broadcasts: Array<[string[], Protocol.Convertor<any>]>
): Promise<void> {
    if (broadcasts.length === 0) {
        return Promise.resolve();
    }
    return new Promise((resolve, reject) => {
        let error: Error | undefined;
        Promise.all(
            broadcasts.map((broadcast) => {
                return producer
                    .broadcast(broadcast[0], broadcast[1].pack(0, undefined))
                    .catch((err: Error) => {
                        error = err;
                    });
            })
        )
            .then(() => {
                if (error !== undefined) {
                    reject(error);
                } else {
                    resolve();
                }
            })
            .catch(reject);
    });
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

    pub fn render(&self, base: &Path, requests: &[Request]) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = String::from(templates::MODULE);
        let mut mods = String::new();
        for request in requests.iter() {
            mods = format!(
                "{}export {{ handler as {}Handler }} from \"./{}\";\n",
                mods,
                helpers::string::first_letter_lowercase(&request.get_request()?.replace(".", "")),
                request.get_request()?.to_lowercase()
            );
        }
        output = output.replace("[[handlers]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("responses");
        if !dest.exists() {
            if let Err(e) = fs::create_dir_all(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join("index.ts"))
    }
}
