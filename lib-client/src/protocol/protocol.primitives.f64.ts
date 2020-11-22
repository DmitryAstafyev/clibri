// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';
import { CBits } from './protocol.sizes';

export class f64 extends Primitive<number> {

    public static getSignature(): string {
        return 'f64';
    }

    public static getSize(): number {
        return 64 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(f64.getSize());
        try {
            buffer.writeDoubleLE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== f64.getSize()) {
            return new Error(`Invalid buffer size. Expected ${f64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readDoubleLE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return f64.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return f64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = f64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
