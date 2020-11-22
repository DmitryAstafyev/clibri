// tslint:disable: class-name
// tslint:disable: max-classes-per-file

export abstract class Primitive<T> {

    private _value: T;

    constructor(value: T) {
        this._value = value;
    }

    public set(value: T) {
        this._value = value;
    }

    public get(): T {
        return this._value;
    }

    public getSignature(): string {
        return '';
    }

    public static encode(value: any): ArrayBufferLike | Error {
        return new Uint8Array();
    }

    public static decode(bytes: ArrayBufferLike): any | Error {
        return;
    }

    abstract encode(): ArrayBufferLike | Error;

    abstract decode(bytes: ArrayBufferLike): T | Error;

}

export interface IPrimitive<T> {

    getSignature(): string;
    get(): T;
    encode(value: any): ArrayBufferLike | Error;
    decode(bytes: ArrayBufferLike): any | Error;

}

export interface ISigned<T> {

    getSignature(): string;
    get(): T;
    encode(): ArrayBufferLike | Error;
    decode(bytes: ArrayBufferLike): T | Error;

}

export interface ISignedDecode<T> {

    getSignature(): string;
    decode(bytes: ArrayBufferLike): T | Error;

}


