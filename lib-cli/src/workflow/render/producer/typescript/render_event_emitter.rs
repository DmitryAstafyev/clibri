use super::{helpers, helpers::render as tools, workflow::event::Event};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_BROADCAST: &str = r#"import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/[[module]]";

export class Output {
[[required_broadcasts]]
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    public broadcast(uuids: string[]): {[[methods_declarations]]
    } {
        const self = this;
        return {[[methods_implementations]]
        };
    }

	public error(): Error | undefined {
		let error: Error | undefined;
		Output.REQUIRED.forEach((ref) => {
			if (error !== undefined) {
				return;
			}
			if (
				this._broadcasts.find((msg) => {
					return msg[1].getSignature() === ref.getSignature();
				}) === undefined
			) {
				error = new Error(
					`Broadcast ${ref.getSignature()} is required, but hasn't been found`
				);
			}
		});
		return error;
	}

    public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
        return this._broadcasts;
    }
}

[[handler]]"#;
    pub const MODULE_NO_BROADCAST: &str = r#"import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { emit } from "../../events/[[module]]";    

[[handler]]"#;
    pub const BROADCAST_IMPL: &str = r#"[[name]](msg: Protocol.[[reference]]): Output {
    if (
        self._broadcasts.find(
            (b) =>
                b[1].getSignature() ===
                Protocol.[[reference]].getSignature()
        ) !== undefined
    ) {
        throw new Error(
            `Broadcast Protocol.[[reference]] already has been defined.`
        );
    }
    self._broadcasts.push([uuids, msg]);
    return self;
},"#;
    pub const HANDLER_WITH_BROADCAST: &str = r#"export function handler(
    event: Protocol.[[event]],
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return emit(event, filter, context, producer).then((output) => {
        const error: Error | undefined = output.error();
        if (error instanceof Error) {
            return Promise.reject(error);
        }
        return broadcastAll(producer, output.broadcasts());
    });
}"#;
    pub const HANDLER_WITHOUT_BROADCAST: &str = r#"export function handler(
    event: Protocol.[[event]],
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return emit(event, filter, context, producer);
}"#;
    pub const HANDLER_DEFAULT_WITH_BROADCAST: &str = r#"export function handler(
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return emit(consumer, filter, context, producer).then((output) => {
        const error: Error | undefined = output.error();
        if (error instanceof Error) {
            return Promise.reject(error);
        }
        return broadcastAll(producer, output.broadcasts());
    });
}"#;
    pub const HANDLER_DEFAULT_WITHOUT_BROADCAST: &str = r#"export function handler(
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return emit(consumer, filter, context, producer);
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
        let event_ref = event.get_reference()?;
        let mut output: String = if !event.broadcasts.is_empty() {
            let mut output = templates::MODULE_WITH_BROADCAST.to_owned();
            output = output.replace(
                "[[required_broadcasts]]",
                &self.get_required_broadcasts(&event)?,
            );
            output = output.replace(
                "[[methods_declarations]]",
                &self.get_methods_declarations(&event)?,
            );
            output = output.replace(
                "[[methods_implementations]]",
                &self.get_methods_implementations(&event)?,
            );
            output
        } else {
            templates::MODULE_NO_BROADCAST.to_owned()
        };
        output = output.replace("[[module]]", &tools::into_ts_path(&event_ref));
        output = output.replace("[[event]]", &event_ref);
        output = output.replace("[[handler]]", &self.get_handler(&event)?);
        helpers::fs::write(dest, output, true)
    }

    fn get_required_broadcasts(&self, event: &Event) -> Result<String, String> {
        let mut output = String::new();
        let mut broadcasts = String::new();
        for broadcast in event
            .broadcasts
            .iter()
            .filter(|broadcast| !broadcast.optional)
        {
            broadcasts = format!("{}\nProtocol.{},", broadcasts, broadcast.reference);
        }
        output = format!(
            r#"static REQUIRED = [{}
];"#,
            tools::inject_tabs(1, broadcasts),
        );
        Ok(tools::inject_tabs(1, output))
    }

    fn get_methods_declarations(&self, event: &Event) -> Result<String, String> {
        let mut output = String::new();
        for broadcast in event.broadcasts.iter() {
            output = format!(
                "{}\n{}(msg: Protocol.{}): Output;",
                output,
                broadcast.reference.replace(".", ""),
                broadcast.reference,
            );
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_methods_implementations(&self, event: &Event) -> Result<String, String> {
        let mut output = String::new();
        for broadcast in event.broadcasts.iter() {
            let mut out = templates::BROADCAST_IMPL.to_owned();
            out = out.replace("[[name]]", &broadcast.reference.replace(".", ""));
            out = out.replace("[[reference]]", &broadcast.reference);
            output = format!("{}\n{}", output, out,);
        }
        Ok(tools::inject_tabs(3, output))
    }

    fn get_handler(&self, event: &Event) -> Result<String, String> {
        let mut output = if self.is_default(event)? {
            if event.broadcasts.is_empty() {
                templates::HANDLER_DEFAULT_WITHOUT_BROADCAST.to_owned()
            } else {
                templates::HANDLER_DEFAULT_WITH_BROADCAST.to_owned()
            }
        } else {
            if event.broadcasts.is_empty() {
                templates::HANDLER_WITHOUT_BROADCAST.to_owned()
            } else {
                templates::HANDLER_WITH_BROADCAST.to_owned()
            }
        };
        output = output.replace("[[event]]", &event.get_reference()?);
        Ok(output)
    }

    fn is_default(&self, event: &Event) -> Result<bool, String> {
        if event.get_reference()? == "connected" || event.get_reference()? == "disconnected" {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_dest_file(&self, base: &Path, event: &Event) -> Result<PathBuf, String> {
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
        let event = event.get_reference()?;
        Ok(dest.join(format!("{}.ts", event.to_lowercase())))
    }
}
