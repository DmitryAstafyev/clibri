use super::{helpers, workflow::event::Event};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_BROADCAST: &str = r#"import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/[[module]]";

export function emit(
    event: Protocol.[[event]],
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "[[event]]" isn't implemented`)
    );
}"#;
    pub const MODULE_WITHOUT_BROADCAST: &str = r#"import { Filter, Producer, Context, Protocol } from "../implementation/events";

export function emit(
    event: Protocol.[[event]],
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return Promise.reject(
        new Error(`Handler for event "[[event]]" isn't implemented`)
    );
}"#;
    pub const DEFAULT_MODULE_WITH_BROADCAST: &str = r#"import {
    Identification,
    Filter,
    Producer,
    Context,
    Protocol,
} from "../implementation/events";
import { Output } from "../implementation/events/[[module]]";

export function emit(
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "[[event]]" isn't implemented`)
    );
}"#;
    pub const DEFAULT_MODULE_WITHOUT_BROADCAST: &str = r#"import {
    Identification,
    Filter,
    Producer,
    Context,
    Protocol,
} from "../implementation/events";

export function emit(
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return Promise.reject(
        new Error(`Handler for event "[[event]]" isn't implemented`)
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

    pub fn render(&self, base: &Path, event: &Event) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, event)?;
        if dest.exists() {
            println!("[SKIP]: {}", dest.to_string_lossy());
            return Ok(());
        }
        let mut output: String = if self.is_default(event)? {
            if event.broadcasts.is_empty() {
                templates::DEFAULT_MODULE_WITHOUT_BROADCAST.to_owned()
            } else {
                templates::DEFAULT_MODULE_WITH_BROADCAST.to_owned()
            }
        } else if event.broadcasts.is_empty() {
            templates::MODULE_WITHOUT_BROADCAST.to_owned()
        } else {
            templates::MODULE_WITH_BROADCAST.to_owned()
        };
        output = output.replace("[[event]]", &event.get_reference()?);
        output = output.replace("[[module]]", &event.get_reference()?.to_lowercase());
        helpers::fs::write(dest, output, true)
    }

    fn is_default(&self, event: &Event) -> Result<bool, String> {
        if event.get_reference()? == "connected" || event.get_reference()? == "disconnected" {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_dest_file(&self, base: &Path, event: &Event) -> Result<PathBuf, String> {
        let dest = base.join("events");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(format!("{}.ts", event.get_reference()?.to_lowercase())))
    }
}
