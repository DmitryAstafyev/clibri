import * as Protocol from "../protocol/protocol";
import { Consumer } from "../index";
import { ERequestState } from "../interfaces/request";

export type TResponseHandler = () => void;
export type TErrHandler = (error: Error) => void;

export class BeaconsLikeMessage extends Protocol.Beacons.LikeMessage {
    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {
        response: undefined,
        err: undefined,
    };
    constructor(beacon: Protocol.Beacons.ILikeMessage) {
        super(beacon);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<void> {
        const consumer: Consumer | Error = Consumer.get();
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
                                    `Request "BeaconsLikeMessage" has been destroyed. Response would not be processed.`
                                )
                            );
                        case ERequestState.Pending:
                            return reject(
                                new Error(
                                    `Unexpected state for request "BeaconsLikeMessage".`
                                )
                            );
                    }
                })
                .catch((err: Error) => {
                    reject(err);
                });
        });
    }

    public response(handler: TResponseHandler): BeaconsLikeMessage {
        this._handlers.response = handler;
        return this;
    }

    public err(handler: TErrHandler): BeaconsLikeMessage {
        this._handlers.err = handler;
        return this;
    }
}