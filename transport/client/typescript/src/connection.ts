import { Client } from 'fiber';

export class IConnectionOptions {
    autoconnect: boolean;
    reconnect: number;
}

export class Connection extends Client {

    private _socket: WebSocket | undefined;
    private _connected: boolean = false;
    private _pending: {
        resolver: () => void;
        rejector: (err: Error) => void;
    } | undefined;
    private _timeout: any = -1;
    private readonly _address: string;
    private readonly _options: IConnectionOptions;

    constructor(addr: string, options: IConnectionOptions = { autoconnect: true, reconnect: 5000 }) {
        super();
        this._address = addr;
        this._options = options;
        this._events.open = this._events.open.bind(this);
        this._events.close = this._events.close.bind(this);
        this._events.message = this._events.message.bind(this);
        this._events.error = this._events.error.bind(this);
        if (this._options.autoconnect) {
            this.connect();
        }
    }

    public send(buffer: ArrayBufferLike): Error | undefined {
        if (!this._connected || this._socket === undefined) {
            return new Error(`Client isn't connected`);
        }
        this._socket.send(buffer);
    }

    public connect(): Promise<void> {
        clearTimeout(this._timeout);
        this._options.reconnect = -1;
        if (this._pending !== undefined) {
            return Promise.reject(new Error(`Connection is already requested`));
        }
        if (this._connected || this._socket !== undefined) {
            return Promise.reject(new Error(`Already connected`));
        }
        return new Promise((resolve, reject) => {
            this._pending = {
                resolver: resolve,
                rejector: reject,
            };
            this._open();
        });
    }

    public disconnect(): Promise<void> {
        this._dropReconnection();
        this._close();
        return Promise.resolve(undefined);
    }

    public destroy(): Promise<void> {
        this._dropReconnection();
        this._close();
        this._pending = undefined;
        return Promise.resolve(undefined);
    }

    private _reconnect() {
        clearTimeout(this._timeout);
        this._timeout = setTimeout(() => {
            this._open();
        }, this._options.reconnect);
    }

    private _dropReconnection() {
        this._options.reconnect = -1;
        clearTimeout(this._timeout);
    }

    private _events: {
        open: () => void,
        close: () => void,
        message: (event: MessageEvent) => void,
        error: (event: Event) => void
    } = {
        open: () => {
            this._connected = true;
            if (this._pending !== undefined) {
                this._pending.resolver();
                this._pending = undefined;
            }
        },
        close: () => {
            this._connected = false;
            if (this._options.reconnect > 0) {
                this._reconnect();
            }
        },
        message: (event: MessageEvent) => { 
            if (!(event.data instanceof Blob)) {
                console.log(`Expecting only Blob data`);
                return;
            }
            event.data.arrayBuffer().then((buffer: ArrayBuffer) => {
                //this._buffer.proceed(buffer);
            }).catch((err: Error) => {
                // console.warn(`Fail get data due error: ${err.message}`)
            });
        },
        error: (event: Event) => {
            this._connected = false;
            if (this._pending !== undefined) {
                this._pending.rejector(new Error(`Fail to connect`));
                this._pending = undefined;
            }
        }
    }

    private _open() {
        if (this._socket !== undefined) {
            throw new Error(`Attempt to open socket while current isn't closed`);
        }
        this._socket = new WebSocket(this._address);
        this._socket.addEventListener('open', this._events.open);
        this._socket.addEventListener('message', this._events.message);
        this._socket.addEventListener('close', this._events.close);
        this._socket.addEventListener('error', this._events.error);
    }

    private _close() {
        if (this._socket === undefined) {
            return;
        }
        this._socket.removeEventListener('open', this._events.open);
        this._socket.removeEventListener('message', this._events.message);
        this._socket.removeEventListener('close', this._events.close);
        this._socket.removeEventListener('error', this._events.error);
        this._socket.close();
        this._socket = undefined;
    }

}

