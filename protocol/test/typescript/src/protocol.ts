
const Tools: {
    append: typeof append;
} = {
    append: append,
};
export function append(parts: ArrayBufferLike[]): ArrayBufferLike {
    const tmp = new Uint8Array(parts.map(arr => arr.byteLength).reduce((acc, cur) => acc + cur));
    let cursor = 0;
    parts.forEach((arr) => {
        tmp.set( new Uint8Array(arr), cursor);
        cursor += arr.byteLength;
    });
    return tmp.buffer;
}

export const CBits = 8;

export enum ESize {
    u8 = 'u8',
    u16 = 'u16',
    u32 = 'u32',
    u64 = 'u64',
}

export abstract class Primitive<T> {

    private _value: T;

    constructor(value: T) {
        this._value = value;
    }

    public set(value: T) {
        this._value = value;
    }

    public get(): T {
        return this._value;
    }

    public getSignature(): string {
        return '';
    }

    public static encode(value: any): ArrayBufferLike | Error {
        return new Uint8Array();
    }

    public static decode(bytes: ArrayBufferLike): any | Error {
        return;
    }

    abstract encode(): ArrayBufferLike | Error;

    abstract decode(bytes: ArrayBufferLike): T | Error;

}

export interface IPrimitive<T> {

    getSignature(): string;
    get(): T;
    encode(value: any): ArrayBufferLike | Error;
    decode(bytes: ArrayBufferLike): any | Error;

}

export interface ISigned<T> {

    getSignature(): string;
    get(): T;
    encode(): ArrayBufferLike | Error;
    decode(bytes: ArrayBufferLike): T | Error;

}

export interface ISignedDecode<T> {

    getSignature(): string;
    decode(bytes: ArrayBufferLike): T | Error;

}

export class u8 extends Primitive<number> {

    public static getSignature(): string {
        return 'u8';
    }

    public static getSize(): number {
        return 8 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(u8.getSize());
        try {
            buffer.writeUInt8(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== u8.getSize()) {
            return new Error(`Invalid buffer size. Expected ${u8.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readUInt8(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return u8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return u8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = u8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class u16 extends Primitive<number> {

    public static getSignature(): string {
        return 'u16';
    }

    public static getSize(): number {
        return 16 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(u16.getSize());
        try {
            buffer.writeUInt16LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== u16.getSize()) {
            return new Error(`Invalid buffer size. Expected ${u16.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readUInt16LE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return u16.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return u16.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = u16.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

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

    public getSignature(): string {
        return u64.getSignature();
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

export class i16 extends Primitive<number> {

    public static getSignature(): string {
        return 'i16';
    }

    public static getSize(): number {
        return 16 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(i16.getSize());
        try {
            buffer.writeInt16LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== i16.getSize()) {
            return new Error(`Invalid buffer size. Expected ${i16.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readInt16LE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return i16.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return i16.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = i16.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class i32 extends Primitive<number> {

    public static getSignature(): string {
        return 'i32';
    }

    public static getSize(): number {
        return 32 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(i32.getSize());
        try {
            buffer.writeInt32LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== i32.getSize()) {
            return new Error(`Invalid buffer size. Expected ${i32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readInt32LE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return i32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return i32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = i32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

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

    public getSignature(): string {
        return i64.getSignature();
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

export class f32 extends Primitive<number> {

    public static getSignature(): string {
        return 'f32';
    }

    public static getSize(): number {
        return 32 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(f32.getSize());
        try {
            buffer.writeFloatLE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number | Error {
        if (bytes.byteLength !== f32.getSize()) {
            return new Error(`Invalid buffer size. Expected ${f32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            const buffer: Buffer = Buffer.from(bytes);
            return buffer.readFloatLE(0);
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return f32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return f32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number | Error {
        const value = f32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

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

export class StrUTF8 extends Primitive<string> {

    public static getSignature(): string {
        return 'strUtf8';
    }

    public static encode(value: string): ArrayBufferLike | Error {
        const encoder = new TextEncoder();
        return encoder.encode(value);
    }

    public static decode(bytes: ArrayBufferLike): string | Error {
        const decoder = new TextDecoder();
        return decoder.decode(bytes);
    }

    public getSignature(): string {
        return StrUTF8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return StrUTF8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): string | Error {
        const value = StrUTF8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayU8 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayU8';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * u8.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeUInt8(val, offset);
                offset += u8.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < u8.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${u8.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readUInt8(offset));
                offset += u8.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayU8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayU8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayU8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayU16 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayU16';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * u16.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeUInt16LE(val, offset);
                offset += u16.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < u16.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${u16.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readUInt16LE(offset));
                offset += u16.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayU16.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayU16.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayU16.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayU32 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayU32';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * u32.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeUInt32LE(val, offset);
                offset += u32.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < u32.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${u32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readUInt32LE(offset));
                offset += u32.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayU32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayU32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayU32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayU64 extends Primitive<Array<bigint>> {

    public static getSignature(): string {
        return 'ArrayU64';
    }

    public static encode(value: Array<bigint>): ArrayBufferLike | Error {
        const len: number = value.length * u64.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeBigUInt64LE(val, offset);
                offset += u64.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): Array<bigint> | Error {
        if (bytes.byteLength < u64.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${u64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: Array<bigint> = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readBigUInt64LE(offset));
                offset += u64.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayU64.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayU64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): Array<bigint> | Error {
        const value = ArrayU64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayI8 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayI8';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * i8.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeInt8(val, offset);
                offset += i8.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < i8.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${i8.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readInt8(offset));
                offset += i8.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayI8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayI8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayI8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayI16 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayI16';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * i16.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeInt16LE(val, offset);
                offset += i16.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < i16.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${i16.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readInt16LE(offset));
                offset += i16.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayI16.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayI16.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayI16.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayI32 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayI32';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * i32.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeInt32LE(val, offset);
                offset += i32.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < i32.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${i32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readInt32LE(offset));
                offset += i32.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayI32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayI32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayI32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayI64 extends Primitive<Array<bigint>> {

    public static getSignature(): string {
        return 'ArrayI64';
    }

    public static encode(value: Array<bigint>): ArrayBufferLike | Error {
        const len: number = value.length * i64.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeBigInt64LE(val, offset);
                offset += i64.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): Array<bigint> | Error {
        if (bytes.byteLength < i64.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${i64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: Array<bigint> = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readBigInt64LE(offset));
                offset += i64.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayI64.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayI64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): Array<bigint> | Error {
        const value = ArrayI64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayF32 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayF32';
    }

    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * f32.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeFloatLE(val, offset);
                offset += f32.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < f32.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${f32.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readFloatLE(offset));
                offset += f32.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayF32.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayF32.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayF32.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class ArrayF64 extends Primitive<number[]> {

    public static getSignature(): string {
        return 'ArrayF64';
    }
    public static encode(value: number[]): ArrayBufferLike | Error {
        const len: number = value.length * f64.getSize();
        const buffer: Buffer = Buffer.alloc(len);
        try {
            let offset: number = 0;
            value.forEach((val) => {
                buffer.writeDoubleLE(val, offset);
                offset += f64.getSize();
            });
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public static decode(bytes: ArrayBufferLike): number[] | Error {
        if (bytes.byteLength < f64.getSize()) {
            return new Error(`Invalid buffer size. Expected at least ${f64.getSize()} bytes, actual ${bytes.byteLength} bytes`);
        }
        try {
            let offset: number = 0;
            const array: number[] = [];
            const buffer: Buffer = Buffer.from(bytes);
            do {
                array.push(buffer.readDoubleLE(offset));
                offset += f64.getSize();
            } while (buffer.byteLength > offset);
            return array;
        } catch (e) {
            return e;
        }
    }

    public getSignature(): string {
        return ArrayF64.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayF64.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): number[] | Error {
        const value = ArrayF64.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

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

export class ArrayStrUTF8 extends Primitive<string[]> {

    public static getSignature(): string {
        return 'ArrayStrUTF8';
    }

    public static encode(value: string[]): ArrayBufferLike | Error {
        let parts: ArrayBufferLike[] = [];
        let len: number = 0;
        try {
            parts = value.map((val) => {
                const buf = StrUTF8.encode(val);
                if (buf instanceof Error) {
                    throw buf;
                }
                len += buf.byteLength;
                return buf;
            });
        } catch (e) {
            return e;
        }
        const pairs: ArrayBufferLike[] = [];
        try {
            parts.forEach((part) => {
                const partLen = u32.encode(part.byteLength);
                if (partLen instanceof Error) {
                    throw partLen;
                }
                pairs.push(partLen);
                pairs.push(part);
            });
        } catch (e) {
            return e;
        }
        return Tools.append(pairs);
    }

    public static decode(bytes: ArrayBufferLike): string[] | Error {
        const buffer = Buffer.from(bytes);
        const strings: string[] = [];
        let offset: number = 0;
        do {
            const len = buffer.readUInt32LE(offset);
            if (isNaN(len) || !isFinite(len)) {
                return new Error(`Invalid length of string in an array`);
            }
            offset += u32.getSize();
            const body = buffer.slice(offset, offset + len);
            const str = StrUTF8.decode(body);
            if (str instanceof Error) {
                return str;
            }
            strings.push(str);
            offset += body.byteLength;
        } while (offset < buffer.byteLength);
        return strings;
    }

    public getSignature(): string {
        return ArrayStrUTF8.getSignature();
    }

    public encode(): ArrayBufferLike | Error {
        return ArrayStrUTF8.encode(this.get());
    }

    public decode(bytes: ArrayBufferLike): string[] | Error {
        const value = ArrayStrUTF8.decode(bytes);
        if (value instanceof Error) {
            return value;
        }
        this.set(value);
        return value;
    }

}

export class Option<T> {

    private _value: ISigned<T>;
    private _id: number;

    constructor(id: number, value: ISigned<T>) {
        if (value === undefined || value === null || typeof value.encode !== 'function' || typeof value.decode !== 'function') {
            throw new Error(`Expected ISigned<T> as value. But has been gotten: ${JSON.stringify(value)}`);
        }
        this._value = value;
        this._id = id;
    }

    public get(): T {
        return this._value.get();
    }

    public getSigned(): ISigned<T> {
        return this._value;
    }

    public getId(): number {
        return this._id;
    }

}

export class Enum {

    private _allowed: string[] = [];
    private _value: Option<any> | undefined;
    private _getter: (id: number) => ISigned<any>;

    constructor(allowed: string[], getter: (id: number) => ISigned<any>) {
        this._allowed = allowed;
        this._getter = getter;
    }

    public set(opt: Option<any>): Error | undefined {
        const signature: string = opt.getSigned().getSignature();
        if (!this._allowed.includes(signature)) {
            return new Error(`Fail to set value with signature "${signature}" because allows only: ${this._allowed.join(', ')}`);
        }
        this._value = opt;
    }

    public get<T>(): T {
        return this._value.get();
    }

    public getValueIndex(): number {
        return this._value.getId();
    }

    public encode(): ArrayBufferLike {
        if (this._value === undefined) {
            return new Uint8Array();
        }
        const body: ArrayBufferLike | Error = this._value.getSigned().encode();
        if (body instanceof Error) {
            throw body;
        }
        const id = u16.encode(this._value.getId());
        if (id instanceof Error) {
            throw id;
        }
        return Tools.append([id, body]);
    }

    public decode(bytes: ArrayBufferLike): Error | undefined {
        const buffer = Buffer.from(bytes);
        const id: number = buffer.readUInt16LE();
        const target: ISigned<any> = this._getter(id);
        const error: Error | undefined = target.decode(bytes.slice(u16.getSize(), buffer.byteLength));
        if (error instanceof Error) {
            return error;
        }
        try {
            this._value = new Option<any>(id, target);
        } catch (e) {
            return new Error(`Fail to decode due error: ${e}`);
        }
    }



}

type u8Alias = u8; const u8Alias = u8;
type u16Alias = u16; const u16Alias = u16;
type u32Alias = u32; const u32Alias = u32;
type u64Alias = u64; const u64Alias = u64;
type i8Alias = i8; const i8Alias = i8;
type i16Alias = i16; const i16Alias = i16;
type i32Alias = i32; const i32Alias = i32;
type i64Alias = i64; const i64Alias = i64;
type f32Alias = f32; const f32Alias = f32;
type f64Alias = f64; const f64Alias = f64;
type boolAlias = bool; const boolAlias = bool;
type StrUTF8Alias = StrUTF8; const StrUTF8Alias = StrUTF8;
type ArrayU8Alias = ArrayU8; const ArrayU8Alias = ArrayU8;
type ArrayU16Alias = ArrayU16; const ArrayU16Alias = ArrayU16;
type ArrayU32Alias = ArrayU32; const ArrayU32Alias = ArrayU32;
type ArrayU64Alias = ArrayU64; const ArrayU64Alias = ArrayU64;
type ArrayI8Alias = ArrayI8; const ArrayI8Alias = ArrayI8;
type ArrayI16Alias = ArrayI16; const ArrayI16Alias = ArrayI16;
type ArrayI32Alias = ArrayI32; const ArrayI32Alias = ArrayI32;
type ArrayI64Alias = ArrayI64; const ArrayI64Alias = ArrayI64;
type ArrayF32Alias = ArrayF32; const ArrayF32Alias = ArrayF32;
type ArrayF64Alias = ArrayF64; const ArrayF64Alias = ArrayF64;
type ArrayBoolAlias = ArrayBool; const ArrayBoolAlias = ArrayBool;
type ArrayStrUTF8Alias = ArrayStrUTF8; const ArrayStrUTF8Alias = ArrayStrUTF8;
type OptionAlias = Option<any>; const OptionAlias = Option;
type EnumAlias = Enum; const EnumAlias = Enum;
type PrimitiveAlias = Primitive<any>; const PrimitiveAlias = Primitive;

namespace Primitives {
    export const u8 = u8Alias; export type u8 = u8Alias;
    export const u16 = u16Alias; export type u16 = u16Alias;
    export const u32 = u32Alias; export type u32 = u32Alias;
    export const u64 = u64Alias; export type u64 = u64Alias;
    export const i8 = i8Alias; export type i8 = i8Alias;
    export const i16 = i16Alias; export type i16 = i16Alias;
    export const i32 = i32Alias; export type i32 = i32Alias;
    export const i64 = i64Alias; export type i64 = i64Alias;
    export const f32 = f32Alias; export type f32 = f32Alias;
    export const f64 = f64Alias; export type f64 = f64Alias;
    export const bool = boolAlias; export type bool = boolAlias;
    export const StrUTF8 = StrUTF8Alias; export type StrUTF8 = StrUTF8Alias;
    export const ArrayU8 = ArrayU8Alias; export type ArrayU8 = ArrayU8Alias;
    export const ArrayU16 = ArrayU16Alias; export type ArrayU16 = ArrayU16Alias;
    export const ArrayU32 = ArrayU32Alias; export type ArrayU32 = ArrayU32Alias;
    export const ArrayU64 = ArrayU64Alias; export type ArrayU64 = ArrayU64Alias;
    export const ArrayI8 = ArrayI8Alias; export type ArrayI8 = ArrayI8Alias;
    export const ArrayI16 = ArrayI16Alias; export type ArrayI16 = ArrayI16Alias;
    export const ArrayI32 = ArrayI32Alias; export type ArrayI32 = ArrayI32Alias;
    export const ArrayI64 = ArrayI64Alias; export type ArrayI64 = ArrayI64Alias;
    export const ArrayF32 = ArrayF32Alias; export type ArrayF32 = ArrayF32Alias;
    export const ArrayF64 = ArrayF64Alias; export type ArrayF64 = ArrayF64Alias;
    export const ArrayBool = ArrayBoolAlias; export type ArrayBool = ArrayBoolAlias;
    export const ArrayStrUTF8 = ArrayStrUTF8Alias; export type ArrayStrUTF8 = ArrayStrUTF8Alias;
    export const Option = OptionAlias; export type Option = OptionAlias;
    export const Enum = EnumAlias; export type Enum = EnumAlias;
    export const Primitive = PrimitiveAlias; export type Primitive = PrimitiveAlias;
}

interface INext {
    id: number;
    body: ArrayBufferLike;
    position: number;
}

export class Storage {

    private _fields: Map<number, ArrayBufferLike> = new Map();

    public read(bytes: ArrayBufferLike): Error | undefined {
        const buffer = Buffer.from(bytes);
        let position: number = 0;
        do {
            const field: INext | Error = this._next(buffer, position);
            if (field === undefined) {
                return undefined;
            }
            if (field instanceof Error) {
                return field;
            }
            position = field.position;
            this._fields.set(field.id, field.body);
        } while (true);
    }

    public get(id: number): ArrayBufferLike | undefined {
        return this._fields.get(id);
    }

    private _getId(buffer: Buffer, position: number): number | Error {
        try {
            return buffer.readUInt8(position);
        } catch (e) {
            return e;
        }
    }

    private _getRank(buffer: Buffer, position: number): ESize | Error {
        try {
            const rank: number = buffer.readUInt8(position);
            switch(rank) {
                case 8: return ESize.u8;
                case 16: return ESize.u16;
                case 32: return ESize.u32;
                case 64: return ESize.u64;
                default: return new Error(`Invalid size rank`);
            }
        } catch (e) {
            return e;
        }
    }

    private _next(buffer: Buffer, position: number): INext | Error | undefined {
        if (buffer.byteLength === position) {
            return undefined;
        }
        if (buffer.byteLength < position) {
            return new Error(`Invalid position in buffer.`);
        }
        // Get id
        const id: number | Error = this._getId(buffer, position);
        if (id instanceof Error) {
            return id;
        }
        position += 2;
        const rank: ESize | Error = this._getRank(buffer, position);
        if (rank instanceof Error) {
            return rank;
        }
        position += 1;
        try {
            let length: number | bigint;
            switch(rank) {
                case ESize.u8:
                    length = buffer.readUInt8(position);
                    position += Primitives.u8.getSize();
                    break;
                case ESize.u16:
                    length = buffer.readUInt16LE(position);
                    position += Primitives.u16.getSize();
                    break;
                case ESize.u32:
                    length = buffer.readUInt32LE(position);
                    position += Primitives.u32.getSize();
                    break;
                case ESize.u64:
                    length = buffer.readBigUInt64LE(position);
                    position += Primitives.u64.getSize();
                    break;
            };
            const body = buffer.slice(position, position + Number(length));
            position += Number(length);
            return { id, body, position };
        } catch (e) {
            return e;
        }
    }

}

export abstract class Convertor {

    public collect(getters: Array<() => ArrayBufferLike | Error>): ArrayBufferLike {
        const buffers: ArrayBufferLike[] = [];
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
        return Tools.append(buffers);
    }

    public getBuffer(id: number, esize: ESize, size: number | bigint, value: ArrayBufferLike | Error): ArrayBufferLike | Error {
        if (value instanceof Error) {
            return value;
        }
        const idBuf: ArrayBufferLike | Error = Primitives.u16.encode(id);
        if (idBuf instanceof Error) {
            return idBuf;
        }
        let sizeType: ArrayBufferLike | Error;
        let sizeValue: ArrayBufferLike | Error;
        if (esize === ESize.u64 && typeof size !== 'bigint') {
            return new Error(`For size ${ESize.u64}, size should be defined as BigInt`);
        } else if ((esize === ESize.u8 || esize === ESize.u16 || esize === ESize.u32) && typeof size === 'bigint') {
            return new Error(`For sizes ${ESize.u8}, ${ESize.u16}, ${ESize.u32}, size should be defined as Number`);
        }
        switch(esize) {
            case ESize.u8:
                sizeType = Primitives.u8.encode(Primitives.u8.getSize() * CBits);
                sizeValue = Primitives.u8.encode(size as number);
                break;
            case ESize.u16:
                sizeType = Primitives.u8.encode(Primitives.u16.getSize() * CBits);
                sizeValue = Primitives.u16.encode(size as number);
                break;
            case ESize.u32:
                sizeType = Primitives.u8.encode(Primitives.u32.getSize() * CBits);
                sizeValue = Primitives.u32.encode(size as number);
                break;
            case ESize.u64:
                sizeType = Primitives.u8.encode(Primitives.u64.getSize() * CBits);
                sizeValue = Primitives.u64.encode(BigInt(size));
                break;
        }
        if (sizeType instanceof Error) {
            return sizeType;
        }
        if (sizeValue instanceof Error) {
            return sizeValue;
        }
        if (sizeType === undefined || sizeValue === undefined) {
            return new Error(`Size type or size value aren't defined`);
        }
        return Tools.append([idBuf, sizeType, sizeValue, value]);
    }

    public getBufferFromBuf<T>(id: number, esize: ESize, encoder: (...args: any[]) => ArrayBufferLike | Error, value: T): ArrayBufferLike | Error {
        const buffer = encoder(value);
        if (buffer instanceof Error) {
            return buffer;
        }
        return this.getBuffer(id, esize, esize === ESize.u64 ? BigInt(buffer.byteLength) : buffer.byteLength, buffer);
    }

    public getStorage(buffer: ArrayBufferLike): Storage | Error {
        const storage: Storage = new Storage();
        const error: Error | undefined = storage.read(buffer);
        if (error instanceof Error) {
            return error;
        }
        return storage;
    }

    public getValue<T>(storage: Storage, id: number, decoder: (buf: ArrayBufferLike) => T | Error): T | Error {
        const buffer = storage.get(id);
        if (buffer === undefined) {
            return new Error(`Fail to find field with ID "${id}"`);
        }
        return decoder(buffer);
    }

    public encodeSelfArray(items: Array<Required<Convertor>>): ArrayBufferLike | Error {
        let error: Error | undefined;
        const buffers: ArrayBufferLike[] = [];
        items.forEach((item: Required<Convertor>) => {
            if (error !== undefined) {
                return;
            }
            const buffer = item.encode();
            if (buffer instanceof Error) {
                error = buffer;
                return;
            }
            const len = u32.encode(buffer.byteLength);
            if (len instanceof Error) {
                error = len;
                return;
            }
            buffers.push(len);
            buffers.push(buffer);
        });
        if (error !== undefined) {
            return error;
        }
        return Tools.append(buffers);
    }

    public decodeSelfArray(bytes: ArrayBufferLike): Array<Required<Convertor>> | Error {
        const buffer = Buffer.from(bytes);
        const selfs: Array<Required<Convertor>> = [];
        let offset: number = 0;
        do {
            const len = buffer.readUInt32LE(offset);
            if (isNaN(len) || !isFinite(len)) {
                return new Error(`Invalid length of ${this.getSignature()}/${this.getId()} in an array`);
            }
            offset += u32.getSize();
            const body = buffer.slice(offset, offset + len);
            const self = this.defaults();
            const err = self.decode(body);
            if (err instanceof Error) {
                return err;
            }
            selfs.push(self);
            offset += body.byteLength;
        } while (offset < buffer.byteLength);
        return selfs;
    }

    public abstract getSignature(): string;
    public abstract getId(): number;
    public abstract encode(): ArrayBufferLike;
    public abstract decode(buffer: ArrayBufferLike): Error | undefined;
    public abstract defaults(): Convertor;

}

const Protocol: {
    Convertor: typeof Convertor,
    Primitives: typeof Primitives,
    ESize: typeof ESize,
} = {
    Convertor: Convertor,
    Primitives: Primitives,
    ESize: ESize,
};


interface EnumWithSctructs {
    a?: OptionA;
    b?: OptionB;
}

interface SyntaxSugarEnum {
    VariantA?: string;
    VariantB?: string;
    VariantC?: string;
}

interface UserType {
    PointA?: Array<number>;
    PointB?: string;
    PointC?: number;
}

interface IStructName {
    age: number;
    name: string;
}
class StructName extends Protocol.Convertor implements IStructName {

    public static defaults(): StructName {
        return new StructName({ 
            age: 0,
            name: '',
        });
    }
    public age: number;
    public name: string;
    constructor(params: IStructName)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'StructName';
    }
    public getId(): number {
        return 1;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBuffer(2, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.age)),
            () => this.getBufferFromBuf<string>(3, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.name),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const age: number | Error = this.getValue<number>(storage, 2, Protocol.Primitives.u8.decode);
        if (age instanceof Error) {
            return age;
        } else {
            this.age = age;
        }
        const name: string | Error = this.getValue<string>(storage, 3, Protocol.Primitives.StrUTF8.decode);
        if (name instanceof Error) {
            return name;
        } else {
            this.name = name;
        }
    }
    public defaults(): StructName {
        return StructName.defaults();
    }
}

interface IOptionA {
    option_a_field_a: string;
    option_a_field_b: string;
}
class OptionA extends Protocol.Convertor implements IOptionA {

    public static defaults(): OptionA {
        return new OptionA({ 
            option_a_field_a: '',
            option_a_field_b: '',
        });
    }
    public option_a_field_a: string;
    public option_a_field_b: string;
    constructor(params: IOptionA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'OptionA';
    }
    public getId(): number {
        return 4;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(5, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_a_field_a),
            () => this.getBufferFromBuf<string>(6, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_a_field_b),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const option_a_field_a: string | Error = this.getValue<string>(storage, 5, Protocol.Primitives.StrUTF8.decode);
        if (option_a_field_a instanceof Error) {
            return option_a_field_a;
        } else {
            this.option_a_field_a = option_a_field_a;
        }
        const option_a_field_b: string | Error = this.getValue<string>(storage, 6, Protocol.Primitives.StrUTF8.decode);
        if (option_a_field_b instanceof Error) {
            return option_a_field_b;
        } else {
            this.option_a_field_b = option_a_field_b;
        }
    }
    public defaults(): OptionA {
        return OptionA.defaults();
    }
}

interface IOptionB {
    option_b_field_a: string;
    option_b_field_b: string;
}
class OptionB extends Protocol.Convertor implements IOptionB {

    public static defaults(): OptionB {
        return new OptionB({ 
            option_b_field_a: '',
            option_b_field_b: '',
        });
    }
    public option_b_field_a: string;
    public option_b_field_b: string;
    constructor(params: IOptionB)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'OptionB';
    }
    public getId(): number {
        return 7;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(8, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_b_field_a),
            () => this.getBufferFromBuf<string>(9, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_b_field_b),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const option_b_field_a: string | Error = this.getValue<string>(storage, 8, Protocol.Primitives.StrUTF8.decode);
        if (option_b_field_a instanceof Error) {
            return option_b_field_a;
        } else {
            this.option_b_field_a = option_b_field_a;
        }
        const option_b_field_b: string | Error = this.getValue<string>(storage, 9, Protocol.Primitives.StrUTF8.decode);
        if (option_b_field_b instanceof Error) {
            return option_b_field_b;
        } else {
            this.option_b_field_b = option_b_field_b;
        }
    }
    public defaults(): OptionB {
        return OptionB.defaults();
    }
}

interface IUser {
    username: Array<string>;
    email: string | undefined;
    usertype: UserType;
    info: StructName;
}
class User extends Protocol.Convertor implements IUser {

    public static defaults(): User {
        return new User({ 
            username: [],
            email: undefined,
            usertype: {},
            info: new StructName({ 
                age: 0,
                name: '',
            }),
        });
    }
    public username: Array<string>;
    public email: string | undefined;
    public usertype: UserType;
    public info: StructName;
    private _usertype: Primitives.Enum;
    constructor(params: IUser)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
        this._usertype = new Primitives.Enum([
            Protocol.Primitives.ArrayU8.getSignature(),
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.u16.getSignature(),
        ], (id: number): ISigned<any> | undefined => {
            switch (id) {
                case 0: return new Protocol.Primitives.ArrayU8([0]);
                case 1: return new Protocol.Primitives.StrUTF8('');
                case 2: return new Protocol.Primitives.u16(0);
            }
        });
        if (Object.keys(this.usertype).length > 1) {
            throw new Error(`Option cannot have more then 1 value. Property "usertype" or class "User"`);
        }
        if (this.usertype.PointA !== undefined) {
            const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.usertype.PointA)));
            if (err instanceof Error) {
                throw err;
            }
        }
        if (this.usertype.PointB !== undefined) {
            const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.usertype.PointB)));
            if (err instanceof Error) {
                throw err;
            }
        }
        if (this.usertype.PointC !== undefined) {
            const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.usertype.PointC)));
            if (err instanceof Error) {
                throw err;
            }
        }
    }
    public getSignature(): string {
        return 'User';
    }
    public getId(): number {
        return 13;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<Array<string>>(14, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
            () => this.email === undefined ? this.getBuffer(15, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(15, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
            () => { const buffer = this._usertype.encode(); return this.getBuffer(16, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this.info.encode(); return this.getBuffer(17, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const username: Array<string> | Error = this.getValue<Array<string>>(storage, 14, Protocol.Primitives.ArrayStrUTF8.decode);
        if (username instanceof Error) {
            return username;
        } else {
            this.username = username;
        }
        const emailBuf: ArrayBufferLike | undefined = storage.get(15);
        if (emailBuf === undefined) {
            return new Error(`Fail to get property email`);
        }
        if (emailBuf.byteLength === 0) {
            this.email = undefined;
        } else {
            const email: string | Error = this.getValue<string>(storage, 15, Protocol.Primitives.StrUTF8.decode);
            if (email instanceof Error) {
                return email;
            } else {
                this.email = email;
            }
        }
        this.usertype = {};
        const usertypeBuf: ArrayBufferLike = storage.get(16);
        if (usertypeBuf === undefined) {
            return new Error(`Fail to get property "usertype"`);
        }
        if (usertypeBuf.byteLength > 0) {
            const usertypeErr: Error | undefined = this._usertype.decode(usertypeBuf);
            if (usertypeErr instanceof Error) {
                return usertypeErr;
            } else {
                switch (this._usertype.getValueIndex()) {
                    case 0: this.usertype.PointA = this._usertype.get<Array<number>>(); break;
                    case 1: this.usertype.PointB = this._usertype.get<string>(); break;
                    case 2: this.usertype.PointC = this._usertype.get<number>(); break;
                }
            }
        }
        const info: StructName = new StructName({ 
            age: 0,
            name: '',
        });
        const infoBuf: ArrayBufferLike = storage.get(17);
        if (infoBuf instanceof Error) {
            return infoBuf;
        }
        const infoErr: Error | undefined = info.decode(infoBuf);
        if (infoErr instanceof Error) {
            return infoErr;
        } else {
            this.info = info;
        }
    }
    public defaults(): User {
        return User.defaults();
    }
}

interface ILogin {
    users: Array<User>;
}
class Login extends Protocol.Convertor implements ILogin {

    public static defaults(): Login {
        return new Login({ 
            users: [],
        });
    }
    public users: Array<User>;
    constructor(params: ILogin)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'Login';
    }
    public getId(): number {
        return 18;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(19, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const arrUserInst: User = User.defaults();
        const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 19, arrUserInst.decodeSelfArray.bind(arrUserInst));
        if (arrUser instanceof Error) {
            return arrUser;
        } else {
            this.users = arrUser as User[];
        }
    }
    public defaults(): Login {
        return Login.defaults();
    }
}

export namespace GroupA {

    interface UserTypeTest {
        PointA?: number;
        PointB?: number;
        PointC?: number;
    }

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        usertype: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public static defaults(): UserA {
            return new UserA({ 
                username: [],
                email: undefined,
                usertype: {},
            });
        }
        public username: Array<string>;
        public email: string | undefined;
        public usertype: UserType;
        private _usertype: Primitives.Enum;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
            this._usertype = new Primitives.Enum([
                Protocol.Primitives.ArrayU8.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.u16.getSignature(),
            ], (id: number): ISigned<any> | undefined => {
                switch (id) {
                    case 0: return new Protocol.Primitives.ArrayU8([0]);
                    case 1: return new Protocol.Primitives.StrUTF8('');
                    case 2: return new Protocol.Primitives.u16(0);
                }
            });
            if (Object.keys(this.usertype).length > 1) {
                throw new Error(`Option cannot have more then 1 value. Property "usertype" or class "UserA"`);
            }
            if (this.usertype.PointA !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.usertype.PointA)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.usertype.PointB !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.usertype.PointB)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.usertype.PointC !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.usertype.PointC)));
                if (err instanceof Error) {
                    throw err;
                }
            }
        }
        public getSignature(): string {
            return 'UserA';
        }
        public getId(): number {
            return 21;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<Array<string>>(22, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                () => this.email === undefined ? this.getBuffer(23, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(23, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => { const buffer = this._usertype.encode(); return this.getBuffer(24, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const username: Array<string> | Error = this.getValue<Array<string>>(storage, 22, Protocol.Primitives.ArrayStrUTF8.decode);
            if (username instanceof Error) {
                return username;
            } else {
                this.username = username;
            }
            const emailBuf: ArrayBufferLike | undefined = storage.get(23);
            if (emailBuf === undefined) {
                return new Error(`Fail to get property email`);
            }
            if (emailBuf.byteLength === 0) {
                this.email = undefined;
            } else {
                const email: string | Error = this.getValue<string>(storage, 23, Protocol.Primitives.StrUTF8.decode);
                if (email instanceof Error) {
                    return email;
                } else {
                    this.email = email;
                }
            }
            this.usertype = {};
            const usertypeBuf: ArrayBufferLike = storage.get(24);
            if (usertypeBuf === undefined) {
                return new Error(`Fail to get property "usertype"`);
            }
            if (usertypeBuf.byteLength > 0) {
                const usertypeErr: Error | undefined = this._usertype.decode(usertypeBuf);
                if (usertypeErr instanceof Error) {
                    return usertypeErr;
                } else {
                    switch (this._usertype.getValueIndex()) {
                        case 0: this.usertype.PointA = this._usertype.get<Array<number>>(); break;
                        case 1: this.usertype.PointB = this._usertype.get<string>(); break;
                        case 2: this.usertype.PointC = this._usertype.get<number>(); break;
                    }
                }
            }
        }
        public defaults(): UserA {
            return UserA.defaults();
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public static defaults(): LoginA {
            return new LoginA({ 
                users: [],
            });
        }
        public users: Array<User>;
        constructor(params: ILoginA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'LoginA';
        }
        public getId(): number {
            return 25;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(26, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const arrUserInst: User = User.defaults();
            const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 26, arrUserInst.decodeSelfArray.bind(arrUserInst));
            if (arrUser instanceof Error) {
                return arrUser;
            } else {
                this.users = arrUser as User[];
            }
        }
        public defaults(): LoginA {
            return LoginA.defaults();
        }
    }

}

export namespace GroupB {

    interface UserTypeTest {
        PointA?: number;
        PointB?: number;
        PointC?: number;
    }

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        usertype: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public static defaults(): UserA {
            return new UserA({ 
                username: [],
                email: undefined,
                usertype: {},
            });
        }
        public username: Array<string>;
        public email: string | undefined;
        public usertype: UserType;
        private _usertype: Primitives.Enum;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
            this._usertype = new Primitives.Enum([
                Protocol.Primitives.ArrayU8.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.u16.getSignature(),
            ], (id: number): ISigned<any> | undefined => {
                switch (id) {
                    case 0: return new Protocol.Primitives.ArrayU8([0]);
                    case 1: return new Protocol.Primitives.StrUTF8('');
                    case 2: return new Protocol.Primitives.u16(0);
                }
            });
            if (Object.keys(this.usertype).length > 1) {
                throw new Error(`Option cannot have more then 1 value. Property "usertype" or class "UserA"`);
            }
            if (this.usertype.PointA !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.usertype.PointA)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.usertype.PointB !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.usertype.PointB)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.usertype.PointC !== undefined) {
                const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.usertype.PointC)));
                if (err instanceof Error) {
                    throw err;
                }
            }
        }
        public getSignature(): string {
            return 'UserA';
        }
        public getId(): number {
            return 29;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<Array<string>>(30, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                () => this.email === undefined ? this.getBuffer(31, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(31, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => { const buffer = this._usertype.encode(); return this.getBuffer(32, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const username: Array<string> | Error = this.getValue<Array<string>>(storage, 30, Protocol.Primitives.ArrayStrUTF8.decode);
            if (username instanceof Error) {
                return username;
            } else {
                this.username = username;
            }
            const emailBuf: ArrayBufferLike | undefined = storage.get(31);
            if (emailBuf === undefined) {
                return new Error(`Fail to get property email`);
            }
            if (emailBuf.byteLength === 0) {
                this.email = undefined;
            } else {
                const email: string | Error = this.getValue<string>(storage, 31, Protocol.Primitives.StrUTF8.decode);
                if (email instanceof Error) {
                    return email;
                } else {
                    this.email = email;
                }
            }
            this.usertype = {};
            const usertypeBuf: ArrayBufferLike = storage.get(32);
            if (usertypeBuf === undefined) {
                return new Error(`Fail to get property "usertype"`);
            }
            if (usertypeBuf.byteLength > 0) {
                const usertypeErr: Error | undefined = this._usertype.decode(usertypeBuf);
                if (usertypeErr instanceof Error) {
                    return usertypeErr;
                } else {
                    switch (this._usertype.getValueIndex()) {
                        case 0: this.usertype.PointA = this._usertype.get<Array<number>>(); break;
                        case 1: this.usertype.PointB = this._usertype.get<string>(); break;
                        case 2: this.usertype.PointC = this._usertype.get<number>(); break;
                    }
                }
            }
        }
        public defaults(): UserA {
            return UserA.defaults();
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public static defaults(): LoginA {
            return new LoginA({ 
                users: [],
            });
        }
        public users: Array<User>;
        constructor(params: ILoginA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'LoginA';
        }
        public getId(): number {
            return 33;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(34, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const arrUserInst: User = User.defaults();
            const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 34, arrUserInst.decodeSelfArray.bind(arrUserInst));
            if (arrUser instanceof Error) {
                return arrUser;
            } else {
                this.users = arrUser as User[];
            }
        }
        public defaults(): LoginA {
            return LoginA.defaults();
        }
    }

    export namespace GroupC {

        interface UserTypeTest {
            PointA?: number;
            PointB?: number;
            PointC?: number;
        }

        interface IUserA {
            username: Array<string>;
            email: string | undefined;
            usertype: UserType;
        }
        class UserA extends Protocol.Convertor implements IUserA {

            public static defaults(): UserA {
                return new UserA({ 
                    username: [],
                    email: undefined,
                    usertype: {},
                });
            }
            public username: Array<string>;
            public email: string | undefined;
            public usertype: UserType;
            private _usertype: Primitives.Enum;
            constructor(params: IUserA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    this[key] = params[key];
                });
                this._usertype = new Primitives.Enum([
                    Protocol.Primitives.ArrayU8.getSignature(),
                    Protocol.Primitives.StrUTF8.getSignature(),
                    Protocol.Primitives.u16.getSignature(),
                ], (id: number): ISigned<any> | undefined => {
                    switch (id) {
                        case 0: return new Protocol.Primitives.ArrayU8([0]);
                        case 1: return new Protocol.Primitives.StrUTF8('');
                        case 2: return new Protocol.Primitives.u16(0);
                    }
                });
                if (Object.keys(this.usertype).length > 1) {
                    throw new Error(`Option cannot have more then 1 value. Property "usertype" or class "UserA"`);
                }
                if (this.usertype.PointA !== undefined) {
                    const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.usertype.PointA)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
                if (this.usertype.PointB !== undefined) {
                    const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.usertype.PointB)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
                if (this.usertype.PointC !== undefined) {
                    const err: Error | undefined = this._usertype.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.usertype.PointC)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
            }
            public getSignature(): string {
                return 'UserA';
            }
            public getId(): number {
                return 37;
            }
            public encode(): ArrayBufferLike {
                return this.collect([
                    () => this.getBufferFromBuf<Array<string>>(38, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                    () => this.email === undefined ? this.getBuffer(39, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(39, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                    () => { const buffer = this._usertype.encode(); return this.getBuffer(40, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }
            public decode(buffer: ArrayBufferLike): Error | undefined {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const username: Array<string> | Error = this.getValue<Array<string>>(storage, 38, Protocol.Primitives.ArrayStrUTF8.decode);
                if (username instanceof Error) {
                    return username;
                } else {
                    this.username = username;
                }
                const emailBuf: ArrayBufferLike | undefined = storage.get(39);
                if (emailBuf === undefined) {
                    return new Error(`Fail to get property email`);
                }
                if (emailBuf.byteLength === 0) {
                    this.email = undefined;
                } else {
                    const email: string | Error = this.getValue<string>(storage, 39, Protocol.Primitives.StrUTF8.decode);
                    if (email instanceof Error) {
                        return email;
                    } else {
                        this.email = email;
                    }
                }
                this.usertype = {};
                const usertypeBuf: ArrayBufferLike = storage.get(40);
                if (usertypeBuf === undefined) {
                    return new Error(`Fail to get property "usertype"`);
                }
                if (usertypeBuf.byteLength > 0) {
                    const usertypeErr: Error | undefined = this._usertype.decode(usertypeBuf);
                    if (usertypeErr instanceof Error) {
                        return usertypeErr;
                    } else {
                        switch (this._usertype.getValueIndex()) {
                            case 0: this.usertype.PointA = this._usertype.get<Array<number>>(); break;
                            case 1: this.usertype.PointB = this._usertype.get<string>(); break;
                            case 2: this.usertype.PointC = this._usertype.get<number>(); break;
                        }
                    }
                }
            }
            public defaults(): UserA {
                return UserA.defaults();
            }
        }

        interface ILoginA {
            users: Array<User>;
        }
        class LoginA extends Protocol.Convertor implements ILoginA {

            public static defaults(): LoginA {
                return new LoginA({ 
                    users: [],
                });
            }
            public users: Array<User>;
            constructor(params: ILoginA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    this[key] = params[key];
                });
            }
            public getSignature(): string {
                return 'LoginA';
            }
            public getId(): number {
                return 41;
            }
            public encode(): ArrayBufferLike {
                return this.collect([
                    () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(42, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
                ]);
            }
            public decode(buffer: ArrayBufferLike): Error | undefined {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const arrUserInst: User = User.defaults();
                const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 42, arrUserInst.decodeSelfArray.bind(arrUserInst));
                if (arrUser instanceof Error) {
                    return arrUser;
                } else {
                    this.users = arrUser as User[];
                }
            }
            public defaults(): LoginA {
                return LoginA.defaults();
            }
        }

    }

}

