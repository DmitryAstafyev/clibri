// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';

const CBits = 8;

export class u64 extends Primitive<bigint> {

    public static getSignature(): string {
        return 'u64';
    }

    public static getSize(): number {
        return 64 / CBits;
    }

    public static encode(value: bigint): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(u64.getSize());
        try {
            buffer.writeBigUInt64LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): bigint | Error {
        if (bytes.byteLength !== u64.getSize()) {
            return new Error(`Invalid buffer size. Expected ${u64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readBigUInt64LE(0);
        } catch (e) {
            return e;
        }
    }

    public encode(): ArrayBufferLike | Error {
        return u64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): bigint | Error {
        const value = u64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }
}
