use super::{helpers, workflow::event::Event};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import * as Protocol from "../../implementation/protocol";
import { Producer } from "../index";

export { Identification, Filter } from "../consumer";
export { Context } from "../../context";
export { Producer } from "../index";
export { ProducerError, ProducerErrorType } from "./error";
export { Protocol };
[[handlers]]export { handler as disconnectedHandler } from "./disconnected";
export { handler as connectedHandler } from "./connected";
export { handler as errorHandler } from "./error";
export { handler as readyHandler } from "./ready";
export { handler as shutdownHandler } from "./shutdown";

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

    pub fn render(&self, base: &Path, events: &[Event]) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut mods = String::new();
        for event in events.iter() {
            if !event.is_default() {
                mods = format!(
                    "{}export {{ handler as {}Handler }} from \"./{}\";\n",
                    mods,
                    helpers::string::first_letter_lowercase(
                        &event.get_reference()?.replace(".", "")
                    ),
                    event.get_reference()?.to_lowercase()
                );
            }
        }
        let output = templates::MODULE.to_owned().replace("[[handlers]]", &mods);
        helpers::fs::write(dest, output, true)
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation").join("events");
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
