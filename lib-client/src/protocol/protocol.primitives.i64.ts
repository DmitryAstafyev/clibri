// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';
import { CBits } from './protocol.sizes';

export class i64 extends Primitive<bigint> {

    public static getSignature(): string {
        return 'i64';
    }

    public static getSize(): number {
        return 64 / CBits;
    }

    public static encode(value: bigint): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(i64.getSize());
        try {
            buffer.writeBigInt64LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): bigint | Error {
        if (bytes.byteLength !== i64.getSize()) {
            return new Error(`Invalid buffer size. Expected ${i64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readBigInt64LE(0);
        } catch (e) {
            return e;
        }
    }

    public encode(): ArrayBufferLike | Error {
        return i64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): bigint | Error {
        const value = i64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
