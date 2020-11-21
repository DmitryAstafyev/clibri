// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import * as Tools from '../tools/index';

import { ISigned } from './protocol.primitives.interface';
import { u16 } from './protocol.primitives.u16';

export class Option<T> {

    private _value: ISigned<T>;
    private _id: number;

    constructor(id: number, value: ISigned<T>) {
        if (value === undefined || value === null || typeof value.encode !== 'function') {
            throw new Error(`Expected ISigned<T> as value. But has been gotten: ${JSON.stringify(value)}`);
        }
        this._value = value;
        this._id = id;
    }

    public encode(): ArrayBufferLike | Error {
        const body: ArrayBufferLike | Error = this._value.encode();
        if (body instanceof Error) {
            return body;
        }
        const id = u16.encode(this._id);
        if (id instanceof Error) {
            return id;
        }
        return Tools.append([id, body]);
    }

    public decode(bytes: ArrayBufferLike, getter: (id: number) => ISigned<any>): Error | undefined {
        const buffer = Buffer.from(bytes);
        const id: number = buffer.readUInt16LE();
        const target: ISigned<any> = getter(id);
        const error: Error | undefined = target.decode(bytes);
        if (error instanceof Error) {
            return error;
        }
        this._id = id;
        this._value = target;
    }

    public get(): T {
        return this._value.get();
    }

    public getSigned(): ISigned<T> {
        return this._value;
    }

}

export class Enum {

    private _allowed: string[] = [];
    private _value: Option<any>;

    constructor(allowed: Array<ISigned<any>>) {
        this._allowed = allowed.map((signed: ISigned<any>) => {
            if (signed === undefined || signed === null || typeof signed.getSignature !== 'function') {
                throw new Error(`Fail to get signatue for ${JSON.stringify(signed)}`);
            }
            return signed.getSignature();
        });
    }

    set(opt: Option<any>): Error | undefined {
        const signature: string = opt.getSigned().getSignature();
        if (!this._allowed.includes(signature)) {
            return new Error(`Fail to set value with signature "${signature}" because allows only: ${this._allowed.join(', ')}`);
        }
        this._value = opt;
    }

    get<T>(): T {
        return this._value.get();
    }

}
