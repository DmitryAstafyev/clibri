// tslint:disable:max-classes-per-file

export class MessageReadingError {

    private _err: Error;

    constructor(err: Error) {
        this._err = err;
    }

    public getErr(): Error {
        return this._err;
    }

}

export class ConnectionError {

    private _evn: Event;

    constructor(evn: Event) {
        this._evn = evn;
    }

    public getEvent(): Event {
        return this._evn;
    }

}
