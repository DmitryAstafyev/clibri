use super::{helpers, helpers::render as tools, workflow::request::Request};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE_WITH_CONCLUSION: &str = r#"import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/[[module]]";

export class Response {
[[required_broadcasts]]
    private _response!: [[expectetions]];
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: [[expectetions]]
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {[[methods_declarations]]
    } {
        const self = this;
        return {[[methods_implementations]]
        };
    }

    public error(): Error | undefined {
        let error: Error | undefined;[[broadcast_checks]]
        return error;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }

    public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
        return this._broadcasts;
    }
}

export function handler(
    request: Protocol.[[request]],
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return response(request, consumer, filter, context, producer).then(
        (res) => {
            const error: Error | undefined = res.error();
            if (error instanceof Error) {
                return Promise.reject(error);
            }
            return producer
                .send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
                .then(() => {
                    return broadcastAll(producer, res.broadcasts());
                });
        }
    );
}"#;
    pub const BROADCAST_IMPL: &str = r#"[[name]](msg: Protocol.[[reference]]): Response {
    if (
        self._response.getSignature() !==
        Protocol.[[conclusion]].getSignature()
    ) {
        throw new Error(
            `Message "Protocol.[[reference]]" can be used only with "Protocol.[[conclusion]]"`
        );
    }
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
    pub const BROADCAST_CHECK: &str = r#"if (
    error === undefined &&
    this._response.getSignature() ===
    Protocol.[[conclusion]].getSignature()
) {
    Response.REQUIRED_[[requered_conclusion]].forEach((ref) => {
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
}"#;
    pub const MODULE_NO_CONCLUSION: &str = r#"import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { response } from "../../responses/[[module]]";

export class Response {
    private _response!: [[expectetions]];

    constructor(res: [[expectetions]]) {
        this._response = res;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }
}

export function handler(
    request: Protocol.[[request]],
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return response(request, consumer, filter, context, producer).then(
        (res) => {
            return producer.send(
                consumer.uuid(),
                res.pack(sequence, consumer.uuid())
            );
        }
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
        let request_ref = request.get_request()?;
        let mut output: String = if request.actions.len() > 1 {
            let mut output = templates::MODULE_WITH_CONCLUSION.to_owned();
            output = output.replace(
                "[[required_broadcasts]]",
                &self.get_required_broadcasts(&request)?,
            );
            output = output.replace("[[expectetions]]", &self.get_expectetions(&request)?);
            output = output.replace(
                "[[methods_declarations]]",
                &self.get_methods_declarations(&request)?,
            );
            output = output.replace(
                "[[methods_implementations]]",
                &self.get_methods_implementations(&request)?,
            );
            output = output.replace(
                "[[broadcast_checks]]",
                &self.get_broadcast_checks(&request)?,
            );
            output
        } else {
            let mut output = templates::MODULE_NO_CONCLUSION.to_owned();
            output = output.replace(
                "[[expectetions]]",
                &format!(
                    "Protocol.{} | Protocol.{}",
                    request.get_response()?,
                    request.get_err()?
                ),
            );
            output
        };
        output = output.replace("[[module]]", &tools::into_ts_path(&request_ref));
        output = output.replace("[[request]]", &request_ref);
        helpers::fs::write(dest, output, true)
    }

    fn get_required_broadcasts(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for action in request.actions.iter() {
            if !action.broadcast.is_empty() {
                let mut broadcasts = String::new();
                for broadcast in action
                    .broadcast
                    .iter()
                    .filter(|broadcast| !broadcast.optional)
                {
                    broadcasts = format!("{}\nProtocol.{},", broadcasts, broadcast.reference);
                }
                output = format!(
                    r#"static REQUIRED_{} = [{}
];"#,
                    action.get_conclusion()?.to_uppercase(),
                    tools::inject_tabs(1, broadcasts),
                );
            }
        }
        Ok(tools::inject_tabs(1, output))
    }

    fn get_expectetions(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for (pos, action) in request.actions.iter().enumerate() {
            output = format!(
                "{}{}Protocol.{}",
                output,
                if pos == 0 { "" } else { "| " },
                action.get_response()?
            );
        }
        output = format!("{} | Protocol.{}", output, request.get_err()?);
        Ok(output)
    }

    fn get_methods_declarations(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for action in request.actions.iter() {
            for broadcast in action.broadcast.iter() {
                output = format!(
                    "{}\n{}(msg: Protocol.{}): Response;",
                    output,
                    broadcast.reference.replace(".", ""),
                    broadcast.reference,
                );
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_methods_implementations(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for action in request.actions.iter() {
            for broadcast in action.broadcast.iter() {
                let mut out = templates::BROADCAST_IMPL.to_owned();
                out = out.replace("[[name]]", &broadcast.reference.replace(".", ""));
                out = out.replace("[[reference]]", &broadcast.reference);
                out = out.replace("[[conclusion]]", &action.get_response()?);
                output = format!("{}\n{}", output, out,);
            }
        }
        Ok(tools::inject_tabs(3, output))
    }

    fn get_broadcast_checks(&self, request: &Request) -> Result<String, String> {
        let mut output = String::new();
        for action in request.actions.iter() {
            if !action.broadcast.is_empty() {
                let mut out = templates::BROADCAST_CHECK.to_owned();
                out = out.replace(
                    "[[requered_conclusion]]",
                    &action.get_conclusion()?.to_uppercase(),
                );
                out = out.replace("[[conclusion]]", &action.get_response()?);
                output = format!("{}\n{}", output, out,);
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
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
        let request = request.get_request()?;
        Ok(dest.join(format!("{}.ts", request.to_lowercase())))
    }
}
