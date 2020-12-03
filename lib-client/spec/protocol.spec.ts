// tslint:disable

/// <reference path="../node_modules/@types/jasmine/index.d.ts" />
/// <reference path="../node_modules/@types/node/index.d.ts" />

//./node_modules/.bin/jasmine-ts src/something.spec.ts
import * as Protocol from '../src/index';
import { ISigned } from '../src/protocol/protocol.primitives.interface';

interface INested {
    u8: number | undefined;
    u16: number;
    u32: number;
    opt: {
        u8?: number;
        u16?: number;
    };
}

class Nested extends Protocol.Convertor implements INested {

    public static defaults(): Nested {
        return new Nested({ u8: 0, u16: 0, u32: 0, opt: {}});
    }

    public u8: number | undefined;
    public u16: number;
    public u32: number;
    public opt: {
        u8?: number;
        u16?: number;
    };
    private _opt: Protocol.Primitives.Enum;

    constructor(params: INested) {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
        this._opt = new Protocol.Primitives.Enum([
            Protocol.Primitives.u8.getSignature(),
            Protocol.Primitives.u16.getSignature(),  
        ], (id: number): ISigned<any> | undefined => {
            switch (id) {
                case 1: return new Protocol.Primitives.u8(0);
                case 2: return new Protocol.Primitives.u16(0);
            }
        });
        if (Object.keys(this.opt).length > 1){
            throw new Error(`Option cannot have more then 1 value. Property "opt" or class "Nested"`);
        }
        if (this.opt.u8 !== undefined) {
            const err: Error | undefined = this._opt.set(new Protocol.Primitives.Option<number>(1, new Protocol.Primitives.u8(this.opt.u8)));
            if (err instanceof Error) {
                throw err;
            }
        }
        if (this.opt.u16 !== undefined) {
            const err: Error | undefined = this._opt.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.opt.u16)));
            if (err instanceof Error) {
                throw err;
            }
        }
    }

    public getSignature(): string {
        return 'nested';
    }

    public getId(): number {
        return 2;
    }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBuffer(1, Protocol.ESize.u8, this.u8 === undefined ? 0 : Protocol.Primitives.u8.getSize(), this.u8 === undefined ? new Uint8Array() : Protocol.Primitives.u8.encode(this.u8)),
            () => this.getBuffer(2, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.u16)),
            () => this.getBuffer(3, Protocol.ESize.u8, Protocol.Primitives.u32.getSize(), Protocol.Primitives.u32.encode(this.u32)),
            () => { const buffer = this._opt.encode(); return this.getBuffer(4, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const u8buf: ArrayBufferLike | undefined = storage.get(1);
        if (u8buf === undefined) {
            return new Error(`Fail to get property u8`);
        }
        if (u8buf.byteLength === 0) {
            this.u8 = undefined;
        } else {
            const u8: number | Error = this.getValue<number>(storage, 1, Protocol.Primitives.u8.decode);
            if (u8 instanceof Error) {
                return u8;
            } else {
                this.u8 = u8;
            }
        }
        const u16: number | Error = this.getValue<number>(storage, 2, Protocol.Primitives.u16.decode);
        if (u16 instanceof Error) {
            return u16;
        } else {
            this.u16 = u16;
        }
        const u32: number | Error = this.getValue<number>(storage, 3, Protocol.Primitives.u32.decode);
        if (u32 instanceof Error) {
            return u32;
        } else {
            this.u32 = u32;
        }
        const optbuf: ArrayBufferLike | undefined = storage.get(4);
        if (optbuf === undefined) {
            return new Error(`Fail to get property u8`);
        }
        this.opt = {};
        if (optbuf.byteLength > 0) {
            const optErr: Error | undefined = this._opt.decode(optbuf);
            if (optErr instanceof Error) {
                return optErr;
            } else {
                switch (this._opt.getValueIndex()) {
                    case 1: this.opt.u8 = this._opt.get<number>(); break;
                    case 2: this.opt.u16 = this._opt.get<number>(); break;
                }
            }
        }
    }

    public defaults(): Nested {
        return Nested.defaults();
    }

}

interface IMessage {
    u8: number;
    u16: number;
    u32: number;
    u64: bigint;
    i8: number;
    i16: number;
    i32: number;
    i64: bigint;
    f32: number;
    f64: number;
    bool: boolean;
    nested: Nested;
    arrNested: Nested[];
    arrU8: number[];
    arrU16: number[];
    arrU32: number[];
    arrU64: Array<bigint>;
    arrI8: number[];
    arrI16: number[];
    arrI32: number[];
    arrI64: Array<bigint>;
    arrF32: number[];
    arrF64: number[];
    str: string;
    arrStr: string[];
    arrBool: boolean[];
}

class Message extends Protocol.Convertor implements IMessage {

    public static defaults(): Message {
        return new Message({
            u8: 0,
            u16: 0,
            u32: 0,
            u64: BigInt(0),
            i8: 0,
            i16: 0,
            i32: 0,
            i64: BigInt(0),
            f32: 0,
            f64: 0,
            bool: false,
            nested: new Nested({ u8: undefined, u16: 0, u32: 0, opt: { } }),
            arrNested: [],
            arrU8: [],
            arrU16: [],
            arrU32: [],
            arrU64: [],
            arrI8: [],
            arrI16: [],
            arrI32: [],
            arrI64: [],
            arrF32: [],
            arrF64: [],
            str: '',
            arrStr: [],
            arrBool: []
        });
    }

    public u8: number;
    public u16: number;
    public u32: number;
    public u64: bigint;
    public i8: number;
    public i16: number;
    public i32: number;
    public i64: bigint;
    public f32: number;
    public f64: number;
    public bool: boolean;
    public nested: Nested;
    public arrNested: Nested[];
    public arrU8: number[];
    public arrU16: number[];
    public arrU32: number[];
    public arrU64: Array<bigint>;
    public arrI8: number[];
    public arrI16: number[];
    public arrI32: number[];
    public arrI64: Array<bigint>;
    public arrF32: number[];
    public arrF64: number[];
    public str: string;
    public arrStr: string[];
    public arrBool: boolean[];

    constructor(params: IMessage) {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }

    public getSignature(): string {
        return 'Message';
    }

    public getId(): number {
        return 1;
    }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBuffer(1, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.u8)),
            () => this.getBuffer(2, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.u16)),
            () => this.getBuffer(3, Protocol.ESize.u8, Protocol.Primitives.u32.getSize(), Protocol.Primitives.u32.encode(this.u32)),
            () => this.getBuffer(4, Protocol.ESize.u8, Protocol.Primitives.u64.getSize(), Protocol.Primitives.u64.encode(this.u64)),
            () => this.getBuffer(5, Protocol.ESize.u8, Protocol.Primitives.i8.getSize(), Protocol.Primitives.i8.encode(this.i8)),
            () => this.getBuffer(6, Protocol.ESize.u8, Protocol.Primitives.i16.getSize(), Protocol.Primitives.i16.encode(this.i16)),
            () => this.getBuffer(7, Protocol.ESize.u8, Protocol.Primitives.i32.getSize(), Protocol.Primitives.i32.encode(this.i32)),
            () => this.getBuffer(8, Protocol.ESize.u8, Protocol.Primitives.i64.getSize(), Protocol.Primitives.i64.encode(this.i64)),
            () => this.getBuffer(9, Protocol.ESize.u8, Protocol.Primitives.f32.getSize(), Protocol.Primitives.f32.encode(this.f32)),
            () => this.getBuffer(10, Protocol.ESize.u8, Protocol.Primitives.f64.getSize(), Protocol.Primitives.f64.encode(this.f64)),
            () => { const buffer = this.nested.encode(); return this.getBuffer(11, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => this.getBufferFromBuf<number[]>(12, Protocol.ESize.u64, Protocol.Primitives.ArrayU8.encode, this.arrU8),
            () => this.getBufferFromBuf<number[]>(13, Protocol.ESize.u64, Protocol.Primitives.ArrayU16.encode, this.arrU16),
            () => this.getBufferFromBuf<number[]>(14, Protocol.ESize.u64, Protocol.Primitives.ArrayU32.encode, this.arrU32),
            () => this.getBufferFromBuf<Array<bigint>>(15, Protocol.ESize.u64, Protocol.Primitives.ArrayU64.encode, this.arrU64),
            () => this.getBufferFromBuf<number[]>(16, Protocol.ESize.u64, Protocol.Primitives.ArrayI8.encode, this.arrI8),
            () => this.getBufferFromBuf<number[]>(17, Protocol.ESize.u64, Protocol.Primitives.ArrayI16.encode, this.arrI16),
            () => this.getBufferFromBuf<number[]>(18, Protocol.ESize.u64, Protocol.Primitives.ArrayI32.encode, this.arrI32),
            () => this.getBufferFromBuf<Array<bigint>>(19, Protocol.ESize.u64, Protocol.Primitives.ArrayI64.encode, this.arrI64),
            () => this.getBufferFromBuf<number[]>(20, Protocol.ESize.u64, Protocol.Primitives.ArrayF32.encode, this.arrF32),
            () => this.getBufferFromBuf<number[]>(21, Protocol.ESize.u64, Protocol.Primitives.ArrayF64.encode, this.arrF64),
            () => this.getBufferFromBuf<string>(22, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.str),
            () => this.getBufferFromBuf<string[]>(23, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.arrStr),
            () => this.getBuffer(24, Protocol.ESize.u8, Protocol.Primitives.bool.getSize(), Protocol.Primitives.bool.encode(this.bool)),
            () => this.getBufferFromBuf<boolean[]>(25, Protocol.ESize.u64, Protocol.Primitives.ArrayBool.encode, this.arrBool),
            () => {
                const self: Nested = Nested.defaults();
                return this.getBufferFromBuf<Nested[]>(26, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.arrNested)
            },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const u8: number | Error = this.getValue<number>(storage, 1, Protocol.Primitives.u8.decode);
        if (u8 instanceof Error) {
            return u8;
        } else {
            this.u8 = u8;
        }
        const u16: number | Error = this.getValue<number>(storage, 2, Protocol.Primitives.u16.decode);
        if (u16 instanceof Error) {
            return u16;
        } else {
            this.u16 = u16;
        }
        const u32: number | Error = this.getValue<number>(storage, 3, Protocol.Primitives.u32.decode);
        if (u32 instanceof Error) {
            return u32;
        } else {
            this.u32 = u32;
        }
        const u64: bigint | Error = this.getValue<bigint>(storage, 4, Protocol.Primitives.u64.decode);
        if (u64 instanceof Error) {
            return u64;
        } else {
            this.u64 = u64;
        }
        const i8: number | Error = this.getValue<number>(storage, 5, Protocol.Primitives.i8.decode);
        if (i8 instanceof Error) {
            return i8;
        } else {
            this.i8 = i8;
        }
        const i16: number | Error = this.getValue<number>(storage, 6, Protocol.Primitives.i16.decode);
        if (i16 instanceof Error) {
            return i16;
        } else {
            this.i16 = i16;
        }
        const i32: number | Error = this.getValue<number>(storage, 7, Protocol.Primitives.i32.decode);
        if (i32 instanceof Error) {
            return i32;
        } else {
            this.i32 = i32;
        }
        const i64: bigint | Error = this.getValue<bigint>(storage, 8, Protocol.Primitives.i64.decode);
        if (i64 instanceof Error) {
            return i64;
        } else {
            this.i64 = i64;
        }
        const f32: number | Error = this.getValue<number>(storage, 9, Protocol.Primitives.f32.decode);
        if (f32 instanceof Error) {
            return f32;
        } else {
            this.f32 = f32;
        }
        const f64: number | Error = this.getValue<number>(storage, 10, Protocol.Primitives.f64.decode);
        if (f64 instanceof Error) {
            return f64;
        } else {
            this.f64 = f64;
        }
        const nested: Nested = new Nested({ u8: undefined, u16: 0, u32: 0, opt: { } });
        const nestedBuf: ArrayBufferLike = storage.get(11);
        if (nestedBuf instanceof Error) {
            return nestedBuf;
        }
        const error: Error | undefined = nested.decode(nestedBuf);
        if (error instanceof Error) {
            return error;
        } else {
            this.nested = nested;
        }
        const arrU8: number[] | Error = this.getValue<number[]>(storage, 12, Protocol.Primitives.ArrayU8.decode);
        if (arrU8 instanceof Error) {
            return arrU8;
        } else {
            this.arrU8 = arrU8;
        }
        const arrU16: number[] | Error = this.getValue<number[]>(storage, 13, Protocol.Primitives.ArrayU16.decode);
        if (arrU16 instanceof Error) {
            return arrU16;
        } else {
            this.arrU16 = arrU16;
        }
        const arrU32: number[] | Error = this.getValue<number[]>(storage, 14, Protocol.Primitives.ArrayU32.decode);
        if (arrU32 instanceof Error) {
            return arrU32;
        } else {
            this.arrU32 = arrU32;
        }
        const arrU64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 15, Protocol.Primitives.ArrayU64.decode);
        if (arrU64 instanceof Error) {
            return arrU64;
        } else {
            this.arrU64 = arrU64;
        }
        const arrI8: number[] | Error = this.getValue<number[]>(storage, 16, Protocol.Primitives.ArrayI8.decode);
        if (arrI8 instanceof Error) {
            return arrI8;
        } else {
            this.arrI8 = arrI8;
        }
        const arrI16: number[] | Error = this.getValue<number[]>(storage, 17, Protocol.Primitives.ArrayI16.decode);
        if (arrI16 instanceof Error) {
            return arrI16;
        } else {
            this.arrI16 = arrI16;
        }
        const arrI32: number[] | Error = this.getValue<number[]>(storage, 18, Protocol.Primitives.ArrayI32.decode);
        if (arrI32 instanceof Error) {
            return arrI32;
        } else {
            this.arrI32 = arrI32;
        }
        const arrI64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 19, Protocol.Primitives.ArrayI64.decode);
        if (arrI64 instanceof Error) {
            return arrI64;
        } else {
            this.arrI64 = arrI64;
        }
        const arrF32: number[] | Error = this.getValue<number[]>(storage, 20, Protocol.Primitives.ArrayF32.decode);
        if (arrF32 instanceof Error) {
            return arrF32;
        } else {
            this.arrF32 = arrF32;
        }
        const arrF64: number[] | Error = this.getValue<number[]>(storage, 21, Protocol.Primitives.ArrayF64.decode);
        if (arrF64 instanceof Error) {
            return arrF64;
        } else {
            this.arrF64 = arrF64;
        }
        const str: string | Error = this.getValue<string>(storage, 22, Protocol.Primitives.StrUTF8.decode);
        if (str instanceof Error) {
            return str;
        } else {
            this.str = str;
        }
        const arrStr: string[] | Error = this.getValue<string[]>(storage, 23, Protocol.Primitives.ArrayStrUTF8.decode);
        if (arrStr instanceof Error) {
            return arrStr;
        } else {
            this.arrStr = arrStr;
        }
        const bool: boolean | Error = this.getValue<boolean>(storage, 24, Protocol.Primitives.bool.decode);
        if (bool instanceof Error) {
            return bool;
        } else {
            this.bool = bool;
        }
        const arrBool: boolean[] | Error = this.getValue<boolean[]>(storage, 25, Protocol.Primitives.ArrayBool.decode);
        if (arrBool instanceof Error) {
            return arrBool;
        } else {
            this.arrBool = arrBool;
        }
        const arrNestedInst: Nested = Nested.defaults();
        const arrNested: Array<any> | Error = this.getValue<Nested[]>(storage, 26, arrNestedInst.decodeSelfArray.bind(arrNestedInst));
        if (arrNested instanceof Error) {
            return arrNested;
        } else {
            this.arrNested = arrNested as Nested[];
        }
    }

    public defaults(): Message {
        return Message.defaults();
    }

}


describe('Protocol tests', () => {

    it('Options / Enum', (done: Function)=> {
        function factory() {
            return new Protocol.Primitives.Enum([
                Protocol.Primitives.u8.getSignature(),
                Protocol.Primitives.u16.getSignature(),
                Protocol.Primitives.u32.getSignature(),
                Protocol.Primitives.u64.getSignature(),
                Protocol.Primitives.i8.getSignature(),
                Protocol.Primitives.i16.getSignature(),
                Protocol.Primitives.i32.getSignature(),
                Protocol.Primitives.i64.getSignature(),
                Protocol.Primitives.f32.getSignature(),
                Protocol.Primitives.f64.getSignature(),
                Protocol.Primitives.bool.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.ArrayU8.getSignature(),
                Protocol.Primitives.ArrayU16.getSignature(),
                Protocol.Primitives.ArrayU32.getSignature(),
                Protocol.Primitives.ArrayU64.getSignature(),
                Protocol.Primitives.ArrayI8.getSignature(),
                Protocol.Primitives.ArrayI16.getSignature(),
                Protocol.Primitives.ArrayI32.getSignature(),
                Protocol.Primitives.ArrayI64.getSignature(),
                Protocol.Primitives.ArrayF32.getSignature(),
                Protocol.Primitives.ArrayF64.getSignature(),
                Protocol.Primitives.ArrayStrUTF8.getSignature(),
                Protocol.Primitives.ArrayBool.getSignature(),
            ], (id: number): ISigned<any> | undefined => {
                switch (id) {
                    case 1: return new Protocol.Primitives.u8(0);
                    case 2: return new Protocol.Primitives.u16(0);
                    case 3: return new Protocol.Primitives.u32(0);
                    case 4: return new Protocol.Primitives.u64(BigInt(0));
                    case 5: return new Protocol.Primitives.i8(0);
                    case 6: return new Protocol.Primitives.i16(0);
                    case 7: return new Protocol.Primitives.i32(0);
                    case 8: return new Protocol.Primitives.i64(BigInt(0));
                    case 9: return new Protocol.Primitives.f32(0);
                    case 10: return new Protocol.Primitives.f64(0);
                    case 11: return new Protocol.Primitives.StrUTF8('');
                    case 12: return new Protocol.Primitives.ArrayU8([]);
                    case 13: return new Protocol.Primitives.ArrayU16([]);
                    case 14: return new Protocol.Primitives.ArrayU32([]);
                    case 15: return new Protocol.Primitives.ArrayU64([]);
                    case 16: return new Protocol.Primitives.ArrayI8([]);
                    case 17: return new Protocol.Primitives.ArrayI16([]);
                    case 18: return new Protocol.Primitives.ArrayI32([]);
                    case 19: return new Protocol.Primitives.ArrayI64([]);
                    case 20: return new Protocol.Primitives.ArrayF32([]);
                    case 21: return new Protocol.Primitives.ArrayF64([]);
                    case 22: return new Protocol.Primitives.ArrayStrUTF8([]);
                    case 23: return new Protocol.Primitives.ArrayBool([]);
                    case 24: return new Protocol.Primitives.bool(false);
                }
            });
        }
        const a = factory();
        const b = factory();
        const options = [
            new Protocol.Primitives.Option<number>(1, new Protocol.Primitives.u8(99)),
            new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(999)),
            new Protocol.Primitives.Option<number>(3, new Protocol.Primitives.u32(9999)),
            new Protocol.Primitives.Option<bigint>(4, new Protocol.Primitives.u64(BigInt(99999))),
            new Protocol.Primitives.Option<number>(5, new Protocol.Primitives.i8(99)),
            new Protocol.Primitives.Option<number>(6, new Protocol.Primitives.i16(999)),
            new Protocol.Primitives.Option<number>(7, new Protocol.Primitives.i32(9999)),
            new Protocol.Primitives.Option<bigint>(8, new Protocol.Primitives.i64(BigInt(99999))),
            new Protocol.Primitives.Option<number>(9, new Protocol.Primitives.f32(999)),
            new Protocol.Primitives.Option<number>(10, new Protocol.Primitives.f64(9999)),
            new Protocol.Primitives.Option<string>(11, new Protocol.Primitives.StrUTF8('Planet')),
            new Protocol.Primitives.Option<number[]>(12, new Protocol.Primitives.ArrayU8([99, 100])),
            new Protocol.Primitives.Option<number[]>(13, new Protocol.Primitives.ArrayU16([99, 100])),
            new Protocol.Primitives.Option<number[]>(14, new Protocol.Primitives.ArrayU32([99, 100])),
            new Protocol.Primitives.Option<bigint[]>(15, new Protocol.Primitives.ArrayU64([BigInt(99999), BigInt(99999)])),
            new Protocol.Primitives.Option<number[]>(16, new Protocol.Primitives.ArrayI8([99, 100])),
            new Protocol.Primitives.Option<number[]>(17, new Protocol.Primitives.ArrayI16([99, 100])),
            new Protocol.Primitives.Option<number[]>(18, new Protocol.Primitives.ArrayI32([99, 100])),
            new Protocol.Primitives.Option<bigint[]>(19, new Protocol.Primitives.ArrayI64([BigInt(99999), BigInt(99999)])),
            new Protocol.Primitives.Option<number[]>(20, new Protocol.Primitives.ArrayF32([99, 100])),
            new Protocol.Primitives.Option<number[]>(21, new Protocol.Primitives.ArrayF64([99, 100])),
            new Protocol.Primitives.Option<string[]>(22, new Protocol.Primitives.ArrayStrUTF8(['Planet A', 'Planet B'])),
            new Protocol.Primitives.Option<boolean[]>(23, new Protocol.Primitives.ArrayBool([true, false])),
            new Protocol.Primitives.Option<boolean>(24, new Protocol.Primitives.bool(true)),
        ];
        options.forEach((opt) => {
            expect(a.set(opt)).toBe(undefined);
            const buf = a.encode();
            if (buf instanceof Error) {
                return fail(buf);
            }
            const err: Error | undefined = b.decode(buf);
            if (err instanceof Error) {
                return fail(err);
            }
            expect(b.get<any>()).toEqual(opt.get());
        });
        const c = new Protocol.Primitives.Enum([
            Protocol.Primitives.u8.getSignature(),
        ], (id: number): ISigned<any> | undefined => {
            switch (id) {
                case 1: return new Protocol.Primitives.u8(0);
            }
        });
        const optU16 = new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(999));
        expect(c.set(optU16)).toBeInstanceOf(Error);
        done();
    });

    it('Encode / Decode', (done: Function)=> {
        const a: Message = new Message({
            u8: 1,
            u16: 2,
            u32: 3,
            u64: BigInt(4),
            i8: 5,
            i16: 6,
            i32: 7,
            i64: BigInt(8),
            f32: 9,
            f64: 10,
            bool: true,
            nested: new Nested({ u8: 10, u16: 11, u32: 12, opt: { u8: 10 } }),
            arrNested: [
                new Nested({ u8: 10, u16: 11, u32: 12, opt: { u8: 10 } }),
                new Nested({ u8: 11, u16: 12, u32: 14, opt: { u8: 11 } }),
                new Nested({ u8: 12, u16: 13, u32: 15, opt: { u16: 12 } })
            ],
            arrU8: [1,2,3,4,5],
            arrU16: [1,2,3,4,5],
            arrU32: [1,2,3,4,5],
            arrU64: [BigInt(1),BigInt(2),BigInt(3),BigInt(4),BigInt(5)],
            arrI8: [1,2,3,4,5],
            arrI16: [1,2,3,4,5],
            arrI32: [1,2,3,4,5],
            arrI64: [BigInt(1),BigInt(2),BigInt(3),BigInt(4),BigInt(5)],
            arrF32: [0.1,0.2,0.3,0.4,0.5],
            arrF64: [0.1,0.2,0.3,0.4,0.5],
            str: "Hello, from string!",
            arrStr: ["string 1", "string 2", "string 3"],
            arrBool: [true, false, true]
        });
        const buffer = a.encode();
        const b: Message = new Message({
            u8: 0,
            u16: 0,
            u32: 0,
            u64: BigInt(0),
            i8: 0,
            i16: 0,
            i32: 0,
            i64: BigInt(0),
            f32: 0,
            f64: 0,
            bool: false,
            nested: new Nested({ u8: undefined, u16: 0, u32: 0, opt: { } }),
            arrNested: [],
            arrU8: [],
            arrU16: [],
            arrU32: [],
            arrU64: [],
            arrI8: [],
            arrI16: [],
            arrI32: [],
            arrI64: [],
            arrF32: [],
            arrF64: [],
            str: '',
            arrStr: [],
            arrBool: []
        });
        const err = b.decode(buffer);
        if (err instanceof Error) {
            expect(true).toBe(false);
        }
        expect(a.u8).toBe(b.u8);
        expect(a.u16).toBe(b.u16);
        expect(a.u32).toBe(b.u32);
        expect(a.u64).toBe(b.u64);
        expect(a.i8).toBe(b.i8);
        expect(a.i16).toBe(b.i16);
        expect(a.i32).toBe(b.i32);
        expect(a.i64).toBe(b.i64);
        expect(a.f32).toBe(b.f32);
        expect(a.f64).toBe(b.f64);
        expect(a.bool).toBe(b.bool);
        expect(a.nested.u16).toBe(b.nested.u16);
        expect(a.nested.u32).toBe(b.nested.u32);
        expect(a.nested.opt.u8).toBe(b.nested.opt.u8);
        expect(a.nested.opt.u16).toBe(b.nested.opt.u16);
        expect(a.arrU8.join(',')).toBe(b.arrU8.join(','));
        expect(a.arrU16.join(',')).toBe(b.arrU16.join(','));
        expect(a.arrU32.join(',')).toBe(b.arrU32.join(','));
        expect(a.arrU64.join(',')).toBe(b.arrU64.join(','));
        expect(a.arrI8.join(',')).toBe(b.arrI8.join(','));
        expect(a.arrI16.join(',')).toBe(b.arrI16.join(','));
        expect(a.arrI32.join(',')).toBe(b.arrI32.join(','));
        expect(a.arrI64.join(',')).toBe(b.arrI64.join(','));
        expect(a.arrF32.join(',')).toBe(b.arrF32.map(i => i.toFixed(1)).join(','));
        expect(a.arrF64.join(',')).toBe(b.arrF64.map(i => i.toFixed(1)).join(','));
        expect(a.str).toBe(b.str);
        expect(a.arrStr.join(',')).toBe(b.arrStr.join(','));
        expect(a.arrBool.join(',')).toBe(b.arrBool.join(','));
        a.arrNested.forEach((aNested: Nested, index: number) => {
            expect(aNested.u8).toBe(b.arrNested[index].u8);
            expect(aNested.u16).toBe(b.arrNested[index].u16);
            expect(aNested.u32).toBe(b.arrNested[index].u32);
        });
        const c = new Nested({ u8: 10, u16: 11, u32: 12, opt: { u8: 10 } });
        const c_buff = c.encode();
        const d = new Nested({ u8: 0, u16: 0, u32: 0, opt: { } });
        const c_error: Error | undefined = d.decode(c_buff);
        if (c_error instanceof Error) {
            fail(c_error);
        }
        expect(c.u8).toBe(d.u8);
        expect(c.u16).toBe(d.u16);
        expect(c.u32).toBe(d.u32);
        expect(c.opt.u8).toBe(d.opt.u8);
        expect(c.opt.u16).toBe(d.opt.u16);

        const e = new Nested({ u8: undefined, u16: 11, u32: 12, opt: { u16: 22 } });
        const e_buff = e.encode();
        const e_error: Error | undefined = d.decode(e_buff);
        if (e_error instanceof Error) {
            fail(e_error);
        }
        expect(d.u8).toBe(e.u8);
        expect(d.u8).toBe(undefined);
        expect(d.u16).toBe(e.u16);
        expect(d.u32).toBe(e.u32);
        expect(d.opt.u8).toBe(e.opt.u8);
        expect(d.opt.u16).toBe(e.opt.u16);

        done();
        /*
        const protocol: ProtocolImpl = new ProtocolImpl();
        const reader: Protocol.In.BufferReader<Messages> = new Protocol.In.BufferReader<Messages>(protocol);
        let count = 0;
        reader.subscribe(Protocol.In.BufferReader.events.message, (msg: Protocol.In.Message<PingInMsgBody>) => {
            expect(msg.getId()).toBe(PingIn.id);
            expect(typeof msg.get().uuid).toBe('string');
            count += 1;
            console.log(msg);
            if (count === 12) {
                expect(reader.size()).toBe(0);
                done();
            }
        });
        const disconnected: PingOut = new PingOut({ uuid: Math.round(Math.random() * Math.random() * 1000000).toFixed(0) });
        let buffer = disconnected.encode();
        for (let i = 10; i >= 0; i -= 1) {
            buffer = append(
                buffer,
                (new PingOut({
                    uuid: Math.round(Math.random() * Math.random() * 1000000).toFixed(0)
                })).encode(),
            );
        }
        const buff = Buffer.from(buffer);
        console.log(`Buffer:: len = ${buff.byteLength}`);
        let offset = 0;
        do {
            let step = Math.floor(Math.random() * 40);
            if (offset + step >= buff.byteLength) {
                step = buff.byteLength - offset;
            }
            reader.proceed(buff.slice(offset, offset + step));
            offset += step;
            if (offset >= buff.byteLength) {
                break;
            }
        } while (true);
        */
    });


});
