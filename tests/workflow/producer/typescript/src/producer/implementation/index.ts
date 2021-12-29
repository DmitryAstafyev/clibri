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
            "2FE9D6137375F6B74B81143B6CA65EEAE6124B6C03C78937C4583DF0B0EF757A",
        WORKFLOW:
            "429F4C595CF69B2A040303F3A7F626CB1188AEB79DBC9DB8AB314ABA1601C1C9",
    };
    private readonly _server: Server;
    private readonly _subscriptions: { [key: string]: Subscription } = {};
    private readonly _consumers: Map<string, Consumer> = new Map();
    private readonly _options: Options;
    private readonly _logger: Logger;
    private readonly _context: Context;

    public readonly events: {        
        eventA: Subject<Protocol.EventA>;
        eventB: Subject<Protocol.EventB>;
        eventsEventA: Subject<Protocol.Events.EventA>;
        eventsEventB: Subject<Protocol.Events.EventB>;
        eventsSubEventA: Subject<Protocol.Events.Sub.EventA>;
        triggerBeaconsEmitter: Subject<Protocol.TriggerBeaconsEmitter>;
        finishConsumerTest: Subject<Protocol.FinishConsumerTest>;
    } = {        
        eventA: new Subject<Protocol.EventA>(),
        eventB: new Subject<Protocol.EventB>(),
        eventsEventA: new Subject<Protocol.Events.EventA>(),
        eventsEventB: new Subject<Protocol.Events.EventB>(),
        eventsSubEventA: new Subject<Protocol.Events.Sub.EventA>(),
        triggerBeaconsEmitter: new Subject<Protocol.TriggerBeaconsEmitter>(),
        finishConsumerTest: new Subject<Protocol.FinishConsumerTest>(),
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
        this._subscriptions.eventAHandlerSub = this.events.eventA.subscribe(
            (event: Protocol.EventA) => {
                this._checkErr(
                    Events.eventAHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.eventBHandlerSub = this.events.eventB.subscribe(
            (event: Protocol.EventB) => {
                this._checkErr(
                    Events.eventBHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.eventsEventAHandlerSub = this.events.eventsEventA.subscribe(
            (event: Protocol.Events.EventA) => {
                this._checkErr(
                    Events.eventsEventAHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.eventsEventBHandlerSub = this.events.eventsEventB.subscribe(
            (event: Protocol.Events.EventB) => {
                this._checkErr(
                    Events.eventsEventBHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.eventsSubEventAHandlerSub = this.events.eventsSubEventA.subscribe(
            (event: Protocol.Events.Sub.EventA) => {
                this._checkErr(
                    Events.eventsSubEventAHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.triggerBeaconsEmitterHandlerSub = this.events.triggerBeaconsEmitter.subscribe(
            (event: Protocol.TriggerBeaconsEmitter) => {
                this._checkErr(
                    Events.triggerBeaconsEmitterHandler(
                        event,
                        new Filter(this._consumers),
                        this._context,
                        this
                    )
                );
            }
        );
        this._subscriptions.finishConsumerTestHandlerSub = this.events.finishConsumerTest.subscribe(
            (event: Protocol.FinishConsumerTest) => {
                this._checkErr(
                    Events.finishConsumerTestHandler(
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
                "StructA"
            );
            if (extracted.exist && !consumer.isHashAccepted()) {
                this._logger.debug(
                    `consumer ${event.uuid} requested identification`
                );
                const assignedUuid = consumer
                    .getIdentification()
                    .key(
                        extracted.body<Protocol.StructA>(),
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
            extracted = this._extract(message.msg, "StructA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.structAHandler(
                        extracted.body<Protocol.StructA>(),
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
            extracted = this._extract(message.msg, "StructC");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.structCHandler(
                        extracted.body<Protocol.StructC>(),
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
            extracted = this._extract(message.msg, "StructD");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.structDHandler(
                        extracted.body<Protocol.StructD>(),
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
            extracted = this._extract(message.msg, "StructF");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.structFHandler(
                        extracted.body<Protocol.StructF>(),
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
            extracted = this._extract(message.msg, "StructEmpty");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.structEmptyHandler(
                        extracted.body<Protocol.StructEmpty>(),
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
            extracted = this._extract(message.msg, "GroupA.StructA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.groupAStructAHandler(
                        extracted.body<Protocol.GroupA.StructA>(),
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
            extracted = this._extract(message.msg, "GroupA.StructB");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.groupAStructBHandler(
                        extracted.body<Protocol.GroupA.StructB>(),
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
            extracted = this._extract(message.msg, "GroupB.GroupC.StructA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.groupBGroupCStructAHandler(
                        extracted.body<Protocol.GroupB.GroupC.StructA>(),
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
            extracted = this._extract(message.msg, "GroupB.StructA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.groupBStructAHandler(
                        extracted.body<Protocol.GroupB.StructA>(),
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
            extracted = this._extract(message.msg, "GroupB.GroupC.StructB");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Responses.groupBGroupCStructBHandler(
                        extracted.body<Protocol.GroupB.GroupC.StructB>(),
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
            extracted = this._extract(message.msg, "BeaconA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconAHandler(
                        extracted.body<Protocol.BeaconA>(),
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
            extracted = this._extract(message.msg, "Beacons.BeaconA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsBeaconAHandler(
                        extracted.body<Protocol.Beacons.BeaconA>(),
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
            extracted = this._extract(message.msg, "Beacons.BeaconB");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsBeaconBHandler(
                        extracted.body<Protocol.Beacons.BeaconB>(),
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
            extracted = this._extract(message.msg, "Beacons.Sub.BeaconA");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsSubBeaconAHandler(
                        extracted.body<Protocol.Beacons.Sub.BeaconA>(),
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
            extracted = this._extract(message.msg, "Beacons.ShutdownServer");
            if (extracted.exist) {
                this._checkErrAndResponse(
                    Beacons.beaconsShutdownServerHandler(
                        extracted.body<Protocol.Beacons.ShutdownServer>(),
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
}