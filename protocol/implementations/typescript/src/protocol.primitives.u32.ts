// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';
import { CBits } from './protocol.sizes';

// injectable
export class u32 extends Primitive<number> {

    public static getSignature(): string {
        return 'u32';
    }

    public static getSize(): number {
        return 32 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(u32.getSize());
        try {
            buffer.writeUInt32LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== u32.getSize()) {
            return new Error(`Invalid buffer size. Expected ${u32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readUInt32LE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return u32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return u32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = u32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
