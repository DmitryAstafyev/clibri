import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TUserLoginRequestResolver = Protocol.UserLogin.Err | Protocol.UserLogin.Accepted | Protocol.UserLogin.Denied;
export type TAcceptHandler = (response: Protocol.UserLogin.Accepted) => void
export type TDenyHandler = (response: Protocol.UserLogin.Denied) => void
export type TErrHandler = (response: Protocol.UserLogin.Err) => void

export class UserLoginRequest extends Protocol.UserLogin.Request {
    private _consumer: Consumer | undefined;
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
    constructor(request: Protocol.UserLogin.IRequest, consumer?: Consumer) {
        super(request);
        this._consumer = consumer;
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            accept: undefined,
            deny: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TUserLoginRequestResolver> {
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
                            return reject(new Error(`Expecting message for "UserLogin.Request".`));
                        } else if (message !== undefined && message.UserLogin !== undefined && message.UserLogin.Accepted !== undefined) {
                            this._handlers.accept !== undefined && this._handlers.accept(message.UserLogin.Accepted);
                            return resolve(message.UserLogin.Accepted);
                        } else if (message !== undefined && message.UserLogin !== undefined && message.UserLogin.Denied !== undefined) {
                            this._handlers.deny !== undefined && this._handlers.deny(message.UserLogin.Denied);
                            return resolve(message.UserLogin.Denied);
                        } else if (message !== undefined && message.UserLogin !== undefined && message.UserLogin.Err !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.UserLogin.Err);
                            return resolve(message.UserLogin.Err);
                        } else {
                            return reject(new Error(`No message in "message.UserLogin" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserLoginRequest" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserLoginRequest".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public accept(handler: TAcceptHandler): UserLoginRequest {
        this._handlers.accept = handler;
        return this;
    }
    public deny(handler: TDenyHandler): UserLoginRequest {
        this._handlers.deny = handler;
        return this;
    }
    public err(handler: TErrHandler): UserLoginRequest {
        this._handlers.err = handler;
        return this;
    }

}
