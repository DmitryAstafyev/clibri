import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TGroupBStructAResolver = Protocol.GroupB.GroupC.StructA | Protocol.GroupB.StructA | Protocol.GroupB.GroupC.StructA;
export type TGroupBStructAHandler = (response: Protocol.GroupB.StructA) => void
export type TGroupBGroupCStructAHandler = (response: Protocol.GroupB.GroupC.StructA) => void
export type TErrHandler = (response: Protocol.GroupB.GroupC.StructA) => void

export class GroupBStructA extends Protocol.GroupB.StructA {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        groupbstructa: TGroupBStructAHandler | undefined;
        groupbgroupcstructa: TGroupBGroupCStructAHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        groupbstructa: undefined,
        groupbgroupcstructa: undefined,
        err: undefined,
    };
    constructor(request: Protocol.GroupB.IStructA) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            groupbstructa: undefined,
            groupbgroupcstructa: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TGroupBStructAResolver> {
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
                        if (message === undefined || message.GroupB === undefined) {
                            return reject(new Error(`Expecting message from "message.GroupB" group.`));
                        } else if (message !== undefined && message.GroupB !== undefined && message.GroupB.StructA !== undefined) {
                            this._handlers.groupbstructa !== undefined && this._handlers.groupbstructa(message.GroupB.StructA);
                            return resolve(message.GroupB.StructA);
                        } else if (message !== undefined && message.GroupB !== undefined && message.GroupB.GroupC !== undefined && message.GroupB.GroupC.StructA !== undefined) {
                            this._handlers.groupbgroupcstructa !== undefined && this._handlers.groupbgroupcstructa(message.GroupB.GroupC.StructA);
                            return resolve(message.GroupB.GroupC.StructA);
                        } else if (message !== undefined && message.GroupB !== undefined && message.GroupB.GroupC !== undefined && message.GroupB.GroupC.StructA !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.GroupB.GroupC.StructA);
                            return resolve(message.GroupB.GroupC.StructA);
                        } else {
                            return reject(new Error(`No message in "message.GroupB.GroupC" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "GroupBStructA" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "GroupBStructA".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public groupbstructa(handler: TGroupBStructAHandler): GroupBStructA {
        this._handlers.groupbstructa = handler;
        return this;
    }
    public groupbgroupcstructa(handler: TGroupBGroupCStructAHandler): GroupBStructA {
        this._handlers.groupbgroupcstructa = handler;
        return this;
    }
    public err(handler: TErrHandler): GroupBStructA {
        this._handlers.err = handler;
        return this;
    }

}
