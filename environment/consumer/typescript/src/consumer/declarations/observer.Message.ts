import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request.states';

export type TMessageResponse = Protocol.Message.Accepted | Protocol.Message.Denied | Protocol.Message.Err;
export type TAcceptHandler = (response: Protocol.Message.Accepted) => void;
export type TDenyHandler = (response: Protocol.Message.Denied) => void;
export type TErrHandler = (response: Protocol.Message.Err) => void;

export class Message extends Protocol.Message.Request {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {
        accept: TAcceptHandler | undefined;
        deny: TDenyHandler | undefined;
        err: TErrHandler | undefined;
    } = {
        accept: undefined,
        deny: undefined,
        err: undefined,
    };

    constructor(request: Protocol.Message.IRequest) {
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

    public send(): Promise<TMessageResponse> {
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
                        if (message.Message === undefined) {
                            return reject(new Error(`Expecting message from "Message" group.`));
                        } else if (message.Message.Accepted !== undefined) {
                            this._handlers.accept !== undefined && this._handlers.accept(message.Message.Accepted);
                            return resolve(message.Message.Accepted);
                        } else if (message.Message.Denied !== undefined) {
                            this._handlers.deny !== undefined && this._handlers.deny(message.Message.Denied);
                            return resolve(message.Message.Denied);
                        } else if (message.Message.Err !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.Message.Err);
                            return resolve(message.Message.Err);
                        } else {
                            return reject(new Error(`No message in "Message" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "Message" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "Message".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }

    public accept(handler: TAcceptHandler): Message {
        this._handlers.accept = handler;
        return this;
    }

    public deny(handler: TDenyHandler): Message {
        this._handlers.deny = handler;
        return this;
    }

    public err(handler: TErrHandler): Message {
        this._handlers.err = handler;
        return this;
    }

}
