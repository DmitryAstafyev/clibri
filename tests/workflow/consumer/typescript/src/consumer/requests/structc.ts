import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TStructCResolver = Protocol.StructE | Protocol.StructB | Protocol.StructF | Protocol.StructD;
export type TCaseBHandler = (response: Protocol.StructB) => void
export type TCaseFHandler = (response: Protocol.StructF) => void
export type TCaseDHandler = (response: Protocol.StructD) => void
export type TErrHandler = (response: Protocol.StructE) => void

export class StructC extends Protocol.StructC {
    private _consumer: Consumer | undefined;
    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        caseb: TCaseBHandler | undefined;
        casef: TCaseFHandler | undefined;
        cased: TCaseDHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        caseb: undefined,
        casef: undefined,
        cased: undefined,
        err: undefined,
    };
    constructor(request: Protocol.IStructC, consumer?: Consumer) {
        super(request);
        this._consumer = consumer;
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            caseb: undefined,
            casef: undefined,
            cased: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TStructCResolver> {
		const consumer: Consumer | Error =
			this._consumer !== undefined ? this._consumer : Consumer.get();
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
                        if (message === undefined) {
                            return reject(new Error(`Expecting message for "StructC".`));
                        } else if (message.StructB !== undefined) {
                            this._handlers.caseb !== undefined && this._handlers.caseb(message.StructB);
                            return resolve(message.StructB);
                        } else if (message.StructF !== undefined) {
                            this._handlers.casef !== undefined && this._handlers.casef(message.StructF);
                            return resolve(message.StructF);
                        } else if (message.StructD !== undefined) {
                            this._handlers.cased !== undefined && this._handlers.cased(message.StructD);
                            return resolve(message.StructD);
                        } else if (message.StructE !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.StructE);
                            return resolve(message.StructE);
                        } else {
                            return reject(new Error(`No message in "message" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "StructC" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "StructC".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public caseb(handler: TCaseBHandler): StructC {
        this._handlers.caseb = handler;
        return this;
    }
    public casef(handler: TCaseFHandler): StructC {
        this._handlers.casef = handler;
        return this;
    }
    public cased(handler: TCaseDHandler): StructC {
        this._handlers.cased = handler;
        return this;
    }
    public err(handler: TErrHandler): StructC {
        this._handlers.err = handler;
        return this;
    }

}
