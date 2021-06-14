import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request';

export type TMessagesRequestResolver = Protocol.Messages.Err | Protocol.Messages.Response;
export type TResponseHandler = (response: Protocol.Messages.Response) => void
export type TErrHandler = (response: Protocol.Messages.Err) => void

export class MessagesRequest extends Protocol.UserLogin.Request {

    private _state: ERequestState = ERequestState.Ready;
    private _handlers: {    
        response: TResponseHandler | undefined;
        err: TErrHandler | undefined;
    } = {    
        response: undefined,
        err: undefined,
    };
    constructor(request: Protocol.UserLogin.IRequest) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
        this._handlers = {            
            response: undefined,
            err: undefined,
        };
    }

    public send(): Promise<TMessagesRequestResolver> {
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
                        if (message === undefined || message.Messages === undefined) {
                            return reject(new Error(`Expecting message from "message.Messages" group.`));
                        } else if (message.Messages.Response !== undefined) {
                            this._handlers.response !== undefined && this._handlers.response(message.Messages.Response);
                            return resolve(message.Messages.Response);
                        } else if (message.Messages.Err !== undefined) {
                            this._handlers.err !== undefined && this._handlers.err(message.Messages.Err);
                            return resolve(message.Messages.Err);
                        } else {
                            return reject(new Error(`No message in "message.Messages" group.`));
                        }
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "MessagesRequest" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "MessagesRequest".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }
    
    public response(handler: TResponseHandler): MessagesRequest {
        this._handlers.response = handler;
        return this;
    }
    
    public err(handler: TErrHandler): MessagesRequest {
        this._handlers.err = handler;
        return this;
    }

}
