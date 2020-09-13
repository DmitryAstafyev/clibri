import { Emitter } from './tools/tools.emitter';
import { BufferReader, EBufferEvents } from './messages/in/buffer.reader';
import { Protocol } from './protocol';
import { OutgoingMessage } from './messages/out/message';

enum EConnectionEvents {
    connect = 'connect',
    message = 'message',
    close = 'close',
    error = 'error',
}

export class Connection<TIncomeMessages> extends Emitter<EConnectionEvents> {

    public static Events = EConnectionEvents;

    private _socket: WebSocket;
    private _connected: boolean = false;
    private _buffer: BufferReader<TIncomeMessages>;
    private _address: string;

    constructor(addr: string, protocol: Protocol<TIncomeMessages>) {
        super();
        this._address = addr;
        this._open = this._open.bind(this);
        this._message = this._message.bind(this);
        this._error = this._error.bind(this);
        this._close = this._close.bind(this);
        this._buffer = new BufferReader(protocol);
        this._buffer.subscribe(EBufferEvents.message, this._onProtocolMsg.bind(this))
        this.reconnect();
    }

    public reconnect() {
        this._unbind();
        this._socket = new WebSocket(this._address);
        this._bind();
    }

    public send(msg: OutgoingMessage) {
        if (!this._connected) {
            return;
        }
        this._socket.send(msg.encode());
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
        this.emit(EConnectionEvents.connect);
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

    private _onProtocolMsg(msg: TIncomeMessages) {
        this.emit(EConnectionEvents.message, msg);
    }

    private _close() {
        this._connected = false;
        this.emit(EConnectionEvents.close);
    }

    private _error(ev: Event) {
        this._connected = false;
        this.emit(EConnectionEvents.error, ev);
    }

}

