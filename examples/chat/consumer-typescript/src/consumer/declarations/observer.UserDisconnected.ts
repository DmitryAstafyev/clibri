import * as Protocol from '../protocol/protocol';

import { Consumer } from '../index';
import { Subscription } from 'fiber';

export abstract class UserDisconnected {

    private _subscriptions: { [key: string]: Subscription } = {};

    constructor() {
        const consumer: Consumer | Error = Consumer.get();
        if (consumer instanceof Error) {
            const error: Error | Subscription = Consumer.wait(this._init.bind(this));
            if (error instanceof Error) {
                throw error;
            }
        } else {
            this._init(consumer);
        }
    }

    public destroy() {
        Object.keys(this._subscriptions).forEach((key: string) => {
            this._subscriptions[key].destroy();
        });
    }

    private _init(consumer: Consumer) {
        this._subscriptions.UserDisconnected = consumer.broadcast.UserDisconnected.subscribe(this.emitted.bind(this));
    }

    public abstract emitted(broadcast: Protocol.Events.UserDisconnected);

}
