import * as Protocol from '../../../lib-client/src/protocol';

enum EEvents {
    connect = 'connect',
    message = 'message',
    close = 'close',
    error = 'error',
}
export class Connection extends Protocol.Tools.Emitter<EEvents> {

    public static Events = EEvents;

    private readonly CONNECT_STR: string = 'ws://127.0.0.1:8088/ws/';
    private _socket: WebSocket;
    private _connected: boolean = false;
    private _buffer: Protocol.In.BufferReader;

    constructor() {
        super();
        this._open = this._open.bind(this);
        this._message = this._message.bind(this);
        this._error = this._error.bind(this);
        this._close = this._close.bind(this);
        this._buffer = new Protocol.In.BufferReader();
        this._buffer.subscribe(Protocol.In.EEvents.message, this._onProtocolMsg.bind(this))
        this.reconnect();
    }

    public reconnect() {
        this._unbind();
        this._socket = new WebSocket(this.CONNECT_STR);
        this._bind();
    }

    public send(buffer: ArrayBufferLike) {
        if (!this._connected) {
            return;
        }
        this._socket.send(buffer);
    }

    private _bind() {
        this._socket.addEventListener('open', this._open);
        this._socket.addEventListener('message', this._message);
        this._socket.addEventListener('close', this._close);
        this._socket.addEventListener('error', this._error);
    }

    private _unbind() {
        if (this._socket === undefined) {
            return;
        }
        this._socket.removeEventListener('open', this._open);
        this._socket.removeEventListener('message', this._message);
        this._socket.removeEventListener('close', this._close);
        this._socket.removeEventListener('error', this._error);
    }

    private _open() {
        this.emit(EEvents.connect);
        this._connected = true;
    }

    private _message(event: MessageEvent) {
        if (!(event.data instanceof Blob)) {
            console.log(`Expecting only Blob data`);
            return;
        }
        event.data.arrayBuffer().then((buffer: ArrayBuffer) => {
            this._buffer.proceed(buffer);
        }).catch((err: Error) => {
            console.warn(`Fail get data due error: ${err.message}`)
        });
    }

    private _onProtocolMsg(msg: Protocol.In.TMessage) {
        this.emit(EEvents.message, msg);
    }

    private _close() {
        this._connected = false;
        this.emit(EEvents.close);
    }

    private _error(ev: Event) {
        this._connected = false;
        this.emit(EEvents.error, ev);
    }

}

