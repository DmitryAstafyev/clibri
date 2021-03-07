// tslint:disable: max-classes-per-file
import { Client } from './client';

import Subject from './tools/tools.subject';
import Subscription from './tools/tools.subscription';
import guid from './tools/tools.guid';
import globals from './tools/tools.globals';

export class Consumer {

    public static GUID: string = guid();

    public static get(): Consumer | Error {
        const global = globals();
        if (global instanceof Error) {
            return global;
        }
        if (global[Consumer.GUID] === undefined) {
            return new Error(`Consumer isn't defined`);
        }
        return global[Consumer.GUID];
    }
 
    private readonly _client: Client;
    private readonly _subscriptions: { [key: string]: Subscription } = {}; 

    public readonly connected: Subject<void> = new Subject(`connected`);
    public readonly disconnected: Subject<void> = new Subject(`disconnected`);
    public readonly error: Subject<Error> = new Subject(`error`);
    public readonly incomes: {
        
    } = {

    };

    constructor(client: Client) {
        this._client = client;
        const global = globals();
        if (global instanceof Error) {
            throw global;
        }
        if (global[Consumer.GUID] !== undefined) {
            throw new Error(`Attempt to init consumer multiple times`);
        }
        global[Consumer.GUID] = this;
    }

    public destroy(): Promise<void> {
        Object.keys(this._subscriptions).forEach(k => this._subscriptions[k].destroy());
        global[Consumer.GUID] = undefined;
        return this._client.destroy();
    }

    public connect(): Promise<void> {
        return this._client.connect();
    }

}

class DummyClient extends Client {
    public send(buffer: Buffer): Error | undefined {
        return undefined;
    }
    public connect(): Promise<void> {
        return Promise.resolve();
    }
    public disconnect(): Promise<void> {
        return Promise.resolve();
    }
    public destroy(): Promise<void> {
        return Promise.resolve();
    }
}

const consumer: Consumer = new Consumer(new DummyClient());
