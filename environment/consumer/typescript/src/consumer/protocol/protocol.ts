
// tslint:disable: max-classes-per-file
// tslint:disable: class-name
// tslint:disable: no-namespace
// tslint:disable: no-shadowed-variable
// tslint:disable: array-type
// tslint:disable: variable-name
import { Buffer } from 'buffer';

const Tools: {
    append: typeof append;
} = {
    append: append,
};
export function append(parts: ArrayBufferLike[]): ArrayBufferLike {
    if (parts.length === 0) {
        return (new Uint8Array()).buffer;
    }
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

    public static MIN: number = 0;
    public static MAX: number = 255;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < u8.MIN || value > u8.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = 0;
    public static MAX: number = 65535;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < u16.MIN || value > u16.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = 0;
    public static MAX: number = 4294967295;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < u32.MIN || value > u32.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = 0;
    public static MAX: number = Number.MAX_SAFE_INTEGER;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'bigint') {
            return new Error(`Invalid type of variable`);
        }
        if (value < u64.MIN || value > u64.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = -128;
    public static MAX: number = 127;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < i8.MIN || value > i8.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = -32768;
    public static MAX: number = 32767;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < i16.MIN || value > i16.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = -2147483648;
    public static MAX: number = 2147483647;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        if (value < i32.MIN || value > i32.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static MIN: number = -Number.MAX_SAFE_INTEGER;
    public static MAX: number = Number.MAX_SAFE_INTEGER;

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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'bigint') {
            return new Error(`Invalid type of variable`);
        }
        if (value < i64.MIN || value > i64.MAX) {
            return new Error(`Out of range.`);
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'number') {
            return new Error(`Invalid type of variable`);
        }
        if (isNaN(value) || !isFinite(value)) {
            return new Error(`Invalid value of variable: ${value}`);
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'boolean') {
            return new Error(`Invalid type of variable`);
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (typeof value !== 'string') {
            return new Error(`Invalid type of variable`);
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = u8.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = u16.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = u32.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = u64.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = i8.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = i16.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = i32.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = i64.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = f32.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = f64.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = bool.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

    public static validate(value: any): Error | undefined {
        if (!(value instanceof Array)) {
            return new Error(`Invalid type of variable`);
        }
        try {
            value.forEach((val: any, index: number) => {
                const err: Error | undefined = StrUTF8.validate(val);
                if (err instanceof Error) {
                    throw new Error(`Error on index #${index}: ${err.message}`);
                }
            });
        } catch (e) {
            return e;
        }
        return undefined;
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

export abstract class Enum<T> {

    private _value: Option<any> | undefined;

    public setValue(opt: Option<any>): Error | undefined {
        const signature: string = opt.getSigned().getSignature();
        if (!this.getAllowed().includes(signature)) {
            return new Error(`Fail to set value with signature "${signature}" because allows only: ${this.getAllowed().join(', ')}`);
        }
        this._value = opt;
    }

    public getValue<E>(): E {
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
        const target: ISigned<any> = this.getOptionValue(id);
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

    public pack(sequence: number): ArrayBufferLike {
        const id: ArrayBufferLike | Error = Primitives.u32.encode(this.getId());
        const signature: ArrayBufferLike | Error = Primitives.u16.encode(this.signature());
        const seq: ArrayBufferLike | Error = Primitives.u32.encode(sequence);
        const ts = BigInt((new Date()).getTime());
        const timestamp: ArrayBufferLike | Error = Primitives.u64.encode(ts);
        if (id instanceof Error) {
            throw new Error(`Fail to encode id (${this.getId()}) due error: ${id.message}`);
        }
        if (signature instanceof Error) {
            throw new Error(`Fail to encode signature (${this.signature()}) due error: ${signature.message}`);
        }
        if (seq instanceof Error) {
            throw new Error(`Fail to encode seq (${this.getId()}) due error: ${seq.message}`);
        }
        if (timestamp instanceof Error) {
            throw new Error(`Fail to encode timestamp (${ts}) due error: ${timestamp.message}`);
        }
        const buffer: ArrayBufferLike = this.encode();
        const len: ArrayBufferLike | Error = Primitives.u64.encode(BigInt(buffer.byteLength));
        if (len instanceof Error) {
            throw new Error(`Fail to encode len (${ts}) due error: ${len.message}`);
        }
        return Tools.append([id, signature, seq, timestamp, len, buffer]);
    }

    public abstract getAllowed(): string[];
    public abstract getOptionValue(id: number): ISigned<any>;
    public abstract get(): T;
    public abstract set(src: T): Error | undefined;
    public abstract signature(): number;
    public abstract getId(): number;

}

export interface IValidator {
    validate(value: any): Error | undefined;
}

export interface IPropScheme {
    prop: string;
    optional?: boolean;
    types?: Required<IValidator>,
    options?: IPropScheme[],
}

export function validate(obj: any, scheme: IPropScheme[]): Error | undefined {
    if (typeof obj !== 'object' || obj === null) {
        return new Error(`Expecting input to be object`);
    }
    const errors: string[] = scheme.map((property: IPropScheme) => {
        if (property.optional && obj[property.prop] === undefined) {
            return undefined;
        }
        if (property.types !== undefined) {
            const err: Error | undefined = property.types.validate(obj[property.prop]);
            if (err instanceof Error) {
                return err.message;
            } else {
                return undefined;
            }
        } else if (property.options instanceof Array) {
            if (typeof obj[property.prop] !== 'object' || obj[property.prop] === null) {
                return `Property "${property.prop}" should be an object, because it's enum`;
            }
            const target: any = obj[property.prop];
            const options: string[] = [];
            try {
                property.options.forEach((prop: IPropScheme) => {
                    if (prop.types === undefined) {
                        throw new Error(`Invalid option description for option "${prop.prop}" of option "${property.prop}"`);
                    }
                    if (target[prop.prop] !== undefined) {
                        options.push(prop.prop);
                        const err: Error | undefined = prop.types.validate(target[prop.prop]);
                        if (err instanceof Error) {
                            throw new Error(`Fail to validate option "${prop.prop}" of option "${property.prop}" due: ${err.message}`);
                        }
                    }
                });
            } catch (e) {
                return e.message;
            }
            if (options.length > 1) {
                return `Enum should have only one definition or nothing. Found values for: ${options.join(', ')}`;
            }
            return undefined;
        } else {
            return `Invalid map definition for property ${property.prop}`
        }
    }).filter(e => e !== undefined);
    return errors.length > 0 ? new Error(errors.join('\n')) : undefined;
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
type EnumAlias = Enum<any>; const EnumAlias = Enum;
type PrimitiveAlias = Primitive<any>; const PrimitiveAlias = Primitive;

export namespace Primitives {
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

    public pack(sequence: number): ArrayBufferLike {
        const id: ArrayBufferLike | Error = Primitives.u32.encode(this.getId());
        const signature: ArrayBufferLike | Error = Primitives.u16.encode(this.signature());
        const seq: ArrayBufferLike | Error = Primitives.u32.encode(sequence);
        const ts = BigInt((new Date()).getTime());
        const timestamp: ArrayBufferLike | Error = Primitives.u64.encode(ts);
        if (id instanceof Error) {
            throw new Error(`Fail to encode id (${this.getId()}) due error: ${id.message}`);
        }
        if (signature instanceof Error) {
            throw new Error(`Fail to encode signature (${this.signature()}) due error: ${signature.message}`);
        }
        if (seq instanceof Error) {
            throw new Error(`Fail to encode seq (${this.getId()}) due error: ${seq.message}`);
        }
        if (timestamp instanceof Error) {
            throw new Error(`Fail to encode timestamp (${ts}) due error: ${timestamp.message}`);
        }
        const buffer: ArrayBufferLike = this.encode();
        const len: ArrayBufferLike | Error = Primitives.u64.encode(BigInt(buffer.byteLength));
        if (len instanceof Error) {
            throw new Error(`Fail to encode len (${ts}) due error: ${len.message}`);
        }
        return Tools.append([id, signature, seq, timestamp, len, buffer]);
    }

    public abstract getSignature(): string;
    public abstract signature(): number;
    public abstract getId(): number;
    public abstract encode(): ArrayBufferLike;
    public abstract decode(buffer: ArrayBufferLike): Error | undefined;
    public abstract defaults(): Convertor;

}

export class MessageHeader {
    public static readonly ID_LENGTH = 4;
    public static readonly SIGN_LENGTH = 2;
    public static readonly SEQ_LENGTH = 4;
    public static readonly TS_LENGTH = 8;
    public static readonly LEN_LENGTH = 8;
    public static readonly SIZE =
        MessageHeader.ID_LENGTH +
        MessageHeader.SIGN_LENGTH +
        MessageHeader.SEQ_LENGTH +
        MessageHeader.TS_LENGTH +
        MessageHeader.LEN_LENGTH;

    public readonly id: number;
    public readonly signature: number;
    public readonly sequence: number;
    public readonly ts: BigInt;
    public readonly len: number;

    constructor(buffer: Buffer) {
        if (MessageHeader.enow(buffer) === false) {
            throw new Error(
                `Cannot parse header because size problem. Buffer: ${buffer.byteLength} bytes; header size: ${MessageHeader.SIZE} bytes`
            );
        } else {
            this.id = buffer.readUInt32LE(0);
            this.signature = buffer.readUInt16LE(MessageHeader.ID_LENGTH);
            this.sequence = buffer.readUInt32LE(MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH);
            this.ts = buffer.readBigUInt64LE(MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH + MessageHeader.SEQ_LENGTH);
            this.len = Number(buffer.readBigUInt64LE(MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH + MessageHeader.SEQ_LENGTH + MessageHeader.TS_LENGTH));
        }
    }

    public static enow(buffer: Buffer): boolean {
        return buffer.byteLength >= MessageHeader.SIZE;
    }

}


export interface IAvailableMessage<T> {
    header: {
        id: number;
        sequence: number;
        timestamp: BigInt;
    },
    msg: T,
    getRef: <Z>() => Z,
}

export abstract class BufferReader<T> {

    private _buffer: Buffer = Buffer.alloc(0);
    private _queue: T[] = [];

    public abstract signature(): number;

    public abstract getMessage(header: MessageHeader, buffer: Buffer | ArrayBuffer | ArrayBufferLike): T | Error;

    public chunk(buffer: Buffer | ArrayBuffer | ArrayBufferLike): Error[] | undefined {
        const errors: Error[] = [];
        this._buffer = Buffer.concat([this._buffer, buffer instanceof Buffer ? buffer : Buffer.from(buffer)]);
        do {
            if (!MessageHeader.enow(this._buffer)) {
                break;
            }
            const header: MessageHeader = new MessageHeader(this._buffer.slice(0, MessageHeader.SIZE));
            if (this._buffer.byteLength < header.len + MessageHeader.SIZE) {
                break;
            }
            if (header.signature !== this.signature()) {
                errors.push(new Error(`Dismatch of signature for message id="${header.id}". Expected signature: ${this.signature()}; gotten: ${header.signature}`));
            } else {
                const msg = this.getMessage(header, this._buffer.slice(MessageHeader.SIZE, MessageHeader.SIZE + header.len));
                if (msg instanceof Error) {
                    errors.push(msg);
                } else {
                    this._queue.push(msg);
                }
                this._buffer = this._buffer.slice(MessageHeader.SIZE + header.len);
            }
        } while (true);
        return errors.length > 0 ? errors : undefined;
    }

    public destroy() {
        // Drop buffer
        this._buffer = Buffer.alloc(0);
        this._queue = [];
    }

    public pending(): number {
        return this._queue.length;
    }

    public len(): number {
        return this._buffer.byteLength;
    }

    public next(): T | undefined {
        return this._queue.length === 0 ? undefined : this._queue.splice(0, 1)[0];
    }

}
type ESizeAlias = ESize; const ESizeAlias = ESize;
type ConvertorAlias = Convertor; const ConvertorAlias = Convertor;
type IPropSchemeAlias = IPropScheme;
const PrimitivesAlias = Primitives;
const validateAlias = validate;

namespace Protocol {
    export const ESize = ESizeAlias; export type ESize = ESizeAlias;
    export const Convertor = ConvertorAlias; export type Convertor = ConvertorAlias;
    export type IPropScheme = IPropSchemeAlias;
    export const Primitives = PrimitivesAlias;
    export const validate = validateAlias;
}


export interface IAvailableMessages {
    UserRole?: IUserRole,
    UserConnected?: UserConnected,
    UserDisconnected?: UserDisconnected,
    Identification?: Identification.IAvailableMessages,
    UserSignIn?: UserSignIn.IAvailableMessages,
    UserJoin?: UserJoin.IAvailableMessages,
    UserLogout?: UserLogout.IAvailableMessages,
}
export interface IUserRole {
    Admin?: string;
    User?: string;
    Manager?: string;
}

export class UserRole extends Protocol.Primitives.Enum<IUserRole> {
    public static from(obj: any): IUserRole | Error {
        const inst = new UserRole();
        let err: Error | undefined;
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            err = inst.decode(obj);
        } else {
            err = inst.set(obj);
        }
        return err instanceof Error ? err : inst.get();
    }
    public static getId(): number { return 8; }
    public from(obj: any): IUserRole | Error {
        return UserRole.from(obj);
    }
    public signature(): number { return 0; }
    public getId(): number { return 8; }
    public getAllowed(): string[] {
        return [
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.StrUTF8.getSignature(),
        ];
    }
    public getOptionValue(id: number): ISigned<any> {
        switch (id) {
            case 0: return new Protocol.Primitives.StrUTF8('');
            case 1: return new Protocol.Primitives.StrUTF8('');
            case 2: return new Protocol.Primitives.StrUTF8('');
        }
    }
    public get(): IUserRole {
        const target: IUserRole = {};
        switch (this.getValueIndex()) {
            case 0: target.Admin = this.getValue<string>(); break;
            case 1: target.User = this.getValue<string>(); break;
            case 2: target.Manager = this.getValue<string>(); break;
        }
        return target;
    }
    public set(src: IUserRole): Error | undefined{
        if (Object.keys(src).length > 1) {
            return new Error(`Option cannot have more then 1 value.`);
        }
        if (src.Admin !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(src.Admin)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.User !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(src.User)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Manager !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(2, new Protocol.Primitives.StrUTF8(src.Manager)));
            if (err instanceof Error) {
                return err;
            }
        }
    }
}

export interface IUserConnected {
    username: string;
    uuid: string;
}
export class UserConnected extends Protocol.Convertor implements IUserConnected, ISigned<UserConnected> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'username', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
    ];

    public static defaults(): UserConnected {
        return new UserConnected({
            username: '',
            uuid: '',
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<UserConnected>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof UserConnected)) {
                            throw new Error(`Expecting instance of UserConnected on index #${index}`);
                        }
                    });
                } catch (e) {
                    return e;
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof UserConnected ? undefined : new Error(`Expecting instance of UserConnected`);
            }};
        }
    }

    public static from(obj: any): UserConnected | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = UserConnected.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, UserConnected.scheme);
            return error instanceof Error ? error : new UserConnected({
                username: obj.username,
                uuid: obj.uuid,
            });
        }
    }

    public username: string;
    public uuid: string;
    public static getSignature(): string { return 'UserConnected'; }
    public static getId(): number { return 9; }


    constructor(params: IUserConnected)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'UserConnected'; }

    public get(): UserConnected { return this; }

    public getId(): number { return 9; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(10, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.username),
            () => this.getBufferFromBuf<string>(11, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const username: string | Error = this.getValue<string>(storage, 10, Protocol.Primitives.StrUTF8.decode);
        if (username instanceof Error) {
            return username;
        } else {
            this.username = username;
        }
        const uuid: string | Error = this.getValue<string>(storage, 11, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
    }

    public defaults(): UserConnected {
        return UserConnected.defaults();
    }
}

export interface IUserDisconnected {
    username: string;
    uuid: string;
}
export class UserDisconnected extends Protocol.Convertor implements IUserDisconnected, ISigned<UserDisconnected> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'username', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
    ];

    public static defaults(): UserDisconnected {
        return new UserDisconnected({
            username: '',
            uuid: '',
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<UserDisconnected>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof UserDisconnected)) {
                            throw new Error(`Expecting instance of UserDisconnected on index #${index}`);
                        }
                    });
                } catch (e) {
                    return e;
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof UserDisconnected ? undefined : new Error(`Expecting instance of UserDisconnected`);
            }};
        }
    }

    public static from(obj: any): UserDisconnected | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = UserDisconnected.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, UserDisconnected.scheme);
            return error instanceof Error ? error : new UserDisconnected({
                username: obj.username,
                uuid: obj.uuid,
            });
        }
    }

    public username: string;
    public uuid: string;
    public static getSignature(): string { return 'UserDisconnected'; }
    public static getId(): number { return 12; }


    constructor(params: IUserDisconnected)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'UserDisconnected'; }

    public get(): UserDisconnected { return this; }

    public getId(): number { return 12; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(13, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.username),
            () => this.getBufferFromBuf<string>(14, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const username: string | Error = this.getValue<string>(storage, 13, Protocol.Primitives.StrUTF8.decode);
        if (username instanceof Error) {
            return username;
        } else {
            this.username = username;
        }
        const uuid: string | Error = this.getValue<string>(storage, 14, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
    }

    public defaults(): UserDisconnected {
        return UserDisconnected.defaults();
    }
}

export namespace Identification {
    export interface IAvailableMessages {
        Key?: Key,
        Response?: Response,
    }

    export interface IKey {
        uuid: string;
        id: bigint | undefined;
        location: string | undefined;
    }
    export class Key extends Protocol.Convertor implements IKey, ISigned<Key> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'id', types: Protocol.Primitives.u64, optional: true, },
            { prop: 'location', types: Protocol.Primitives.StrUTF8, optional: true, },
        ];

        public static defaults(): Key {
            return new Identification.Key({
                uuid: '',
                id: undefined,
                location: undefined,
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Key>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Key)) {
                                throw new Error(`Expecting instance of Key on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Key ? undefined : new Error(`Expecting instance of Key`);
                }};
            }
        }

        public static from(obj: any): Key | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Key.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Key.scheme);
                return error instanceof Error ? error : new Key({
                    uuid: obj.uuid,
                    id: obj.id,
                    location: obj.location,
                });
            }
        }

        public uuid: string;
        public id: bigint | undefined;
        public location: string | undefined;
        public static getSignature(): string { return 'Key'; }
        public static getId(): number { return 2; }


        constructor(params: IKey)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Key'; }

        public get(): Key { return this; }

        public getId(): number { return 2; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(3, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
                () => this.id === undefined ? this.getBuffer(4, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(4, Protocol.ESize.u8, Protocol.Primitives.u64.getSize(), Protocol.Primitives.u64.encode(this.id)),
                () => this.location === undefined ? this.getBuffer(5, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(5, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.location),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 3, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
            const idBuf: ArrayBufferLike | undefined = storage.get(4);
            if (idBuf === undefined) {
                return new Error(`Fail to get property id`);
            }
            if (idBuf.byteLength === 0) {
                this.id = undefined;
            } else {
                const id: bigint | Error = this.getValue<bigint>(storage, 4, Protocol.Primitives.u64.decode);
                if (id instanceof Error) {
                    return id;
                } else {
                    this.id = id;
                }
            }
            const locationBuf: ArrayBufferLike | undefined = storage.get(5);
            if (locationBuf === undefined) {
                return new Error(`Fail to get property location`);
            }
            if (locationBuf.byteLength === 0) {
                this.location = undefined;
            } else {
                const location: string | Error = this.getValue<string>(storage, 5, Protocol.Primitives.StrUTF8.decode);
                if (location instanceof Error) {
                    return location;
                } else {
                    this.location = location;
                }
            }
        }

        public defaults(): Key {
            return Key.defaults();
        }
    }

    export interface IResponse {
        uuid: string;
    }
    export class Response extends Protocol.Convertor implements IResponse, ISigned<Response> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Response {
            return new Identification.Response({
                uuid: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Response>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Response)) {
                                throw new Error(`Expecting instance of Response on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Response ? undefined : new Error(`Expecting instance of Response`);
                }};
            }
        }

        public static from(obj: any): Response | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Response.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Response.scheme);
                return error instanceof Error ? error : new Response({
                    uuid: obj.uuid,
                });
            }
        }

        public uuid: string;
        public static getSignature(): string { return 'Response'; }
        public static getId(): number { return 6; }


        constructor(params: IResponse)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Response'; }

        public get(): Response { return this; }

        public getId(): number { return 6; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(7, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 7, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
        }

        public defaults(): Response {
            return Response.defaults();
        }
    }

}

export namespace UserSignIn {
    export interface IAvailableMessages {
        Request?: Request,
        Accepted?: Accepted,
        Denied?: Denied,
        Err?: Err,
    }

    export interface IRequest {
        email: string;
        hash: string;
    }
    export class Request extends Protocol.Convertor implements IRequest, ISigned<Request> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'email', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'hash', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Request {
            return new UserSignIn.Request({
                email: '',
                hash: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Request>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Request)) {
                                throw new Error(`Expecting instance of Request on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Request ? undefined : new Error(`Expecting instance of Request`);
                }};
            }
        }

        public static from(obj: any): Request | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Request.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Request.scheme);
                return error instanceof Error ? error : new Request({
                    email: obj.email,
                    hash: obj.hash,
                });
            }
        }

        public email: string;
        public hash: string;
        public static getSignature(): string { return 'Request'; }
        public static getId(): number { return 16; }


        constructor(params: IRequest)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Request'; }

        public get(): Request { return this; }

        public getId(): number { return 16; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(17, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => this.getBufferFromBuf<string>(18, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.hash),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const email: string | Error = this.getValue<string>(storage, 17, Protocol.Primitives.StrUTF8.decode);
            if (email instanceof Error) {
                return email;
            } else {
                this.email = email;
            }
            const hash: string | Error = this.getValue<string>(storage, 18, Protocol.Primitives.StrUTF8.decode);
            if (hash instanceof Error) {
                return hash;
            } else {
                this.hash = hash;
            }
        }

        public defaults(): Request {
            return Request.defaults();
        }
    }

    export interface IAccepted {
        uuid: string;
    }
    export class Accepted extends Protocol.Convertor implements IAccepted, ISigned<Accepted> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Accepted {
            return new UserSignIn.Accepted({
                uuid: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Accepted>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Accepted)) {
                                throw new Error(`Expecting instance of Accepted on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Accepted ? undefined : new Error(`Expecting instance of Accepted`);
                }};
            }
        }

        public static from(obj: any): Accepted | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Accepted.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Accepted.scheme);
                return error instanceof Error ? error : new Accepted({
                    uuid: obj.uuid,
                });
            }
        }

        public uuid: string;
        public static getSignature(): string { return 'Accepted'; }
        public static getId(): number { return 19; }


        constructor(params: IAccepted)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Accepted'; }

        public get(): Accepted { return this; }

        public getId(): number { return 19; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(20, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 20, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
        }

        public defaults(): Accepted {
            return Accepted.defaults();
        }
    }

    export interface IDenied {
        reason: string;
    }
    export class Denied extends Protocol.Convertor implements IDenied, ISigned<Denied> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'reason', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Denied {
            return new UserSignIn.Denied({
                reason: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Denied>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Denied)) {
                                throw new Error(`Expecting instance of Denied on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Denied ? undefined : new Error(`Expecting instance of Denied`);
                }};
            }
        }

        public static from(obj: any): Denied | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Denied.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Denied.scheme);
                return error instanceof Error ? error : new Denied({
                    reason: obj.reason,
                });
            }
        }

        public reason: string;
        public static getSignature(): string { return 'Denied'; }
        public static getId(): number { return 21; }


        constructor(params: IDenied)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Denied'; }

        public get(): Denied { return this; }

        public getId(): number { return 21; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(22, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.reason),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const reason: string | Error = this.getValue<string>(storage, 22, Protocol.Primitives.StrUTF8.decode);
            if (reason instanceof Error) {
                return reason;
            } else {
                this.reason = reason;
            }
        }

        public defaults(): Denied {
            return Denied.defaults();
        }
    }

    export interface IErr {
        error: string;
    }
    export class Err extends Protocol.Convertor implements IErr, ISigned<Err> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'error', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Err {
            return new UserSignIn.Err({
                error: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Err>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Err)) {
                                throw new Error(`Expecting instance of Err on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Err ? undefined : new Error(`Expecting instance of Err`);
                }};
            }
        }

        public static from(obj: any): Err | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Err.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Err.scheme);
                return error instanceof Error ? error : new Err({
                    error: obj.error,
                });
            }
        }

        public error: string;
        public static getSignature(): string { return 'Err'; }
        public static getId(): number { return 23; }


        constructor(params: IErr)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Err'; }

        public get(): Err { return this; }

        public getId(): number { return 23; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(24, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.error),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const error: string | Error = this.getValue<string>(storage, 24, Protocol.Primitives.StrUTF8.decode);
            if (error instanceof Error) {
                return error;
            } else {
                this.error = error;
            }
        }

        public defaults(): Err {
            return Err.defaults();
        }
    }

}

export namespace UserJoin {
    export interface IAvailableMessages {
        Request?: Request,
        Accepted?: Accepted,
        Denied?: Denied,
        Err?: Err,
    }

    export interface IRequest {
        email: string;
        username: string;
        role: IUserRole;
    }
    export class Request extends Protocol.Convertor implements IRequest, ISigned<Request> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'email', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'username', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'role', optional: false, options: [
                { prop: 'Admin', types: Protocol.Primitives.StrUTF8, optional: false, },
                { prop: 'User', types: Protocol.Primitives.StrUTF8, optional: false, },
                { prop: 'Manager', types: Protocol.Primitives.StrUTF8, optional: false, },
            ] },
        ];

        public static defaults(): Request {
            return new UserJoin.Request({
                email: '',
                username: '',
                role: {},
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Request>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Request)) {
                                throw new Error(`Expecting instance of Request on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Request ? undefined : new Error(`Expecting instance of Request`);
                }};
            }
        }

        public static from(obj: any): Request | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Request.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Request.scheme);
                return error instanceof Error ? error : new Request({
                    email: obj.email,
                    username: obj.username,
                    role: obj.role,
                });
            }
        }

        public email: string;
        public username: string;
        public role: IUserRole;
        private _role: Primitives.Enum;
        public static getSignature(): string { return 'Request'; }
        public static getId(): number { return 26; }


        constructor(params: IRequest)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
            this._role = new UserRole()
            this._role.set(this.role);
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Request'; }

        public get(): Request { return this; }

        public getId(): number { return 26; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(27, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => this.getBufferFromBuf<string>(28, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.username),
                () => { const buffer = this._role.encode(); return this.getBuffer(29, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const email: string | Error = this.getValue<string>(storage, 27, Protocol.Primitives.StrUTF8.decode);
            if (email instanceof Error) {
                return email;
            } else {
                this.email = email;
            }
            const username: string | Error = this.getValue<string>(storage, 28, Protocol.Primitives.StrUTF8.decode);
            if (username instanceof Error) {
                return username;
            } else {
                this.username = username;
            }
            this.role = {};
            const roleBuf: ArrayBufferLike = storage.get(29);
            if (roleBuf === undefined) {
                return new Error(`Fail to get property "role"`);
            }
            if (roleBuf.byteLength > 0) {
                const roleErr: Error | undefined = this._role.decode(roleBuf);
                if (roleErr instanceof Error) {
                    return roleErr;
                } else {
                    this.role = this._role.get();
                }
            }
        }

        public defaults(): Request {
            return Request.defaults();
        }
    }

    export interface IAccepted {
        uuid: string;
    }
    export class Accepted extends Protocol.Convertor implements IAccepted, ISigned<Accepted> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Accepted {
            return new UserJoin.Accepted({
                uuid: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Accepted>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Accepted)) {
                                throw new Error(`Expecting instance of Accepted on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Accepted ? undefined : new Error(`Expecting instance of Accepted`);
                }};
            }
        }

        public static from(obj: any): Accepted | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Accepted.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Accepted.scheme);
                return error instanceof Error ? error : new Accepted({
                    uuid: obj.uuid,
                });
            }
        }

        public uuid: string;
        public static getSignature(): string { return 'Accepted'; }
        public static getId(): number { return 30; }


        constructor(params: IAccepted)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Accepted'; }

        public get(): Accepted { return this; }

        public getId(): number { return 30; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(31, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 31, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
        }

        public defaults(): Accepted {
            return Accepted.defaults();
        }
    }

    export interface IDenied {
        reason: string;
    }
    export class Denied extends Protocol.Convertor implements IDenied, ISigned<Denied> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'reason', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Denied {
            return new UserJoin.Denied({
                reason: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Denied>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Denied)) {
                                throw new Error(`Expecting instance of Denied on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Denied ? undefined : new Error(`Expecting instance of Denied`);
                }};
            }
        }

        public static from(obj: any): Denied | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Denied.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Denied.scheme);
                return error instanceof Error ? error : new Denied({
                    reason: obj.reason,
                });
            }
        }

        public reason: string;
        public static getSignature(): string { return 'Denied'; }
        public static getId(): number { return 32; }


        constructor(params: IDenied)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Denied'; }

        public get(): Denied { return this; }

        public getId(): number { return 32; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(33, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.reason),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const reason: string | Error = this.getValue<string>(storage, 33, Protocol.Primitives.StrUTF8.decode);
            if (reason instanceof Error) {
                return reason;
            } else {
                this.reason = reason;
            }
        }

        public defaults(): Denied {
            return Denied.defaults();
        }
    }

    export interface IErr {
        error: string;
    }
    export class Err extends Protocol.Convertor implements IErr, ISigned<Err> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'error', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Err {
            return new UserJoin.Err({
                error: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Err>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Err)) {
                                throw new Error(`Expecting instance of Err on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Err ? undefined : new Error(`Expecting instance of Err`);
                }};
            }
        }

        public static from(obj: any): Err | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Err.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Err.scheme);
                return error instanceof Error ? error : new Err({
                    error: obj.error,
                });
            }
        }

        public error: string;
        public static getSignature(): string { return 'Err'; }
        public static getId(): number { return 34; }


        constructor(params: IErr)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Err'; }

        public get(): Err { return this; }

        public getId(): number { return 34; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(35, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.error),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const error: string | Error = this.getValue<string>(storage, 35, Protocol.Primitives.StrUTF8.decode);
            if (error instanceof Error) {
                return error;
            } else {
                this.error = error;
            }
        }

        public defaults(): Err {
            return Err.defaults();
        }
    }

}

export namespace UserLogout {
    export interface IAvailableMessages {
        Request?: Request,
        Done?: Done,
        Err?: Err,
    }

    export interface IRequest {
        uuid: string;
    }
    export class Request extends Protocol.Convertor implements IRequest, ISigned<Request> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Request {
            return new UserLogout.Request({
                uuid: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Request>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Request)) {
                                throw new Error(`Expecting instance of Request on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Request ? undefined : new Error(`Expecting instance of Request`);
                }};
            }
        }

        public static from(obj: any): Request | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Request.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Request.scheme);
                return error instanceof Error ? error : new Request({
                    uuid: obj.uuid,
                });
            }
        }

        public uuid: string;
        public static getSignature(): string { return 'Request'; }
        public static getId(): number { return 37; }


        constructor(params: IRequest)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Request'; }

        public get(): Request { return this; }

        public getId(): number { return 37; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(38, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 38, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
        }

        public defaults(): Request {
            return Request.defaults();
        }
    }

    export interface IDone {
    }
    export class Done extends Protocol.Convertor implements IDone, ISigned<Done> {

        public static scheme: Protocol.IPropScheme[] = [
        ];

        public static defaults(): Done {
            return new UserLogout.Done({
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Done>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Done)) {
                                throw new Error(`Expecting instance of Done on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Done ? undefined : new Error(`Expecting instance of Done`);
                }};
            }
        }

        public static from(obj: any): Done | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Done.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Done.scheme);
                return error instanceof Error ? error : new Done({
                });
            }
        }

        public static getSignature(): string { return 'Done'; }
        public static getId(): number { return 39; }


        constructor(params: IDone)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Done'; }

        public get(): Done { return this; }

        public getId(): number { return 39; }

        public encode(): ArrayBufferLike {
            return this.collect([
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
        }

        public defaults(): Done {
            return Done.defaults();
        }
    }

    export interface IErr {
        error: string;
    }
    export class Err extends Protocol.Convertor implements IErr, ISigned<Err> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'error', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): Err {
            return new UserLogout.Err({
                error: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<Err>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof Err)) {
                                throw new Error(`Expecting instance of Err on index #${index}`);
                            }
                        });
                    } catch (e) {
                        return e;
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof Err ? undefined : new Error(`Expecting instance of Err`);
                }};
            }
        }

        public static from(obj: any): Err | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = Err.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, Err.scheme);
                return error instanceof Error ? error : new Err({
                    error: obj.error,
                });
            }
        }

        public error: string;
        public static getSignature(): string { return 'Err'; }
        public static getId(): number { return 40; }


        constructor(params: IErr)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'Err'; }

        public get(): Err { return this; }

        public getId(): number { return 40; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(41, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.error),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const error: string | Error = this.getValue<string>(storage, 41, Protocol.Primitives.StrUTF8.decode);
            if (error instanceof Error) {
                return error;
            } else {
                this.error = error;
            }
        }

        public defaults(): Err {
            return Err.defaults();
        }
    }

}

export class BufferReaderMessages extends BufferReader<IAvailableMessage<IAvailableMessages>> {
    public signature(): number { return 0; }
    public getMessage(header: MessageHeader, buffer: Buffer | ArrayBuffer | ArrayBufferLike): IAvailableMessage<IAvailableMessages> | Error {
        let instance: any;
        let enum_instance: any = {};
        let err: Error | undefined;
        switch (header.id) {
            case 8:
                instance = new UserRole();
                if (instance.decode(buffer) instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserRole: instance }, getRef: () => instance };
            case 2:
                instance = Identification.Key.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Identification: { Key: instance } }, getRef: () => instance };
            case 6:
                instance = Identification.Response.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Identification: { Response: instance } }, getRef: () => instance };
            case 9:
                instance = UserConnected.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserConnected: instance }, getRef: () => instance };
            case 12:
                instance = UserDisconnected.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserDisconnected: instance }, getRef: () => instance };
            case 16:
                instance = UserSignIn.Request.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserSignIn: { Request: instance } }, getRef: () => instance };
            case 19:
                instance = UserSignIn.Accepted.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserSignIn: { Accepted: instance } }, getRef: () => instance };
            case 21:
                instance = UserSignIn.Denied.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserSignIn: { Denied: instance } }, getRef: () => instance };
            case 23:
                instance = UserSignIn.Err.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserSignIn: { Err: instance } }, getRef: () => instance };
            case 26:
                instance = UserJoin.Request.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserJoin: { Request: instance } }, getRef: () => instance };
            case 30:
                instance = UserJoin.Accepted.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserJoin: { Accepted: instance } }, getRef: () => instance };
            case 32:
                instance = UserJoin.Denied.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserJoin: { Denied: instance } }, getRef: () => instance };
            case 34:
                instance = UserJoin.Err.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserJoin: { Err: instance } }, getRef: () => instance };
            case 37:
                instance = UserLogout.Request.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserLogout: { Request: instance } }, getRef: () => instance };
            case 39:
                instance = UserLogout.Done.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserLogout: { Done: instance } }, getRef: () => instance };
            case 40:
                instance = UserLogout.Err.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { UserLogout: { Err: instance } }, getRef: () => instance };
        }
    }
}

