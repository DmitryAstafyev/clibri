// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';
import { CBits } from './protocol.sizes';

export class i8 extends Primitive<number> {

    public static getSignature(): string {
        return 'i8';
    }

    public static getSize(): number {
        return 8 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(i8.getSize());
        try {
            buffer.writeInt8(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== i8.getSize()) {
            return new Error(`Invalid buffer size. Expected ${i8.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readInt8(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return i8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return i8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = i8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
