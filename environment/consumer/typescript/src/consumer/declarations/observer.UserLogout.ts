import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { ERequestState } from '../interfaces/request.states';

export abstract class UserLogout extends Protocol.UserLogout.Request {

    private _state: ERequestState = ERequestState.Ready;

    constructor(request: Protocol.UserLogout.IRequest) {
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
                        if (message.UserLogout === undefined) {
                            return reject(new Error(`Expecting message from "UserLogout" group.`));
                        } else if (message.UserLogout.Done !== undefined) {
                            this.done(message.UserLogout.Done);
                        } else {
                            return reject(new Error(`No message in "UserLogout" group.`));
                        }
                        return resolve();
                    case ERequestState.Destroyed:
                        return reject(new Error(`Request "UserLogout" has been destroyed. Response would not be processed.`));
                    case ERequestState.Pending:
                        return reject(new Error(`Unexpected state for request "UserLogout".`));
                }
            }).catch((err: Error) => {
                reject(err);
            });
        });
        
    }

    public abstract done(response: Protocol.UserLogout.Done);
    public abstract error(response: Protocol.UserLogout.Err);

}
