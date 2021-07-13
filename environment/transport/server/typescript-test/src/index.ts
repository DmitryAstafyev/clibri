import { Server } from 'fiber-websocket-server';
import * as WebSocket from 'ws';
import { MultiBar, Presets } from 'cli-progress';

import {
    Logger,
    DefaultLogger,
    IServerReceivedEvent,
} from 'fiber';

Logger.setGlobalLevel(1);

const CLIENT_MSG = "Hello from client";
const CLIENTS_COUNT = 5000;

class Connection {

    private _socket: WebSocket;
    private _id: number;
    private _connected: (client: Connection) => void;
    private _success: (uuid: string) => void;
    private _closed: (uuid: string) => void;
    private _uuid: string | undefined;

    constructor(
        id: number,
        connected: (client: Connection) => void,
        success: (uuid: string) => void,
        closed: (uuid: string) => void,
    ) {
        this._id = id;
        this._socket = new WebSocket('ws://127.0.0.1:8080');
        this._socket.addEventListener('open', this._onOpen.bind(this));
        this._socket.addEventListener('message', this._onData.bind(this));
        this._socket.addEventListener('close', this._onClose.bind(this));
        this._socket.binaryType = 'nodebuffer';
        this._connected = connected;
        this._success = success;
        this._closed = closed;
    }

    public start(): Error | undefined {
        if (this._socket.readyState !== WebSocket.OPEN) {
            return new Error(`Client #${this._id} isn't connected`);
        }
        this._send();
    }

    private _onOpen() {
        this._connected(this);
    }

    private _onClose() {
        this._closed(this._uuid as string);
    }

    private _onData(event: WebSocket.MessageEvent) {
        const uuid: string = Buffer.from(event.data as any).toString('ascii');
        this._uuid = uuid;
        this._success(uuid);
    }

    private _send() {
        this._socket.send(Buffer.from(CLIENT_MSG, 'ascii'));
    }

}

enum EClientState {
    connected = 1,
    disconnected = 2,
    recieved = 3,
}

const logger: Logger = new DefaultLogger('Server-test');
const server: Server = new Server('127.0.0.1:8080');
const clients: Map<string, EClientState> = new Map();
const stats: {
    errors: string[],
    created: number,
    connected: number,
    disconnected: number,
    done: number,
    mscreated: number,
    msconnected: number,
    msdisconnected: number,
    msdone: number,
} = {
    errors: [],
    created: 0,
    connected: 0,
    disconnected: 0,
    done: 0,
    mscreated: Date.now(),
    msconnected: Date.now(),
    msdisconnected: Date.now(),
    msdone: Date.now(),
};
const multibar = new MultiBar({
    clearOnComplete: false,
    hideCursor: true

}, Presets.shades_grey);
const bars: {
    created: MultiBar,
    connected: MultiBar,
    disconnected: MultiBar,
    done: MultiBar,
} = {
    created: multibar.create(CLIENTS_COUNT, 0),
    connected: multibar.create(CLIENTS_COUNT, 0),
    disconnected: multibar.create(CLIENTS_COUNT, 0),
    done: multibar.create(CLIENTS_COUNT, 0),
};

function test() {
    server.getEvents().connected.subscribe((uuid: string) => {
        clients.set(uuid, EClientState.connected);
        stats.connected += 1;
        bars.connected.increment();
    });
    server.getEvents().disconnected.subscribe((uuid: string) => {
        clients.set(uuid, EClientState.disconnected);
        stats.disconnected += 1;
        bars.disconnected.increment();
    });
    server.getEvents().received.subscribe((event: IServerReceivedEvent) => {
        const content: string = event.buffer.toString('ascii');
        if (!clients.has(event.uuid)) {
            stats.errors.push(`Client ${event.uuid} isn't found`);
        } else if (content !== CLIENT_MSG) {
            stats.errors.push(`Client ${event.uuid} Has been gotten unexpected data: ${event.buffer}`)
        } else {
            server.send(Buffer.from(event.uuid, 'ascii'), event.uuid);
            clients.set(event.uuid, EClientState.recieved);
        }
    });
    server.listen().then(() => {
        logger.debug(`Server is started`);
        for (let i = CLIENTS_COUNT; i >= 1; i -= 1) {
            new Connection(i,
                (client: Connection) => {
                    stats.created += 1;
                    bars.created.increment();
                    client.start();
                },
                (uuid: string) => {
                    server.disconnect(uuid).catch((err: Error) => {
                        stats.errors.push(err.message);
                    });
                },
                (uuid: string) => {
                    done();
                },
            );
        }
    }).catch((err: Error) => {
        logger.err(`Fail to start server: ${err.message}`);
    });
}

function done() {
    stats.done += 1;
    bars.done.increment();
    if (stats.created === CLIENTS_COUNT && stats.mscreated > 100000000) {
        stats.mscreated = Date.now() - stats.mscreated;
    }
    if (stats.connected === CLIENTS_COUNT && stats.msconnected > 100000000) {
        stats.msconnected = Date.now() - stats.msconnected;
    }
    if (stats.disconnected === CLIENTS_COUNT && stats.msdisconnected > 100000000) {
        stats.msdisconnected = Date.now() - stats.msdisconnected;
    }
    if (stats.done === CLIENTS_COUNT) {
        multibar.stop();
        console.log('='.repeat(50));
        console.log(`
Done in ${Date.now() - stats.msdone} ms}
Created      : ${stats.created} in ${stats.mscreated} ms;
Connected    : ${stats.connected} in ${stats.msconnected} ms;
Disconnected : ${stats.disconnected} in ${stats.msdisconnected} ms;
Errors       : ${stats.errors.length};`);
        const sStats: {
            connections: number,
            clients: number,
        } = server.getStats();
        console.log('='.repeat(50));
        console.log(`
Connections  : ${sStats.connections};
Clients      : ${sStats.clients};`);
        server.shutdown().then(() => {
            logger.debug(`Server is down.`);
        }).catch((err: Error) => {
            logger.err(`Fail to shutdown server. Error: ${err.message}`);
        });
    }

}

test();