import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TStructEmptyResolver = Protocol.StructEmptyA | Protocol.StructEmptyB;
export type TResponseHandler = (response: Protocol.StructEmptyB) => void
export type TErrHandler = (response: Protocol.StructEmptyA) => void

export class StructEmpty extends Protocol.StructEmpty {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        response: undefined,
        err: undefined,
    };
    constructor(request: Protocol.IStructEmpty) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TStructEmptyResolver> {
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
                console.log(message);
                switch (this._state) {
                    case ERequestState.Pending:
                        this._state = ERequestState.Ready;
                        if (message === undefined) {
                            return reject(new Error(`Expecting message from "message" group.`));
                        } else if (message.StructEmptyB !== undefined) {
                            this._handlers.response !== undefined && this._handlers.response(message.StructEmptyB);
                            return resolve(message.StructEmptyB);
                        } else if (message.StructEmptyA !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.StructEmptyA);
                            return resolve(message.StructEmptyA);
                        } else {
                            return reject(new Error(`No message in "message" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "StructEmpty" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "StructEmpty".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public response(handler: TResponseHandler): StructEmpty {
        this._handlers.response = handler;
        return this;
    }
    
    public err(handler: TErrHandler): StructEmpty {
        this._handlers.err = handler;
        return this;
    }

}
