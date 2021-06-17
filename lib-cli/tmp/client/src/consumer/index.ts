// tslint:disable: max-classes-per-file

import { Client, Logger, Subscription, Subject, guid, globals } from 'fiber';
import { IOptions, Options } from './options';

import * as Protocol from './protocol/protocol';

export { Protocol };
export { UserLoginRequest } from './requests/userlogin.request';
export { UsersRequest } from './requests/users.request';
export { MessageRequest } from './requests/message.request';
export { MessagesRequest } from './requests/messages.request';

export namespace ExtError {

    export type TError = ClientError | ConsumerError;

    export class ClientError {
        public message: string;
        constructor(message: string) {
            this.message = message;
        }
    }

    export class ConsumerError {
        public message: string;
        constructor(message: string) {
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
    private _uuid: string | undefined;
    private _key: Protocol.Identification.ISelfKey;
    private _sequence: number = 1;

    public readonly connected: Subject<void> = new Subject(`connected`);
    public readonly ready: Subject<string> = new Subject(`ready`);
    public readonly disconnected: Subject<void> = new Subject(`disconnected`);
    public readonly error: Subject<ExtError.TError> = new Subject(`error`);
    public readonly broadcast: {        
        EventsUserConnected: Subject<Protocol.Events.UserConnected>,
        EventsMessage: Subject<Protocol.Events.Message>,
        EventsUserDisconnected: Subject<Protocol.Events.UserDisconnected>,
    } = {        
        EventsUserConnected: new Subject<Protocol.Events.UserConnected>(),
        EventsMessage: new Subject<Protocol.Events.Message>(),
        EventsUserDisconnected: new Subject<Protocol.Events.UserDisconnected>(),
    };

    public get uuid(): string {
        if (this._uuid === undefined) {
            this._logger.warn(`Consumer UUID is requested, but it isn't defined yet.`);
        }
        return this._uuid;
    }

    public set uuid(value: string) {
        if (value === undefined) {
            this._uuid = undefined;
            this._logger.debug(`Consumer UUID is dropped`);
        } else if (typeof value !== 'string' || value.trim() === '') {
            this._logger.err(`Fail set consumer UUID because value has invalid type (${typeof value}) or invalid value (${value})`);
        } else if (this._uuid !== undefined) {
            this._logger.err(`No way to set consumer UUID multiple times`);
        } else {
            this._uuid = value;
            this._logger.debug(`Consumer UUID = ${value}`);
        }
    }

    constructor(client: Client, key: Protocol.Identification.ISelfKey, options?: IOptions) {
        this._client = client;
        this._key = key;
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
        this._subscriptions.connected = this._client.getEvents().connected.subscribe(this._onClientConnected.bind(this));
        this._subscriptions.disconnected = this._client.getEvents().disconnected.subscribe(this._onClientDisconnected.bind(this));
        this._subscriptions.error = this._client.getEvents().error.subscribe(this._onClientError.bind(this));
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

    public assign(key: Protocol.Identification.ISelfKey): Promise<string> {
        return new Promise((resolve, reject) => {
            const request: Protocol.Identification.SelfKey = new Protocol.Identification.SelfKey(key);
            const sequence: number = this.getSequence();
            this.request(request.pack(sequence), sequence).then((response: Protocol.IAvailableMessages) => {
                if (response.Identification === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message from "Identification" group.`)));
                }
                if (response.Identification.SelfKeyResponse === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message "Identification.SelfKeyResponse".`)));
                }
                this.uuid = response.Identification.SelfKeyResponse.uuid;
                this._logger.debug(`Consumer is assigned with uuid ${this.uuid}`);
                resolve(response.Identification.SelfKeyResponse.uuid);
            }).catch((err: Error) => {
                reject(new Error(this._logger.err(`Fail assing consumer due error: ${err.message}`)));
            });
        });
    }

    public getSequence(): number {
        return this._sequence ++;
    }

    private _onClientConnected() {
        this._logger.debug(`Client is connected`);
        this.assign(this._key).then((uuid: string) => {
            this.ready.emit(uuid);
        }).catch((err: Error) => {
            this._logger.err(`Default assign prodecure is failed.`);
        });
        this.connected.emit();
    }

    private _onClientDisconnected() {
        this._logger.debug(`Client is disconnected`);
        this.uuid = undefined;
        this.disconnected.emit();
    }

    private _onClientError(error: Error) {
        this._logger.debug(`Client error: ${error.message}`);
        this.error.emit(new ExtError.ClientError(error.message));
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
                    case Protocol.Events.UserConnected.getId():
                        this.broadcast.EventsUserConnected.emit(msg.getRef<Protocol.Events.UserConnected>());
                        break;
                    case Protocol.Events.Message.getId():
                        this.broadcast.EventsMessage.emit(msg.getRef<Protocol.Events.Message>());
                        break;
                    case Protocol.Events.UserDisconnected.getId():
                        this.broadcast.EventsUserDisconnected.emit(msg.getRef<Protocol.Events.UserDisconnected>());
                        break;
                    default:
                        this._logger.warn(`Has been gotten unexpected message ID=${id};`)
                        break;
                }
            }
            
        } while (true);
    }
}