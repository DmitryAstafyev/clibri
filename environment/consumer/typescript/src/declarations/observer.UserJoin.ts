import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request.states';

export abstract class UserJoin extends Protocol.UserJoin.Request {

    private _state: ERequestState = ERequestState.Ready;

    constructor(request: Protocol.UserJoin.IRequest) {
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
                        if (message.UserJoin === undefined) {
                            return reject(new Error(`Expecting message from "UserJoin" group.`));
                        } else if (message.UserJoin.Accepted !== undefined) {
                            this.accept(message.UserJoin.Accepted);
                        } else if (message.UserJoin.Denied !== undefined) {
                            this.deny(message.UserJoin.Denied);
                        } else if (message.UserJoin.Err !== undefined) {
                            this.error(message.UserJoin.Err);
                        } else {
                            return reject(new Error(`No message in "UserJoin" group.`));
                        }
                        return resolve();
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserJoin" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserJoin".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
        
    }

    public abstract accept(response: Protocol.UserJoin.Accepted);
    public abstract deny(response: Protocol.UserJoin.Denied);
    public abstract error(response: Protocol.UserJoin.Err);

}
