import { Client, Subject, IClientSubjects, Logger, ELogLevel } from 'fiber';
import { IConnectionOptions, ConnectionOptions } from './options';

Logger.setGlobalLevel(ELogLevel.verb);

export class Connection extends Client {

    private _socket: WebSocket | undefined;
    private _connected: boolean = false;
    private _pending: {
        resolver: () => void;
        rejector: (err: Error) => void;
    } | undefined;
    private _timeout: any = -1;
    private _logger: Logger;
    private readonly _address: string;
    private readonly _options: ConnectionOptions;
    private readonly _subjects: IClientSubjects = {
        connected: new Subject<void>('connected'),
        disconnected: new Subject<void>('disconnected'),
        error: new Subject<Error>('error'),
        data: new Subject<ArrayBufferLike>('data'),
    };

    constructor(addr: string, options?: IConnectionOptions) {
        super();
        this._address = addr;
        this._options = new ConnectionOptions('Connection', options);
        this._logger = this._options.logger;
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
            return new Error(this._logger.debug(`Client isn't connected`));
        }
        this._socket.send(buffer);
    }

    public connect(): Promise<void> {
        clearTimeout(this._timeout);
        if (this._pending !== undefined) {
            return Promise.reject(new Error(this._logger.debug(`Connection is already requested`)));
        }
        if (this._connected || this._socket !== undefined) {
            return Promise.reject(new Error(this._logger.debug(`Already connected`)));
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
        this._drop();
        this._close();
        return Promise.resolve(undefined);
    }

    public destroy(): Promise<void> {
        this._drop();
        this._close();
        this._pending = undefined;
        Object.keys(this._subjects).forEach((key: string) => {
            this._subjects[key].destroy();
        });
        return Promise.resolve(undefined);
    }

    public getEvents(): IClientSubjects {
        return this._subjects;
    }

    private _reconnect() {
        if (this._options.reconnect <= 0) {
            return;
        }
        clearTimeout(this._timeout);
        this._timeout = setTimeout(() => {
            this._open();
        }, this._options.reconnect);
        this._logger.debug(`Will reconnect in ${this._options.reconnect} ms`);
    }

    private _drop() {
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
            this._subjects.connected.emit();
        },
        close: () => {
            this._connected = false;
            this._close();
            this._subjects.disconnected.emit();
        },
        message: (event: MessageEvent) => {
            if (!(event.data instanceof Blob)) {
                this._subjects.error.emit(new Error(this._logger.debug(`Expecting only Blob data`)));
                return;
            }
            event.data.arrayBuffer().then((buffer: ArrayBuffer) => {
                this._subjects.data.emit(buffer);
            }).catch((err: Error) => {
                this._subjects.error.emit(new Error(`Error on message event: ${err.message}`));
            });
        },
        error: (event: Event) => {
            this._connected = false;
            if (this._pending !== undefined) {
                this._pending.rejector(new Error(this._logger.debug(`Fail to connect`)));
                this._pending = undefined;
            }
            this._subjects.error.emit(new Error(this._logger.debug(`Connection error`)));
            this._close();
        }
    }

    private _open() {
        if (this._socket !== undefined) {
            throw new Error(this._logger.err(`Attempt to open socket while current isn't closed`));
        }
        this._socket = new WebSocket(this._address);
        this._socket.addEventListener('open', this._events.open);
        this._socket.addEventListener('message', this._events.message);
        this._socket.addEventListener('close', this._events.close);
        this._socket.addEventListener('error', this._events.error);
    }

    private _close() {
        if (this._socket !== undefined) {
            this._socket.removeEventListener('open', this._events.open);
            this._socket.removeEventListener('message', this._events.message);
            this._socket.removeEventListener('close', this._events.close);
            this._socket.removeEventListener('error', this._events.error);
            this._socket.close();
            this._socket = undefined;
        }
        this._reconnect();
    }

}

