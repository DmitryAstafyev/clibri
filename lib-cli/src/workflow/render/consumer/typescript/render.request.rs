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
import { ERequestState } from '../interfaces/request.states';

[[types_declarations]]

export class [[reference]] extends Protocol.UserLogin.Request {

    private _state: ERequestState = ERequestState.Ready;
    [[handlers]]
    constructor(request: Protocol.UserLogin.IRequest) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {
            accept: undefined,
            deny: undefined,
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
                        if (message.UserLogin === undefined) {
                            return reject(new Error(`Expecting message from "UserLogin" group.`));
                        } else if (message.UserLogin.Accepted !== undefined) {
                            this._handlers.accept !== undefined && this._handlers.accept(message.UserLogin.Accepted);
                            return resolve(message.UserLogin.Accepted);
                        } else if (message.UserLogin.Denied !== undefined) {
                            this._handlers.deny !== undefined && this._handlers.deny(message.UserLogin.Denied);
                            return resolve(message.UserLogin.Denied);
                        } else if (message.UserLogin.Err !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.UserLogin.Err);
                            return resolve(message.UserLogin.Err);
                        } else {
                            return reject(new Error(`No message in "UserLogin" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserLogin" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserLogin".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }

    public accept(handler: TAcceptHandler): UserLogin {
        this._handlers.accept = handler;
        return this;
    }

    public deny(handler: TDenyHandler): UserLogin {
        this._handlers.deny = handler;
        return this;
    }

    public err(handler: TErrHandler): UserLogin {
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
        output = output.replace("[[handlers]]", &self.get_handlers(request)?);
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
                    println!("{:?}", request);
                    return Err(String::from("Action doesn't have bound conclusion name"));
                };
            }
        }
        output = output.replace("[[declarations]]", &tools::inject_tabs(2, declarations));
        output = output.replace("[[init]]", &tools::inject_tabs(2, init));
        Ok(output)
    }

    fn get_dest_file(&self, base: &Path, request: &Request) -> Result<PathBuf, String> {
        let dest = base.join("declarations");
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

