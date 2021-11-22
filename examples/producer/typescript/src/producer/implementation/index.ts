import { Consumer, Filter } from "./consumer";
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
            "F63F41ECDA9067B12F9F9CF312473B95E472CC39C08A02CC8C37738EF34DCCBE",
        WORKFLOW:
            "E7D70BA4B25E620214D8F4D926E5162ACE2D1C6A9758DDEDEDCBAEC35FF91FC5",
    };
    private readonly _server: Server;
    private readonly _subscriptions: { [key: string]: Subscription } = {};
    private readonly _consumers: Map<string, Consumer> = new Map();
    private readonly _options: Options;
    private readonly _logger: Logger;
    private readonly _context: Context;

    public readonly events: {        
        serverEventsUserKickOff: Subject<Protocol.ServerEvents.UserKickOff>;
        serverEventsUserAlert: Subject<Protocol.ServerEvents.UserAlert>;
    } = {        
        serverEventsUserKickOff: new Subject<Protocol.ServerEvents.UserKickOff>(),
        serverEventsUserAlert: new Subject<Protocol.ServerEvents.UserAlert>(),
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
            .received.subscribe(this._onClientReceived.bind(this));        
        this._subscriptions.serverEventsUserKickOffHandlerSub = this.events.serverEventsUserKickOff.subscribe(
            (event: Protocol.ServerEvents.UserKickOff) => {
                this._checkErr(
                    Events.serverEventsUserKickOffHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.serverEventsUserAlertHandlerSub = this.events.serverEventsUserAlert.subscribe(
            (event: Protocol.ServerEvents.UserAlert) => {
                this._checkErr(
                    Events.serverEventsUserAlertHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
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
                "Identification.SelfKey"
            );
            if (extracted.exist) {
                this._logger.debug(
                    `consumer ${event.uuid} requested identification`
                );
                const assignedUuid = consumer
                    .getIdentification()
                    .key(
                        extracted.body<Protocol.Identification.SelfKey>(),
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
            }            
            extracted = this._extract(message.msg, "UserLogin.Request");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.userLoginRequestHandler(
                        extracted.body<Protocol.UserLogin.Request>(),
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
            }
            extracted = this._extract(message.msg, "Users.Request");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.usersRequestHandler(
                        extracted.body<Protocol.Users.Request>(),
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
            }
            extracted = this._extract(message.msg, "Message.Request");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.messageRequestHandler(
                        extracted.body<Protocol.Message.Request>(),
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
            }
            extracted = this._extract(message.msg, "Messages.Request");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.messagesRequestHandler(
                        extracted.body<Protocol.Messages.Request>(),
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
            }            
            extracted = this._extract(message.msg, "Beacons.LikeUser");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsLikeUserHandler(
                        extracted.body<Protocol.Beacons.LikeUser>(),
                        consumer.getIdentification(),
                        new Filter(this._consumers),
                        this._context,
                        this,
                        message.header.sequence
                    ),
                    event.uuid,
                    consumer
                );
            }
            extracted = this._extract(message.msg, "Beacons.LikeMessage");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsLikeMessageHandler(
                        extracted.body<Protocol.Beacons.LikeMessage>(),
                        consumer.getIdentification(),
                        new Filter(this._consumers),
                        this._context,
                        this,
                        message.header.sequence
                    ),
                    event.uuid,
                    consumer
                );
            }
            this._receivingErr(
                new Error(
                    `unknown message from ${event.uuid} has been received`
                ),
                event.uuid,
                consumer
            );
            this._logger.err(
                `Unknown message header: ${JSON.stringify(message.header)}`
            );
            const msgStrBody = JSON.stringify(message.msg);
            this._logger.verb(
                `Unknown message body: ${msgStrBody.substr(
                    0,
                    msgStrBody.length > 500 ? 500 : msgStrBody.length
                )}`
            );
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
}