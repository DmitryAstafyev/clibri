
import { Connection, Tools, ConnectionError, MessageReadingError } from '../../../lib-client/src/index';
import { ProtocolImpl, IncomeMessages, PingOut } from './protocol';

export class Application {

    private readonly CONNECT_STR: string = 'ws://127.0.0.1:8088/ws/';
    private _protocol: ProtocolImpl = new ProtocolImpl();
    private _connection: Connection<IncomeMessages>;
    private _timer: any;

    constructor() {
        console.log(`Creating connection...`);
        this._connection = new Connection<IncomeMessages>(this.CONNECT_STR, this._protocol);
        this._connection.subscribe(Connection.Events.connect, this._connected.bind(this));
        this._connection.subscribe(Connection.Events.message, this._message.bind(this));
        this._connection.subscribe(Connection.Events.error, this._error.bind(this));
        this._connection.subscribe(Connection.Events.close, this._close.bind(this));
    }

    private _connected() {
        console.log(`Connected`);
        this._next();
    }

    private _message(event: MessageEvent) {
        console.log(event);
    }

    private _close() {
        console.log(`Connection is closed.`);
        this._stop();
        console.log(`Will try reconnect in 2 sec`);
        setTimeout(() => {
            this._connection.reconnect();
        }, 2000);
    }

    private _error(error: ConnectionError | MessageReadingError) {
        if (error instanceof ConnectionError) {
            return this._close();
        }
        if (error instanceof MessageReadingError) {
            console.log(`Fail to read message, due error: ${error.getErr().message}`);
        }
    }

    private _next() {
        this._timer = setTimeout(() => {
            const ping: PingOut = new PingOut({ uuid: Tools.guid() });
            console.log(`Sending: ${JSON.stringify(ping)}`);
            this._connection.send(ping);
            this._next();
        }, 1000);
    }

    private _stop() {
        clearTimeout(this._timer);
    }
}