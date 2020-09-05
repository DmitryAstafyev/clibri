
import * as Protocol from '../../../lib-client/src/protocol';
import { Connection } from './connection';

export class Application {

    private _connection: Connection;
    private _timer: any;

    constructor() {
        this._connection = new Connection();
        this._connection.subscribe(Connection.Events.connect, this._connected.bind(this));
        this._connection.subscribe(Connection.Events.message, this._message.bind(this));
        this._connection.subscribe(Connection.Events.error, this._close.bind(this));
        this._connection.subscribe(Connection.Events.close, this._close.bind(this));
    }

    private _connected() {
        this._next();
    }

    private _message(event: MessageEvent) {
        console.log(event);
    }

    private _close(ev?: Error) {
        this._stop();
        if (ev !== undefined) {
            console.log(`Will try reconnect in 2 sec`);
            setTimeout(() => {
                this._connection.reconnect();
            }, 2000);
        }
    }

    private _next() {
        this._timer = setTimeout(() => {
            const ping: Protocol.Out.Ping = new Protocol.Out.Ping({ guid: Protocol.Tools.guid() });
            this._connection.send(ping.encode());
            this._next();
        }, 1000);
    }

    private _stop() {
        clearTimeout(this._timer);
    }
}