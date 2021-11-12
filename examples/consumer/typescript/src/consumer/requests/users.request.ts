import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TUsersRequestResolver = Protocol.Users.Err | Protocol.Users.Response;
export type TResponseHandler = (response: Protocol.Users.Response) => void
export type TErrHandler = (response: Protocol.Users.Err) => void

export class UsersRequest extends Protocol.Users.Request {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        response: undefined,
        err: undefined,
    };
    constructor(request: Protocol.Users.IRequest) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TUsersRequestResolver> {
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
                        if (message === undefined || message.Users === undefined) {
                            return reject(new Error(`Expecting message from "message.Users" group.`));
                        } else if (message.Users.Response !== undefined) {
                            this._handlers.response !== undefined && this._handlers.response(message.Users.Response);
                            return resolve(message.Users.Response);
                        } else if (message.Users.Err !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.Users.Err);
                            return resolve(message.Users.Err);
                        } else {
                            return reject(new Error(`No message in "message.Users" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UsersRequest" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UsersRequest".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public response(handler: TResponseHandler): UsersRequest {
        this._handlers.response = handler;
        return this;
    }
    
    public err(handler: TErrHandler): UsersRequest {
        this._handlers.err = handler;
        return this;
    }

}
