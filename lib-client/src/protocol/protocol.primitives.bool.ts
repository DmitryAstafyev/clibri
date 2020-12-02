// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from './protocol.primitives.interface';
import { CBits } from './protocol.sizes';

export class bool extends Primitive<boolean> {

    public static getSignature(): string {
        return 'bool';
    }

    public static getSize(): number {
        return 8 / CBits;
    }

    public static encode(value: boolean): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(bool.getSize());
        try {
            buffer.writeUInt8(value ? 1 : 0);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): boolean | Error {
        if (bytes.byteLength !== bool.getSize()) {
            return new Error(`Invalid buffer size. Expected ${bool.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return Math.round(buffer.readUInt8(0)) === 1;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return bool.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return bool.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): boolean | Error {
        const value = bool.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
