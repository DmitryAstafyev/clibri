// tslint:disable

/// <reference path="../node_modules/@types/jasmine/index.d.ts" />
/// <reference path="../node_modules/@types/node/index.d.ts" />

//./node_modules/.bin/jasmine-ts src/something.spec.ts
import * as Protocol from '../src/index';

interface IPing {
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
}

class Ping extends Protocol.Encode implements IPing {

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

    constructor(params: IPing) {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getId(): number {
        return 1;
    }

    public encode(): ArrayBufferLike {
        const buffers: ArrayBufferLike[] = [];
        const getters: Array<() => ArrayBufferLike | Error> = [
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
        ];
        try {
            getters.forEach((getter: () => ArrayBufferLike | Error) => {
                const buf: ArrayBufferLike | Error = getter();
                if (buf instanceof Error) {
                    throw buf;
                }
                buffers.push(buf);
            });
        } catch (e) {
            return e;
        }
        return Protocol.Tools.append(buffers);

    }

}


describe('Protocol tests', () => {

    it('Buffer', (done: Function)=> {
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
