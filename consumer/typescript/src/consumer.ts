// tslint:disable: max-classes-per-file
import { Client } from './client';

import * as Protocol from './protocol/protocol';

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
    private readonly _pending: Map<number, (response: Protocol.IAvailableMessages) => void> = new Map();
    private readonly _buffer: Protocol.BufferReaderMessages = new Protocol.BufferReaderMessages();

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
        this._subscriptions.data = this._client.data.subscribe(this._onData.bind(this));
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

    public request(buffer: ArrayBufferLike, sequence?: number): Promise<Protocol.IAvailableMessages> {
        if (sequence !== undefined && this._pending.has(sequence)) {
            return Promise.reject(new Error(`Request with sequence #${sequence} has been already sent and pending for response`));
        }
        const error: Error | undefined = this._client.send(buffer);
        if (error instanceof Error) {
            return Promise.reject(error);
        }
        if (sequence === undefined) {
            return Promise.resolve({});
        }
        return new Promise((resolve) => {
            this._pending.set(sequence, resolve);
        });
    }

    private _onData(buffer: ArrayBufferLike) {
        const errors: Error[] | undefined = this._buffer.chunk(buffer);
        if (errors instanceof Array) {
            // Here is logs messages
            return;
        }
        do {
            const msg: Protocol.IAvailableMessage<Protocol.IAvailableMessages> | undefined =  this._buffer.next();
            if (msg === undefined) {
                return;
            }
            const pending = this._pending.get(msg.header.sequence);
            if (pending !== undefined) {
                this._pending.delete(msg.header.sequence);
                pending(msg.msg);
            } else {
                // TODO: Broadcasting
            }
        } while (true);
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
