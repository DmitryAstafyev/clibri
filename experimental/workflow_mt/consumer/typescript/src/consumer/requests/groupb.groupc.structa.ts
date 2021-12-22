import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TGroupBGroupCStructAResolver = Protocol.GroupA.StructB | Protocol.GroupB.GroupC.StructB;
export type TResponseHandler = (response: Protocol.GroupB.GroupC.StructB) => void
export type TErrHandler = (response: Protocol.GroupA.StructB) => void

export class GroupBGroupCStructA extends Protocol.GroupB.GroupC.StructA {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        response: undefined,
        err: undefined,
    };
    constructor(request: Protocol.GroupB.GroupC.IStructA) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TGroupBGroupCStructAResolver> {
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
                        if (message === undefined || message.GroupB === undefined || message.GroupB.GroupC === undefined) {
                            return reject(new Error(`Expecting message from "message.GroupB.GroupC" group.`));
                        } else if (message.GroupB.GroupC.StructB !== undefined) {
                            this._handlers.response !== undefined && this._handlers.response(message.GroupB.GroupC.StructB);
                            return resolve(message.GroupB.GroupC.StructB);
                        } else if (message !== undefined && message.GroupA !== undefined && message.GroupA.StructB !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.GroupA.StructB);
                            return resolve(message.GroupA.StructB);
                        } else {
                            return reject(new Error(`No message in "message.GroupA" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "GroupBGroupCStructA" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "GroupBGroupCStructA".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public response(handler: TResponseHandler): GroupBGroupCStructA {
        this._handlers.response = handler;
        return this;
    }
    
    public err(handler: TErrHandler): GroupBGroupCStructA {
        this._handlers.err = handler;
        return this;
    }

}
