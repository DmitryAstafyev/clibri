// tslint:disable: max-classes-per-file
import { Client } from './client';
import { ILogger } from './interfaces/logger.interface';

import * as Protocol from './protocol/protocol';

import Subject from './tools/tools.subject';
import Subscription from './tools/tools.subscription';
import guid from './tools/tools.guid';
import globals from './tools/tools.globals';

export namespace ConsumerError {

    export type TError = Handeling<any>;

    export class Handeling<T> {
        public request: T;
        public message: string;
        constructor(request: T, message: string) {
            this.request = request;
            this.message = message;
        }
    }

    export class Requesting<T> {
        public request: T;
        public message: string;
        constructor(request: T, message: string) {
            this.request = request;
            this.message = message;
        }
    }

}

export class Consumer {

    public static GUID: string = guid();
    public static GUID_SUBS: string = guid();

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

    public static wait(handler: () => void): Error | Subscription {
        const global = globals();
        if (global instanceof Error) {
            return global;
        }
        if (global[Consumer.GUID] !== undefined) {
            return new Error(`Consumer is already created`);
        }
        if (global[Consumer.GUID_SUBS] === undefined) {
            global[Consumer.GUID_SUBS] = new Subject(`ConsumerSubscription`);
        }
        return (global[Consumer.GUID_SUBS] as Subject<Consumer>).subscribe(handler);
    }
 
    private readonly _client: Client;
    private readonly _subscriptions: { [key: string]: Subscription } = {};
    private readonly _pending: Map<number, (response: Protocol.IAvailableMessages) => void> = new Map();
    private readonly _buffer: Protocol.BufferReaderMessages = new Protocol.BufferReaderMessages();
    private _sequence: number = 0;

    public readonly connected: Subject<void> = new Subject(`connected`);
    public readonly disconnected: Subject<void> = new Subject(`disconnected`);
    public readonly error: Subject<ConsumerError.TError> = new Subject(`error`);
    public readonly broadcast: {
        UserConnected: Subject<Protocol.UserConnected>,
        UserDisconnected: Subject<Protocol.UserDisconnected>,
    } = {
        UserConnected: new Subject<Protocol.UserConnected>(),
        UserDisconnected: new Subject<Protocol.UserDisconnected>(),
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
        ((subject: Subject<Consumer>) => {
            subject.emit(this);
            subject.destroy();
        })((global[Consumer.GUID_SUBS] as Subject<Consumer>));
        global[Consumer.GUID_SUBS] = undefined;
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

    public getSequence(): number {
        return this._sequence ++;
    }

    public logs(): ILogger {
        return {
            warm: (msg: string) => {},
            err: (msg: string) => {},
            debug: (msg: string) => {},
            verb: (msg: string) => {},
            info: (msg: string) => {},
        };
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
                switch (msg.getRef<any>().getId()) {
                    case Protocol.UserConnected.getId():
                        this.broadcast.UserConnected.emit(msg.getRef<Protocol.UserConnected>());
                        break;
                    case Protocol.UserDisconnected.getId():
                        this.broadcast.UserDisconnected.emit(msg.getRef<Protocol.UserDisconnected>());
                        break;
                    default:
                        // Here is logs messages
                        break;
                }
            }
        } while (true);
    }
}