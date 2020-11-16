// tslint:disable

/// <reference path="../node_modules/@types/jasmine/index.d.ts" />
/// <reference path="../node_modules/@types/node/index.d.ts" />

//./node_modules/.bin/jasmine-ts src/something.spec.ts
import * as Protocol from '../src/index';

interface PingInMsgBody {
    uuid: string;
}

class PingIn extends Protocol.In.Message<PingInMsgBody> {

    public static readonly id: number = 1;

    public readonly id: number = PingIn.id;

    public validate(): Error | undefined {
        if (typeof this.getMsg().struct.uuid !== 'string') {
            return new Error(`Expecting "uuid" be a string`);
        }
        if (this.getMsg().struct.uuid.trim() === '') {
            return new Error(`Expecting "uuid" would not be empty`);
        }
        return undefined;
    }
}

class PingOut extends Protocol.Out.Message<PingInMsgBody> {

    public static readonly id: number = 1;

    public readonly id: number = PingOut.id;

}

type Messages = typeof PingIn;

class ProtocolImpl extends Protocol.Protocol<Messages> {

    public getMsgRefs(): { [key: number]: Messages } {
        return {
            [PingIn.id]: PingIn,
        }
    }

}

function append(a: ArrayBufferLike, b: ArrayBufferLike): ArrayBufferLike {
    const tmp = new Uint8Array(a.byteLength + b.byteLength);
    tmp.set( new Uint8Array(a), 0);
    tmp.set( new Uint8Array(b), a.byteLength );
    return tmp.buffer;
}

describe('Protocol tests', () => {

    it('Buffer', (done: Function)=> {
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
    });


});
