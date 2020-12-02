// tslint:disable: class-name
// tslint:disable: max-classes-per-file

import { u8 } from './protocol.primitives.u8';
import { Primitive } from './protocol.primitives.interface';

export class ArrayBool extends Primitive<boolean[]> {

    public static getSignature(): string {
        return 'ArrayBool';
    }

    public static encode(value: boolean[]): ArrayBufferLike | Error {
        const len: number = value.length * u8.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeUInt8(val ? 1 : 0, offset);
                offset += u8.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): boolean[] | Error {
        if (bytes.byteLength < u8.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${u8.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: boolean[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(Math.round(buffer.readUInt8(offset)) === 1 ? true : false);
                offset += u8.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayBool.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayBool.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): boolean[] | Error {
        const value = ArrayBool.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}
