// tslint:disable: max-classes-per-file

import { UserJoin } from './declarations/observer.UserJoin';
import { UserLogout } from './declarations/observer.UserLogout';
import { UserSignIn } from './declarations/observer.UserSignIn';
import { UserConnected } from './declarations/observer.UserConnected';
import { UserDisconnected } from './declarations/observer.UserDisconnected';
import { Client, Logger, Subscription, Subject, guid, globals } from 'fiber';
import { IOptions, Options } from './options';

import * as Protocol from './protocol/protocol';

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
    private readonly _logger: Logger;
    private readonly _options: Options;
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

    constructor(client: Client, options?: IOptions) {
        this._client = client;
        this._options = new Options(`Consumer ${Consumer.GUID}`, options);
        this._logger = this._options.logger;
        const global = globals();
        if (global instanceof Error) {
            throw global;
        }
        if (global[Consumer.GUID] !== undefined) {
            throw new Error(`Attempt to init consumer multiple times`);
        }
        this._subscriptions.data = this._client.getEvents().data.subscribe(this._onData.bind(this));
        global[Consumer.GUID] = this;
        if (global[Consumer.GUID_SUBS] !== undefined) {
            ((subject: Subject<Consumer>) => {
                subject.emit(this);
                subject.destroy();
            })((global[Consumer.GUID_SUBS] as Subject<Consumer>));
            global[Consumer.GUID_SUBS] = undefined;
        }
    }

    public destroy(): Promise<void> {
        const global = globals();
        Object.keys(this._subscriptions).forEach(k => this._subscriptions[k].destroy());
        global[Consumer.GUID] = undefined;
        return this._client.destroy();
    }

    public request(buffer: ArrayBufferLike, sequence?: number): Promise<Protocol.IAvailableMessages> {
        if (sequence !== undefined && this._pending.has(sequence)) {
            return Promise.reject(new Error(this._logger.debug(`Request with sequence #${sequence} has been already sent and pending for response`)));
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

    private _onData(buffer: ArrayBufferLike) {
        const errors: Error[] | undefined = this._buffer.chunk(buffer);
        if (errors instanceof Array) {
            this._logger.err(`Fail to process chunk of data due error(s):\n\t${errors.map(e => e.message).join('\n\t')}`)
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
                const id: number = msg.getRef<any>().getId();
                switch (id) {
                    case Protocol.UserConnected.getId():
                        this.broadcast.UserConnected.emit(msg.getRef<Protocol.UserConnected>());
                        break;
                    case Protocol.UserDisconnected.getId():
                        this.broadcast.UserDisconnected.emit(msg.getRef<Protocol.UserDisconnected>());
                        break;
                    default:
                        this._logger.warn(`Has been gotten unexpected message ID=${id}.`)
                        break;
                }
            }
        } while (true);
    }
}