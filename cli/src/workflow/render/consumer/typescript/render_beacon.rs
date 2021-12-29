use super::{helpers, workflow::beacon::Broadcast};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import * as Protocol from "../protocol/protocol";
import { Consumer } from "../index";
import { ERequestState } from "../interfaces/request";

export type TResponseHandler = () => void;
export type TErrHandler = (error: Error) => void;

export class [[reference]] extends Protocol.[[struct_ref]] {
    private _consumer: Consumer | undefined;
    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {
        response: undefined,
        err: undefined,
    };
    constructor(beacon: Protocol.[[struct_interface]], consumer?: Consumer) {
        super(beacon);
        this._consumer = consumer;
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<void> {
		const consumer: Consumer | Error =
			this._consumer !== undefined ? this._consumer : Consumer.get();
        if (consumer instanceof Error) {
            return Promise.reject(consumer);
        }
        if (this._state === ERequestState.Pending) {
            return Promise.reject(
                new Error(`Cannot send request while previous isn't finished`)
            );
        }
        if (this._state === ERequestState.Destroyed) {
            return Promise.reject(
                new Error(`Cannot send request as soon as it's destroyed`)
            );
        }
        const sequence: number = consumer.getSequence();
        this._state = ERequestState.Pending;
        return new Promise((resolve, reject) => {
            consumer
                .request(this.pack(sequence), sequence)
                .then((response: Protocol.IAvailableMessages) => {
                    switch (this._state) {
                        case ERequestState.Pending:
                            this._state = ERequestState.Ready;
                            let error: Error | undefined;
                            if (response.InternalServiceGroup === undefined) {
                                error = new Error(
                                    `Expecting message from "InternalServiceGroup" group.`
                                );
                            } else if (
                                response.InternalServiceGroup
                                    .BeaconConfirmation === undefined
                            ) {
                                error = new Error(
                                    `Expecting message "InternalServiceGroup.Confirmation".`
                                );
                            } else if (
                                typeof response.InternalServiceGroup
                                    .BeaconConfirmation.error === "string"
                            ) {
                                error = new Error(
                                    response.InternalServiceGroup.BeaconConfirmation.error
                                );
                            }
                            if (error instanceof Error) {
                                this._handlers.err !== undefined &&
                                    this._handlers.err(error);
                                reject(error);
                            } else {
                                this._handlers.response !== undefined &&
                                    this._handlers.response();
                                resolve();
                            }
                        case ERequestState.Destroyed:
                            return reject(
                                new Error(
                                    `Request "[[reference]]" has been destroyed. Response would not be processed.`
                                )
                            );
                        case ERequestState.Pending:
                            return reject(
                                new Error(
                                    `Unexpected state for request "[[reference]]".`
                                )
                            );
                    }
                })
                .catch((err: Error) => {
                    reject(err);
                });
        });
    }

    public response(handler: TResponseHandler): [[reference]] {
        this._handlers.response = handler;
        return this;
    }

    public err(handler: TErrHandler): [[reference]] {
        this._handlers.err = handler;
        return this;
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

    pub fn render(&self, base: &Path, beacon: &Broadcast) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base, beacon)?;
        let mut output: String = templates::MODULE.to_owned();
        output = output.replace("[[reference]]", &(beacon.reference).replace(".", ""));
        output = output.replace("[[struct_ref]]", &beacon.reference);
        output = output.replace("[[struct_interface]]", &self.get_beacon_interface(beacon)?);
        helpers::fs::write(dest, output, true)
    }

    fn get_beacon_interface(&self, beacon: &Broadcast) -> Result<String, String> {
        let full = beacon.reference.clone();
        let mut parts: Vec<&str> = full.split('.').collect();
        if parts.is_empty() {
            Err(String::from("Invalid reference to struct for request"))
        } else {
            let ref_to_interface = format!("I{}", parts[parts.len() - 1]);
            let last = parts.len() - 1;
            parts[last] = &ref_to_interface;
            Ok(parts.join("."))
        }
    }

    fn get_dest_file(&self, base: &Path, beacon: &Broadcast) -> Result<PathBuf, String> {
        let dest = base.join("beacons");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
                return Err(format!(
                    "Fail to create dest folder {}. Error: {}",
                    dest.to_string_lossy(),
                    e
                ));
            }
        }
        Ok(dest.join(format!("{}.ts", beacon.reference.to_lowercase())))
    }
}
