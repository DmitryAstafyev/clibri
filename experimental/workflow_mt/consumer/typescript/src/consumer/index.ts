// tslint:disable: max-classes-per-file

import { Client, Logger, Subscription, Subject, guid, globals } from 'clibri';
import { IOptions, Options } from './options';

import * as Protocol from './protocol/protocol';

export { Protocol };
export { StructA } from './requests/structa';
export { StructC } from './requests/structc';
export { StructD } from './requests/structd';
export { StructF } from './requests/structf';
export { GroupAStructA } from './requests/groupa.structa';
export { GroupAStructB } from './requests/groupa.structb';
export { GroupBGroupCStructA } from './requests/groupb.groupc.structa';
export { GroupBStructA } from './requests/groupb.structa';
export { GroupBGroupCStructB } from './requests/groupb.groupc.structb';
export { StructA as BeaconStructA } from './beacons/structa';
export { StructB as BeaconStructB } from './beacons/structb';
export { GroupAStructA as BeaconGroupAStructA } from './beacons/groupa.structa';
export { GroupBGroupCStructA as BeaconGroupBGroupCStructA } from './beacons/groupb.groupc.structa';

// tslint:disable-next-line: no-namespace
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

    public static PROTOCOL_HASH: string = "3866462310227764BF9F29517C27840E42ED4F0FB6D2208426C15DF29AFF29E5";
    public static WORKFLOW_HASH: string = "762B9AC5C68559FCEF1ED3A33DD4CADB0DB807715F9DEB84612C85CCC046C887";
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
    private _key: Protocol.IStructA;
    private _sequence: number = 1;

    public readonly connected: Subject<void> = new Subject(`connected`);
    public readonly ready: Subject<string> = new Subject(`ready`);
    public readonly disconnected: Subject<void> = new Subject(`disconnected`);
    public readonly error: Subject<ExtError.TError> = new Subject(`error`);
    public readonly broadcast: {        
        StructD: Subject<Protocol.StructD>,
        StructF: Subject<Protocol.StructF>,
        StructJ: Subject<Protocol.StructJ>,
        GroupBGroupCStructB: Subject<Protocol.GroupB.GroupC.StructB>,
        StructB: Subject<Protocol.StructB>,
        StructC: Subject<Protocol.StructC>,
        StructA: Subject<Protocol.StructA>,
        GroupAStructA: Subject<Protocol.GroupA.StructA>,
        GroupAStructB: Subject<Protocol.GroupA.StructB>,
        GroupBStructA: Subject<Protocol.GroupB.StructA>,
        GroupBGroupCStructA: Subject<Protocol.GroupB.GroupC.StructA>,
    } = {        
        StructD: new Subject<Protocol.StructD>(),
        StructF: new Subject<Protocol.StructF>(),
        StructJ: new Subject<Protocol.StructJ>(),
        GroupBGroupCStructB: new Subject<Protocol.GroupB.GroupC.StructB>(),
        StructB: new Subject<Protocol.StructB>(),
        StructC: new Subject<Protocol.StructC>(),
        StructA: new Subject<Protocol.StructA>(),
        GroupAStructA: new Subject<Protocol.GroupA.StructA>(),
        GroupAStructB: new Subject<Protocol.GroupA.StructB>(),
        GroupBStructA: new Subject<Protocol.GroupB.StructA>(),
        GroupBGroupCStructA: new Subject<Protocol.GroupB.GroupC.StructA>(),
    };

    public get uuid(): string {
        if (this._uuid === undefined) {
			throw new Error(
				`Consumer UUID is requested, but it isn't defined yet.`
			);
        }
        return this._uuid;
    }

    public set uuid(value: string | undefined) {
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

    constructor(client: Client, key: Protocol.IStructA, options?: IOptions) {
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

    public assign(key: Protocol.IStructA): Promise<string> {
        return new Promise((resolve, reject) => {
            const request: Protocol.StructA = new Protocol.StructA(key);
            const sequence: number = this.getSequence();
            this.request(request.pack(sequence), sequence).then((response: Protocol.IAvailableMessages) => {
                if (response.InternalServiceGroup === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message from "InternalServiceGroup" group.`)));
                }
                if (response.InternalServiceGroup.SelfKeyResponse === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message "InternalServiceGroup.SelfKeyResponse".`)));
                }
                this.uuid = response.InternalServiceGroup.SelfKeyResponse.uuid;
                this._logger.debug(`Consumer is assigned with uuid ${this.uuid}`);
                resolve(response.InternalServiceGroup.SelfKeyResponse.uuid);
            }).catch((err: Error) => {
                reject(new Error(this._logger.err(`Fail assing consumer due error: ${err.message}`)));
            });
        });
    }

    public getSequence(): number {
        return this._sequence ++;
    }

    private _hash(): Promise<void> {
        return new Promise((resolve, reject) => {
            const request: Protocol.InternalServiceGroup.HashRequest = new Protocol.InternalServiceGroup.HashRequest({
                protocol: Consumer.PROTOCOL_HASH,
                workflow: Consumer.WORKFLOW_HASH,
            });
            const sequence: number = this.getSequence();
            this.request(request.pack(sequence), sequence).then((response: Protocol.IAvailableMessages) => {
                if (response.InternalServiceGroup === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message from "InternalServiceGroup" group.`)));
                }
                if (response.InternalServiceGroup.HashResponse === undefined) {
                    return reject(new Error(this._logger.err(`Expecting message "InternalServiceGroup.HashResponse".`)));
                }
                if (response.InternalServiceGroup.HashResponse.error !== undefined) {
                    reject(new Error(response.InternalServiceGroup.HashResponse.error));
                } else {
                    resolve(undefined);
                }
            }).catch((err: Error) => {
                reject(new Error(this._logger.err(`Fail check consumer's hash due error: ${err.message}`)));
            });
        });
    }

    private _onClientConnected() {
        this._logger.debug(`Client is connected`);
        this.assign(this._key).then((uuid: string) => {
            this._hash().then(() => {
                this._logger.debug(`Protocol and workflow hashes has been accepted`);
                this.ready.emit(uuid);
            }).catch((err: Error) => {
                this._logger.err(`Consumer has isn't accepted: ${err.message}\n\t- protocol hash: ${Consumer.PROTOCOL_HASH}\n\t- workflow hash: ${Consumer.WORKFLOW_HASH}`);
            });
        }).catch((err: Error) => {
            this._logger.err(`Default assign prodecure is failed: ${err.message}`);
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
                    case Protocol.StructD.getId():
                        this.broadcast.StructD.emit(msg.getRef<Protocol.StructD>());
                        break;
                    case Protocol.StructF.getId():
                        this.broadcast.StructF.emit(msg.getRef<Protocol.StructF>());
                        break;
                    case Protocol.StructJ.getId():
                        this.broadcast.StructJ.emit(msg.getRef<Protocol.StructJ>());
                        break;
                    case Protocol.GroupB.GroupC.StructB.getId():
                        this.broadcast.GroupBGroupCStructB.emit(msg.getRef<Protocol.GroupB.GroupC.StructB>());
                        break;
                    case Protocol.StructB.getId():
                        this.broadcast.StructB.emit(msg.getRef<Protocol.StructB>());
                        break;
                    case Protocol.StructC.getId():
                        this.broadcast.StructC.emit(msg.getRef<Protocol.StructC>());
                        break;
                    case Protocol.StructA.getId():
                        this.broadcast.StructA.emit(msg.getRef<Protocol.StructA>());
                        break;
                    case Protocol.GroupA.StructA.getId():
                        this.broadcast.GroupAStructA.emit(msg.getRef<Protocol.GroupA.StructA>());
                        break;
                    case Protocol.GroupA.StructB.getId():
                        this.broadcast.GroupAStructB.emit(msg.getRef<Protocol.GroupA.StructB>());
                        break;
                    case Protocol.GroupB.StructA.getId():
                        this.broadcast.GroupBStructA.emit(msg.getRef<Protocol.GroupB.StructA>());
                        break;
                    case Protocol.GroupB.GroupC.StructA.getId():
                        this.broadcast.GroupBGroupCStructA.emit(msg.getRef<Protocol.GroupB.GroupC.StructA>());
                        break;
                    default:
                        this._logger.warn(`Has been gotten unexpected message ID=${id};`)
                        break;
                }
            }
            
        } while (true);
    }
}