import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request.states';

export abstract class UserLogin extends Protocol.UserLogin.Request {

    private _state: ERequestState = ERequestState.Ready;

    constructor(request: Protocol.UserLogin.IRequest) {
        super(request);
    }

    public destroy() {
        this._state = ERequestState.Destroyed;
    }

    public send(): Promise<void> {
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
                        if (message.UserLogin === undefined) {
                            return reject(new Error(`Expecting message from "UserLogin" group.`));
                        } else if (message.UserLogin.Accepted !== undefined) {
                            this.accept(message.UserLogin.Accepted);
                        } else if (message.UserLogin.Denied !== undefined) {
                            this.deny(message.UserLogin.Denied);
                        } else if (message.UserLogin.Err !== undefined) {
                            this.error(message.UserLogin.Err);
                        } else {
                            return reject(new Error(`No message in "UserLogin" group.`));
                        }
                        return resolve();
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserLogin" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserLogin".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
    }

    public abstract accept(response: Protocol.UserLogin.Accepted);
    public abstract deny(response: Protocol.UserLogin.Denied);
    public abstract error(response: Protocol.UserLogin.Err);

}
