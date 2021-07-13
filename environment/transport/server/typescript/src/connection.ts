import * as WebSocket from 'ws';
import {
    Subject,
    IServerError,
    EServerErrorType,
    EServerErrorContext,
    guid,
    Logger,
    DefaultLogger,
} from 'fiber';

export interface IConnectionSubjects {
    disconnected: Subject<void>;
    data: Subject<Buffer>;
    error: Subject<IServerError>;
}
export class Connection {

    private readonly _socket: WebSocket;
    private readonly _logger: Logger;
    private readonly _guid: string = guid();
    private readonly _subjects: IConnectionSubjects = {
        disconnected: new Subject(),
        data: new Subject(),
        error: new Subject(),
    };
    private readonly _pending: {
        resolver: undefined | (() => void),
        rejector: undefined | ((err: IServerError) => void),
    } = {
        resolver: undefined,
        rejector: undefined,
    }

    constructor(socket: WebSocket, logger?: Logger) {
        this._socket = socket;
        this._logger = logger === undefined ? new DefaultLogger(`Connection ${this._guid}`) : logger;
        this._onMessage = this._onMessage.bind(this);
        this._onError = this._onError.bind(this);
        this._onOpen = this._onOpen.bind(this);
        this._onClose = this._onClose.bind(this);
        this._socket.addEventListener('message', this._onMessage);
        this._socket.addEventListener('error', this._onError);
        this._socket.addEventListener('open', this._onOpen);
        this._socket.addEventListener('close', this._onClose);
    }

    public established(): Promise<void> {
        return new Promise((resolve, reject) => {
            if (this._socket.readyState === WebSocket.OPEN) {
                return resolve();
            }
            this._pending.resolver = resolve;
            this._pending.rejector = reject;
        });
    }

    public destroy() {
        this._socket.removeEventListener('message', this._onMessage);
        this._socket.removeEventListener('error', this._onError);
        this._socket.removeEventListener('open', this._onOpen);
        this._socket.removeEventListener('close', this._onClose);
        this._subjects.disconnected.destroy();
        this._subjects.error.destroy();
        this._pending.resolver = undefined;
        this._pending.rejector = undefined;
        if (this._socket.readyState !== WebSocket.CLOSED && this._socket.readyState !== WebSocket.CLOSING) {
            this._socket.close();
        }
    }

    public getGuid(): string {
        return this._guid;
    }

    public getSubjects(): IConnectionSubjects {
        return this._subjects;
    }

    public send(buffer: ArrayBufferLike): void {
        this._socket.send(buffer);
    }

    public _onEstablished(error?: IServerError): void {
        if (this._pending.resolver === undefined && this._pending.rejector === undefined) {
            return;
        }
        if (error === undefined) {
            this._pending.resolver();
        } else {
            this._pending.rejector(error);
        }
        this._pending.resolver = undefined;
        this._pending.rejector = undefined;
    }

    private _onMessage(event: WebSocket.MessageEvent) {
        if (!(event.data instanceof Buffer)) {
            this._logger.warn(`Invalid message has been gotten`);
            this._subjects.error.emit({
                uuid: this._guid,
                type: EServerErrorType.UnexpectedDataFormat,
                context: EServerErrorContext.connection,
                message: `Expecting Blob, but has been gotten: ${typeof event.data}`,
            });
            return;
        }
        this._logger.verb(`${event.data.byteLength} bytes has been gotten`);
        this._subjects.data.emit(event.data);
    }

    private _onError(event: WebSocket.ErrorEvent) {
        const error: IServerError = {
            uuid: this._guid,
            type: EServerErrorType.ConnectionError,
            context: EServerErrorContext.connection,
            message: `Connection error.`,
            event,
        };
        this._subjects.error.emit(error);
        this._onEstablished(error);
    }

    private _onOpen(event: WebSocket.OpenEvent) {
        this._logger.verb(`Connection is opened`);
        this._onEstablished();
    }

    private _onClose(event: WebSocket.CloseEvent) {
        this._subjects.disconnected.emit();
        this._logger.verb(`Connection is closed`);
    }

}