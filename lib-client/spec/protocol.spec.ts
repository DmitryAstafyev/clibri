// tslint:disable

/// <reference path="../node_modules/@types/jasmine/index.d.ts" />
/// <reference path="../node_modules/@types/node/index.d.ts" />

//./node_modules/.bin/jasmine-ts src/something.spec.ts
import * as Protocol from '../src/index';

interface INested {
    u16: number;
    u32: number;
}

class Nested extends Protocol.Convertor implements INested {

    public u16: number;
    public u32: number;

    constructor(params: INested) {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getId(): number {
        return 2;
    }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBuffer(1, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.u16)),
            () => this.getBuffer(2, Protocol.ESize.u8, Protocol.Primitives.u32.getSize(), Protocol.Primitives.u32.encode(this.u32)),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const u16: number | Error = this.getValue<number>(storage, 1, Protocol.Primitives.u16.decode);
        if (u16 instanceof Error) {
            return u16;
        } else {
            this.u16 = u16;
        }
        const u32: number | Error = this.getValue<number>(storage, 2, Protocol.Primitives.u32.decode);
        if (u32 instanceof Error) {
            return u32;
        } else {
            this.u32 = u32;
        }
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
    nested: Nested;
    arrU8: number[];
}

class Message extends Protocol.Convertor implements IMessage {

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
    public nested: Nested;
    public arrU8: number[];

    constructor(params: IMessage) {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
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
            () => {
                const buffer = this.nested.encode();
                return this.getBuffer(11, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer)
            },
            () => {
                const buffer = Protocol.Primitives.ArrayU8.encode(this.arrU8);
                if (buffer instanceof Error) {
                    return buffer;
                }
                return this.getBuffer(12, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer);
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
        const nested: Nested = new Nested({ u16: 0, u32: 0 });
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
    }

}


describe('Protocol tests', () => {

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
            nested: new Nested({ u16: 11, u32: 12 }),
            arrU8: [1,2,3,4,5],
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
            nested: new Nested({ u16: 0, u32: 0 }),
            arrU8: [],
        });
        const err = b.decode(buffer);
        if (err instanceof Error) {
            console.log(err);
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
        expect(a.nested.u16).toBe(b.nested.u16);
        expect(a.nested.u32).toBe(b.nested.u32);
        console.log(buffer);
        console.log(b);
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
