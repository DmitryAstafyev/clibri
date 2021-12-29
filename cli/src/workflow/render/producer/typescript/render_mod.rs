use super::{helpers, helpers::render as tools, workflow::store::Store, Protocol};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod templates {
    pub const MODULE: &str = r#"import { Consumer, Filter } from "./consumer";
import { ProducerError, ProducerErrorType } from "./events";
import { Context } from "../context";

import * as Events from "./events";
import * as Responses from "./responses";
import * as Beacons from "./beacons";
import * as Protocol from "./protocol";

import {
    Server,
    Subscription,
    Subject,
    IServerError,
    IServerReceivedEvent,
    Options,
    Logger,
    ConsumerErrorHandelingStrategy,
    ProducerIdentificationStrategy,
} from "clibri";

export class Producer {
    static hash: {
        PROTOCOL: string;
        WORKFLOW: string;
    } = {
        PROTOCOL:
            "[[protocol_hash]]",
        WORKFLOW:
            "[[workflow_hash]]",
    };
    private readonly _server: Server;
    private readonly _subscriptions: { [key: string]: Subscription } = {};
    private readonly _consumers: Map<string, Consumer> = new Map();
    private readonly _options: Options;
    private readonly _logger: Logger;
    private readonly _context: Context;

    public readonly events: {[[events_subjects_dec]]
    } = {[[events_subjects_def]]
    };

    constructor(server: Server, context: Context, options?: Options) {
        this._server = server;
        this._context = context;
        this._options = options === undefined ? new Options({}) : options;
        this._logger = this._options.logger.clone(`Producer`);
        this._subscriptions.ready = this._server
            .getEvents()
            .ready.subscribe(this._onServerReady.bind(this));
        this._subscriptions.connected = this._server
            .getEvents()
            .connected.subscribe(this._onClientConnected.bind(this));
        this._subscriptions.disconnected = this._server
            .getEvents()
            .disconnected.subscribe(this._onClientDisconnected.bind(this));
        this._subscriptions.error = this._server
            .getEvents()
            .error.subscribe(this._onServerError.bind(this));
        this._subscriptions.shutdown = this._server
            .getEvents()
            .shutdown.subscribe(this._onServerShutdown.bind(this));
        this._subscriptions.received = this._server
            .getEvents()
            .received.subscribe(this._onClientReceived.bind(this));[[events_subjects_sub]]
    }

    public destroy(): Promise<void> {
        Object.keys(this._subscriptions).forEach((key: string) => {
            this._subscriptions[key].destroy();
        });
        return this._server.shutdown();
    }

    public listen(): Promise<void> {
        let error: Error | undefined;
        return this._server
            .listen()
            .catch((err: Error) => {
                error = err;
            })
            .finally(() => {
                if (error !== undefined) {
                    const errMsg = error.message;
                    return this.destroy()
                        .catch((err: Error) => {
                            this._logger.err(
                                `Fail shutdown server: ${err.message}`
                            );
                        })
                        .finally(() => {
                            return Promise.reject(
                                new Error(
                                    this._logger.err(
                                        `Fail to start server; error: ${errMsg}`
                                    )
                                )
                            );
                        });
                } else {
                    return Promise.resolve();
                }
            });
    }

    public send(uuid: string, buffer: ArrayBufferLike): Promise<void> {
        return this._server.send(buffer, uuid);
    }

    public broadcast(uuids: string[], buffer: ArrayBufferLike): Promise<void> {
        if (uuids.length === 0) {
            return Promise.resolve();
        }
        return Promise.all(
            uuids.map((uuid) =>
                this._server.send(buffer, uuid).catch((err: Error) => {
                    this._logger.err(
                        `fail to send data to ${uuid}; error: ${err.message}`
                    );
                })
            )
        ).then(() => {
            return Promise.resolve();
        });
    }

    public disconnect(uuid: string): Promise<void> {
        return this._server.disconnect(uuid);
    }

    private _onServerReady() {
        this._checkErr(Events.readyHandler(this._context, this));
    }

    private _onServerShutdown() {
        this._checkErr(Events.shutdownHandler(this._context, this));
    }

    private _onServerError(error: IServerError) {
        this._checkErr(
            Events.errorHandler(
                error,
                this._context,
                this,
                undefined,
                undefined
            )
        );
    }

    private _onClientConnected(uuid: string) {
        if (this._consumers.has(uuid)) {
            this._checkErr(
                Events.errorHandler(
                    new ProducerError(
                        this._logger.warn(`Consumer ${uuid} already exist`),
                        ProducerErrorType.Connection
                    ),
                    this._context,
                    this,
                    undefined,
                    undefined
                )
            );
            return;
        }
        const consumer = new Consumer(uuid, this._options, this._logger);
        this._logger.debug(`new connection accepted: ${uuid}`);
        this._consumers.set(uuid, consumer);
        this._checkErr(
            Events.connectedHandler(
                consumer.getIdentification(),
                new Filter(this._consumers),
                this._context,
                this
            )
        );
    }

    private _onClientDisconnected(uuid: string) {
        const consumer = this._consumers.get(uuid);
        if (consumer === undefined) {
            this._checkErr(
                Events.errorHandler(
                    new ProducerError(
                        this._logger.warn(
                            `Attempt to disconnect consumer ${uuid}; but consumer doesn't exist`
                        ),
                        ProducerErrorType.Disconnection
                    ),
                    this._context,
                    this,
                    undefined,
                    undefined
                )
            );
            return;
        }
        this._logger.debug(`consumer ${uuid} is disconnected`);
        this._consumers.delete(uuid);
        this._checkErr(
            Events.disconnectedHandler(
                consumer.getIdentification(),
                new Filter(this._consumers),
                this._context,
                this
            )
        );
    }

    private _onClientReceived(event: IServerReceivedEvent) {
        this._logger.verb(
            `new chunk of data from ${event.uuid} has been gotten; ${event.buffer.byteLength} bytes.`
        );
        const consumer = this._consumers.get(event.uuid);
        if (consumer === undefined) {
            return this._receivingErr(
                new Error(
                    `Received data for consumer ${event.uuid}, but consumer doesn't exist`
                ),
                event.uuid,
                undefined
            );
        }
        if (consumer.getIdentification().isDiscredited()) {
            return this._receivingErr(
                new Error(
                    `consumer ${event.uuid} discredited, received data would be ignored`
                ),
                event.uuid,
                consumer
            );
        }
        const error = consumer.chunk(event.buffer);
        if (error instanceof Error) {
            return this._receivingErr(error, event.uuid, consumer);
        }
        do {
            const message = consumer.message();
            if (message === undefined) {
                return;
            }
            let extracted = this._extract(
                message.msg,
                "[[self_key]]"
            );
            if (extracted.exist && !consumer.isHashAccepted()) {
                this._logger.debug(
                    `consumer ${event.uuid} requested identification`
                );
                const assignedUuid = consumer
                    .getIdentification()
                    .key(
                        extracted.body<Protocol.[[self_key]]>(),
                        true
                    );
                this.send(
                    assignedUuid,
                    new Protocol.InternalServiceGroup.SelfKeyResponse({
                        uuid: assignedUuid,
                    }).pack(message.header.sequence, assignedUuid)
                ).catch((err: Error) => {
                    this._logger.err(
                        `fail to send identification confirmation to ${assignedUuid}; error: ${err.message}`
                    );
                });
                continue;
            }
            extracted = this._extract(
                message.msg,
                "InternalServiceGroup.HashRequest"
            );
            if (extracted.exist) {
                this._logger.debug(
                    `consumer ${event.uuid} requested hash check`
                );
                let hashErr: ProducerError | undefined;
                const consumerHash =
                    extracted.body<Protocol.InternalServiceGroup.HashRequest>();
                if (consumerHash.protocol !== Producer.hash.PROTOCOL) {
                    hashErr = new ProducerError(
                        this._logger.warn(
                            `Consumer ${event.uuid} has invalid protocol; valid protocol hash ${Producer.hash.PROTOCOL}; consumer protocol: ${consumerHash.protocol}.`
                        ),
                        ProducerErrorType.ProtocolHash
                    );
                } else if (consumerHash.workflow !== Producer.hash.WORKFLOW) {
                    hashErr = new ProducerError(
                        this._logger.warn(
                            `Consumer ${event.uuid} has invalid workflow; valid workflow hash ${Producer.hash.WORKFLOW}; consumer workflow: ${consumerHash.workflow}.`
                        ),
                        ProducerErrorType.WorkflowHash
                    );
                }
                this.send(
                    event.uuid,
                    new Protocol.InternalServiceGroup.HashResponse({
                        error:
                            hashErr === undefined ? undefined : `Invalid hash`,
                    }).pack(message.header.sequence, event.uuid)
                ).catch((err: Error) => {
                    this._logger.err(
                        `Fail to send hash check response to ${event.uuid}; error: ${err.message}`
                    );
                });
                if (hashErr !== undefined) {
                    this.disconnect(event.uuid).catch((err: Error) => {
                        this._logger.err(
                            `Fail to disconnect consumer ${event.uuid} because hash; error: ${err.message}`
                        );
                    });
                    this._checkErr(
                        Events.errorHandler(
                            hashErr,
                            this._context,
                            this,
                            consumer.getIdentification(),
                            new Filter(this._consumers)
                        )
                    );
                } else {
                    consumer.acceptHash();
                }
                continue;
            }
            if (!consumer.isHashAccepted()) {
                this.disconnect(event.uuid).catch((err: Error) => {
                    this._logger.err(
                        `Fail to disconnect consumer ${event.uuid} because not accepted hash; error: ${err.message}`
                    );
                });
                this._checkErr(
                    Events.errorHandler(
                        new ProducerError(
                            this._logger.warn(
                                `Consumer ${event.uuid} has not checked and not accepted hash`
                            ),
                            ProducerErrorType.HashError
                        ),
                        this._context,
                        this,
                        consumer.getIdentification(),
                        new Filter(this._consumers)
                    )
                );
                continue;
            }
            if (!consumer.getIdentification().hasKey()) {
                this.disconnect(event.uuid).catch((err: Error) => {
                    this._logger.err(
                        `Fail to disconnect consumer ${event.uuid} because key isn't setup; error: ${err.message}`
                    );
                });
                this._checkErr(
                    Events.errorHandler(
                        new ProducerError(
                            this._logger.warn(
                                `Consumer ${event.uuid} isn't setup self-key`
                            ),
                            ProducerErrorType.KeyError
                        ),
                        this._context,
                        this,
                        consumer.getIdentification(),
                        new Filter(this._consumers)
                    )
                );
                continue;
            }
            if (
                !consumer.getIdentification().assigned() &&
                this._options.producerIndentificationStrategy !==
                    ProducerIdentificationStrategy.Ignore
            ) {
                if (
                    this._options.producerIndentificationStrategy ===
                    ProducerIdentificationStrategy.Log
                ) {
                    this._logger.info(
                        `Consumer ${event.uuid} doesn't have assigned key.`
                    );
                }
                if (
                    this._options.producerIndentificationStrategy ===
                        ProducerIdentificationStrategy.Disconnect ||
                    this._options.producerIndentificationStrategy ===
                        ProducerIdentificationStrategy.EmitErrorAndDisconnect
                ) {
                    this.disconnect(event.uuid).catch((err: Error) => {
                        this._logger.err(
                            `Fail to disconnect consumer ${event.uuid} because assigned key isn't setup; error: ${err.message}`
                        );
                    });
                }
                if (
                    this._options.producerIndentificationStrategy ===
                        ProducerIdentificationStrategy.EmitError ||
                    this._options.producerIndentificationStrategy ===
                        ProducerIdentificationStrategy.EmitErrorAndDisconnect
                ) {
                    this._checkErr(
                        Events.errorHandler(
                            new ProducerError(
                                this._logger.warn(
                                    `Consumer ${event.uuid} isn't setup assigned-key`
                                ),
                                ProducerErrorType.AssignedKeyError
                            ),
                            this._context,
                            this,
                            consumer.getIdentification(),
                            new Filter(this._consumers)
                        )
                    );
                }
                continue;
            }[[requests]][[beacons]]
            this._receivingErr(
                new Error(
                    `unknown message from ${event.uuid} has been received`
                ),
                event.uuid,
                consumer
            );
			this._logger.err(
				`Unknown message header: id=${message.header.id}; sequence=${message.header.sequence}`
			);
			try {
				const msgStrBody = JSON.stringify(message.msg);
				this._logger.verb(
					`Unknown message body: ${msgStrBody.substr(
						0,
						msgStrBody.length > 500 ? 500 : msgStrBody.length
					)}`
				);
			} catch (_) {}
            break;
        } while (true);
    }

    private _receivingErr(
        error: Error,
        uuid: string,
        consumer: Consumer | undefined
    ): void {
        if (
            this._options.consumerErrorHandelingStrategy ===
            ConsumerErrorHandelingStrategy.Log
        ) {
            this._logger.warn(`consumer ${uuid} gives error: ${error.message}`);
        }
        if (
            this._options.consumerErrorHandelingStrategy ===
                ConsumerErrorHandelingStrategy.Disconnect ||
            this._options.consumerErrorHandelingStrategy ===
                ConsumerErrorHandelingStrategy.EmitErrorAndDisconnect
        ) {
            if (consumer !== undefined) {
                this.disconnect(uuid)
                    .then(() => {
                        this._logger.debug(
                            `consumer ${uuid} has been disconnected`
                        );
                    })
                    .catch((err: Error) => {
                        this._logger.err(
                            `fail to disconnect ${uuid}; error: ${err.message}`
                        );
                    });
            }
        }
        if (
            this._options.consumerErrorHandelingStrategy ===
                ConsumerErrorHandelingStrategy.EmitError ||
            this._options.consumerErrorHandelingStrategy ===
                ConsumerErrorHandelingStrategy.EmitErrorAndDisconnect
        ) {
            this._checkErr(
                Events.errorHandler(
                    new ProducerError(
                        this._logger.warn(
                            `consumer ${uuid} error: ${error.message}`
                        ),
                        ProducerErrorType.ProcessingIncomeData
                    ),
                    this._context,
                    this,
                    consumer !== undefined
                        ? consumer.getIdentification()
                        : undefined,
                    consumer !== undefined
                        ? new Filter(this._consumers)
                        : undefined
                )
            );
        }
    }

    private _checkErrAndResponse(
        handler: Promise<void>,
        uuid: string,
        consumer: Consumer | undefined
    ) {
        handler.catch((error: Error) => {
            this._receivingErr(error, uuid, consumer);
        });
    }

    private _checkErr(handler: Promise<void>) {
        handler.catch((error: Error) => {
            this._logger.err(error.message);
        });
    }

    private _tryToGet(src: any, path: string): any {
        if (typeof src !== "object" || src === undefined || src === null) {
            return undefined;
        }
        let target: any = src;
        path.split(".").forEach((part: string) => {
            if (target === undefined) {
                return;
            }
            target = (target as any)[part];
            if (
                typeof target !== "object" ||
                target === undefined ||
                target === null
            ) {
                target = undefined;
            }
        });
        return target;
    }

    private _extract(
        src: any,
        path: string
    ): {
        exist: boolean;
        body<T>(): T;
    } {
        const target = this._tryToGet(src, path);
        return {
            exist: target !== undefined,
            body<T>(): T {
                if (target === undefined) {
                    throw new Error(`Fail to extract message by path ${path}`);
                }
                return target as T;
            },
        };
    }
}"#;
    pub const REQUEST: &str = r#"extracted = this._extract(message.msg, "[[ref]]");
if (extracted.exist) {
    this._checkErrAndResponse(
        Responses.[[handler]](
            extracted.body<Protocol.[[ref]]>(),
            consumer.getIdentification(),
            new Filter(this._consumers),
            this._context,
            this,
            message.header.sequence
        ),
        event.uuid,
        consumer
    );
    continue;
}"#;
    pub const BEACON: &str = r#"extracted = this._extract(message.msg, "[[ref]]");
if (extracted.exist) {
    this._checkErrAndResponse(
        Beacons.[[handler]](
            extracted.body<Protocol.[[ref]]>(),
            consumer.getIdentification(),
            new Filter(this._consumers),
            this._context,
            this,
            message.header.sequence
        ),
        event.uuid,
        consumer
    );
    continue;
}"#;
    pub const EVENT_SUBSCRIPTION: &str = r#"this._subscriptions.[[handler]]Sub = this.events.[[event]].subscribe(
    (event: Protocol.[[ref]]) => {
        this._checkErr(
            Events.[[handler]](
                event,
                new Filter(this._consumers),
                this._context,
                this
            )
        );
    }
);"#;
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

    pub fn render(&self, base: &Path, store: &Store, protocol: &Protocol) -> Result<(), String> {
        let dest: PathBuf = self.get_dest_file(base)?;
        let mut output = templates::MODULE.to_owned();
        output = output.replace("[[requests]]", &self.get_requests(store)?);
        output = output.replace("[[beacons]]", &self.get_beacons(store)?);
        output = output.replace(
            "[[events_subjects_dec]]",
            &self.get_events_subjects_dec(store)?,
        );
        output = output.replace(
            "[[events_subjects_def]]",
            &self.get_events_subjects_def(store)?,
        );
        output = output.replace(
            "[[events_subjects_sub]]",
            &self.get_events_subjects_sub(store)?,
        );
        output = output.replace("[[self_key]]", &store.get_config()?.get_self()?);
        output = output.replace("[[protocol_hash]]", &protocol.get_hash());
        output = output.replace("[[workflow_hash]]", &store.get_hash());
        helpers::fs::write(dest, output, true)
    }

    fn get_requests(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for request in store.requests.iter() {
            let mut request_output: String = String::from(templates::REQUEST);
            request_output = request_output.replace("[[ref]]", &request.get_request()?);
            request_output = request_output.replace(
                "[[handler]]",
                &format!(
                    "{}Handler",
                    helpers::string::first_letter_lowercase(
                        &request.get_request()?.replace(".", "")
                    )
                ),
            );
            output = format!("{}\n{}", output, request_output);
        }
        Ok(tools::inject_tabs(3, output))
    }

    fn get_beacons(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for beacon in store.beacons.iter() {
            let mut beacon_output: String = String::from(templates::BEACON);
            beacon_output = beacon_output.replace("[[ref]]", &beacon.reference);
            beacon_output = beacon_output.replace(
                "[[handler]]",
                &format!(
                    "{}Handler",
                    helpers::string::first_letter_lowercase(&beacon.reference.replace(".", ""))
                ),
            );
            output = format!("{}\n{}", output, beacon_output);
        }
        Ok(tools::inject_tabs(3, output))
    }

    fn get_events_subjects_dec(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for event in store.events.iter() {
            if !event.is_default() {
                output = format!(
                    "{}\n{}: Subject<Protocol.{}>;",
                    output,
                    helpers::string::first_letter_lowercase(
                        &event.get_reference()?.replace(".", "")
                    ),
                    event.get_reference()?
                );
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_events_subjects_def(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for event in store.events.iter() {
            if !event.is_default() {
                output = format!(
                    "{}\n{}: new Subject<Protocol.{}>(),",
                    output,
                    helpers::string::first_letter_lowercase(
                        &event.get_reference()?.replace(".", "")
                    ),
                    event.get_reference()?
                );
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_events_subjects_sub(&self, store: &Store) -> Result<String, String> {
        let mut output: String = String::new();
        for event in store.events.iter() {
            if !event.is_default() {
                let mut sub = templates::EVENT_SUBSCRIPTION.to_owned();
                sub = sub.replace(
                    "[[handler]]",
                    &format!(
                        "{}Handler",
                        helpers::string::first_letter_lowercase(
                            &event.get_reference()?.replace(".", "")
                        )
                    ),
                );
                sub = sub.replace(
                    "[[event]]",
                    &helpers::string::first_letter_lowercase(
                        &event.get_reference()?.replace(".", ""),
                    ),
                );
                sub = sub.replace("[[ref]]", &event.get_reference()?);
                output = format!("{}\n{}", output, sub);
            }
        }
        Ok(tools::inject_tabs(2, output))
    }

    fn get_dest_file(&self, base: &Path) -> Result<PathBuf, String> {
        let dest = base.join("implementation");
        if !dest.exists() {
            if let Err(e) = fs::create_dir(&dest) {
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
