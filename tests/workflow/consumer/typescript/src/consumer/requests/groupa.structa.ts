import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TGroupAStructAResolver = Protocol.GroupA.StructB | Protocol.StructA | Protocol.StructB;
export type TRootAHandler = (response: Protocol.StructA) => void
export type TRootBHandler = (response: Protocol.StructB) => void
export type TErrHandler = (response: Protocol.GroupA.StructB) => void

export class GroupAStructA extends Protocol.GroupA.StructA {
    private _consumer: Consumer | undefined;
    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        roota: TRootAHandler | undefined;
        rootb: TRootBHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        roota: undefined,
        rootb: undefined,
        err: undefined,
    };
    constructor(request: Protocol.GroupA.IStructA, consumer?: Consumer) {
        super(request);
        this._consumer = consumer;
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            roota: undefined,
            rootb: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TGroupAStructAResolver> {
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
                            return reject(new Error(`Expecting message for "GroupA.StructA".`));
                        } else if (message.StructA !== undefined) {
                            this._handlers.roota !== undefined && this._handlers.roota(message.StructA);
                            return resolve(message.StructA);
                        } else if (message.StructB !== undefined) {
                            this._handlers.rootb !== undefined && this._handlers.rootb(message.StructB);
                            return resolve(message.StructB);
                        } else if (message !== undefined && message.GroupA !== undefined && message.GroupA.StructB !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.GroupA.StructB);
                            return resolve(message.GroupA.StructB);
                        } else {
                            return reject(new Error(`No message in "message.GroupA" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "GroupAStructA" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "GroupAStructA".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public roota(handler: TRootAHandler): GroupAStructA {
        this._handlers.roota = handler;
        return this;
    }
    public rootb(handler: TRootBHandler): GroupAStructA {
        this._handlers.rootb = handler;
        return this;
    }
    public err(handler: TErrHandler): GroupAStructA {
        this._handlers.err = handler;
        return this;
    }

}
