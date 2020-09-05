import { IMessage } from '../message.holder';

export abstract class Message<T> {

    private _msg: IMessage;

    constructor(msg: IMessage) {
        this._msg = msg;
    }

    public abstract readonly id: number;
    public abstract validate(): Error | undefined;

    public getMsg(): IMessage {
        return this._msg;
    }

    public getErr(): Error | undefined {
        if (typeof this._msg !== 'object' || this._msg === null) {
            return new Error(`"msg" object expected to be {object}.`);
        }
        if (this.id !== this._msg.id) {
            return new Error(`ID of message dismatch. Income ID=${this._msg.id}; expected ID=${this.id}`);
        }
        return undefined;
    }

    public get(): T {
        if (this.validate() instanceof Error) {
            throw this.validate()
        }
        return this._msg.struct as T;
    }

    public getId(): number {
        return this.id;
    }

}