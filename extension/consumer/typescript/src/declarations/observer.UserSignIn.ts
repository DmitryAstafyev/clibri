import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request.states';

export abstract class UserSignIn extends Protocol.UserSignIn.Request {

    private _state: ERequestState = ERequestState.Ready;

    constructor(request: Protocol.UserSignIn.IRequest) {
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
            consumer.request(this.pack(sequence)).then((message: Protocol.IAvailableMessages) => {
                switch (this._state) {
                    case ERequestState.Pending:
                        this._state = ERequestState.Ready;
                        if (message.UserSignIn === undefined) {
                            return reject(new Error(`Expecting message from "UserSignIn" group.`));
                        } else if (message.UserSignIn.Accepted !== undefined) {
                            this.accept(message.UserSignIn.Accepted);
                        } else if (message.UserSignIn.Denied !== undefined) {
                            this.deny(message.UserSignIn.Denied);
                        } else if (message.UserSignIn.Err !== undefined) {
                            this.error(message.UserSignIn.Err);
                        } else {
                            return reject(new Error(`No message in "UserSignIn" group.`));
                        }
                        return resolve();
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserSignIn" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserSignIn".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
        
    }

    public abstract accept(response: Protocol.UserSignIn.Accepted);
    public abstract deny(response: Protocol.UserSignIn.Denied);
    public abstract error(response: Protocol.UserSignIn.Err);

}
