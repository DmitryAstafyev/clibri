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
    pub const MODULE: &str =
r#"import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

[[types_declarations]]

export class [[reference]] extends Protocol.UserLogin.Request {

    private _state: ERequestState = ERequestState.Ready;
[[handlers]]
    constructor(request: Protocol.UserLogin.IRequest) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {[[handlers_defs]]
            err: undefined,
        };
    }

    public send(): Promise<[[resolver]]> {
        const consumer: Consumer | Error = Consumer.get();
        if (consumer instanceof Error) {
            return Promise.reject(consumer);
        }
        if (this._state === ERequestState.Pending) {
            return Promise.reject(new Error(`Cannot send request while previous isn't finished`));
        }
        if (this._state === ERequestState.Destroyed) {
            return Promise.reject(new Error(`Cannot send request as soon as it's destroyed`));
        }
        const sequence: number = consumer.getSequence();
        this._state = ERequestState.Pending;
        return new Promise((resolve, reject) => {
            consumer.request(this.pack(sequence), sequence).then((message: Protocol.IAvailableMessages) => {
                switch (this._state) {
                    case ERequestState.Pending:
                        this._state = ERequestState.Ready;
[[response_handler]]
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "[[reference]]" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "[[reference]]".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
[[handlers_setters]]
    public err(handler: TErrHandler): [[reference]] {
        this._handlers.err = handler;
        return this;
    }

}
"#;
    pub const HANDLERS: &str = 
r#"private _handlers: {[[declarations]]
    err: TErrHandler | undefined;
} = {[[init]]
    err: undefined,
};"#;
    pub const HANDLER_SETTER: &str =
r#"public [[name]](handler: T[[type]]Handler): [[reference]] {
    this._handlers.[[name]] = handler;
    return this;
}"#;
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
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[types_declarations]]", &self.get_types_declarations(request)?);
        output = output.replace("[[reference]]", &(request.get_request()?).replace(".", ""));
        output = output.replace("[[resolver]]", &self.get_resolver_type(request)?);
        output = output.replace("[[handlers]]", &tools::inject_tabs(1, self.get_handlers(request)?));
        output = output.replace("[[handlers_defs]]", &tools::inject_tabs(3, self.get_handlers_defs(request)?));
        output = output.replace("[[handlers_setters]]", &tools::inject_tabs(1, self.get_handlers_setters(request)?));
        output = output.replace("[[response_handler]]", &tools::inject_tabs(6, self.get_response_handler(request)?));
        helpers::fs::write(dest, output, true)
    }

    fn get_types_declarations(&self, request: &Request) -> Result<String, String> {
        let mut output: String = format!("export type {} = Protocol.{}", self.get_resolver_type(request)?, request.get_err()?);
        if request.actions.len() > 1 {
            for action in &request.actions {
                let reference = if let Some(reference) = action.response.as_ref() {
                    reference
                } else {
                    return Err(String::from("Action doesn't have bound response reference"));
                };
                output = format!(
                    "{} | Protocol.{}",
                    output,
                    reference,
                );
            }
            output = format!("{};", output);
            for action in &request.actions {
                let name = if let Some(name) = action.conclusion.as_ref() {
                    name
                } else {
                    println!("{:?}", request);
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
                let reference = if let Some(reference) = action.response.as_ref() {
                    reference
                } else {
                    return Err(String::from("Action doesn't have bound response reference"));
                };
                output = format!(
                    "{}\nexport type T{}Handler = (response: Protocol.{}) => void",
                    output,
                    name,
                    reference,
                );
            }
        } else {
            output = format!(
                "{} | Protocol.{};",
                output,
                request.get_response()?,
            );
            output = format!(
                "{}\nexport type TResponseHandler = (response: Protocol.{}) => void",
                output,
                request.get_response()?,
            );
            }
        output = format!(
            "{}\nexport type TErrHandler = (response: Protocol.{}) => void",
            output,
            request.get_err()?,
        );
        Ok(output)
    }

    fn get_handlers(&self, request: &Request) -> Result<String, String> {
        let mut output: String = String::from(templates::HANDLERS);
        let mut declarations: String = String::new();
        let mut init: String = String::new();
        if request.actions.len() > 1 {
            for action in &request.actions {
                if let Some(name) = action.conclusion.as_ref() {
                    declarations = format!(
                        "{}\n{}: T{}Handler | undefined;",
                        declarations,
                        name.to_lowercase(),
                        name,
                    );
                    init = format!(
                        "{}\n{}: undefined,",
                        init,
                        name.to_lowercase(),
                    );
                } else {
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
            }
        } else {
            declarations = String::from("\nresponse: TResponseHandler | undefined;");
            init = String::from("\nresponse: undefined,");
        }
        output = output.replace("[[declarations]]", &tools::inject_tabs(1, declarations));
        output = output.replace("[[init]]", &tools::inject_tabs(1, init));
        Ok(output)
    }

    fn get_handlers_defs(&self, request: &Request) -> Result<String, String> {
        let mut output: String = String::new();
        if request.actions.len() > 1 {
            for action in &request.actions {
                if let Some(name) = action.conclusion.as_ref() {
                    output = format!(
                        "{}\n{}: undefined,",
                        output,
                        name.to_lowercase(),
                    );
                } else {
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
            }
        } else {
            output = String::from("\nresponse: undefined,")
        }
        Ok(output)
    }

    fn get_handlers_setters(&self, request: &Request) -> Result<String, String> {
        let mut output: String = String::new();
        if request.actions.len() > 1 {
            for action in &request.actions {
                if let Some(name) = action.conclusion.as_ref() {
                    output = format!(
                        "{}\n{}",
                        output,
                        templates::HANDLER_SETTER
                            .replace("[[name]]", &name.to_lowercase())
                            .replace("[[type]]", &name)
                            .replace("[[reference]]", &(request.get_request()?).replace(".", "")),
                    );
                } else {
                    println!("{:?}", request);
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
            }
        } else {
            output = format!(
                "\n{}\n",
                templates::HANDLER_SETTER
                    .replace("[[name]]", "response")
                    .replace("[[type]]", "Response")
                    .replace("[[reference]]", &(request.get_request()?).replace(".", "")),
            );
        }
        Ok(output)
    }

    fn get_response_handler(&self, request: &Request) -> Result<String, String> {
        let reference: String = request.get_request()?;
        let parts: Vec<&str> = reference.split('.').collect();
        let mut check_group: String = String::from("if (message === undefined");
        let mut group: String = String::from("message");
        for (pos, part) in parts.iter().enumerate() {
            if pos < parts.len() - 1 {
                group = format!("{}.{}", group, part);
                check_group = format!("{} || {} === undefined", check_group, group);
            }
        }
        let mut output: String = format!(
r#"{}) {{
    return reject(new Error(`Expecting message from "{}" group.`));
}} "#, check_group, group);
        if request.actions.len() > 1 {
            for action in &request.actions {
                if let Some(name) = action.conclusion.as_ref() {
                    output = format!("{}{}", output,
r#"else if ([[group]].[[name]] !== undefined) {
    this._handlers.[[handler]] !== undefined && this._handlers.[[handler]]([[group]].[[name]]);
    return resolve([[group]].[[name]]);
} "#.replace("[[group]]", &group)
    .replace("[[name]]", &name)
    .replace("[[handler]]", &name.to_lowercase()));
                } else {
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
            }
        } else {
            output = format!("{}{}", output,
r#"else if (message.[[response]] !== undefined) {
    this._handlers.response !== undefined && this._handlers.response(message.[[response]]);
    return resolve(message.[[response]]);
} "#.replace("[[response]]", &request.get_response()?));
        }
        output = format!("{}{}", output,
r#"else if (message.[[error]] !== undefined) {
    this._handlers.err !== undefined && this._handlers.err(message.[[error]]);
    return resolve(message.[[error]]);
} else {
    return reject(new Error(`No message in "[[group]]" group.`));
}"#.replace("[[error]]", &request.get_err()?)
    .replace("[[group]]", &group));
        Ok(output)
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
        let dest = base.join("requests");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!("Fail to create dest folder {}. Error: {}", dest.to_string_lossy(), e));
            }
        }
        let request = request.get_request()?;
        Ok(dest.join(format!("{}.ts", request.to_lowercase())))
    }

    fn get_resolver_type(&self, request: &Request) -> Result<String, String> {
        Ok(format!("T{}Resolver", request.get_request()?.replace(".", "")))
    }

}

