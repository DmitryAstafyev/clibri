
// tslint:disable: max-classes-per-file
// tslint:disable: class-name
// tslint:disable: no-namespace
// tslint:disable: no-shadowed-variable
// tslint:disable: array-type
// tslint:disable: variable-name

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
		return "u8";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== u8.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${u8.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readUInt8(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "u16";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== u16.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${u16.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readUInt16LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "u32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== u32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${u32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readUInt32LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "u64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): bigint | Error {
		if (bytes.byteLength !== u64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${u64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readBigUInt64LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "bigint") {
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
		return "i8";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== i8.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${i8.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readInt8(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "i16";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== i16.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${i16.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readInt16LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "i32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== i32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${i32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readInt32LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "i64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): bigint | Error {
		if (bytes.byteLength !== i64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${i64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readBigInt64LE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "bigint") {
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
		return "f32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== f32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${f32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readFloatLE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "f64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number | Error {
		if (bytes.byteLength !== f64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${f64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return buffer.readDoubleLE(0);
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "number") {
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
		return "bool";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): boolean | Error {
		if (bytes.byteLength !== bool.getSize()) {
			return new Error(
				`Invalid buffer size. Expected ${bool.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			const buffer: Buffer = Buffer.from(bytes);
			return Math.round(buffer.readUInt8(0)) === 1;
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static validate(value: any): Error | undefined {
		if (typeof value !== "boolean") {
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
		return "ArrayU8";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < u8.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${u8.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayU16";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < u16.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${u16.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayU32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < u32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${u32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayU64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): Array<bigint> | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < u64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${u64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayI8";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < i8.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${i8.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayI16";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < i16.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${i16.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayI32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < i32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${i32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayI64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): Array<bigint> | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < i64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${i64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayF32";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < f32.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${f32.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayF64";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): number[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < f64.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${f64.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayBool";
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
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	public static decode(bytes: ArrayBufferLike): boolean[] | Error {
		if (bytes.byteLength === 0) {
			return [];
		}
		if (bytes.byteLength < u8.getSize()) {
			return new Error(
				`Invalid buffer size. Expected at least ${u8.getSize()} bytes, actual ${
					bytes.byteLength
				} bytes`
			);
		}
		try {
			let offset: number = 0;
			const array: boolean[] = [];
			const buffer: Buffer = Buffer.from(bytes);
			do {
				array.push(
					Math.round(buffer.readUInt8(offset)) === 1 ? true : false
				);
				offset += u8.getSize();
			} while (buffer.byteLength > offset);
			return array;
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		return "ArrayStrUTF8";
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
		return Tools.append(pairs);
	}

	public static decode(bytes: ArrayBufferLike): string[] | Error {
		const buffer = Buffer.from(bytes);
		const strings: string[] = [];
		if (buffer.byteLength === 0) {
			return strings;
		} else if (buffer.byteLength < u32.getSize()) {
			return new Error(
				`Invalid size marker. Expecting u64 (size ${u32.getSize()} bytes), but size of buffer: ${
					buffer.byteLength
				} bytes.`
			);
		}
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
		if (
			value === undefined ||
			value === null ||
			typeof value.encode !== "function" ||
			typeof value.decode !== "function"
		) {
			throw new Error(
				`Expected ISigned<T> as value. But has been gotten: ${JSON.stringify(
					value
				)}`
			);
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
			return new Error(
				`Fail to set value with signature "${signature}" because allows only: ${this.getAllowed().join(
					", "
				)}`
			);
		}
		this._value = opt;
	}

	public getValue<E>(): E {
		if (this._value === undefined) {
			throw new Error(`Value of enum isn't defined yet.`);
		}
		return this._value.get();
	}

	public getValueIndex(): number {
		if (this._value === undefined) {
			throw new Error(`Value of enum isn't defined yet.`);
		}
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
		const error: Error | undefined = target.decode(
			bytes.slice(u16.getSize(), buffer.byteLength)
		);
		if (error instanceof Error) {
			return error;
		}
		try {
			this._value = new Option<any>(id, target);
		} catch (e) {
			return new Error(`Fail to decode due error: ${e}`);
		}
	}

	public pack(sequence: number, uuid?: string): ArrayBufferLike {
		const id: ArrayBufferLike | Error = Primitives.u32.encode(this.getId());
		const signature: ArrayBufferLike | Error = Primitives.u16.encode(
			this.signature()
		);
		const seq: ArrayBufferLike | Error = Primitives.u32.encode(sequence);
		const ts = BigInt(new Date().getTime());
		const timestamp: ArrayBufferLike | Error = Primitives.u64.encode(ts);
		if (id instanceof Error) {
			throw new Error(
				`Fail to encode id (${this.getId()}) due error: ${id.message}`
			);
		}
		if (signature instanceof Error) {
			throw new Error(
				`Fail to encode signature (${this.signature()}) due error: ${
					signature.message
				}`
			);
		}
		if (seq instanceof Error) {
			throw new Error(
				`Fail to encode seq (${this.getId()}) due error: ${seq.message}`
			);
		}
		if (timestamp instanceof Error) {
			throw new Error(
				`Fail to encode timestamp (${ts}) due error: ${timestamp.message}`
			);
		}
		const buffer: ArrayBufferLike | Error = (() => {
			const middleware: PackingMiddleware | undefined =
				getPackingMiddleware();
			if (middleware instanceof PackingMiddleware) {
				return middleware.encode(
					this.encode(),
					this.getId(),
					sequence,
					uuid
				);
			} else {
				return this.encode();
			}
		})();
		if (buffer instanceof Error) {
			throw buffer;
		}
		const len: ArrayBufferLike | Error = Primitives.u64.encode(
			BigInt(buffer.byteLength)
		);
		if (len instanceof Error) {
			throw new Error(
				`Fail to encode len (${ts}) due error: ${len.message}`
			);
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
	types?: Required<IValidator>;
	options?: IPropScheme[];
}

export function validate(obj: any, scheme: IPropScheme[]): Error | undefined {
	if (typeof obj !== "object" || obj === null) {
		return new Error(`Expecting input to be object`);
	}
	const errors: string[] = scheme
		.map((property: IPropScheme) => {
			if (property.optional && obj[property.prop] === undefined) {
				return undefined;
			}
			if (property.types !== undefined) {
				const err: Error | undefined = property.types.validate(
					obj[property.prop]
				);
				if (err instanceof Error) {
					return err.message;
				} else {
					return undefined;
				}
			} else if (property.options instanceof Array) {
				if (
					typeof obj[property.prop] !== "object" ||
					obj[property.prop] === null
				) {
					return `Property "${property.prop}" should be an object, because it's enum`;
				}
				const target: any = obj[property.prop];
				const options: string[] = [];
				try {
					property.options.forEach((prop: IPropScheme) => {
						if (prop.types === undefined) {
							throw new Error(
								`Invalid option description for option "${prop.prop}" of option "${property.prop}"`
							);
						}
						if (target[prop.prop] !== undefined) {
							options.push(prop.prop);
							const err: Error | undefined = prop.types.validate(
								target[prop.prop]
							);
							if (err instanceof Error) {
								throw new Error(
									`Fail to validate option "${prop.prop}" of option "${property.prop}" due: ${err.message}`
								);
							}
						}
					});
				} catch (err) {
					return err instanceof Error
						? err.message
						: `Unknown error: ${err}`;
				}
				if (options.length > 1) {
					return `Enum should have only one definition or nothing. Found values for: ${options.join(
						", "
					)}`;
				}
				return undefined;
			} else {
				return `Invalid map definition for property ${property.prop}`;
			}
		})
		.filter((err) => err !== undefined) as string[];
	return errors.length > 0 ? new Error(errors.join("\n")) : undefined;
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
			const field: INext | Error | undefined = this._next(
				buffer,
				position
			);
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
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}

	private _getRank(buffer: Buffer, position: number): ESize | Error {
		try {
			const rank: number = buffer.readUInt8(position);
			switch (rank) {
				case 8:
					return ESize.u8;
				case 16:
					return ESize.u16;
				case 32:
					return ESize.u32;
				case 64:
					return ESize.u64;
				default:
					return new Error(`Invalid size rank: ${rank}`);
			}
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
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
			switch (rank) {
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
			}
			const body = buffer.slice(position, position + Number(length));
			position += Number(length);
			return { id, body, position };
		} catch (err) {
			return err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
	}
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
			this.sequence = buffer.readUInt32LE(
				MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH
			);
			this.ts = buffer.readBigUInt64LE(
				MessageHeader.ID_LENGTH +
					MessageHeader.SIGN_LENGTH +
					MessageHeader.SEQ_LENGTH
			);
			this.len = Number(
				buffer.readBigUInt64LE(
					MessageHeader.ID_LENGTH +
						MessageHeader.SIGN_LENGTH +
						MessageHeader.SEQ_LENGTH +
						MessageHeader.TS_LENGTH
				)
			);
		}
	}

	public static enow(buffer: Buffer): boolean {
		return buffer.byteLength >= MessageHeader.SIZE;
	}
}

declare var window: Window | undefined;
declare var global: any | undefined;

export function globals(): Window | any | Error {
	if (typeof window === "object" && window !== null) {
		return window;
	} else if (typeof global === "object" && global !== null) {
		return global;
	} else {
		return new Error(`Fail to find global namespece ()`);
	}
}

export function getPackingMiddleware(): PackingMiddleware | undefined {
	const space = globals();
	if (space instanceof Error) {
		return undefined;
	}
	return space[PackingMiddleware.GUID];
}

export abstract class PackingMiddleware {
	static GUID: string = "___clibriPackingMiddleware___";

	constructor() {
		const space = globals();
		if (space instanceof Error) {
			console.error(
				`Fail to bind PackingMiddleware as soon as fail to find global object (window or NodeJS global)`
			);
			return;
		}
		if (space[PackingMiddleware.GUID] !== undefined) {
			console.warn(`PackingMiddleware instance is overwritten.`);
		}
		space[PackingMiddleware.GUID] = this;
	}

	public decode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		return buffer;
	}

	public encode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		return buffer;
	}
}


export interface IAvailableMessage<T> {
	header: {
		id: number;
		sequence: number;
		timestamp: BigInt;
	};
	msg: T;
	getRef: <Z>() => Z;
}

export abstract class BufferReader<T> {
	private _buffer: Buffer = Buffer.alloc(0);
	private _queue: T[] = [];

	public abstract signature(): number;

	public abstract getMessage(
		header: MessageHeader,
		buffer: Buffer | ArrayBuffer | ArrayBufferLike
	): T | Error;

	public chunk(
		buffer: Buffer | ArrayBuffer | ArrayBufferLike,
		uuid?: string
	): Error[] | undefined {
		const errors: Error[] = [];
		this._buffer = Buffer.concat([
			this._buffer,
			buffer instanceof Buffer ? buffer : Buffer.from(buffer),
		]);
		do {
			if (!MessageHeader.enow(this._buffer)) {
				break;
			}
			const header: MessageHeader = new MessageHeader(
				this._buffer.slice(0, MessageHeader.SIZE)
			);
			if (this._buffer.byteLength < header.len + MessageHeader.SIZE) {
				break;
			}
			if (header.signature !== this.signature()) {
				errors.push(
					new Error(
						`Dismatch of signature for message id="${
							header.id
						}". Expected signature: ${this.signature()}; gotten: ${
							header.signature
						}`
					)
				);
			} else {
				const body: ArrayBufferLike | Error = (() => {
					const middleware: PackingMiddleware | undefined =
						getPackingMiddleware();
					if (middleware instanceof PackingMiddleware) {
						return middleware.decode(
							this._buffer.slice(
								MessageHeader.SIZE,
								MessageHeader.SIZE + header.len
							),
							header.id,
							header.sequence,
							uuid
						);
					} else {
						return this._buffer.slice(
							MessageHeader.SIZE,
							MessageHeader.SIZE + header.len
						);
					}
				})();
				if (body instanceof Error) {
					errors.push(body);
				} else {
					const msg = this.getMessage(header, body);
					if (msg instanceof Error) {
						errors.push(msg);
					} else {
						this._queue.push(msg);
					}
				}
				this._buffer = this._buffer.slice(
					MessageHeader.SIZE + header.len
				);
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
		return this._queue.length === 0
			? undefined
			: this._queue.splice(0, 1)[0];
	}
}

export abstract class Convertor<T> {
	public collect(
		getters: Array<() => ArrayBufferLike | Error>
	): ArrayBufferLike {
		const buffers: ArrayBufferLike[] = [];
		try {
			getters.forEach((getter: () => ArrayBufferLike | Error) => {
				const buf: ArrayBufferLike | Error = getter();
				if (buf instanceof Error) {
					throw buf;
				}
				buffers.push(buf);
			});
		} catch (err) {
			throw err instanceof Error
				? err
				: new Error(`Unknown error: ${err}`);
		}
		return Tools.append(buffers);
	}

	public getBuffer(
		id: number,
		esize: ESize,
		size: number | bigint,
		value: ArrayBufferLike | Error
	): ArrayBufferLike | Error {
		if (value instanceof Error) {
			return value;
		}
		const idBuf: ArrayBufferLike | Error = Primitives.u16.encode(id);
		if (idBuf instanceof Error) {
			return idBuf;
		}
		let sizeType: ArrayBufferLike | Error;
		let sizeValue: ArrayBufferLike | Error;
		if (esize === ESize.u64 && typeof size !== "bigint") {
			return new Error(
				`For size ${ESize.u64}, size should be defined as BigInt`
			);
		} else if (
			(esize === ESize.u8 ||
				esize === ESize.u16 ||
				esize === ESize.u32) &&
			typeof size === "bigint"
		) {
			return new Error(
				`For sizes ${ESize.u8}, ${ESize.u16}, ${ESize.u32}, size should be defined as Number`
			);
		}
		switch (esize) {
			case ESize.u8:
				sizeType = Primitives.u8.encode(
					Primitives.u8.getSize() * CBits
				);
				sizeValue = Primitives.u8.encode(size as number);
				break;
			case ESize.u16:
				sizeType = Primitives.u8.encode(
					Primitives.u16.getSize() * CBits
				);
				sizeValue = Primitives.u16.encode(size as number);
				break;
			case ESize.u32:
				sizeType = Primitives.u8.encode(
					Primitives.u32.getSize() * CBits
				);
				sizeValue = Primitives.u32.encode(size as number);
				break;
			case ESize.u64:
				sizeType = Primitives.u8.encode(
					Primitives.u64.getSize() * CBits
				);
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

	public getBufferFromBuf<T>(
		id: number,
		esize: ESize,
		encoder: (...args: any[]) => ArrayBufferLike | Error,
		value: T
	): ArrayBufferLike | Error {
		const buffer = encoder(value);
		if (buffer instanceof Error) {
			return buffer;
		}
		return this.getBuffer(
			id,
			esize,
			esize === ESize.u64 ? BigInt(buffer.byteLength) : buffer.byteLength,
			buffer
		);
	}

	public getStorage(buffer: ArrayBufferLike): Storage | Error {
		const storage: Storage = new Storage();
		const error: Error | undefined = storage.read(buffer);
		if (error instanceof Error) {
			return error;
		}
		return storage;
	}

	public getValue<T>(
		storage: Storage,
		id: number,
		decoder: (buf: ArrayBufferLike) => T | Error
	): T | Error {
		const buffer = storage.get(id);
		if (buffer === undefined) {
			return new Error(`Fail to find field with ID "${id}"`);
		}
		return decoder(buffer);
	}

	public encodeSelfArray(
		items: Array<Required<Convertor<T>>>
	): ArrayBufferLike | Error {
		let error: Error | undefined;
		const buffers: ArrayBufferLike[] = [];
		items.forEach((item: Required<Convertor<T>>) => {
			if (error !== undefined) {
				return;
			}
			const buffer = item.encode();
			if (buffer instanceof Error) {
				error = buffer;
				return;
			}
			const len = u64.encode(BigInt(buffer.byteLength));
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

	public decodeSelfArray(bytes: ArrayBufferLike): T[] | Error {
		const buffer = Buffer.from(bytes);
		const selfs: T[] = [];
		if (buffer.byteLength === 0) {
			return selfs;
		} else if (buffer.byteLength < u64.getSize()) {
			return new Error(
				`Invalid size marker. Expecting u64 (size ${u64.getSize()} bytes), but size of buffer: ${
					buffer.byteLength
				} bytes.`
			);
		}
		let offset: number = 0;
		do {
			const len = buffer.readBigUInt64LE(offset);
			if (isNaN(Number(len)) || !isFinite(Number(len))) {
				return new Error(
					`Invalid length of ${this.getSignature()}/${this.getId()} in an array`
				);
			}
			offset += u64.getSize();
			const body = buffer.slice(offset, offset + Number(len));
			const self = this.defaults();
			if (typeof (self as any).decode !== "function") {
				throw new Error(
					`Object ${self} isn't instance of Convertor<T>`
				);
			}
			const err = (self as any).decode(body);
			if (err instanceof Error) {
				return err;
			}
			selfs.push(self);
			offset += body.byteLength;
		} while (offset < buffer.byteLength);
		return selfs;
	}

	public pack(sequence: number, uuid?: string): ArrayBufferLike {
		const id: ArrayBufferLike | Error = Primitives.u32.encode(this.getId());
		const signature: ArrayBufferLike | Error = Primitives.u16.encode(
			this.signature()
		);
		const seq: ArrayBufferLike | Error = Primitives.u32.encode(sequence);
		const ts = BigInt(new Date().getTime());
		const timestamp: ArrayBufferLike | Error = Primitives.u64.encode(ts);
		if (id instanceof Error) {
			throw new Error(
				`Fail to encode id (${this.getId()}) due error: ${id.message}`
			);
		}
		if (signature instanceof Error) {
			throw new Error(
				`Fail to encode signature (${this.signature()}) due error: ${
					signature.message
				}`
			);
		}
		if (seq instanceof Error) {
			throw new Error(
				`Fail to encode seq (${this.getId()}) due error: ${seq.message}`
			);
		}
		if (timestamp instanceof Error) {
			throw new Error(
				`Fail to encode timestamp (${ts}) due error: ${timestamp.message}`
			);
		}
		const buffer: ArrayBufferLike | Error = (() => {
			const middleware: PackingMiddleware | undefined =
				getPackingMiddleware();
			if (middleware instanceof PackingMiddleware) {
				return middleware.encode(
					this.encode(),
					this.getId(),
					sequence,
					uuid
				);
			} else {
				return this.encode();
			}
		})();
		if (buffer instanceof Error) {
			throw buffer;
		}
		const len: ArrayBufferLike | Error = Primitives.u64.encode(
			BigInt(buffer.byteLength)
		);
		if (len instanceof Error) {
			throw new Error(
				`Fail to encode len (${ts}) due error: ${len.message}`
			);
		}
		return Tools.append([id, signature, seq, timestamp, len, buffer]);
	}

	public abstract getSignature(): string;
	public abstract signature(): number;
	public abstract getId(): number;
	public abstract encode(): ArrayBufferLike;
	public abstract decode(buffer: ArrayBufferLike): Error | T;
	public abstract defaults(): T;
}

type ESizeAlias = ESize;
const ESizeAlias = ESize;
type ConvertorAlias<T> = Convertor<T>;
const ConvertorAlias = Convertor;
type IPropSchemeAlias = IPropScheme;
const PrimitivesAlias = Primitives;
const validateAlias = validate;

export namespace Protocol {
	export const ESize = ESizeAlias;
	export type ESize = ESizeAlias;
	export const Convertor = ConvertorAlias;
	export type Convertor<T> = ConvertorAlias<T>;
	export type IPropScheme = IPropSchemeAlias;
	export const Primitives = PrimitivesAlias;
	export const validate = validateAlias;
}


export interface IAvailableMessages {
    EnumA?: IEnumA,
    EnumB?: IEnumB,
    EnumC?: IEnumC,
    StructA?: StructA,
    StructB?: StructB,
    StructC?: StructC,
    StructD?: StructD,
    StructE?: StructE,
    StructF?: StructF,
    StructG?: StructG,
    TriggerBeaconsEmitter?: TriggerBeaconsEmitter,
    StructEmpty?: StructEmpty,
    StructEmptyA?: StructEmptyA,
    StructEmptyB?: StructEmptyB,
    StructJ?: StructJ,
    TriggerBeacons?: TriggerBeacons,
    FinishConsumerTest?: FinishConsumerTest,
    FinishConsumerTestBroadcast?: FinishConsumerTestBroadcast,
    BeaconA?: BeaconA,
    EventA?: EventA,
    EventB?: EventB,
    Beacons?: Beacons.IAvailableMessages,
    GroupA?: GroupA.IAvailableMessages,
    GroupB?: GroupB.IAvailableMessages,
    GroupD?: GroupD.IAvailableMessages,
    Events?: Events.IAvailableMessages,
    InternalServiceGroup?: InternalServiceGroup.IAvailableMessages,
}
export interface IEnumA {
    Option_a?: string;
    Option_b?: string;
}

export class EnumA extends Protocol.Primitives.Enum<IEnumA> {
    public static from(obj: any): IEnumA | Error {
        const inst = new EnumA();
        let err: Error | undefined;
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            err = inst.decode(obj);
        } else {
            err = inst.set(obj);
        }
        return err instanceof Error ? err : inst.get();
    }
    public static getId(): number { return 1; }
    public from(obj: any): IEnumA | Error {
        return EnumA.from(obj);
    }
    public signature(): number { return 0; }
    public getId(): number { return 1; }
    public getAllowed(): string[] {
        return [
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.StrUTF8.getSignature(),
        ];
    }
    public getOptionValue(id: number): ISigned<any> {
        switch (id) {
            case 0: return new Protocol.Primitives.StrUTF8('');
            case 1: return new Protocol.Primitives.StrUTF8('');
            default: throw new Error(`No option with id=${id}`);
        }
    }
    public get(): IEnumA {
        const target: IEnumA = {};
        switch (this.getValueIndex()) {
            case 0: target.Option_a = this.getValue<string>(); break;
            case 1: target.Option_b = this.getValue<string>(); break;
        }
        return target;
    }
    public set(src: IEnumA): Error | undefined{
        if (Object.keys(src).length > 1) {
            return new Error(`Option cannot have more then 1 value.`);
        }
        if (src.Option_a !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(src.Option_a)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_b !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(src.Option_b)));
            if (err instanceof Error) {
                return err;
            }
        }
    }
}

export interface IEnumB {
    Option_str?: string;
    Option_u8?: number;
    Option_u16?: number;
    Option_u32?: number;
    Option_u64?: bigint;
    Option_i8?: number;
    Option_i16?: number;
    Option_i32?: number;
    Option_i64?: bigint;
    Option_f32?: number;
    Option_f64?: number;
}

export class EnumB extends Protocol.Primitives.Enum<IEnumB> {
    public static from(obj: any): IEnumB | Error {
        const inst = new EnumB();
        let err: Error | undefined;
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            err = inst.decode(obj);
        } else {
            err = inst.set(obj);
        }
        return err instanceof Error ? err : inst.get();
    }
    public static getId(): number { return 2; }
    public from(obj: any): IEnumB | Error {
        return EnumB.from(obj);
    }
    public signature(): number { return 0; }
    public getId(): number { return 2; }
    public getAllowed(): string[] {
        return [
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.u8.getSignature(),
            Protocol.Primitives.u16.getSignature(),
            Protocol.Primitives.u32.getSignature(),
            Protocol.Primitives.u64.getSignature(),
            Protocol.Primitives.i8.getSignature(),
            Protocol.Primitives.i16.getSignature(),
            Protocol.Primitives.i32.getSignature(),
            Protocol.Primitives.i64.getSignature(),
            Protocol.Primitives.f32.getSignature(),
            Protocol.Primitives.f64.getSignature(),
        ];
    }
    public getOptionValue(id: number): ISigned<any> {
        switch (id) {
            case 0: return new Protocol.Primitives.StrUTF8('');
            case 1: return new Protocol.Primitives.u8(0);
            case 2: return new Protocol.Primitives.u16(0);
            case 3: return new Protocol.Primitives.u32(0);
            case 4: return new Protocol.Primitives.u64(BigInt(0));
            case 5: return new Protocol.Primitives.i8(0);
            case 6: return new Protocol.Primitives.i16(0);
            case 7: return new Protocol.Primitives.i32(0);
            case 8: return new Protocol.Primitives.i64(BigInt(0));
            case 9: return new Protocol.Primitives.f32(0);
            case 10: return new Protocol.Primitives.f64(0);
            default: throw new Error(`No option with id=${id}`);
        }
    }
    public get(): IEnumB {
        const target: IEnumB = {};
        switch (this.getValueIndex()) {
            case 0: target.Option_str = this.getValue<string>(); break;
            case 1: target.Option_u8 = this.getValue<number>(); break;
            case 2: target.Option_u16 = this.getValue<number>(); break;
            case 3: target.Option_u32 = this.getValue<number>(); break;
            case 4: target.Option_u64 = this.getValue<bigint>(); break;
            case 5: target.Option_i8 = this.getValue<number>(); break;
            case 6: target.Option_i16 = this.getValue<number>(); break;
            case 7: target.Option_i32 = this.getValue<number>(); break;
            case 8: target.Option_i64 = this.getValue<bigint>(); break;
            case 9: target.Option_f32 = this.getValue<number>(); break;
            case 10: target.Option_f64 = this.getValue<number>(); break;
        }
        return target;
    }
    public set(src: IEnumB): Error | undefined{
        if (Object.keys(src).length > 1) {
            return new Error(`Option cannot have more then 1 value.`);
        }
        if (src.Option_str !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(src.Option_str)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u8 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(1, new Protocol.Primitives.u8(src.Option_u8)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u16 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(src.Option_u16)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(3, new Protocol.Primitives.u32(src.Option_u32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<bigint>(4, new Protocol.Primitives.u64(src.Option_u64)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i8 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(5, new Protocol.Primitives.i8(src.Option_i8)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i16 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(6, new Protocol.Primitives.i16(src.Option_i16)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(7, new Protocol.Primitives.i32(src.Option_i32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<bigint>(8, new Protocol.Primitives.i64(src.Option_i64)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_f32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(9, new Protocol.Primitives.f32(src.Option_f32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_f64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<number>(10, new Protocol.Primitives.f64(src.Option_f64)));
            if (err instanceof Error) {
                return err;
            }
        }
    }
}

export interface IEnumC {
    Option_str?: Array<string>;
    Option_u8?: Array<number>;
    Option_u16?: Array<number>;
    Option_u32?: Array<number>;
    Option_u64?: Array<bigint>;
    Option_i8?: Array<number>;
    Option_i16?: Array<number>;
    Option_i32?: Array<number>;
    Option_i64?: Array<bigint>;
    Option_f32?: Array<number>;
    Option_f64?: Array<number>;
}

export class EnumC extends Protocol.Primitives.Enum<IEnumC> {
    public static from(obj: any): IEnumC | Error {
        const inst = new EnumC();
        let err: Error | undefined;
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            err = inst.decode(obj);
        } else {
            err = inst.set(obj);
        }
        return err instanceof Error ? err : inst.get();
    }
    public static getId(): number { return 3; }
    public from(obj: any): IEnumC | Error {
        return EnumC.from(obj);
    }
    public signature(): number { return 0; }
    public getId(): number { return 3; }
    public getAllowed(): string[] {
        return [
            Protocol.Primitives.ArrayStrUTF8.getSignature(),
            Protocol.Primitives.ArrayU8.getSignature(),
            Protocol.Primitives.ArrayU16.getSignature(),
            Protocol.Primitives.ArrayU32.getSignature(),
            Protocol.Primitives.ArrayU64.getSignature(),
            Protocol.Primitives.ArrayI8.getSignature(),
            Protocol.Primitives.ArrayI16.getSignature(),
            Protocol.Primitives.ArrayI32.getSignature(),
            Protocol.Primitives.ArrayI64.getSignature(),
            Protocol.Primitives.ArrayF32.getSignature(),
            Protocol.Primitives.ArrayF64.getSignature(),
        ];
    }
    public getOptionValue(id: number): ISigned<any> {
        switch (id) {
            case 0: return new Protocol.Primitives.ArrayStrUTF8(['']);
            case 1: return new Protocol.Primitives.ArrayU8([0]);
            case 2: return new Protocol.Primitives.ArrayU16([0]);
            case 3: return new Protocol.Primitives.ArrayU32([0]);
            case 4: return new Protocol.Primitives.ArrayU64([BigInt(0)]);
            case 5: return new Protocol.Primitives.ArrayI8([0]);
            case 6: return new Protocol.Primitives.ArrayI16([0]);
            case 7: return new Protocol.Primitives.ArrayI32([0]);
            case 8: return new Protocol.Primitives.ArrayI64([BigInt(0)]);
            case 9: return new Protocol.Primitives.ArrayF32([0]);
            case 10: return new Protocol.Primitives.ArrayF64([0]);
            default: throw new Error(`No option with id=${id}`);
        }
    }
    public get(): IEnumC {
        const target: IEnumC = {};
        switch (this.getValueIndex()) {
            case 0: target.Option_str = this.getValue<Array<string>>(); break;
            case 1: target.Option_u8 = this.getValue<Array<number>>(); break;
            case 2: target.Option_u16 = this.getValue<Array<number>>(); break;
            case 3: target.Option_u32 = this.getValue<Array<number>>(); break;
            case 4: target.Option_u64 = this.getValue<Array<bigint>>(); break;
            case 5: target.Option_i8 = this.getValue<Array<number>>(); break;
            case 6: target.Option_i16 = this.getValue<Array<number>>(); break;
            case 7: target.Option_i32 = this.getValue<Array<number>>(); break;
            case 8: target.Option_i64 = this.getValue<Array<bigint>>(); break;
            case 9: target.Option_f32 = this.getValue<Array<number>>(); break;
            case 10: target.Option_f64 = this.getValue<Array<number>>(); break;
        }
        return target;
    }
    public set(src: IEnumC): Error | undefined{
        if (Object.keys(src).length > 1) {
            return new Error(`Option cannot have more then 1 value.`);
        }
        if (src.Option_str !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<string>>(0, new Protocol.Primitives.ArrayStrUTF8(src.Option_str)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u8 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(1, new Protocol.Primitives.ArrayU8(src.Option_u8)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u16 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(2, new Protocol.Primitives.ArrayU16(src.Option_u16)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(3, new Protocol.Primitives.ArrayU32(src.Option_u32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_u64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<bigint>>(4, new Protocol.Primitives.ArrayU64(src.Option_u64)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i8 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(5, new Protocol.Primitives.ArrayI8(src.Option_i8)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i16 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(6, new Protocol.Primitives.ArrayI16(src.Option_i16)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(7, new Protocol.Primitives.ArrayI32(src.Option_i32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_i64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<bigint>>(8, new Protocol.Primitives.ArrayI64(src.Option_i64)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_f32 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(9, new Protocol.Primitives.ArrayF32(src.Option_f32)));
            if (err instanceof Error) {
                return err;
            }
        }
        if (src.Option_f64 !== undefined) {
            const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<Array<number>>(10, new Protocol.Primitives.ArrayF64(src.Option_f64)));
            if (err instanceof Error) {
                return err;
            }
        }
    }
}

export interface IStructA {
    field_str: string;
    field_str_empty: string;
    field_u8: number;
    field_u16: number;
    field_u32: number;
    field_u64: bigint;
    field_i8: number;
    field_i16: number;
    field_i32: number;
    field_i64: bigint;
    field_f32: number;
    field_f64: number;
    field_bool: boolean;
}
export class StructA extends Protocol.Convertor<StructA> implements IStructA, ISigned<StructA> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_str', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'field_str_empty', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
        { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
        { prop: 'field_u32', types: Protocol.Primitives.u32, optional: false, },
        { prop: 'field_u64', types: Protocol.Primitives.u64, optional: false, },
        { prop: 'field_i8', types: Protocol.Primitives.i8, optional: false, },
        { prop: 'field_i16', types: Protocol.Primitives.i16, optional: false, },
        { prop: 'field_i32', types: Protocol.Primitives.i32, optional: false, },
        { prop: 'field_i64', types: Protocol.Primitives.i64, optional: false, },
        { prop: 'field_f32', types: Protocol.Primitives.f32, optional: false, },
        { prop: 'field_f64', types: Protocol.Primitives.f64, optional: false, },
        { prop: 'field_bool', types: Protocol.Primitives.bool, optional: false, },
    ];

    public static defaults(): StructA {
        return new StructA({
            field_str: '',
            field_str_empty: '',
            field_u8: 0,
            field_u16: 0,
            field_u32: 0,
            field_u64: BigInt(0),
            field_i8: 0,
            field_i16: 0,
            field_i32: 0,
            field_i64: BigInt(0),
            field_f32: 0,
            field_f64: 0,
            field_bool: true,
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructA>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructA)) {
                            throw new Error(`Expecting instance of StructA on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructA ? undefined : new Error(`Expecting instance of StructA`);
            }};
        }
    }

    public static from(obj: any): StructA | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructA.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructA.scheme);
            return error instanceof Error ? error : new StructA({
                field_str: obj.field_str,
                field_str_empty: obj.field_str_empty,
                field_u8: obj.field_u8,
                field_u16: obj.field_u16,
                field_u32: obj.field_u32,
                field_u64: obj.field_u64,
                field_i8: obj.field_i8,
                field_i16: obj.field_i16,
                field_i32: obj.field_i32,
                field_i64: obj.field_i64,
                field_f32: obj.field_f32,
                field_f64: obj.field_f64,
                field_bool: obj.field_bool,
            });
        }
    }

    public field_str!: string;
    public field_str_empty!: string;
    public field_u8!: number;
    public field_u16!: number;
    public field_u32!: number;
    public field_u64!: bigint;
    public field_i8!: number;
    public field_i16!: number;
    public field_i32!: number;
    public field_i64!: bigint;
    public field_f32!: number;
    public field_f64!: number;
    public field_bool!: boolean;
    public static getSignature(): string { return 'StructA'; }
    public static getId(): number { return 4; }


    constructor(params: IStructA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructA'; }

    public get(): StructA { return this; }

    public getId(): number { return 4; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(5, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.field_str),
            () => this.getBufferFromBuf<string>(6, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.field_str_empty),
            () => this.getBuffer(7, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
            () => this.getBuffer(8, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
            () => this.getBuffer(9, Protocol.ESize.u8, Protocol.Primitives.u32.getSize(), Protocol.Primitives.u32.encode(this.field_u32)),
            () => this.getBuffer(10, Protocol.ESize.u8, Protocol.Primitives.u64.getSize(), Protocol.Primitives.u64.encode(this.field_u64)),
            () => this.getBuffer(11, Protocol.ESize.u8, Protocol.Primitives.i8.getSize(), Protocol.Primitives.i8.encode(this.field_i8)),
            () => this.getBuffer(12, Protocol.ESize.u8, Protocol.Primitives.i16.getSize(), Protocol.Primitives.i16.encode(this.field_i16)),
            () => this.getBuffer(13, Protocol.ESize.u8, Protocol.Primitives.i32.getSize(), Protocol.Primitives.i32.encode(this.field_i32)),
            () => this.getBuffer(14, Protocol.ESize.u8, Protocol.Primitives.i64.getSize(), Protocol.Primitives.i64.encode(this.field_i64)),
            () => this.getBuffer(15, Protocol.ESize.u8, Protocol.Primitives.f32.getSize(), Protocol.Primitives.f32.encode(this.field_f32)),
            () => this.getBuffer(16, Protocol.ESize.u8, Protocol.Primitives.f64.getSize(), Protocol.Primitives.f64.encode(this.field_f64)),
            () => this.getBuffer(17, Protocol.ESize.u8, Protocol.Primitives.bool.getSize(), Protocol.Primitives.bool.encode(this.field_bool)),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructA {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_str: string | Error = this.getValue<string>(storage, 5, Protocol.Primitives.StrUTF8.decode);
        if (field_str instanceof Error) {
            return field_str;
        } else {
            this.field_str = field_str;
        }
        const field_str_empty: string | Error = this.getValue<string>(storage, 6, Protocol.Primitives.StrUTF8.decode);
        if (field_str_empty instanceof Error) {
            return field_str_empty;
        } else {
            this.field_str_empty = field_str_empty;
        }
        const field_u8: number | Error = this.getValue<number>(storage, 7, Protocol.Primitives.u8.decode);
        if (field_u8 instanceof Error) {
            return field_u8;
        } else {
            this.field_u8 = field_u8;
        }
        const field_u16: number | Error = this.getValue<number>(storage, 8, Protocol.Primitives.u16.decode);
        if (field_u16 instanceof Error) {
            return field_u16;
        } else {
            this.field_u16 = field_u16;
        }
        const field_u32: number | Error = this.getValue<number>(storage, 9, Protocol.Primitives.u32.decode);
        if (field_u32 instanceof Error) {
            return field_u32;
        } else {
            this.field_u32 = field_u32;
        }
        const field_u64: bigint | Error = this.getValue<bigint>(storage, 10, Protocol.Primitives.u64.decode);
        if (field_u64 instanceof Error) {
            return field_u64;
        } else {
            this.field_u64 = field_u64;
        }
        const field_i8: number | Error = this.getValue<number>(storage, 11, Protocol.Primitives.i8.decode);
        if (field_i8 instanceof Error) {
            return field_i8;
        } else {
            this.field_i8 = field_i8;
        }
        const field_i16: number | Error = this.getValue<number>(storage, 12, Protocol.Primitives.i16.decode);
        if (field_i16 instanceof Error) {
            return field_i16;
        } else {
            this.field_i16 = field_i16;
        }
        const field_i32: number | Error = this.getValue<number>(storage, 13, Protocol.Primitives.i32.decode);
        if (field_i32 instanceof Error) {
            return field_i32;
        } else {
            this.field_i32 = field_i32;
        }
        const field_i64: bigint | Error = this.getValue<bigint>(storage, 14, Protocol.Primitives.i64.decode);
        if (field_i64 instanceof Error) {
            return field_i64;
        } else {
            this.field_i64 = field_i64;
        }
        const field_f32: number | Error = this.getValue<number>(storage, 15, Protocol.Primitives.f32.decode);
        if (field_f32 instanceof Error) {
            return field_f32;
        } else {
            this.field_f32 = field_f32;
        }
        const field_f64: number | Error = this.getValue<number>(storage, 16, Protocol.Primitives.f64.decode);
        if (field_f64 instanceof Error) {
            return field_f64;
        } else {
            this.field_f64 = field_f64;
        }
        const field_bool: boolean | Error = this.getValue<boolean>(storage, 17, Protocol.Primitives.bool.decode);
        if (field_bool instanceof Error) {
            return field_bool;
        } else {
            this.field_bool = field_bool;
        }
        return this;
    }

    public defaults(): StructA {
        return StructA.defaults();
    }
}

export interface IStructB {
    field_str: Array<string>;
    field_u8: Array<number>;
    field_u16: Array<number>;
    field_u32: Array<number>;
    field_u64: Array<bigint>;
    field_i8: Array<number>;
    field_i16: Array<number>;
    field_i32: Array<number>;
    field_i64: Array<bigint>;
    field_f32: Array<number>;
    field_f64: Array<number>;
    field_bool: Array<boolean>;
    field_struct: Array<StructA>;
    field_str_empty: Array<string>;
    field_u8_empty: Array<number>;
    field_u16_empty: Array<number>;
    field_u32_empty: Array<number>;
    field_u64_empty: Array<bigint>;
    field_i8_empty: Array<number>;
    field_i16_empty: Array<number>;
    field_i32_empty: Array<number>;
    field_i64_empty: Array<bigint>;
    field_f32_empty: Array<number>;
    field_f64_empty: Array<number>;
    field_bool_empty: Array<boolean>;
    field_struct_empty: Array<StructA>;
}
export class StructB extends Protocol.Convertor<StructB> implements IStructB, ISigned<StructB> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_str', types: Protocol.Primitives.ArrayStrUTF8, optional: false, },
        { prop: 'field_u8', types: Protocol.Primitives.ArrayU8, optional: false, },
        { prop: 'field_u16', types: Protocol.Primitives.ArrayU16, optional: false, },
        { prop: 'field_u32', types: Protocol.Primitives.ArrayU32, optional: false, },
        { prop: 'field_u64', types: Protocol.Primitives.ArrayU64, optional: false, },
        { prop: 'field_i8', types: Protocol.Primitives.ArrayI8, optional: false, },
        { prop: 'field_i16', types: Protocol.Primitives.ArrayI16, optional: false, },
        { prop: 'field_i32', types: Protocol.Primitives.ArrayI32, optional: false, },
        { prop: 'field_i64', types: Protocol.Primitives.ArrayI64, optional: false, },
        { prop: 'field_f32', types: Protocol.Primitives.ArrayF32, optional: false, },
        { prop: 'field_f64', types: Protocol.Primitives.ArrayF64, optional: false, },
        { prop: 'field_bool', types: Protocol.Primitives.ArrayBool, optional: false, },
        { prop: 'field_struct', types: StructA.getValidator(true), optional: false },
        { prop: 'field_str_empty', types: Protocol.Primitives.ArrayStrUTF8, optional: false, },
        { prop: 'field_u8_empty', types: Protocol.Primitives.ArrayU8, optional: false, },
        { prop: 'field_u16_empty', types: Protocol.Primitives.ArrayU16, optional: false, },
        { prop: 'field_u32_empty', types: Protocol.Primitives.ArrayU32, optional: false, },
        { prop: 'field_u64_empty', types: Protocol.Primitives.ArrayU64, optional: false, },
        { prop: 'field_i8_empty', types: Protocol.Primitives.ArrayI8, optional: false, },
        { prop: 'field_i16_empty', types: Protocol.Primitives.ArrayI16, optional: false, },
        { prop: 'field_i32_empty', types: Protocol.Primitives.ArrayI32, optional: false, },
        { prop: 'field_i64_empty', types: Protocol.Primitives.ArrayI64, optional: false, },
        { prop: 'field_f32_empty', types: Protocol.Primitives.ArrayF32, optional: false, },
        { prop: 'field_f64_empty', types: Protocol.Primitives.ArrayF64, optional: false, },
        { prop: 'field_bool_empty', types: Protocol.Primitives.ArrayBool, optional: false, },
        { prop: 'field_struct_empty', types: StructA.getValidator(true), optional: false },
    ];

    public static defaults(): StructB {
        return new StructB({
            field_str: [],
            field_u8: [],
            field_u16: [],
            field_u32: [],
            field_u64: [],
            field_i8: [],
            field_i16: [],
            field_i32: [],
            field_i64: [],
            field_f32: [],
            field_f64: [],
            field_bool: [],
            field_struct: [],
            field_str_empty: [],
            field_u8_empty: [],
            field_u16_empty: [],
            field_u32_empty: [],
            field_u64_empty: [],
            field_i8_empty: [],
            field_i16_empty: [],
            field_i32_empty: [],
            field_i64_empty: [],
            field_f32_empty: [],
            field_f64_empty: [],
            field_bool_empty: [],
            field_struct_empty: [],
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructB>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructB)) {
                            throw new Error(`Expecting instance of StructB on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructB ? undefined : new Error(`Expecting instance of StructB`);
            }};
        }
    }

    public static from(obj: any): StructB | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructB.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructB.scheme);
            return error instanceof Error ? error : new StructB({
                field_str: obj.field_str,
                field_u8: obj.field_u8,
                field_u16: obj.field_u16,
                field_u32: obj.field_u32,
                field_u64: obj.field_u64,
                field_i8: obj.field_i8,
                field_i16: obj.field_i16,
                field_i32: obj.field_i32,
                field_i64: obj.field_i64,
                field_f32: obj.field_f32,
                field_f64: obj.field_f64,
                field_bool: obj.field_bool,
                field_struct: obj.field_struct,
                field_str_empty: obj.field_str_empty,
                field_u8_empty: obj.field_u8_empty,
                field_u16_empty: obj.field_u16_empty,
                field_u32_empty: obj.field_u32_empty,
                field_u64_empty: obj.field_u64_empty,
                field_i8_empty: obj.field_i8_empty,
                field_i16_empty: obj.field_i16_empty,
                field_i32_empty: obj.field_i32_empty,
                field_i64_empty: obj.field_i64_empty,
                field_f32_empty: obj.field_f32_empty,
                field_f64_empty: obj.field_f64_empty,
                field_bool_empty: obj.field_bool_empty,
                field_struct_empty: obj.field_struct_empty,
            });
        }
    }

    public field_str!: Array<string>;
    public field_u8!: Array<number>;
    public field_u16!: Array<number>;
    public field_u32!: Array<number>;
    public field_u64!: Array<bigint>;
    public field_i8!: Array<number>;
    public field_i16!: Array<number>;
    public field_i32!: Array<number>;
    public field_i64!: Array<bigint>;
    public field_f32!: Array<number>;
    public field_f64!: Array<number>;
    public field_bool!: Array<boolean>;
    public field_struct!: Array<StructA>;
    public field_str_empty!: Array<string>;
    public field_u8_empty!: Array<number>;
    public field_u16_empty!: Array<number>;
    public field_u32_empty!: Array<number>;
    public field_u64_empty!: Array<bigint>;
    public field_i8_empty!: Array<number>;
    public field_i16_empty!: Array<number>;
    public field_i32_empty!: Array<number>;
    public field_i64_empty!: Array<bigint>;
    public field_f32_empty!: Array<number>;
    public field_f64_empty!: Array<number>;
    public field_bool_empty!: Array<boolean>;
    public field_struct_empty!: Array<StructA>;
    public static getSignature(): string { return 'StructB'; }
    public static getId(): number { return 18; }


    constructor(params: IStructB)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructB'; }

    public get(): StructB { return this; }

    public getId(): number { return 18; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<Array<string>>(19, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.field_str),
            () => this.getBufferFromBuf<Array<number>>(20, Protocol.ESize.u64, Protocol.Primitives.ArrayU8.encode, this.field_u8),
            () => this.getBufferFromBuf<Array<number>>(21, Protocol.ESize.u64, Protocol.Primitives.ArrayU16.encode, this.field_u16),
            () => this.getBufferFromBuf<Array<number>>(22, Protocol.ESize.u64, Protocol.Primitives.ArrayU32.encode, this.field_u32),
            () => this.getBufferFromBuf<Array<bigint>>(23, Protocol.ESize.u64, Protocol.Primitives.ArrayU64.encode, this.field_u64),
            () => this.getBufferFromBuf<Array<number>>(24, Protocol.ESize.u64, Protocol.Primitives.ArrayI8.encode, this.field_i8),
            () => this.getBufferFromBuf<Array<number>>(25, Protocol.ESize.u64, Protocol.Primitives.ArrayI16.encode, this.field_i16),
            () => this.getBufferFromBuf<Array<number>>(26, Protocol.ESize.u64, Protocol.Primitives.ArrayI32.encode, this.field_i32),
            () => this.getBufferFromBuf<Array<bigint>>(27, Protocol.ESize.u64, Protocol.Primitives.ArrayI64.encode, this.field_i64),
            () => this.getBufferFromBuf<Array<number>>(28, Protocol.ESize.u64, Protocol.Primitives.ArrayF32.encode, this.field_f32),
            () => this.getBufferFromBuf<Array<number>>(29, Protocol.ESize.u64, Protocol.Primitives.ArrayF64.encode, this.field_f64),
            () => this.getBufferFromBuf<Array<boolean>>(30, Protocol.ESize.u64, Protocol.Primitives.ArrayBool.encode, this.field_bool),
            () => { const self: StructA = StructA.defaults(); return this.getBufferFromBuf<StructA[]>(31, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.field_struct); },
            () => this.getBufferFromBuf<Array<string>>(32, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.field_str_empty),
            () => this.getBufferFromBuf<Array<number>>(33, Protocol.ESize.u64, Protocol.Primitives.ArrayU8.encode, this.field_u8_empty),
            () => this.getBufferFromBuf<Array<number>>(34, Protocol.ESize.u64, Protocol.Primitives.ArrayU16.encode, this.field_u16_empty),
            () => this.getBufferFromBuf<Array<number>>(35, Protocol.ESize.u64, Protocol.Primitives.ArrayU32.encode, this.field_u32_empty),
            () => this.getBufferFromBuf<Array<bigint>>(36, Protocol.ESize.u64, Protocol.Primitives.ArrayU64.encode, this.field_u64_empty),
            () => this.getBufferFromBuf<Array<number>>(37, Protocol.ESize.u64, Protocol.Primitives.ArrayI8.encode, this.field_i8_empty),
            () => this.getBufferFromBuf<Array<number>>(38, Protocol.ESize.u64, Protocol.Primitives.ArrayI16.encode, this.field_i16_empty),
            () => this.getBufferFromBuf<Array<number>>(39, Protocol.ESize.u64, Protocol.Primitives.ArrayI32.encode, this.field_i32_empty),
            () => this.getBufferFromBuf<Array<bigint>>(40, Protocol.ESize.u64, Protocol.Primitives.ArrayI64.encode, this.field_i64_empty),
            () => this.getBufferFromBuf<Array<number>>(41, Protocol.ESize.u64, Protocol.Primitives.ArrayF32.encode, this.field_f32_empty),
            () => this.getBufferFromBuf<Array<number>>(42, Protocol.ESize.u64, Protocol.Primitives.ArrayF64.encode, this.field_f64_empty),
            () => this.getBufferFromBuf<Array<boolean>>(43, Protocol.ESize.u64, Protocol.Primitives.ArrayBool.encode, this.field_bool_empty),
            () => { const self: StructA = StructA.defaults(); return this.getBufferFromBuf<StructA[]>(44, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.field_struct_empty); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructB {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_str: Array<string> | Error = this.getValue<Array<string>>(storage, 19, Protocol.Primitives.ArrayStrUTF8.decode);
        if (field_str instanceof Error) {
            return field_str;
        } else {
            this.field_str = field_str;
        }
        const field_u8: Array<number> | Error = this.getValue<Array<number>>(storage, 20, Protocol.Primitives.ArrayU8.decode);
        if (field_u8 instanceof Error) {
            return field_u8;
        } else {
            this.field_u8 = field_u8;
        }
        const field_u16: Array<number> | Error = this.getValue<Array<number>>(storage, 21, Protocol.Primitives.ArrayU16.decode);
        if (field_u16 instanceof Error) {
            return field_u16;
        } else {
            this.field_u16 = field_u16;
        }
        const field_u32: Array<number> | Error = this.getValue<Array<number>>(storage, 22, Protocol.Primitives.ArrayU32.decode);
        if (field_u32 instanceof Error) {
            return field_u32;
        } else {
            this.field_u32 = field_u32;
        }
        const field_u64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 23, Protocol.Primitives.ArrayU64.decode);
        if (field_u64 instanceof Error) {
            return field_u64;
        } else {
            this.field_u64 = field_u64;
        }
        const field_i8: Array<number> | Error = this.getValue<Array<number>>(storage, 24, Protocol.Primitives.ArrayI8.decode);
        if (field_i8 instanceof Error) {
            return field_i8;
        } else {
            this.field_i8 = field_i8;
        }
        const field_i16: Array<number> | Error = this.getValue<Array<number>>(storage, 25, Protocol.Primitives.ArrayI16.decode);
        if (field_i16 instanceof Error) {
            return field_i16;
        } else {
            this.field_i16 = field_i16;
        }
        const field_i32: Array<number> | Error = this.getValue<Array<number>>(storage, 26, Protocol.Primitives.ArrayI32.decode);
        if (field_i32 instanceof Error) {
            return field_i32;
        } else {
            this.field_i32 = field_i32;
        }
        const field_i64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 27, Protocol.Primitives.ArrayI64.decode);
        if (field_i64 instanceof Error) {
            return field_i64;
        } else {
            this.field_i64 = field_i64;
        }
        const field_f32: Array<number> | Error = this.getValue<Array<number>>(storage, 28, Protocol.Primitives.ArrayF32.decode);
        if (field_f32 instanceof Error) {
            return field_f32;
        } else {
            this.field_f32 = field_f32;
        }
        const field_f64: Array<number> | Error = this.getValue<Array<number>>(storage, 29, Protocol.Primitives.ArrayF64.decode);
        if (field_f64 instanceof Error) {
            return field_f64;
        } else {
            this.field_f64 = field_f64;
        }
        const field_bool: Array<boolean> | Error = this.getValue<Array<boolean>>(storage, 30, Protocol.Primitives.ArrayBool.decode);
        if (field_bool instanceof Error) {
            return field_bool;
        } else {
            this.field_bool = field_bool;
        }
        const arrfield_structInst: StructA = StructA.defaults();
        const arrfield_struct: Array<any> | Error = this.getValue<StructA[]>(storage, 31, arrfield_structInst.decodeSelfArray.bind(arrfield_structInst));
        if (arrfield_struct instanceof Error) {
            return arrfield_struct;
        } else {
            this.field_struct = arrfield_struct as StructA[];
        }
        const field_str_empty: Array<string> | Error = this.getValue<Array<string>>(storage, 32, Protocol.Primitives.ArrayStrUTF8.decode);
        if (field_str_empty instanceof Error) {
            return field_str_empty;
        } else {
            this.field_str_empty = field_str_empty;
        }
        const field_u8_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 33, Protocol.Primitives.ArrayU8.decode);
        if (field_u8_empty instanceof Error) {
            return field_u8_empty;
        } else {
            this.field_u8_empty = field_u8_empty;
        }
        const field_u16_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 34, Protocol.Primitives.ArrayU16.decode);
        if (field_u16_empty instanceof Error) {
            return field_u16_empty;
        } else {
            this.field_u16_empty = field_u16_empty;
        }
        const field_u32_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 35, Protocol.Primitives.ArrayU32.decode);
        if (field_u32_empty instanceof Error) {
            return field_u32_empty;
        } else {
            this.field_u32_empty = field_u32_empty;
        }
        const field_u64_empty: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 36, Protocol.Primitives.ArrayU64.decode);
        if (field_u64_empty instanceof Error) {
            return field_u64_empty;
        } else {
            this.field_u64_empty = field_u64_empty;
        }
        const field_i8_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 37, Protocol.Primitives.ArrayI8.decode);
        if (field_i8_empty instanceof Error) {
            return field_i8_empty;
        } else {
            this.field_i8_empty = field_i8_empty;
        }
        const field_i16_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 38, Protocol.Primitives.ArrayI16.decode);
        if (field_i16_empty instanceof Error) {
            return field_i16_empty;
        } else {
            this.field_i16_empty = field_i16_empty;
        }
        const field_i32_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 39, Protocol.Primitives.ArrayI32.decode);
        if (field_i32_empty instanceof Error) {
            return field_i32_empty;
        } else {
            this.field_i32_empty = field_i32_empty;
        }
        const field_i64_empty: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 40, Protocol.Primitives.ArrayI64.decode);
        if (field_i64_empty instanceof Error) {
            return field_i64_empty;
        } else {
            this.field_i64_empty = field_i64_empty;
        }
        const field_f32_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 41, Protocol.Primitives.ArrayF32.decode);
        if (field_f32_empty instanceof Error) {
            return field_f32_empty;
        } else {
            this.field_f32_empty = field_f32_empty;
        }
        const field_f64_empty: Array<number> | Error = this.getValue<Array<number>>(storage, 42, Protocol.Primitives.ArrayF64.decode);
        if (field_f64_empty instanceof Error) {
            return field_f64_empty;
        } else {
            this.field_f64_empty = field_f64_empty;
        }
        const field_bool_empty: Array<boolean> | Error = this.getValue<Array<boolean>>(storage, 43, Protocol.Primitives.ArrayBool.decode);
        if (field_bool_empty instanceof Error) {
            return field_bool_empty;
        } else {
            this.field_bool_empty = field_bool_empty;
        }
        const arrfield_struct_emptyInst: StructA = StructA.defaults();
        const arrfield_struct_empty: Array<any> | Error = this.getValue<StructA[]>(storage, 44, arrfield_struct_emptyInst.decodeSelfArray.bind(arrfield_struct_emptyInst));
        if (arrfield_struct_empty instanceof Error) {
            return arrfield_struct_empty;
        } else {
            this.field_struct_empty = arrfield_struct_empty as StructA[];
        }
        return this;
    }

    public defaults(): StructB {
        return StructB.defaults();
    }
}

export interface IStructC {
    field_str: string | undefined;
    field_u8: number | undefined;
    field_u16: number | undefined;
    field_u32: number | undefined;
    field_u64: bigint | undefined;
    field_i8: number | undefined;
    field_i16: number | undefined;
    field_i32: number | undefined;
    field_i64: bigint | undefined;
    field_f32: number | undefined;
    field_f64: number | undefined;
    field_bool: boolean | undefined;
}
export class StructC extends Protocol.Convertor<StructC> implements IStructC, ISigned<StructC> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_str', types: Protocol.Primitives.StrUTF8, optional: true, },
        { prop: 'field_u8', types: Protocol.Primitives.u8, optional: true, },
        { prop: 'field_u16', types: Protocol.Primitives.u16, optional: true, },
        { prop: 'field_u32', types: Protocol.Primitives.u32, optional: true, },
        { prop: 'field_u64', types: Protocol.Primitives.u64, optional: true, },
        { prop: 'field_i8', types: Protocol.Primitives.i8, optional: true, },
        { prop: 'field_i16', types: Protocol.Primitives.i16, optional: true, },
        { prop: 'field_i32', types: Protocol.Primitives.i32, optional: true, },
        { prop: 'field_i64', types: Protocol.Primitives.i64, optional: true, },
        { prop: 'field_f32', types: Protocol.Primitives.f32, optional: true, },
        { prop: 'field_f64', types: Protocol.Primitives.f64, optional: true, },
        { prop: 'field_bool', types: Protocol.Primitives.bool, optional: true, },
    ];

    public static defaults(): StructC {
        return new StructC({
            field_str: undefined,
            field_u8: undefined,
            field_u16: undefined,
            field_u32: undefined,
            field_u64: undefined,
            field_i8: undefined,
            field_i16: undefined,
            field_i32: undefined,
            field_i64: undefined,
            field_f32: undefined,
            field_f64: undefined,
            field_bool: undefined,
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructC>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructC)) {
                            throw new Error(`Expecting instance of StructC on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructC ? undefined : new Error(`Expecting instance of StructC`);
            }};
        }
    }

    public static from(obj: any): StructC | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructC.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructC.scheme);
            return error instanceof Error ? error : new StructC({
                field_str: obj.field_str,
                field_u8: obj.field_u8,
                field_u16: obj.field_u16,
                field_u32: obj.field_u32,
                field_u64: obj.field_u64,
                field_i8: obj.field_i8,
                field_i16: obj.field_i16,
                field_i32: obj.field_i32,
                field_i64: obj.field_i64,
                field_f32: obj.field_f32,
                field_f64: obj.field_f64,
                field_bool: obj.field_bool,
            });
        }
    }

    public field_str!: string | undefined;
    public field_u8!: number | undefined;
    public field_u16!: number | undefined;
    public field_u32!: number | undefined;
    public field_u64!: bigint | undefined;
    public field_i8!: number | undefined;
    public field_i16!: number | undefined;
    public field_i32!: number | undefined;
    public field_i64!: bigint | undefined;
    public field_f32!: number | undefined;
    public field_f64!: number | undefined;
    public field_bool!: boolean | undefined;
    public static getSignature(): string { return 'StructC'; }
    public static getId(): number { return 45; }


    constructor(params: IStructC)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructC'; }

    public get(): StructC { return this; }

    public getId(): number { return 45; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.field_str === undefined ? this.getBuffer(46, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(46, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.field_str),
            () => this.field_u8 === undefined ? this.getBuffer(47, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(47, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
            () => this.field_u16 === undefined ? this.getBuffer(48, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(48, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
            () => this.field_u32 === undefined ? this.getBuffer(49, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(49, Protocol.ESize.u8, Protocol.Primitives.u32.getSize(), Protocol.Primitives.u32.encode(this.field_u32)),
            () => this.field_u64 === undefined ? this.getBuffer(50, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(50, Protocol.ESize.u8, Protocol.Primitives.u64.getSize(), Protocol.Primitives.u64.encode(this.field_u64)),
            () => this.field_i8 === undefined ? this.getBuffer(51, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(51, Protocol.ESize.u8, Protocol.Primitives.i8.getSize(), Protocol.Primitives.i8.encode(this.field_i8)),
            () => this.field_i16 === undefined ? this.getBuffer(52, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(52, Protocol.ESize.u8, Protocol.Primitives.i16.getSize(), Protocol.Primitives.i16.encode(this.field_i16)),
            () => this.field_i32 === undefined ? this.getBuffer(53, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(53, Protocol.ESize.u8, Protocol.Primitives.i32.getSize(), Protocol.Primitives.i32.encode(this.field_i32)),
            () => this.field_i64 === undefined ? this.getBuffer(54, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(54, Protocol.ESize.u8, Protocol.Primitives.i64.getSize(), Protocol.Primitives.i64.encode(this.field_i64)),
            () => this.field_f32 === undefined ? this.getBuffer(55, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(55, Protocol.ESize.u8, Protocol.Primitives.f32.getSize(), Protocol.Primitives.f32.encode(this.field_f32)),
            () => this.field_f64 === undefined ? this.getBuffer(56, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(56, Protocol.ESize.u8, Protocol.Primitives.f64.getSize(), Protocol.Primitives.f64.encode(this.field_f64)),
            () => this.field_bool === undefined ? this.getBuffer(57, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBuffer(57, Protocol.ESize.u8, Protocol.Primitives.bool.getSize(), Protocol.Primitives.bool.encode(this.field_bool)),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructC {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_strBuf: ArrayBufferLike | undefined = storage.get(46);
        if (field_strBuf === undefined) {
            return new Error(`Fail to get property field_str (id=46)`);
        }
        if (field_strBuf.byteLength === 0) {
            this.field_str = undefined;
        } else {
            const field_str: string | Error = this.getValue<string>(storage, 46, Protocol.Primitives.StrUTF8.decode);
            if (field_str instanceof Error) {
                return field_str;
            } else {
                this.field_str = field_str;
            }
        }
        const field_u8Buf: ArrayBufferLike | undefined = storage.get(47);
        if (field_u8Buf === undefined) {
            return new Error(`Fail to get property field_u8 (id=47)`);
        }
        if (field_u8Buf.byteLength === 0) {
            this.field_u8 = undefined;
        } else {
            const field_u8: number | Error = this.getValue<number>(storage, 47, Protocol.Primitives.u8.decode);
            if (field_u8 instanceof Error) {
                return field_u8;
            } else {
                this.field_u8 = field_u8;
            }
        }
        const field_u16Buf: ArrayBufferLike | undefined = storage.get(48);
        if (field_u16Buf === undefined) {
            return new Error(`Fail to get property field_u16 (id=48)`);
        }
        if (field_u16Buf.byteLength === 0) {
            this.field_u16 = undefined;
        } else {
            const field_u16: number | Error = this.getValue<number>(storage, 48, Protocol.Primitives.u16.decode);
            if (field_u16 instanceof Error) {
                return field_u16;
            } else {
                this.field_u16 = field_u16;
            }
        }
        const field_u32Buf: ArrayBufferLike | undefined = storage.get(49);
        if (field_u32Buf === undefined) {
            return new Error(`Fail to get property field_u32 (id=49)`);
        }
        if (field_u32Buf.byteLength === 0) {
            this.field_u32 = undefined;
        } else {
            const field_u32: number | Error = this.getValue<number>(storage, 49, Protocol.Primitives.u32.decode);
            if (field_u32 instanceof Error) {
                return field_u32;
            } else {
                this.field_u32 = field_u32;
            }
        }
        const field_u64Buf: ArrayBufferLike | undefined = storage.get(50);
        if (field_u64Buf === undefined) {
            return new Error(`Fail to get property field_u64 (id=50)`);
        }
        if (field_u64Buf.byteLength === 0) {
            this.field_u64 = undefined;
        } else {
            const field_u64: bigint | Error = this.getValue<bigint>(storage, 50, Protocol.Primitives.u64.decode);
            if (field_u64 instanceof Error) {
                return field_u64;
            } else {
                this.field_u64 = field_u64;
            }
        }
        const field_i8Buf: ArrayBufferLike | undefined = storage.get(51);
        if (field_i8Buf === undefined) {
            return new Error(`Fail to get property field_i8 (id=51)`);
        }
        if (field_i8Buf.byteLength === 0) {
            this.field_i8 = undefined;
        } else {
            const field_i8: number | Error = this.getValue<number>(storage, 51, Protocol.Primitives.i8.decode);
            if (field_i8 instanceof Error) {
                return field_i8;
            } else {
                this.field_i8 = field_i8;
            }
        }
        const field_i16Buf: ArrayBufferLike | undefined = storage.get(52);
        if (field_i16Buf === undefined) {
            return new Error(`Fail to get property field_i16 (id=52)`);
        }
        if (field_i16Buf.byteLength === 0) {
            this.field_i16 = undefined;
        } else {
            const field_i16: number | Error = this.getValue<number>(storage, 52, Protocol.Primitives.i16.decode);
            if (field_i16 instanceof Error) {
                return field_i16;
            } else {
                this.field_i16 = field_i16;
            }
        }
        const field_i32Buf: ArrayBufferLike | undefined = storage.get(53);
        if (field_i32Buf === undefined) {
            return new Error(`Fail to get property field_i32 (id=53)`);
        }
        if (field_i32Buf.byteLength === 0) {
            this.field_i32 = undefined;
        } else {
            const field_i32: number | Error = this.getValue<number>(storage, 53, Protocol.Primitives.i32.decode);
            if (field_i32 instanceof Error) {
                return field_i32;
            } else {
                this.field_i32 = field_i32;
            }
        }
        const field_i64Buf: ArrayBufferLike | undefined = storage.get(54);
        if (field_i64Buf === undefined) {
            return new Error(`Fail to get property field_i64 (id=54)`);
        }
        if (field_i64Buf.byteLength === 0) {
            this.field_i64 = undefined;
        } else {
            const field_i64: bigint | Error = this.getValue<bigint>(storage, 54, Protocol.Primitives.i64.decode);
            if (field_i64 instanceof Error) {
                return field_i64;
            } else {
                this.field_i64 = field_i64;
            }
        }
        const field_f32Buf: ArrayBufferLike | undefined = storage.get(55);
        if (field_f32Buf === undefined) {
            return new Error(`Fail to get property field_f32 (id=55)`);
        }
        if (field_f32Buf.byteLength === 0) {
            this.field_f32 = undefined;
        } else {
            const field_f32: number | Error = this.getValue<number>(storage, 55, Protocol.Primitives.f32.decode);
            if (field_f32 instanceof Error) {
                return field_f32;
            } else {
                this.field_f32 = field_f32;
            }
        }
        const field_f64Buf: ArrayBufferLike | undefined = storage.get(56);
        if (field_f64Buf === undefined) {
            return new Error(`Fail to get property field_f64 (id=56)`);
        }
        if (field_f64Buf.byteLength === 0) {
            this.field_f64 = undefined;
        } else {
            const field_f64: number | Error = this.getValue<number>(storage, 56, Protocol.Primitives.f64.decode);
            if (field_f64 instanceof Error) {
                return field_f64;
            } else {
                this.field_f64 = field_f64;
            }
        }
        const field_boolBuf: ArrayBufferLike | undefined = storage.get(57);
        if (field_boolBuf === undefined) {
            return new Error(`Fail to get property field_bool (id=57)`);
        }
        if (field_boolBuf.byteLength === 0) {
            this.field_bool = undefined;
        } else {
            const field_bool: boolean | Error = this.getValue<boolean>(storage, 57, Protocol.Primitives.bool.decode);
            if (field_bool instanceof Error) {
                return field_bool;
            } else {
                this.field_bool = field_bool;
            }
        }
        return this;
    }

    public defaults(): StructC {
        return StructC.defaults();
    }
}

export interface IStructD {
    field_str: Array<string> | undefined;
    field_u8: Array<number> | undefined;
    field_u16: Array<number> | undefined;
    field_u32: Array<number> | undefined;
    field_u64: Array<bigint> | undefined;
    field_i8: Array<number> | undefined;
    field_i16: Array<number> | undefined;
    field_i32: Array<number> | undefined;
    field_i64: Array<bigint> | undefined;
    field_f32: Array<number> | undefined;
    field_f64: Array<number> | undefined;
    field_bool: Array<boolean> | undefined;
}
export class StructD extends Protocol.Convertor<StructD> implements IStructD, ISigned<StructD> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_str', types: Protocol.Primitives.ArrayStrUTF8, optional: true, },
        { prop: 'field_u8', types: Protocol.Primitives.ArrayU8, optional: true, },
        { prop: 'field_u16', types: Protocol.Primitives.ArrayU16, optional: true, },
        { prop: 'field_u32', types: Protocol.Primitives.ArrayU32, optional: true, },
        { prop: 'field_u64', types: Protocol.Primitives.ArrayU64, optional: true, },
        { prop: 'field_i8', types: Protocol.Primitives.ArrayI8, optional: true, },
        { prop: 'field_i16', types: Protocol.Primitives.ArrayI16, optional: true, },
        { prop: 'field_i32', types: Protocol.Primitives.ArrayI32, optional: true, },
        { prop: 'field_i64', types: Protocol.Primitives.ArrayI64, optional: true, },
        { prop: 'field_f32', types: Protocol.Primitives.ArrayF32, optional: true, },
        { prop: 'field_f64', types: Protocol.Primitives.ArrayF64, optional: true, },
        { prop: 'field_bool', types: Protocol.Primitives.ArrayBool, optional: true, },
    ];

    public static defaults(): StructD {
        return new StructD({
            field_str: undefined,
            field_u8: undefined,
            field_u16: undefined,
            field_u32: undefined,
            field_u64: undefined,
            field_i8: undefined,
            field_i16: undefined,
            field_i32: undefined,
            field_i64: undefined,
            field_f32: undefined,
            field_f64: undefined,
            field_bool: undefined,
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructD>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructD)) {
                            throw new Error(`Expecting instance of StructD on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructD ? undefined : new Error(`Expecting instance of StructD`);
            }};
        }
    }

    public static from(obj: any): StructD | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructD.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructD.scheme);
            return error instanceof Error ? error : new StructD({
                field_str: obj.field_str,
                field_u8: obj.field_u8,
                field_u16: obj.field_u16,
                field_u32: obj.field_u32,
                field_u64: obj.field_u64,
                field_i8: obj.field_i8,
                field_i16: obj.field_i16,
                field_i32: obj.field_i32,
                field_i64: obj.field_i64,
                field_f32: obj.field_f32,
                field_f64: obj.field_f64,
                field_bool: obj.field_bool,
            });
        }
    }

    public field_str!: Array<string> | undefined;
    public field_u8!: Array<number> | undefined;
    public field_u16!: Array<number> | undefined;
    public field_u32!: Array<number> | undefined;
    public field_u64!: Array<bigint> | undefined;
    public field_i8!: Array<number> | undefined;
    public field_i16!: Array<number> | undefined;
    public field_i32!: Array<number> | undefined;
    public field_i64!: Array<bigint> | undefined;
    public field_f32!: Array<number> | undefined;
    public field_f64!: Array<number> | undefined;
    public field_bool!: Array<boolean> | undefined;
    public static getSignature(): string { return 'StructD'; }
    public static getId(): number { return 58; }


    constructor(params: IStructD)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructD'; }

    public get(): StructD { return this; }

    public getId(): number { return 58; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.field_str === undefined ? this.getBuffer(59, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<string>>(59, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.field_str),
            () => this.field_u8 === undefined ? this.getBuffer(60, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(60, Protocol.ESize.u64, Protocol.Primitives.ArrayU8.encode, this.field_u8),
            () => this.field_u16 === undefined ? this.getBuffer(61, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(61, Protocol.ESize.u64, Protocol.Primitives.ArrayU16.encode, this.field_u16),
            () => this.field_u32 === undefined ? this.getBuffer(62, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(62, Protocol.ESize.u64, Protocol.Primitives.ArrayU32.encode, this.field_u32),
            () => this.field_u64 === undefined ? this.getBuffer(63, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<bigint>>(63, Protocol.ESize.u64, Protocol.Primitives.ArrayU64.encode, this.field_u64),
            () => this.field_i8 === undefined ? this.getBuffer(64, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(64, Protocol.ESize.u64, Protocol.Primitives.ArrayI8.encode, this.field_i8),
            () => this.field_i16 === undefined ? this.getBuffer(65, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(65, Protocol.ESize.u64, Protocol.Primitives.ArrayI16.encode, this.field_i16),
            () => this.field_i32 === undefined ? this.getBuffer(66, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(66, Protocol.ESize.u64, Protocol.Primitives.ArrayI32.encode, this.field_i32),
            () => this.field_i64 === undefined ? this.getBuffer(67, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<bigint>>(67, Protocol.ESize.u64, Protocol.Primitives.ArrayI64.encode, this.field_i64),
            () => this.field_f32 === undefined ? this.getBuffer(68, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(68, Protocol.ESize.u64, Protocol.Primitives.ArrayF32.encode, this.field_f32),
            () => this.field_f64 === undefined ? this.getBuffer(69, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<number>>(69, Protocol.ESize.u64, Protocol.Primitives.ArrayF64.encode, this.field_f64),
            () => this.field_bool === undefined ? this.getBuffer(70, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<Array<boolean>>(70, Protocol.ESize.u64, Protocol.Primitives.ArrayBool.encode, this.field_bool),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructD {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_strBuf: ArrayBufferLike | undefined = storage.get(59);
        if (field_strBuf === undefined) {
            return new Error(`Fail to get property field_str (id=59)`);
        }
        if (field_strBuf.byteLength === 0) {
            this.field_str = undefined;
        } else {
            const field_str: Array<string> | Error = this.getValue<Array<string>>(storage, 59, Protocol.Primitives.ArrayStrUTF8.decode);
            if (field_str instanceof Error) {
                return field_str;
            } else {
                this.field_str = field_str;
            }
        }
        const field_u8Buf: ArrayBufferLike | undefined = storage.get(60);
        if (field_u8Buf === undefined) {
            return new Error(`Fail to get property field_u8 (id=60)`);
        }
        if (field_u8Buf.byteLength === 0) {
            this.field_u8 = undefined;
        } else {
            const field_u8: Array<number> | Error = this.getValue<Array<number>>(storage, 60, Protocol.Primitives.ArrayU8.decode);
            if (field_u8 instanceof Error) {
                return field_u8;
            } else {
                this.field_u8 = field_u8;
            }
        }
        const field_u16Buf: ArrayBufferLike | undefined = storage.get(61);
        if (field_u16Buf === undefined) {
            return new Error(`Fail to get property field_u16 (id=61)`);
        }
        if (field_u16Buf.byteLength === 0) {
            this.field_u16 = undefined;
        } else {
            const field_u16: Array<number> | Error = this.getValue<Array<number>>(storage, 61, Protocol.Primitives.ArrayU16.decode);
            if (field_u16 instanceof Error) {
                return field_u16;
            } else {
                this.field_u16 = field_u16;
            }
        }
        const field_u32Buf: ArrayBufferLike | undefined = storage.get(62);
        if (field_u32Buf === undefined) {
            return new Error(`Fail to get property field_u32 (id=62)`);
        }
        if (field_u32Buf.byteLength === 0) {
            this.field_u32 = undefined;
        } else {
            const field_u32: Array<number> | Error = this.getValue<Array<number>>(storage, 62, Protocol.Primitives.ArrayU32.decode);
            if (field_u32 instanceof Error) {
                return field_u32;
            } else {
                this.field_u32 = field_u32;
            }
        }
        const field_u64Buf: ArrayBufferLike | undefined = storage.get(63);
        if (field_u64Buf === undefined) {
            return new Error(`Fail to get property field_u64 (id=63)`);
        }
        if (field_u64Buf.byteLength === 0) {
            this.field_u64 = undefined;
        } else {
            const field_u64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 63, Protocol.Primitives.ArrayU64.decode);
            if (field_u64 instanceof Error) {
                return field_u64;
            } else {
                this.field_u64 = field_u64;
            }
        }
        const field_i8Buf: ArrayBufferLike | undefined = storage.get(64);
        if (field_i8Buf === undefined) {
            return new Error(`Fail to get property field_i8 (id=64)`);
        }
        if (field_i8Buf.byteLength === 0) {
            this.field_i8 = undefined;
        } else {
            const field_i8: Array<number> | Error = this.getValue<Array<number>>(storage, 64, Protocol.Primitives.ArrayI8.decode);
            if (field_i8 instanceof Error) {
                return field_i8;
            } else {
                this.field_i8 = field_i8;
            }
        }
        const field_i16Buf: ArrayBufferLike | undefined = storage.get(65);
        if (field_i16Buf === undefined) {
            return new Error(`Fail to get property field_i16 (id=65)`);
        }
        if (field_i16Buf.byteLength === 0) {
            this.field_i16 = undefined;
        } else {
            const field_i16: Array<number> | Error = this.getValue<Array<number>>(storage, 65, Protocol.Primitives.ArrayI16.decode);
            if (field_i16 instanceof Error) {
                return field_i16;
            } else {
                this.field_i16 = field_i16;
            }
        }
        const field_i32Buf: ArrayBufferLike | undefined = storage.get(66);
        if (field_i32Buf === undefined) {
            return new Error(`Fail to get property field_i32 (id=66)`);
        }
        if (field_i32Buf.byteLength === 0) {
            this.field_i32 = undefined;
        } else {
            const field_i32: Array<number> | Error = this.getValue<Array<number>>(storage, 66, Protocol.Primitives.ArrayI32.decode);
            if (field_i32 instanceof Error) {
                return field_i32;
            } else {
                this.field_i32 = field_i32;
            }
        }
        const field_i64Buf: ArrayBufferLike | undefined = storage.get(67);
        if (field_i64Buf === undefined) {
            return new Error(`Fail to get property field_i64 (id=67)`);
        }
        if (field_i64Buf.byteLength === 0) {
            this.field_i64 = undefined;
        } else {
            const field_i64: Array<bigint> | Error = this.getValue<Array<bigint>>(storage, 67, Protocol.Primitives.ArrayI64.decode);
            if (field_i64 instanceof Error) {
                return field_i64;
            } else {
                this.field_i64 = field_i64;
            }
        }
        const field_f32Buf: ArrayBufferLike | undefined = storage.get(68);
        if (field_f32Buf === undefined) {
            return new Error(`Fail to get property field_f32 (id=68)`);
        }
        if (field_f32Buf.byteLength === 0) {
            this.field_f32 = undefined;
        } else {
            const field_f32: Array<number> | Error = this.getValue<Array<number>>(storage, 68, Protocol.Primitives.ArrayF32.decode);
            if (field_f32 instanceof Error) {
                return field_f32;
            } else {
                this.field_f32 = field_f32;
            }
        }
        const field_f64Buf: ArrayBufferLike | undefined = storage.get(69);
        if (field_f64Buf === undefined) {
            return new Error(`Fail to get property field_f64 (id=69)`);
        }
        if (field_f64Buf.byteLength === 0) {
            this.field_f64 = undefined;
        } else {
            const field_f64: Array<number> | Error = this.getValue<Array<number>>(storage, 69, Protocol.Primitives.ArrayF64.decode);
            if (field_f64 instanceof Error) {
                return field_f64;
            } else {
                this.field_f64 = field_f64;
            }
        }
        const field_boolBuf: ArrayBufferLike | undefined = storage.get(70);
        if (field_boolBuf === undefined) {
            return new Error(`Fail to get property field_bool (id=70)`);
        }
        if (field_boolBuf.byteLength === 0) {
            this.field_bool = undefined;
        } else {
            const field_bool: Array<boolean> | Error = this.getValue<Array<boolean>>(storage, 70, Protocol.Primitives.ArrayBool.decode);
            if (field_bool instanceof Error) {
                return field_bool;
            } else {
                this.field_bool = field_bool;
            }
        }
        return this;
    }

    public defaults(): StructD {
        return StructD.defaults();
    }
}

export interface IStructE {
    field_a: IEnumA;
    field_b: IEnumB;
    field_c: IEnumC;
}
export class StructE extends Protocol.Convertor<StructE> implements IStructE, ISigned<StructE> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_a', optional: false, options: [
            { prop: 'Option_a', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'Option_b', types: Protocol.Primitives.StrUTF8, optional: false, },
        ] },
        { prop: 'field_b', optional: false, options: [
            { prop: 'Option_str', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'Option_u8', types: Protocol.Primitives.u8, optional: false, },
            { prop: 'Option_u16', types: Protocol.Primitives.u16, optional: false, },
            { prop: 'Option_u32', types: Protocol.Primitives.u32, optional: false, },
            { prop: 'Option_u64', types: Protocol.Primitives.u64, optional: false, },
            { prop: 'Option_i8', types: Protocol.Primitives.i8, optional: false, },
            { prop: 'Option_i16', types: Protocol.Primitives.i16, optional: false, },
            { prop: 'Option_i32', types: Protocol.Primitives.i32, optional: false, },
            { prop: 'Option_i64', types: Protocol.Primitives.i64, optional: false, },
            { prop: 'Option_f32', types: Protocol.Primitives.f32, optional: false, },
            { prop: 'Option_f64', types: Protocol.Primitives.f64, optional: false, },
        ] },
        { prop: 'field_c', optional: false, options: [
            { prop: 'Option_str', types: Protocol.Primitives.ArrayStrUTF8, optional: false, },
            { prop: 'Option_u8', types: Protocol.Primitives.ArrayU8, optional: false, },
            { prop: 'Option_u16', types: Protocol.Primitives.ArrayU16, optional: false, },
            { prop: 'Option_u32', types: Protocol.Primitives.ArrayU32, optional: false, },
            { prop: 'Option_u64', types: Protocol.Primitives.ArrayU64, optional: false, },
            { prop: 'Option_i8', types: Protocol.Primitives.ArrayI8, optional: false, },
            { prop: 'Option_i16', types: Protocol.Primitives.ArrayI16, optional: false, },
            { prop: 'Option_i32', types: Protocol.Primitives.ArrayI32, optional: false, },
            { prop: 'Option_i64', types: Protocol.Primitives.ArrayI64, optional: false, },
            { prop: 'Option_f32', types: Protocol.Primitives.ArrayF32, optional: false, },
            { prop: 'Option_f64', types: Protocol.Primitives.ArrayF64, optional: false, },
        ] },
    ];

    public static defaults(): StructE {
        return new StructE({
            field_a: {},
            field_b: {},
            field_c: {},
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructE>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructE)) {
                            throw new Error(`Expecting instance of StructE on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructE ? undefined : new Error(`Expecting instance of StructE`);
            }};
        }
    }

    public static from(obj: any): StructE | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructE.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructE.scheme);
            return error instanceof Error ? error : new StructE({
                field_a: obj.field_a,
                field_b: obj.field_b,
                field_c: obj.field_c,
            });
        }
    }

    public field_a!: IEnumA;
    public field_b!: IEnumB;
    public field_c!: IEnumC;
    private _field_a: Primitives.Enum;
    private _field_b: Primitives.Enum;
    private _field_c: Primitives.Enum;
    public static getSignature(): string { return 'StructE'; }
    public static getId(): number { return 71; }


    constructor(params: IStructE)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
        this._field_a = new EnumA()
        this._field_a.set(this.field_a);
        this._field_b = new EnumB()
        this._field_b.set(this.field_b);
        this._field_c = new EnumC()
        this._field_c.set(this.field_c);
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructE'; }

    public get(): StructE { return this; }

    public getId(): number { return 71; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => { const buffer = this._field_a.encode(); return this.getBuffer(72, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this._field_b.encode(); return this.getBuffer(73, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this._field_c.encode(); return this.getBuffer(74, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructE {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        this.field_a = {};
        const field_aBuf: ArrayBufferLike | undefined = storage.get(72);
        if (field_aBuf === undefined) {
            return new Error(`Fail to get property "field_a"`);
        }
        if (field_aBuf.byteLength > 0) {
            const field_aErr: Error | undefined = this._field_a.decode(field_aBuf);
            if (field_aErr instanceof Error) {
                return field_aErr;
            } else {
                this.field_a = this._field_a.get();
            }
        }
        this.field_b = {};
        const field_bBuf: ArrayBufferLike | undefined = storage.get(73);
        if (field_bBuf === undefined) {
            return new Error(`Fail to get property "field_b"`);
        }
        if (field_bBuf.byteLength > 0) {
            const field_bErr: Error | undefined = this._field_b.decode(field_bBuf);
            if (field_bErr instanceof Error) {
                return field_bErr;
            } else {
                this.field_b = this._field_b.get();
            }
        }
        this.field_c = {};
        const field_cBuf: ArrayBufferLike | undefined = storage.get(74);
        if (field_cBuf === undefined) {
            return new Error(`Fail to get property "field_c"`);
        }
        if (field_cBuf.byteLength > 0) {
            const field_cErr: Error | undefined = this._field_c.decode(field_cBuf);
            if (field_cErr instanceof Error) {
                return field_cErr;
            } else {
                this.field_c = this._field_c.get();
            }
        }
        return this;
    }

    public defaults(): StructE {
        return StructE.defaults();
    }
}

export interface IStructF {
    field_a: IEnumA | undefined;
    field_b: IEnumB | undefined;
    field_c: IEnumC | undefined;
}
export class StructF extends Protocol.Convertor<StructF> implements IStructF, ISigned<StructF> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_a', optional: true, options: [
            { prop: 'Option_a', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'Option_b', types: Protocol.Primitives.StrUTF8, optional: false, },
        ] },
        { prop: 'field_b', optional: true, options: [
            { prop: 'Option_str', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'Option_u8', types: Protocol.Primitives.u8, optional: false, },
            { prop: 'Option_u16', types: Protocol.Primitives.u16, optional: false, },
            { prop: 'Option_u32', types: Protocol.Primitives.u32, optional: false, },
            { prop: 'Option_u64', types: Protocol.Primitives.u64, optional: false, },
            { prop: 'Option_i8', types: Protocol.Primitives.i8, optional: false, },
            { prop: 'Option_i16', types: Protocol.Primitives.i16, optional: false, },
            { prop: 'Option_i32', types: Protocol.Primitives.i32, optional: false, },
            { prop: 'Option_i64', types: Protocol.Primitives.i64, optional: false, },
            { prop: 'Option_f32', types: Protocol.Primitives.f32, optional: false, },
            { prop: 'Option_f64', types: Protocol.Primitives.f64, optional: false, },
        ] },
        { prop: 'field_c', optional: true, options: [
            { prop: 'Option_str', types: Protocol.Primitives.ArrayStrUTF8, optional: false, },
            { prop: 'Option_u8', types: Protocol.Primitives.ArrayU8, optional: false, },
            { prop: 'Option_u16', types: Protocol.Primitives.ArrayU16, optional: false, },
            { prop: 'Option_u32', types: Protocol.Primitives.ArrayU32, optional: false, },
            { prop: 'Option_u64', types: Protocol.Primitives.ArrayU64, optional: false, },
            { prop: 'Option_i8', types: Protocol.Primitives.ArrayI8, optional: false, },
            { prop: 'Option_i16', types: Protocol.Primitives.ArrayI16, optional: false, },
            { prop: 'Option_i32', types: Protocol.Primitives.ArrayI32, optional: false, },
            { prop: 'Option_i64', types: Protocol.Primitives.ArrayI64, optional: false, },
            { prop: 'Option_f32', types: Protocol.Primitives.ArrayF32, optional: false, },
            { prop: 'Option_f64', types: Protocol.Primitives.ArrayF64, optional: false, },
        ] },
    ];

    public static defaults(): StructF {
        return new StructF({
            field_a: undefined,
            field_b: undefined,
            field_c: undefined,
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructF>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructF)) {
                            throw new Error(`Expecting instance of StructF on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructF ? undefined : new Error(`Expecting instance of StructF`);
            }};
        }
    }

    public static from(obj: any): StructF | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructF.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructF.scheme);
            return error instanceof Error ? error : new StructF({
                field_a: obj.field_a,
                field_b: obj.field_b,
                field_c: obj.field_c,
            });
        }
    }

    public field_a!: IEnumA | undefined;
    public field_b!: IEnumB | undefined;
    public field_c!: IEnumC | undefined;
    private _field_a: Primitives.Enum;
    private _field_b: Primitives.Enum;
    private _field_c: Primitives.Enum;
    public static getSignature(): string { return 'StructF'; }
    public static getId(): number { return 75; }


    constructor(params: IStructF)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
        this._field_a = new EnumA()
        this.field_a !== undefined && this._field_a.set(this.field_a);
        this._field_b = new EnumB()
        this.field_b !== undefined && this._field_b.set(this.field_b);
        this._field_c = new EnumC()
        this.field_c !== undefined && this._field_c.set(this.field_c);
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructF'; }

    public get(): StructF { return this; }

    public getId(): number { return 75; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => {if (this.field_a === undefined) { return this.getBuffer(76, Protocol.ESize.u8, 0, new Uint8Array()); } const buffer = this._field_a.encode(); return this.getBuffer(76, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => {if (this.field_b === undefined) { return this.getBuffer(77, Protocol.ESize.u8, 0, new Uint8Array()); } const buffer = this._field_b.encode(); return this.getBuffer(77, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => {if (this.field_c === undefined) { return this.getBuffer(78, Protocol.ESize.u8, 0, new Uint8Array()); } const buffer = this._field_c.encode(); return this.getBuffer(78, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructF {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_aBuf: ArrayBufferLike | undefined = storage.get(76);
        if (field_aBuf === undefined) {
            return new Error(`Fail to get property field_a (id=76)`);
        }
        if (field_aBuf.byteLength === 0) {
            this.field_a = undefined;
        } else {
            this.field_a = {};
            const field_aBuf: ArrayBufferLike | undefined = storage.get(76);
            if (field_aBuf === undefined) {
                return new Error(`Fail to get property "field_a"`);
            }
            if (field_aBuf.byteLength > 0) {
                const field_aErr: Error | undefined = this._field_a.decode(field_aBuf);
                if (field_aErr instanceof Error) {
                    return field_aErr;
                } else {
                    this.field_a = this._field_a.get();
                }
            }
        }
        const field_bBuf: ArrayBufferLike | undefined = storage.get(77);
        if (field_bBuf === undefined) {
            return new Error(`Fail to get property field_b (id=77)`);
        }
        if (field_bBuf.byteLength === 0) {
            this.field_b = undefined;
        } else {
            this.field_b = {};
            const field_bBuf: ArrayBufferLike | undefined = storage.get(77);
            if (field_bBuf === undefined) {
                return new Error(`Fail to get property "field_b"`);
            }
            if (field_bBuf.byteLength > 0) {
                const field_bErr: Error | undefined = this._field_b.decode(field_bBuf);
                if (field_bErr instanceof Error) {
                    return field_bErr;
                } else {
                    this.field_b = this._field_b.get();
                }
            }
        }
        const field_cBuf: ArrayBufferLike | undefined = storage.get(78);
        if (field_cBuf === undefined) {
            return new Error(`Fail to get property field_c (id=78)`);
        }
        if (field_cBuf.byteLength === 0) {
            this.field_c = undefined;
        } else {
            this.field_c = {};
            const field_cBuf: ArrayBufferLike | undefined = storage.get(78);
            if (field_cBuf === undefined) {
                return new Error(`Fail to get property "field_c"`);
            }
            if (field_cBuf.byteLength > 0) {
                const field_cErr: Error | undefined = this._field_c.decode(field_cBuf);
                if (field_cErr instanceof Error) {
                    return field_cErr;
                } else {
                    this.field_c = this._field_c.get();
                }
            }
        }
        return this;
    }

    public defaults(): StructF {
        return StructF.defaults();
    }
}

export interface IStructG {
    field_a: StructA;
    field_b: StructB;
}
export class StructG extends Protocol.Convertor<StructG> implements IStructG, ISigned<StructG> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_a', types: StructA.getValidator(false), optional: false },
        { prop: 'field_b', types: StructB.getValidator(false), optional: false },
    ];

    public static defaults(): StructG {
        return new StructG({
            field_a: new StructA({
                field_str: '',
                field_str_empty: '',
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: BigInt(0),
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: BigInt(0),
                field_f32: 0,
                field_f64: 0,
                field_bool: true,
            }),
            field_b: new StructB({
                field_str: [],
                field_u8: [],
                field_u16: [],
                field_u32: [],
                field_u64: [],
                field_i8: [],
                field_i16: [],
                field_i32: [],
                field_i64: [],
                field_f32: [],
                field_f64: [],
                field_bool: [],
                field_struct: [],
                field_str_empty: [],
                field_u8_empty: [],
                field_u16_empty: [],
                field_u32_empty: [],
                field_u64_empty: [],
                field_i8_empty: [],
                field_i16_empty: [],
                field_i32_empty: [],
                field_i64_empty: [],
                field_f32_empty: [],
                field_f64_empty: [],
                field_bool_empty: [],
                field_struct_empty: [],
            }),
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructG>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructG)) {
                            throw new Error(`Expecting instance of StructG on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructG ? undefined : new Error(`Expecting instance of StructG`);
            }};
        }
    }

    public static from(obj: any): StructG | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructG.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructG.scheme);
            return error instanceof Error ? error : new StructG({
                field_a: obj.field_a,
                field_b: obj.field_b,
            });
        }
    }

    public field_a!: StructA;
    public field_b!: StructB;
    public static getSignature(): string { return 'StructG'; }
    public static getId(): number { return 79; }


    constructor(params: IStructG)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructG'; }

    public get(): StructG { return this; }

    public getId(): number { return 79; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => { const buffer = this.field_a.encode(); return this.getBuffer(80, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this.field_b.encode(); return this.getBuffer(81, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructG {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_a: StructA = new StructA({
            field_str: '',
            field_str_empty: '',
            field_u8: 0,
            field_u16: 0,
            field_u32: 0,
            field_u64: BigInt(0),
            field_i8: 0,
            field_i16: 0,
            field_i32: 0,
            field_i64: BigInt(0),
            field_f32: 0,
            field_f64: 0,
            field_bool: true,
        });
        const field_aBuf: ArrayBufferLike | undefined = storage.get(80);
        if (field_aBuf === undefined) {
            return new Error(`Fail to find field "field_a" (id=80).`);
        }
        const field_aErr: Error | StructA = field_a.decode(field_aBuf);
        if (field_aErr instanceof Error) {
            return field_aErr;
        } else {
            this.field_a = field_a;
        }
        const field_b: StructB = new StructB({
            field_str: [],
            field_u8: [],
            field_u16: [],
            field_u32: [],
            field_u64: [],
            field_i8: [],
            field_i16: [],
            field_i32: [],
            field_i64: [],
            field_f32: [],
            field_f64: [],
            field_bool: [],
            field_struct: [],
            field_str_empty: [],
            field_u8_empty: [],
            field_u16_empty: [],
            field_u32_empty: [],
            field_u64_empty: [],
            field_i8_empty: [],
            field_i16_empty: [],
            field_i32_empty: [],
            field_i64_empty: [],
            field_f32_empty: [],
            field_f64_empty: [],
            field_bool_empty: [],
            field_struct_empty: [],
        });
        const field_bBuf: ArrayBufferLike | undefined = storage.get(81);
        if (field_bBuf === undefined) {
            return new Error(`Fail to find field "field_b" (id=81).`);
        }
        const field_bErr: Error | StructB = field_b.decode(field_bBuf);
        if (field_bErr instanceof Error) {
            return field_bErr;
        } else {
            this.field_b = field_b;
        }
        return this;
    }

    public defaults(): StructG {
        return StructG.defaults();
    }
}

export interface ITriggerBeaconsEmitter {
    uuid: string;
}
export class TriggerBeaconsEmitter extends Protocol.Convertor<TriggerBeaconsEmitter> implements ITriggerBeaconsEmitter, ISigned<TriggerBeaconsEmitter> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
    ];

    public static defaults(): TriggerBeaconsEmitter {
        return new TriggerBeaconsEmitter({
            uuid: '',
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<TriggerBeaconsEmitter>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof TriggerBeaconsEmitter)) {
                            throw new Error(`Expecting instance of TriggerBeaconsEmitter on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof TriggerBeaconsEmitter ? undefined : new Error(`Expecting instance of TriggerBeaconsEmitter`);
            }};
        }
    }

    public static from(obj: any): TriggerBeaconsEmitter | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = TriggerBeaconsEmitter.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, TriggerBeaconsEmitter.scheme);
            return error instanceof Error ? error : new TriggerBeaconsEmitter({
                uuid: obj.uuid,
            });
        }
    }

    public uuid!: string;
    public static getSignature(): string { return 'TriggerBeaconsEmitter'; }
    public static getId(): number { return 82; }


    constructor(params: ITriggerBeaconsEmitter)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'TriggerBeaconsEmitter'; }

    public get(): TriggerBeaconsEmitter { return this; }

    public getId(): number { return 82; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(83, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | TriggerBeaconsEmitter {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const uuid: string | Error = this.getValue<string>(storage, 83, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
        return this;
    }

    public defaults(): TriggerBeaconsEmitter {
        return TriggerBeaconsEmitter.defaults();
    }
}

export interface IStructEmpty {
}
export class StructEmpty extends Protocol.Convertor<StructEmpty> implements IStructEmpty, ISigned<StructEmpty> {

    public static scheme: Protocol.IPropScheme[] = [
    ];

    public static defaults(): StructEmpty {
        return new StructEmpty({
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructEmpty>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructEmpty)) {
                            throw new Error(`Expecting instance of StructEmpty on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructEmpty ? undefined : new Error(`Expecting instance of StructEmpty`);
            }};
        }
    }

    public static from(obj: any): StructEmpty | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructEmpty.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructEmpty.scheme);
            return error instanceof Error ? error : new StructEmpty({
            });
        }
    }

    public static getSignature(): string { return 'StructEmpty'; }
    public static getId(): number { return 84; }


    constructor(params: IStructEmpty)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructEmpty'; }

    public get(): StructEmpty { return this; }

    public getId(): number { return 84; }

    public encode(): ArrayBufferLike {
        return this.collect([
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructEmpty {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        return this;
    }

    public defaults(): StructEmpty {
        return StructEmpty.defaults();
    }
}

export interface IStructEmptyA {
}
export class StructEmptyA extends Protocol.Convertor<StructEmptyA> implements IStructEmptyA, ISigned<StructEmptyA> {

    public static scheme: Protocol.IPropScheme[] = [
    ];

    public static defaults(): StructEmptyA {
        return new StructEmptyA({
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructEmptyA>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructEmptyA)) {
                            throw new Error(`Expecting instance of StructEmptyA on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructEmptyA ? undefined : new Error(`Expecting instance of StructEmptyA`);
            }};
        }
    }

    public static from(obj: any): StructEmptyA | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructEmptyA.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructEmptyA.scheme);
            return error instanceof Error ? error : new StructEmptyA({
            });
        }
    }

    public static getSignature(): string { return 'StructEmptyA'; }
    public static getId(): number { return 85; }


    constructor(params: IStructEmptyA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructEmptyA'; }

    public get(): StructEmptyA { return this; }

    public getId(): number { return 85; }

    public encode(): ArrayBufferLike {
        return this.collect([
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructEmptyA {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        return this;
    }

    public defaults(): StructEmptyA {
        return StructEmptyA.defaults();
    }
}

export interface IStructEmptyB {
}
export class StructEmptyB extends Protocol.Convertor<StructEmptyB> implements IStructEmptyB, ISigned<StructEmptyB> {

    public static scheme: Protocol.IPropScheme[] = [
    ];

    public static defaults(): StructEmptyB {
        return new StructEmptyB({
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructEmptyB>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructEmptyB)) {
                            throw new Error(`Expecting instance of StructEmptyB on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructEmptyB ? undefined : new Error(`Expecting instance of StructEmptyB`);
            }};
        }
    }

    public static from(obj: any): StructEmptyB | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructEmptyB.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructEmptyB.scheme);
            return error instanceof Error ? error : new StructEmptyB({
            });
        }
    }

    public static getSignature(): string { return 'StructEmptyB'; }
    public static getId(): number { return 86; }


    constructor(params: IStructEmptyB)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructEmptyB'; }

    public get(): StructEmptyB { return this; }

    public getId(): number { return 86; }

    public encode(): ArrayBufferLike {
        return this.collect([
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructEmptyB {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        return this;
    }

    public defaults(): StructEmptyB {
        return StructEmptyB.defaults();
    }
}

export interface IStructJ {
    field_a: StructA | undefined;
    field_b: StructB | undefined;
    field_c: StructEmpty;
}
export class StructJ extends Protocol.Convertor<StructJ> implements IStructJ, ISigned<StructJ> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field_a', types: StructA.getValidator(false), optional: true },
        { prop: 'field_b', types: StructB.getValidator(false), optional: true },
        { prop: 'field_c', types: StructEmpty.getValidator(false), optional: false },
    ];

    public static defaults(): StructJ {
        return new StructJ({
            field_a: undefined,
            field_b: undefined,
            field_c: new StructEmpty({
            }),
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<StructJ>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof StructJ)) {
                            throw new Error(`Expecting instance of StructJ on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof StructJ ? undefined : new Error(`Expecting instance of StructJ`);
            }};
        }
    }

    public static from(obj: any): StructJ | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = StructJ.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, StructJ.scheme);
            return error instanceof Error ? error : new StructJ({
                field_a: obj.field_a,
                field_b: obj.field_b,
                field_c: obj.field_c,
            });
        }
    }

    public field_a!: StructA | undefined;
    public field_b!: StructB | undefined;
    public field_c!: StructEmpty;
    public static getSignature(): string { return 'StructJ'; }
    public static getId(): number { return 87; }


    constructor(params: IStructJ)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'StructJ'; }

    public get(): StructJ { return this; }

    public getId(): number { return 87; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => {if (this.field_a === undefined) { return this.getBuffer(88, Protocol.ESize.u8, 0, new Uint8Array()); } const buffer = this.field_a.encode(); return this.getBuffer(88, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => {if (this.field_b === undefined) { return this.getBuffer(89, Protocol.ESize.u8, 0, new Uint8Array()); } const buffer = this.field_b.encode(); return this.getBuffer(89, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this.field_c.encode(); return this.getBuffer(90, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | StructJ {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field_aBuf: ArrayBufferLike | undefined = storage.get(88);
        if (field_aBuf === undefined) {
            return new Error(`Fail to get property field_a (id=88)`);
        }
        if (field_aBuf.byteLength === 0) {
            this.field_a = undefined;
        } else {
            const field_a: StructA = new StructA({
                field_str: '',
                field_str_empty: '',
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: BigInt(0),
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: BigInt(0),
                field_f32: 0,
                field_f64: 0,
                field_bool: true,
            });
            const field_aBuf: ArrayBufferLike | undefined = storage.get(88);
            if (field_aBuf === undefined) {
                return new Error(`Fail to find field "field_a" (id=88).`);
            }
            const field_aErr: Error | StructA = field_a.decode(field_aBuf);
            if (field_aErr instanceof Error) {
                return field_aErr;
            } else {
                this.field_a = field_a;
            }
        }
        const field_bBuf: ArrayBufferLike | undefined = storage.get(89);
        if (field_bBuf === undefined) {
            return new Error(`Fail to get property field_b (id=89)`);
        }
        if (field_bBuf.byteLength === 0) {
            this.field_b = undefined;
        } else {
            const field_b: StructB = new StructB({
                field_str: [],
                field_u8: [],
                field_u16: [],
                field_u32: [],
                field_u64: [],
                field_i8: [],
                field_i16: [],
                field_i32: [],
                field_i64: [],
                field_f32: [],
                field_f64: [],
                field_bool: [],
                field_struct: [],
                field_str_empty: [],
                field_u8_empty: [],
                field_u16_empty: [],
                field_u32_empty: [],
                field_u64_empty: [],
                field_i8_empty: [],
                field_i16_empty: [],
                field_i32_empty: [],
                field_i64_empty: [],
                field_f32_empty: [],
                field_f64_empty: [],
                field_bool_empty: [],
                field_struct_empty: [],
            });
            const field_bBuf: ArrayBufferLike | undefined = storage.get(89);
            if (field_bBuf === undefined) {
                return new Error(`Fail to find field "field_b" (id=89).`);
            }
            const field_bErr: Error | StructB = field_b.decode(field_bBuf);
            if (field_bErr instanceof Error) {
                return field_bErr;
            } else {
                this.field_b = field_b;
            }
        }
        const field_c: StructEmpty = new StructEmpty({
        });
        const field_cBuf: ArrayBufferLike | undefined = storage.get(90);
        if (field_cBuf === undefined) {
            return new Error(`Fail to find field "field_c" (id=90).`);
        }
        const field_cErr: Error | StructEmpty = field_c.decode(field_cBuf);
        if (field_cErr instanceof Error) {
            return field_cErr;
        } else {
            this.field_c = field_c;
        }
        return this;
    }

    public defaults(): StructJ {
        return StructJ.defaults();
    }
}

export interface ITriggerBeacons {
}
export class TriggerBeacons extends Protocol.Convertor<TriggerBeacons> implements ITriggerBeacons, ISigned<TriggerBeacons> {

    public static scheme: Protocol.IPropScheme[] = [
    ];

    public static defaults(): TriggerBeacons {
        return new TriggerBeacons({
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<TriggerBeacons>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof TriggerBeacons)) {
                            throw new Error(`Expecting instance of TriggerBeacons on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof TriggerBeacons ? undefined : new Error(`Expecting instance of TriggerBeacons`);
            }};
        }
    }

    public static from(obj: any): TriggerBeacons | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = TriggerBeacons.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, TriggerBeacons.scheme);
            return error instanceof Error ? error : new TriggerBeacons({
            });
        }
    }

    public static getSignature(): string { return 'TriggerBeacons'; }
    public static getId(): number { return 91; }


    constructor(params: ITriggerBeacons)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'TriggerBeacons'; }

    public get(): TriggerBeacons { return this; }

    public getId(): number { return 91; }

    public encode(): ArrayBufferLike {
        return this.collect([
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | TriggerBeacons {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        return this;
    }

    public defaults(): TriggerBeacons {
        return TriggerBeacons.defaults();
    }
}

export interface IFinishConsumerTest {
    uuid: string;
}
export class FinishConsumerTest extends Protocol.Convertor<FinishConsumerTest> implements IFinishConsumerTest, ISigned<FinishConsumerTest> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
    ];

    public static defaults(): FinishConsumerTest {
        return new FinishConsumerTest({
            uuid: '',
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<FinishConsumerTest>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof FinishConsumerTest)) {
                            throw new Error(`Expecting instance of FinishConsumerTest on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof FinishConsumerTest ? undefined : new Error(`Expecting instance of FinishConsumerTest`);
            }};
        }
    }

    public static from(obj: any): FinishConsumerTest | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = FinishConsumerTest.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, FinishConsumerTest.scheme);
            return error instanceof Error ? error : new FinishConsumerTest({
                uuid: obj.uuid,
            });
        }
    }

    public uuid!: string;
    public static getSignature(): string { return 'FinishConsumerTest'; }
    public static getId(): number { return 92; }


    constructor(params: IFinishConsumerTest)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'FinishConsumerTest'; }

    public get(): FinishConsumerTest { return this; }

    public getId(): number { return 92; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(93, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | FinishConsumerTest {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const uuid: string | Error = this.getValue<string>(storage, 93, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
        return this;
    }

    public defaults(): FinishConsumerTest {
        return FinishConsumerTest.defaults();
    }
}

export interface IFinishConsumerTestBroadcast {
}
export class FinishConsumerTestBroadcast extends Protocol.Convertor<FinishConsumerTestBroadcast> implements IFinishConsumerTestBroadcast, ISigned<FinishConsumerTestBroadcast> {

    public static scheme: Protocol.IPropScheme[] = [
    ];

    public static defaults(): FinishConsumerTestBroadcast {
        return new FinishConsumerTestBroadcast({
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<FinishConsumerTestBroadcast>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof FinishConsumerTestBroadcast)) {
                            throw new Error(`Expecting instance of FinishConsumerTestBroadcast on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof FinishConsumerTestBroadcast ? undefined : new Error(`Expecting instance of FinishConsumerTestBroadcast`);
            }};
        }
    }

    public static from(obj: any): FinishConsumerTestBroadcast | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = FinishConsumerTestBroadcast.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, FinishConsumerTestBroadcast.scheme);
            return error instanceof Error ? error : new FinishConsumerTestBroadcast({
            });
        }
    }

    public static getSignature(): string { return 'FinishConsumerTestBroadcast'; }
    public static getId(): number { return 94; }


    constructor(params: IFinishConsumerTestBroadcast)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'FinishConsumerTestBroadcast'; }

    public get(): FinishConsumerTestBroadcast { return this; }

    public getId(): number { return 94; }

    public encode(): ArrayBufferLike {
        return this.collect([
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | FinishConsumerTestBroadcast {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        return this;
    }

    public defaults(): FinishConsumerTestBroadcast {
        return FinishConsumerTestBroadcast.defaults();
    }
}

export interface IBeaconA {
    field: StructA;
}
export class BeaconA extends Protocol.Convertor<BeaconA> implements IBeaconA, ISigned<BeaconA> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'field', types: StructA.getValidator(false), optional: false },
    ];

    public static defaults(): BeaconA {
        return new BeaconA({
            field: new StructA({
                field_str: '',
                field_str_empty: '',
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: BigInt(0),
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: BigInt(0),
                field_f32: 0,
                field_f64: 0,
                field_bool: true,
            }),
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<BeaconA>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof BeaconA)) {
                            throw new Error(`Expecting instance of BeaconA on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof BeaconA ? undefined : new Error(`Expecting instance of BeaconA`);
            }};
        }
    }

    public static from(obj: any): BeaconA | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = BeaconA.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, BeaconA.scheme);
            return error instanceof Error ? error : new BeaconA({
                field: obj.field,
            });
        }
    }

    public field!: StructA;
    public static getSignature(): string { return 'BeaconA'; }
    public static getId(): number { return 95; }


    constructor(params: IBeaconA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'BeaconA'; }

    public get(): BeaconA { return this; }

    public getId(): number { return 95; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => { const buffer = this.field.encode(); return this.getBuffer(96, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | BeaconA {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const field: StructA = new StructA({
            field_str: '',
            field_str_empty: '',
            field_u8: 0,
            field_u16: 0,
            field_u32: 0,
            field_u64: BigInt(0),
            field_i8: 0,
            field_i16: 0,
            field_i32: 0,
            field_i64: BigInt(0),
            field_f32: 0,
            field_f64: 0,
            field_bool: true,
        });
        const fieldBuf: ArrayBufferLike | undefined = storage.get(96);
        if (fieldBuf === undefined) {
            return new Error(`Fail to find field "field" (id=96).`);
        }
        const fieldErr: Error | StructA = field.decode(fieldBuf);
        if (fieldErr instanceof Error) {
            return fieldErr;
        } else {
            this.field = field;
        }
        return this;
    }

    public defaults(): BeaconA {
        return BeaconA.defaults();
    }
}

export interface IEventA {
    uuid: string;
    field_a: StructB;
    field_b: StructC;
}
export class EventA extends Protocol.Convertor<EventA> implements IEventA, ISigned<EventA> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'field_a', types: StructB.getValidator(false), optional: false },
        { prop: 'field_b', types: StructC.getValidator(false), optional: false },
    ];

    public static defaults(): EventA {
        return new EventA({
            uuid: '',
            field_a: new StructB({
                field_str: [],
                field_u8: [],
                field_u16: [],
                field_u32: [],
                field_u64: [],
                field_i8: [],
                field_i16: [],
                field_i32: [],
                field_i64: [],
                field_f32: [],
                field_f64: [],
                field_bool: [],
                field_struct: [],
                field_str_empty: [],
                field_u8_empty: [],
                field_u16_empty: [],
                field_u32_empty: [],
                field_u64_empty: [],
                field_i8_empty: [],
                field_i16_empty: [],
                field_i32_empty: [],
                field_i64_empty: [],
                field_f32_empty: [],
                field_f64_empty: [],
                field_bool_empty: [],
                field_struct_empty: [],
            }),
            field_b: new StructC({
                field_str: undefined,
                field_u8: undefined,
                field_u16: undefined,
                field_u32: undefined,
                field_u64: undefined,
                field_i8: undefined,
                field_i16: undefined,
                field_i32: undefined,
                field_i64: undefined,
                field_f32: undefined,
                field_f64: undefined,
                field_bool: undefined,
            }),
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<EventA>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof EventA)) {
                            throw new Error(`Expecting instance of EventA on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof EventA ? undefined : new Error(`Expecting instance of EventA`);
            }};
        }
    }

    public static from(obj: any): EventA | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = EventA.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, EventA.scheme);
            return error instanceof Error ? error : new EventA({
                uuid: obj.uuid,
                field_a: obj.field_a,
                field_b: obj.field_b,
            });
        }
    }

    public uuid!: string;
    public field_a!: StructB;
    public field_b!: StructC;
    public static getSignature(): string { return 'EventA'; }
    public static getId(): number { return 133; }


    constructor(params: IEventA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'EventA'; }

    public get(): EventA { return this; }

    public getId(): number { return 133; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(134, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            () => { const buffer = this.field_a.encode(); return this.getBuffer(135, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this.field_b.encode(); return this.getBuffer(136, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | EventA {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const uuid: string | Error = this.getValue<string>(storage, 134, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
        const field_a: StructB = new StructB({
            field_str: [],
            field_u8: [],
            field_u16: [],
            field_u32: [],
            field_u64: [],
            field_i8: [],
            field_i16: [],
            field_i32: [],
            field_i64: [],
            field_f32: [],
            field_f64: [],
            field_bool: [],
            field_struct: [],
            field_str_empty: [],
            field_u8_empty: [],
            field_u16_empty: [],
            field_u32_empty: [],
            field_u64_empty: [],
            field_i8_empty: [],
            field_i16_empty: [],
            field_i32_empty: [],
            field_i64_empty: [],
            field_f32_empty: [],
            field_f64_empty: [],
            field_bool_empty: [],
            field_struct_empty: [],
        });
        const field_aBuf: ArrayBufferLike | undefined = storage.get(135);
        if (field_aBuf === undefined) {
            return new Error(`Fail to find field "field_a" (id=135).`);
        }
        const field_aErr: Error | StructB = field_a.decode(field_aBuf);
        if (field_aErr instanceof Error) {
            return field_aErr;
        } else {
            this.field_a = field_a;
        }
        const field_b: StructC = new StructC({
            field_str: undefined,
            field_u8: undefined,
            field_u16: undefined,
            field_u32: undefined,
            field_u64: undefined,
            field_i8: undefined,
            field_i16: undefined,
            field_i32: undefined,
            field_i64: undefined,
            field_f32: undefined,
            field_f64: undefined,
            field_bool: undefined,
        });
        const field_bBuf: ArrayBufferLike | undefined = storage.get(136);
        if (field_bBuf === undefined) {
            return new Error(`Fail to find field "field_b" (id=136).`);
        }
        const field_bErr: Error | StructC = field_b.decode(field_bBuf);
        if (field_bErr instanceof Error) {
            return field_bErr;
        } else {
            this.field_b = field_b;
        }
        return this;
    }

    public defaults(): EventA {
        return EventA.defaults();
    }
}

export interface IEventB {
    uuid: string;
    field_a: StructC;
}
export class EventB extends Protocol.Convertor<EventB> implements IEventB, ISigned<EventB> {

    public static scheme: Protocol.IPropScheme[] = [
        { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        { prop: 'field_a', types: StructC.getValidator(false), optional: false },
    ];

    public static defaults(): EventB {
        return new EventB({
            uuid: '',
            field_a: new StructC({
                field_str: undefined,
                field_u8: undefined,
                field_u16: undefined,
                field_u32: undefined,
                field_u64: undefined,
                field_i8: undefined,
                field_i16: undefined,
                field_i32: undefined,
                field_i64: undefined,
                field_f32: undefined,
                field_f64: undefined,
                field_bool: undefined,
            }),
        });
    }

    public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
        if (array) {
            return { validate(obj: any): Error | undefined {
                if (!(obj instanceof Array)) {
                    return new Error(`Expecting Array<EventB>`);
                }
                try {
                    obj.forEach((o, index: number) => {
                        if (!(o instanceof EventB)) {
                            throw new Error(`Expecting instance of EventB on index #${index}`);
                        }
                    });
                } catch (err) {
                    return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                }
            }};
        } else {
            return { validate(obj: any): Error | undefined {
                return obj instanceof EventB ? undefined : new Error(`Expecting instance of EventB`);
            }};
        }
    }

    public static from(obj: any): EventB | Error {
        if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
            const inst = EventB.defaults();
            const err = inst.decode(obj);
            return err instanceof Error ? err : inst;
        } else {
            const error: Error | undefined = Protocol.validate(obj, EventB.scheme);
            return error instanceof Error ? error : new EventB({
                uuid: obj.uuid,
                field_a: obj.field_a,
            });
        }
    }

    public uuid!: string;
    public field_a!: StructC;
    public static getSignature(): string { return 'EventB'; }
    public static getId(): number { return 137; }


    constructor(params: IEventB)  {
        super();
        Object.keys(params).forEach((key: string) => {
            (this as any)[key] = (params as any)[key];
        });
    }

    public signature(): number { return 0; }

    public getSignature(): string { return 'EventB'; }

    public get(): EventB { return this; }

    public getId(): number { return 137; }

    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(138, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            () => { const buffer = this.field_a.encode(); return this.getBuffer(139, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }

    public decode(buffer: ArrayBufferLike): Error | EventB {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const uuid: string | Error = this.getValue<string>(storage, 138, Protocol.Primitives.StrUTF8.decode);
        if (uuid instanceof Error) {
            return uuid;
        } else {
            this.uuid = uuid;
        }
        const field_a: StructC = new StructC({
            field_str: undefined,
            field_u8: undefined,
            field_u16: undefined,
            field_u32: undefined,
            field_u64: undefined,
            field_i8: undefined,
            field_i16: undefined,
            field_i32: undefined,
            field_i64: undefined,
            field_f32: undefined,
            field_f64: undefined,
            field_bool: undefined,
        });
        const field_aBuf: ArrayBufferLike | undefined = storage.get(139);
        if (field_aBuf === undefined) {
            return new Error(`Fail to find field "field_a" (id=139).`);
        }
        const field_aErr: Error | StructC = field_a.decode(field_aBuf);
        if (field_aErr instanceof Error) {
            return field_aErr;
        } else {
            this.field_a = field_a;
        }
        return this;
    }

    public defaults(): EventB {
        return EventB.defaults();
    }
}

export namespace Beacons {
    export interface IAvailableMessages {
        ShutdownServer?: ShutdownServer,
        BeaconA?: BeaconA,
        BeaconB?: BeaconB,
        Sub?: Sub.IAvailableMessages,
    }

    export interface IShutdownServer {
    }
    export class ShutdownServer extends Protocol.Convertor<ShutdownServer> implements IShutdownServer, ISigned<ShutdownServer> {

        public static scheme: Protocol.IPropScheme[] = [
        ];

        public static defaults(): ShutdownServer {
            return new Beacons.ShutdownServer({
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<ShutdownServer>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof ShutdownServer)) {
                                throw new Error(`Expecting instance of ShutdownServer on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof ShutdownServer ? undefined : new Error(`Expecting instance of ShutdownServer`);
                }};
            }
        }

        public static from(obj: any): ShutdownServer | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = ShutdownServer.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, ShutdownServer.scheme);
                return error instanceof Error ? error : new ShutdownServer({
                });
            }
        }

        public static getSignature(): string { return 'ShutdownServer'; }
        public static getId(): number { return 98; }


        constructor(params: IShutdownServer)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'ShutdownServer'; }

        public get(): ShutdownServer { return this; }

        public getId(): number { return 98; }

        public encode(): ArrayBufferLike {
            return this.collect([
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | ShutdownServer {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            return this;
        }

        public defaults(): ShutdownServer {
            return ShutdownServer.defaults();
        }
    }

    export interface IBeaconA {
    }
    export class BeaconA extends Protocol.Convertor<BeaconA> implements IBeaconA, ISigned<BeaconA> {

        public static scheme: Protocol.IPropScheme[] = [
        ];

        public static defaults(): BeaconA {
            return new Beacons.BeaconA({
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<BeaconA>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof BeaconA)) {
                                throw new Error(`Expecting instance of BeaconA on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof BeaconA ? undefined : new Error(`Expecting instance of BeaconA`);
                }};
            }
        }

        public static from(obj: any): BeaconA | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = BeaconA.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, BeaconA.scheme);
                return error instanceof Error ? error : new BeaconA({
                });
            }
        }

        public static getSignature(): string { return 'BeaconA'; }
        public static getId(): number { return 99; }


        constructor(params: IBeaconA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'BeaconA'; }

        public get(): BeaconA { return this; }

        public getId(): number { return 99; }

        public encode(): ArrayBufferLike {
            return this.collect([
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | BeaconA {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            return this;
        }

        public defaults(): BeaconA {
            return BeaconA.defaults();
        }
    }

    export interface IBeaconB {
        field: StructB;
    }
    export class BeaconB extends Protocol.Convertor<BeaconB> implements IBeaconB, ISigned<BeaconB> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'field', types: StructB.getValidator(false), optional: false },
        ];

        public static defaults(): BeaconB {
            return new Beacons.BeaconB({
                field: new StructB({
                    field_str: [],
                    field_u8: [],
                    field_u16: [],
                    field_u32: [],
                    field_u64: [],
                    field_i8: [],
                    field_i16: [],
                    field_i32: [],
                    field_i64: [],
                    field_f32: [],
                    field_f64: [],
                    field_bool: [],
                    field_struct: [],
                    field_str_empty: [],
                    field_u8_empty: [],
                    field_u16_empty: [],
                    field_u32_empty: [],
                    field_u64_empty: [],
                    field_i8_empty: [],
                    field_i16_empty: [],
                    field_i32_empty: [],
                    field_i64_empty: [],
                    field_f32_empty: [],
                    field_f64_empty: [],
                    field_bool_empty: [],
                    field_struct_empty: [],
                }),
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<BeaconB>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof BeaconB)) {
                                throw new Error(`Expecting instance of BeaconB on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof BeaconB ? undefined : new Error(`Expecting instance of BeaconB`);
                }};
            }
        }

        public static from(obj: any): BeaconB | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = BeaconB.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, BeaconB.scheme);
                return error instanceof Error ? error : new BeaconB({
                    field: obj.field,
                });
            }
        }

        public field!: StructB;
        public static getSignature(): string { return 'BeaconB'; }
        public static getId(): number { return 100; }


        constructor(params: IBeaconB)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'BeaconB'; }

        public get(): BeaconB { return this; }

        public getId(): number { return 100; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const buffer = this.field.encode(); return this.getBuffer(101, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | BeaconB {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const field: StructB = new StructB({
                field_str: [],
                field_u8: [],
                field_u16: [],
                field_u32: [],
                field_u64: [],
                field_i8: [],
                field_i16: [],
                field_i32: [],
                field_i64: [],
                field_f32: [],
                field_f64: [],
                field_bool: [],
                field_struct: [],
                field_str_empty: [],
                field_u8_empty: [],
                field_u16_empty: [],
                field_u32_empty: [],
                field_u64_empty: [],
                field_i8_empty: [],
                field_i16_empty: [],
                field_i32_empty: [],
                field_i64_empty: [],
                field_f32_empty: [],
                field_f64_empty: [],
                field_bool_empty: [],
                field_struct_empty: [],
            });
            const fieldBuf: ArrayBufferLike | undefined = storage.get(101);
            if (fieldBuf === undefined) {
                return new Error(`Fail to find field "field" (id=101).`);
            }
            const fieldErr: Error | StructB = field.decode(fieldBuf);
            if (fieldErr instanceof Error) {
                return fieldErr;
            } else {
                this.field = field;
            }
            return this;
        }

        public defaults(): BeaconB {
            return BeaconB.defaults();
        }
    }

    export namespace Sub {
        export interface IAvailableMessages {
            BeaconA?: BeaconA,
        }

        export interface IBeaconA {
            field: StructG;
        }
        export class BeaconA extends Protocol.Convertor<BeaconA> implements IBeaconA, ISigned<BeaconA> {

            public static scheme: Protocol.IPropScheme[] = [
                { prop: 'field', types: StructG.getValidator(false), optional: false },
            ];

            public static defaults(): BeaconA {
                return new Beacons.Sub.BeaconA({
                    field: new StructG({
                        field_a: new StructA({
                            field_str: '',
                            field_str_empty: '',
                            field_u8: 0,
                            field_u16: 0,
                            field_u32: 0,
                            field_u64: BigInt(0),
                            field_i8: 0,
                            field_i16: 0,
                            field_i32: 0,
                            field_i64: BigInt(0),
                            field_f32: 0,
                            field_f64: 0,
                            field_bool: true,
                        }),
                        field_b: new StructB({
                            field_str: [],
                            field_u8: [],
                            field_u16: [],
                            field_u32: [],
                            field_u64: [],
                            field_i8: [],
                            field_i16: [],
                            field_i32: [],
                            field_i64: [],
                            field_f32: [],
                            field_f64: [],
                            field_bool: [],
                            field_struct: [],
                            field_str_empty: [],
                            field_u8_empty: [],
                            field_u16_empty: [],
                            field_u32_empty: [],
                            field_u64_empty: [],
                            field_i8_empty: [],
                            field_i16_empty: [],
                            field_i32_empty: [],
                            field_i64_empty: [],
                            field_f32_empty: [],
                            field_f64_empty: [],
                            field_bool_empty: [],
                            field_struct_empty: [],
                        }),
                    }),
                });
            }

            public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
                if (array) {
                    return { validate(obj: any): Error | undefined {
                        if (!(obj instanceof Array)) {
                            return new Error(`Expecting Array<BeaconA>`);
                        }
                        try {
                            obj.forEach((o, index: number) => {
                                if (!(o instanceof BeaconA)) {
                                    throw new Error(`Expecting instance of BeaconA on index #${index}`);
                                }
                            });
                        } catch (err) {
                            return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                        }
                    }};
                } else {
                    return { validate(obj: any): Error | undefined {
                        return obj instanceof BeaconA ? undefined : new Error(`Expecting instance of BeaconA`);
                    }};
                }
            }

            public static from(obj: any): BeaconA | Error {
                if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                    const inst = BeaconA.defaults();
                    const err = inst.decode(obj);
                    return err instanceof Error ? err : inst;
                } else {
                    const error: Error | undefined = Protocol.validate(obj, BeaconA.scheme);
                    return error instanceof Error ? error : new BeaconA({
                        field: obj.field,
                    });
                }
            }

            public field!: StructG;
            public static getSignature(): string { return 'BeaconA'; }
            public static getId(): number { return 103; }


            constructor(params: IBeaconA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    (this as any)[key] = (params as any)[key];
                });
            }

            public signature(): number { return 0; }

            public getSignature(): string { return 'BeaconA'; }

            public get(): BeaconA { return this; }

            public getId(): number { return 103; }

            public encode(): ArrayBufferLike {
                return this.collect([
                    () => { const buffer = this.field.encode(); return this.getBuffer(104, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }

            public decode(buffer: ArrayBufferLike): Error | BeaconA {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const field: StructG = new StructG({
                    field_a: new StructA({
                        field_str: '',
                        field_str_empty: '',
                        field_u8: 0,
                        field_u16: 0,
                        field_u32: 0,
                        field_u64: BigInt(0),
                        field_i8: 0,
                        field_i16: 0,
                        field_i32: 0,
                        field_i64: BigInt(0),
                        field_f32: 0,
                        field_f64: 0,
                        field_bool: true,
                    }),
                    field_b: new StructB({
                        field_str: [],
                        field_u8: [],
                        field_u16: [],
                        field_u32: [],
                        field_u64: [],
                        field_i8: [],
                        field_i16: [],
                        field_i32: [],
                        field_i64: [],
                        field_f32: [],
                        field_f64: [],
                        field_bool: [],
                        field_struct: [],
                        field_str_empty: [],
                        field_u8_empty: [],
                        field_u16_empty: [],
                        field_u32_empty: [],
                        field_u64_empty: [],
                        field_i8_empty: [],
                        field_i16_empty: [],
                        field_i32_empty: [],
                        field_i64_empty: [],
                        field_f32_empty: [],
                        field_f64_empty: [],
                        field_bool_empty: [],
                        field_struct_empty: [],
                    }),
                });
                const fieldBuf: ArrayBufferLike | undefined = storage.get(104);
                if (fieldBuf === undefined) {
                    return new Error(`Fail to find field "field" (id=104).`);
                }
                const fieldErr: Error | StructG = field.decode(fieldBuf);
                if (fieldErr instanceof Error) {
                    return fieldErr;
                } else {
                    this.field = field;
                }
                return this;
            }

            public defaults(): BeaconA {
                return BeaconA.defaults();
            }
        }

    }

}

export namespace GroupA {
    export interface IAvailableMessages {
        EnumA?: IEnumA,
        StructA?: StructA,
        StructB?: StructB,
    }

    export interface IEnumA {
        Option_a?: string;
        Option_b?: string;
    }

    export class EnumA extends Protocol.Primitives.Enum<IEnumA> {
        public static from(obj: any): IEnumA | Error {
            const inst = new EnumA();
            let err: Error | undefined;
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                err = inst.decode(obj);
            } else {
                err = inst.set(obj);
            }
            return err instanceof Error ? err : inst.get();
        }
        public static getId(): number { return 106; }
        public from(obj: any): IEnumA | Error {
            return EnumA.from(obj);
        }
        public signature(): number { return 0; }
        public getId(): number { return 106; }
        public getAllowed(): string[] {
            return [
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
            ];
        }
        public getOptionValue(id: number): ISigned<any> {
            switch (id) {
                case 0: return new Protocol.Primitives.StrUTF8('');
                case 1: return new Protocol.Primitives.StrUTF8('');
                default: throw new Error(`No option with id=${id}`);
            }
        }
        public get(): IEnumA {
            const target: IEnumA = {};
            switch (this.getValueIndex()) {
                case 0: target.Option_a = this.getValue<string>(); break;
                case 1: target.Option_b = this.getValue<string>(); break;
            }
            return target;
        }
        public set(src: IEnumA): Error | undefined{
            if (Object.keys(src).length > 1) {
                return new Error(`Option cannot have more then 1 value.`);
            }
            if (src.Option_a !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(src.Option_a)));
                if (err instanceof Error) {
                    return err;
                }
            }
            if (src.Option_b !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(src.Option_b)));
                if (err instanceof Error) {
                    return err;
                }
            }
        }
    }

    export interface IStructA {
        field_u8: number;
        field_u16: number;
        opt: IEnumA;
    }
    export class StructA extends Protocol.Convertor<StructA> implements IStructA, ISigned<StructA> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
            { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
            { prop: 'opt', optional: false, options: [
                { prop: 'Option_a', types: Protocol.Primitives.StrUTF8, optional: false, },
                { prop: 'Option_b', types: Protocol.Primitives.StrUTF8, optional: false, },
            ] },
        ];

        public static defaults(): StructA {
            return new GroupA.StructA({
                field_u8: 0,
                field_u16: 0,
                opt: {},
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<StructA>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof StructA)) {
                                throw new Error(`Expecting instance of StructA on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof StructA ? undefined : new Error(`Expecting instance of StructA`);
                }};
            }
        }

        public static from(obj: any): StructA | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = StructA.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, StructA.scheme);
                return error instanceof Error ? error : new StructA({
                    field_u8: obj.field_u8,
                    field_u16: obj.field_u16,
                    opt: obj.opt,
                });
            }
        }

        public field_u8!: number;
        public field_u16!: number;
        public opt!: IEnumA;
        private _opt: Primitives.Enum;
        public static getSignature(): string { return 'StructA'; }
        public static getId(): number { return 107; }


        constructor(params: IStructA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
            this._opt = new EnumA()
            this._opt.set(this.opt);
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'StructA'; }

        public get(): StructA { return this; }

        public getId(): number { return 107; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBuffer(108, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
                () => this.getBuffer(109, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
                () => { const buffer = this._opt.encode(); return this.getBuffer(110, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | StructA {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const field_u8: number | Error = this.getValue<number>(storage, 108, Protocol.Primitives.u8.decode);
            if (field_u8 instanceof Error) {
                return field_u8;
            } else {
                this.field_u8 = field_u8;
            }
            const field_u16: number | Error = this.getValue<number>(storage, 109, Protocol.Primitives.u16.decode);
            if (field_u16 instanceof Error) {
                return field_u16;
            } else {
                this.field_u16 = field_u16;
            }
            this.opt = {};
            const optBuf: ArrayBufferLike | undefined = storage.get(110);
            if (optBuf === undefined) {
                return new Error(`Fail to get property "opt"`);
            }
            if (optBuf.byteLength > 0) {
                const optErr: Error | undefined = this._opt.decode(optBuf);
                if (optErr instanceof Error) {
                    return optErr;
                } else {
                    this.opt = this._opt.get();
                }
            }
            return this;
        }

        public defaults(): StructA {
            return StructA.defaults();
        }
    }

    export interface IStructB {
        field_u8: number;
        field_u16: number;
        strct: GroupA.StructA;
    }
    export class StructB extends Protocol.Convertor<StructB> implements IStructB, ISigned<StructB> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
            { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
            { prop: 'strct', types: GroupA.StructA.getValidator(false), optional: false },
        ];

        public static defaults(): StructB {
            return new GroupA.StructB({
                field_u8: 0,
                field_u16: 0,
                strct: new GroupA.StructA({
                    field_u8: 0,
                    field_u16: 0,
                    opt: {},
                }),
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<StructB>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof StructB)) {
                                throw new Error(`Expecting instance of StructB on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof StructB ? undefined : new Error(`Expecting instance of StructB`);
                }};
            }
        }

        public static from(obj: any): StructB | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = StructB.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, StructB.scheme);
                return error instanceof Error ? error : new StructB({
                    field_u8: obj.field_u8,
                    field_u16: obj.field_u16,
                    strct: obj.strct,
                });
            }
        }

        public field_u8!: number;
        public field_u16!: number;
        public strct!: GroupA.StructA;
        public static getSignature(): string { return 'StructB'; }
        public static getId(): number { return 111; }


        constructor(params: IStructB)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'StructB'; }

        public get(): StructB { return this; }

        public getId(): number { return 111; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBuffer(112, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
                () => this.getBuffer(113, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
                () => { const buffer = this.strct.encode(); return this.getBuffer(114, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | StructB {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const field_u8: number | Error = this.getValue<number>(storage, 112, Protocol.Primitives.u8.decode);
            if (field_u8 instanceof Error) {
                return field_u8;
            } else {
                this.field_u8 = field_u8;
            }
            const field_u16: number | Error = this.getValue<number>(storage, 113, Protocol.Primitives.u16.decode);
            if (field_u16 instanceof Error) {
                return field_u16;
            } else {
                this.field_u16 = field_u16;
            }
            const strct: StructA = new GroupA.StructA({
                field_u8: 0,
                field_u16: 0,
                opt: {},
            });
            const strctBuf: ArrayBufferLike | undefined = storage.get(114);
            if (strctBuf === undefined) {
                return new Error(`Fail to find field "strct" (id=114).`);
            }
            const strctErr: Error | StructA = strct.decode(strctBuf);
            if (strctErr instanceof Error) {
                return strctErr;
            } else {
                this.strct = strct;
            }
            return this;
        }

        public defaults(): StructB {
            return StructB.defaults();
        }
    }

}

export namespace GroupB {
    export interface IAvailableMessages {
        StructA?: StructA,
        GroupC?: GroupC.IAvailableMessages,
    }

    export interface IStructA {
        field_u8: number;
        field_u16: number;
    }
    export class StructA extends Protocol.Convertor<StructA> implements IStructA, ISigned<StructA> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
            { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
        ];

        public static defaults(): StructA {
            return new GroupB.StructA({
                field_u8: 0,
                field_u16: 0,
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<StructA>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof StructA)) {
                                throw new Error(`Expecting instance of StructA on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof StructA ? undefined : new Error(`Expecting instance of StructA`);
                }};
            }
        }

        public static from(obj: any): StructA | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = StructA.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, StructA.scheme);
                return error instanceof Error ? error : new StructA({
                    field_u8: obj.field_u8,
                    field_u16: obj.field_u16,
                });
            }
        }

        public field_u8!: number;
        public field_u16!: number;
        public static getSignature(): string { return 'StructA'; }
        public static getId(): number { return 116; }


        constructor(params: IStructA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'StructA'; }

        public get(): StructA { return this; }

        public getId(): number { return 116; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBuffer(117, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
                () => this.getBuffer(118, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | StructA {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const field_u8: number | Error = this.getValue<number>(storage, 117, Protocol.Primitives.u8.decode);
            if (field_u8 instanceof Error) {
                return field_u8;
            } else {
                this.field_u8 = field_u8;
            }
            const field_u16: number | Error = this.getValue<number>(storage, 118, Protocol.Primitives.u16.decode);
            if (field_u16 instanceof Error) {
                return field_u16;
            } else {
                this.field_u16 = field_u16;
            }
            return this;
        }

        public defaults(): StructA {
            return StructA.defaults();
        }
    }

    export namespace GroupC {
        export interface IAvailableMessages {
            StructA?: StructA,
            StructB?: StructB,
        }

        export interface IStructA {
            field_u8: number;
            field_u16: number;
        }
        export class StructA extends Protocol.Convertor<StructA> implements IStructA, ISigned<StructA> {

            public static scheme: Protocol.IPropScheme[] = [
                { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
                { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
            ];

            public static defaults(): StructA {
                return new GroupB.GroupC.StructA({
                    field_u8: 0,
                    field_u16: 0,
                });
            }

            public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
                if (array) {
                    return { validate(obj: any): Error | undefined {
                        if (!(obj instanceof Array)) {
                            return new Error(`Expecting Array<StructA>`);
                        }
                        try {
                            obj.forEach((o, index: number) => {
                                if (!(o instanceof StructA)) {
                                    throw new Error(`Expecting instance of StructA on index #${index}`);
                                }
                            });
                        } catch (err) {
                            return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                        }
                    }};
                } else {
                    return { validate(obj: any): Error | undefined {
                        return obj instanceof StructA ? undefined : new Error(`Expecting instance of StructA`);
                    }};
                }
            }

            public static from(obj: any): StructA | Error {
                if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                    const inst = StructA.defaults();
                    const err = inst.decode(obj);
                    return err instanceof Error ? err : inst;
                } else {
                    const error: Error | undefined = Protocol.validate(obj, StructA.scheme);
                    return error instanceof Error ? error : new StructA({
                        field_u8: obj.field_u8,
                        field_u16: obj.field_u16,
                    });
                }
            }

            public field_u8!: number;
            public field_u16!: number;
            public static getSignature(): string { return 'StructA'; }
            public static getId(): number { return 120; }


            constructor(params: IStructA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    (this as any)[key] = (params as any)[key];
                });
            }

            public signature(): number { return 0; }

            public getSignature(): string { return 'StructA'; }

            public get(): StructA { return this; }

            public getId(): number { return 120; }

            public encode(): ArrayBufferLike {
                return this.collect([
                    () => this.getBuffer(121, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
                    () => this.getBuffer(122, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
                ]);
            }

            public decode(buffer: ArrayBufferLike): Error | StructA {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const field_u8: number | Error = this.getValue<number>(storage, 121, Protocol.Primitives.u8.decode);
                if (field_u8 instanceof Error) {
                    return field_u8;
                } else {
                    this.field_u8 = field_u8;
                }
                const field_u16: number | Error = this.getValue<number>(storage, 122, Protocol.Primitives.u16.decode);
                if (field_u16 instanceof Error) {
                    return field_u16;
                } else {
                    this.field_u16 = field_u16;
                }
                return this;
            }

            public defaults(): StructA {
                return StructA.defaults();
            }
        }

        export interface IStructB {
            field_u8: number;
            field_u16: number;
            strct: GroupB.GroupC.StructA;
        }
        export class StructB extends Protocol.Convertor<StructB> implements IStructB, ISigned<StructB> {

            public static scheme: Protocol.IPropScheme[] = [
                { prop: 'field_u8', types: Protocol.Primitives.u8, optional: false, },
                { prop: 'field_u16', types: Protocol.Primitives.u16, optional: false, },
                { prop: 'strct', types: GroupB.GroupC.StructA.getValidator(false), optional: false },
            ];

            public static defaults(): StructB {
                return new GroupB.GroupC.StructB({
                    field_u8: 0,
                    field_u16: 0,
                    strct: new GroupB.GroupC.StructA({
                        field_u8: 0,
                        field_u16: 0,
                    }),
                });
            }

            public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
                if (array) {
                    return { validate(obj: any): Error | undefined {
                        if (!(obj instanceof Array)) {
                            return new Error(`Expecting Array<StructB>`);
                        }
                        try {
                            obj.forEach((o, index: number) => {
                                if (!(o instanceof StructB)) {
                                    throw new Error(`Expecting instance of StructB on index #${index}`);
                                }
                            });
                        } catch (err) {
                            return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                        }
                    }};
                } else {
                    return { validate(obj: any): Error | undefined {
                        return obj instanceof StructB ? undefined : new Error(`Expecting instance of StructB`);
                    }};
                }
            }

            public static from(obj: any): StructB | Error {
                if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                    const inst = StructB.defaults();
                    const err = inst.decode(obj);
                    return err instanceof Error ? err : inst;
                } else {
                    const error: Error | undefined = Protocol.validate(obj, StructB.scheme);
                    return error instanceof Error ? error : new StructB({
                        field_u8: obj.field_u8,
                        field_u16: obj.field_u16,
                        strct: obj.strct,
                    });
                }
            }

            public field_u8!: number;
            public field_u16!: number;
            public strct!: GroupB.GroupC.StructA;
            public static getSignature(): string { return 'StructB'; }
            public static getId(): number { return 123; }


            constructor(params: IStructB)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    (this as any)[key] = (params as any)[key];
                });
            }

            public signature(): number { return 0; }

            public getSignature(): string { return 'StructB'; }

            public get(): StructB { return this; }

            public getId(): number { return 123; }

            public encode(): ArrayBufferLike {
                return this.collect([
                    () => this.getBuffer(124, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.field_u8)),
                    () => this.getBuffer(125, Protocol.ESize.u8, Protocol.Primitives.u16.getSize(), Protocol.Primitives.u16.encode(this.field_u16)),
                    () => { const buffer = this.strct.encode(); return this.getBuffer(126, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }

            public decode(buffer: ArrayBufferLike): Error | StructB {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const field_u8: number | Error = this.getValue<number>(storage, 124, Protocol.Primitives.u8.decode);
                if (field_u8 instanceof Error) {
                    return field_u8;
                } else {
                    this.field_u8 = field_u8;
                }
                const field_u16: number | Error = this.getValue<number>(storage, 125, Protocol.Primitives.u16.decode);
                if (field_u16 instanceof Error) {
                    return field_u16;
                } else {
                    this.field_u16 = field_u16;
                }
                const strct: StructA = new GroupB.GroupC.StructA({
                    field_u8: 0,
                    field_u16: 0,
                });
                const strctBuf: ArrayBufferLike | undefined = storage.get(126);
                if (strctBuf === undefined) {
                    return new Error(`Fail to find field "strct" (id=126).`);
                }
                const strctErr: Error | StructA = strct.decode(strctBuf);
                if (strctErr instanceof Error) {
                    return strctErr;
                } else {
                    this.strct = strct;
                }
                return this;
            }

            public defaults(): StructB {
                return StructB.defaults();
            }
        }

    }

}

export namespace GroupD {
    export interface IAvailableMessages {
        EnumP?: IEnumP,
        StructP?: StructP,
    }

    export interface IEnumP {
        Option_a?: StructA;
        Option_b?: GroupD.StructP;
        Option_c?: GroupB.StructA;
        Option_d?: GroupB.GroupC.StructA;
    }

    export class EnumP extends Protocol.Primitives.Enum<IEnumP> {
        public static from(obj: any): IEnumP | Error {
            const inst = new EnumP();
            let err: Error | undefined;
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                err = inst.decode(obj);
            } else {
                err = inst.set(obj);
            }
            return err instanceof Error ? err : inst.get();
        }
        public static getId(): number { return 132; }
        public from(obj: any): IEnumP | Error {
            return EnumP.from(obj);
        }
        public signature(): number { return 0; }
        public getId(): number { return 132; }
        public getAllowed(): string[] {
            return [
                StructA.getSignature(),
                GroupD.StructP.getSignature(),
                GroupB.StructA.getSignature(),
                GroupB.GroupC.StructA.getSignature(),
            ];
        }
        public getOptionValue(id: number): ISigned<any> {
            switch (id) {
                case 0: return StructA.defaults();
                case 1: return GroupD.StructP.defaults();
                case 2: return GroupB.StructA.defaults();
                case 3: return GroupB.GroupC.StructA.defaults();
                default: throw new Error(`No option with id=${id}`);
            }
        }
        public get(): IEnumP {
            const target: IEnumP = {};
            switch (this.getValueIndex()) {
                case 0: target.Option_a = this.getValue<StructA>(); break;
                case 1: target.Option_b = this.getValue<GroupD.StructP>(); break;
                case 2: target.Option_c = this.getValue<GroupB.StructA>(); break;
                case 3: target.Option_d = this.getValue<GroupB.GroupC.StructA>(); break;
            }
            return target;
        }
        public set(src: IEnumP): Error | undefined{
            if (Object.keys(src).length > 1) {
                return new Error(`Option cannot have more then 1 value.`);
            }
            if (src.Option_a !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<StructA>(0, src.Option_a));
                if (err instanceof Error) {
                    return err;
                }
            }
            if (src.Option_b !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<GroupD.StructP>(1, src.Option_b));
                if (err instanceof Error) {
                    return err;
                }
            }
            if (src.Option_c !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<GroupB.StructA>(2, src.Option_c));
                if (err instanceof Error) {
                    return err;
                }
            }
            if (src.Option_d !== undefined) {
                const err: Error | undefined = this.setValue(new Protocol.Primitives.Option<GroupB.GroupC.StructA>(3, src.Option_d));
                if (err instanceof Error) {
                    return err;
                }
            }
        }
    }

    export interface IStructP {
        field_a: StructA;
        field_b: GroupB.StructA;
        field_c: GroupB.GroupC.StructA;
    }
    export class StructP extends Protocol.Convertor<StructP> implements IStructP, ISigned<StructP> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'field_a', types: StructA.getValidator(false), optional: false },
            { prop: 'field_b', types: GroupB.StructA.getValidator(false), optional: false },
            { prop: 'field_c', types: GroupB.GroupC.StructA.getValidator(false), optional: false },
        ];

        public static defaults(): StructP {
            return new GroupD.StructP({
                field_a: new StructA({
                    field_str: '',
                    field_str_empty: '',
                    field_u8: 0,
                    field_u16: 0,
                    field_u32: 0,
                    field_u64: BigInt(0),
                    field_i8: 0,
                    field_i16: 0,
                    field_i32: 0,
                    field_i64: BigInt(0),
                    field_f32: 0,
                    field_f64: 0,
                    field_bool: true,
                }),
                field_b: new GroupB.StructA({
                    field_u8: 0,
                    field_u16: 0,
                }),
                field_c: new GroupB.GroupC.StructA({
                    field_u8: 0,
                    field_u16: 0,
                }),
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<StructP>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof StructP)) {
                                throw new Error(`Expecting instance of StructP on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof StructP ? undefined : new Error(`Expecting instance of StructP`);
                }};
            }
        }

        public static from(obj: any): StructP | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = StructP.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, StructP.scheme);
                return error instanceof Error ? error : new StructP({
                    field_a: obj.field_a,
                    field_b: obj.field_b,
                    field_c: obj.field_c,
                });
            }
        }

        public field_a!: StructA;
        public field_b!: GroupB.StructA;
        public field_c!: GroupB.GroupC.StructA;
        public static getSignature(): string { return 'StructP'; }
        public static getId(): number { return 128; }


        constructor(params: IStructP)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'StructP'; }

        public get(): StructP { return this; }

        public getId(): number { return 128; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const buffer = this.field_a.encode(); return this.getBuffer(129, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                () => { const buffer = this.field_b.encode(); return this.getBuffer(130, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                () => { const buffer = this.field_c.encode(); return this.getBuffer(131, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | StructP {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const field_a: StructA = new StructA({
                field_str: '',
                field_str_empty: '',
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: BigInt(0),
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: BigInt(0),
                field_f32: 0,
                field_f64: 0,
                field_bool: true,
            });
            const field_aBuf: ArrayBufferLike | undefined = storage.get(129);
            if (field_aBuf === undefined) {
                return new Error(`Fail to find field "field_a" (id=129).`);
            }
            const field_aErr: Error | StructA = field_a.decode(field_aBuf);
            if (field_aErr instanceof Error) {
                return field_aErr;
            } else {
                this.field_a = field_a;
            }
            const field_b: GroupB.StructA = new GroupB.StructA({
                field_u8: 0,
                field_u16: 0,
            });
            const field_bBuf: ArrayBufferLike | undefined = storage.get(130);
            if (field_bBuf === undefined) {
                return new Error(`Fail to find field "field_b" (id=130).`);
            }
            const field_bErr: Error | GroupB.StructA = field_b.decode(field_bBuf);
            if (field_bErr instanceof Error) {
                return field_bErr;
            } else {
                this.field_b = field_b;
            }
            const field_c: GroupB.GroupC.StructA = new GroupB.GroupC.StructA({
                field_u8: 0,
                field_u16: 0,
            });
            const field_cBuf: ArrayBufferLike | undefined = storage.get(131);
            if (field_cBuf === undefined) {
                return new Error(`Fail to find field "field_c" (id=131).`);
            }
            const field_cErr: Error | GroupB.GroupC.StructA = field_c.decode(field_cBuf);
            if (field_cErr instanceof Error) {
                return field_cErr;
            } else {
                this.field_c = field_c;
            }
            return this;
        }

        public defaults(): StructP {
            return StructP.defaults();
        }
    }

}

export namespace Events {
    export interface IAvailableMessages {
        EventA?: EventA,
        EventB?: EventB,
        Sub?: Sub.IAvailableMessages,
    }

    export interface IEventA {
        uuid: string;
        field_a: StructA;
        field_b: StructB;
    }
    export class EventA extends Protocol.Convertor<EventA> implements IEventA, ISigned<EventA> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'field_a', types: StructA.getValidator(false), optional: false },
            { prop: 'field_b', types: StructB.getValidator(false), optional: false },
        ];

        public static defaults(): EventA {
            return new Events.EventA({
                uuid: '',
                field_a: new StructA({
                    field_str: '',
                    field_str_empty: '',
                    field_u8: 0,
                    field_u16: 0,
                    field_u32: 0,
                    field_u64: BigInt(0),
                    field_i8: 0,
                    field_i16: 0,
                    field_i32: 0,
                    field_i64: BigInt(0),
                    field_f32: 0,
                    field_f64: 0,
                    field_bool: true,
                }),
                field_b: new StructB({
                    field_str: [],
                    field_u8: [],
                    field_u16: [],
                    field_u32: [],
                    field_u64: [],
                    field_i8: [],
                    field_i16: [],
                    field_i32: [],
                    field_i64: [],
                    field_f32: [],
                    field_f64: [],
                    field_bool: [],
                    field_struct: [],
                    field_str_empty: [],
                    field_u8_empty: [],
                    field_u16_empty: [],
                    field_u32_empty: [],
                    field_u64_empty: [],
                    field_i8_empty: [],
                    field_i16_empty: [],
                    field_i32_empty: [],
                    field_i64_empty: [],
                    field_f32_empty: [],
                    field_f64_empty: [],
                    field_bool_empty: [],
                    field_struct_empty: [],
                }),
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<EventA>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof EventA)) {
                                throw new Error(`Expecting instance of EventA on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof EventA ? undefined : new Error(`Expecting instance of EventA`);
                }};
            }
        }

        public static from(obj: any): EventA | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = EventA.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, EventA.scheme);
                return error instanceof Error ? error : new EventA({
                    uuid: obj.uuid,
                    field_a: obj.field_a,
                    field_b: obj.field_b,
                });
            }
        }

        public uuid!: string;
        public field_a!: StructA;
        public field_b!: StructB;
        public static getSignature(): string { return 'EventA'; }
        public static getId(): number { return 141; }


        constructor(params: IEventA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'EventA'; }

        public get(): EventA { return this; }

        public getId(): number { return 141; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(142, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
                () => { const buffer = this.field_a.encode(); return this.getBuffer(143, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                () => { const buffer = this.field_b.encode(); return this.getBuffer(144, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | EventA {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 142, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
            const field_a: StructA = new StructA({
                field_str: '',
                field_str_empty: '',
                field_u8: 0,
                field_u16: 0,
                field_u32: 0,
                field_u64: BigInt(0),
                field_i8: 0,
                field_i16: 0,
                field_i32: 0,
                field_i64: BigInt(0),
                field_f32: 0,
                field_f64: 0,
                field_bool: true,
            });
            const field_aBuf: ArrayBufferLike | undefined = storage.get(143);
            if (field_aBuf === undefined) {
                return new Error(`Fail to find field "field_a" (id=143).`);
            }
            const field_aErr: Error | StructA = field_a.decode(field_aBuf);
            if (field_aErr instanceof Error) {
                return field_aErr;
            } else {
                this.field_a = field_a;
            }
            const field_b: StructB = new StructB({
                field_str: [],
                field_u8: [],
                field_u16: [],
                field_u32: [],
                field_u64: [],
                field_i8: [],
                field_i16: [],
                field_i32: [],
                field_i64: [],
                field_f32: [],
                field_f64: [],
                field_bool: [],
                field_struct: [],
                field_str_empty: [],
                field_u8_empty: [],
                field_u16_empty: [],
                field_u32_empty: [],
                field_u64_empty: [],
                field_i8_empty: [],
                field_i16_empty: [],
                field_i32_empty: [],
                field_i64_empty: [],
                field_f32_empty: [],
                field_f64_empty: [],
                field_bool_empty: [],
                field_struct_empty: [],
            });
            const field_bBuf: ArrayBufferLike | undefined = storage.get(144);
            if (field_bBuf === undefined) {
                return new Error(`Fail to find field "field_b" (id=144).`);
            }
            const field_bErr: Error | StructB = field_b.decode(field_bBuf);
            if (field_bErr instanceof Error) {
                return field_bErr;
            } else {
                this.field_b = field_b;
            }
            return this;
        }

        public defaults(): EventA {
            return EventA.defaults();
        }
    }

    export interface IEventB {
        uuid: string;
        field_a: GroupA.StructA;
        field_b: GroupA.StructB;
        field_c: GroupB.StructA;
    }
    export class EventB extends Protocol.Convertor<EventB> implements IEventB, ISigned<EventB> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'field_a', types: GroupA.StructA.getValidator(false), optional: false },
            { prop: 'field_b', types: GroupA.StructB.getValidator(false), optional: false },
            { prop: 'field_c', types: GroupB.StructA.getValidator(false), optional: false },
        ];

        public static defaults(): EventB {
            return new Events.EventB({
                uuid: '',
                field_a: new GroupA.StructA({
                    field_u8: 0,
                    field_u16: 0,
                    opt: {},
                }),
                field_b: new GroupA.StructB({
                    field_u8: 0,
                    field_u16: 0,
                    strct: new GroupA.StructA({
                        field_u8: 0,
                        field_u16: 0,
                        opt: {},
                    }),
                }),
                field_c: new GroupB.StructA({
                    field_u8: 0,
                    field_u16: 0,
                }),
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<EventB>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof EventB)) {
                                throw new Error(`Expecting instance of EventB on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof EventB ? undefined : new Error(`Expecting instance of EventB`);
                }};
            }
        }

        public static from(obj: any): EventB | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = EventB.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, EventB.scheme);
                return error instanceof Error ? error : new EventB({
                    uuid: obj.uuid,
                    field_a: obj.field_a,
                    field_b: obj.field_b,
                    field_c: obj.field_c,
                });
            }
        }

        public uuid!: string;
        public field_a!: GroupA.StructA;
        public field_b!: GroupA.StructB;
        public field_c!: GroupB.StructA;
        public static getSignature(): string { return 'EventB'; }
        public static getId(): number { return 145; }


        constructor(params: IEventB)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'EventB'; }

        public get(): EventB { return this; }

        public getId(): number { return 145; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(146, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
                () => { const buffer = this.field_a.encode(); return this.getBuffer(147, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                () => { const buffer = this.field_b.encode(); return this.getBuffer(148, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                () => { const buffer = this.field_c.encode(); return this.getBuffer(149, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | EventB {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 146, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
            const field_a: GroupA.StructA = new GroupA.StructA({
                field_u8: 0,
                field_u16: 0,
                opt: {},
            });
            const field_aBuf: ArrayBufferLike | undefined = storage.get(147);
            if (field_aBuf === undefined) {
                return new Error(`Fail to find field "field_a" (id=147).`);
            }
            const field_aErr: Error | GroupA.StructA = field_a.decode(field_aBuf);
            if (field_aErr instanceof Error) {
                return field_aErr;
            } else {
                this.field_a = field_a;
            }
            const field_b: GroupA.StructB = new GroupA.StructB({
                field_u8: 0,
                field_u16: 0,
                strct: new GroupA.StructA({
                    field_u8: 0,
                    field_u16: 0,
                    opt: {},
                }),
            });
            const field_bBuf: ArrayBufferLike | undefined = storage.get(148);
            if (field_bBuf === undefined) {
                return new Error(`Fail to find field "field_b" (id=148).`);
            }
            const field_bErr: Error | GroupA.StructB = field_b.decode(field_bBuf);
            if (field_bErr instanceof Error) {
                return field_bErr;
            } else {
                this.field_b = field_b;
            }
            const field_c: GroupB.StructA = new GroupB.StructA({
                field_u8: 0,
                field_u16: 0,
            });
            const field_cBuf: ArrayBufferLike | undefined = storage.get(149);
            if (field_cBuf === undefined) {
                return new Error(`Fail to find field "field_c" (id=149).`);
            }
            const field_cErr: Error | GroupB.StructA = field_c.decode(field_cBuf);
            if (field_cErr instanceof Error) {
                return field_cErr;
            } else {
                this.field_c = field_c;
            }
            return this;
        }

        public defaults(): EventB {
            return EventB.defaults();
        }
    }

    export namespace Sub {
        export interface IAvailableMessages {
            EventA?: EventA,
        }

        export interface IEventA {
            uuid: string;
            field_a: GroupB.GroupC.StructA;
            field_b: GroupB.GroupC.StructB;
        }
        export class EventA extends Protocol.Convertor<EventA> implements IEventA, ISigned<EventA> {

            public static scheme: Protocol.IPropScheme[] = [
                { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
                { prop: 'field_a', types: GroupB.GroupC.StructA.getValidator(false), optional: false },
                { prop: 'field_b', types: GroupB.GroupC.StructB.getValidator(false), optional: false },
            ];

            public static defaults(): EventA {
                return new Events.Sub.EventA({
                    uuid: '',
                    field_a: new GroupB.GroupC.StructA({
                        field_u8: 0,
                        field_u16: 0,
                    }),
                    field_b: new GroupB.GroupC.StructB({
                        field_u8: 0,
                        field_u16: 0,
                        strct: new GroupB.GroupC.StructA({
                            field_u8: 0,
                            field_u16: 0,
                        }),
                    }),
                });
            }

            public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
                if (array) {
                    return { validate(obj: any): Error | undefined {
                        if (!(obj instanceof Array)) {
                            return new Error(`Expecting Array<EventA>`);
                        }
                        try {
                            obj.forEach((o, index: number) => {
                                if (!(o instanceof EventA)) {
                                    throw new Error(`Expecting instance of EventA on index #${index}`);
                                }
                            });
                        } catch (err) {
                            return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                        }
                    }};
                } else {
                    return { validate(obj: any): Error | undefined {
                        return obj instanceof EventA ? undefined : new Error(`Expecting instance of EventA`);
                    }};
                }
            }

            public static from(obj: any): EventA | Error {
                if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                    const inst = EventA.defaults();
                    const err = inst.decode(obj);
                    return err instanceof Error ? err : inst;
                } else {
                    const error: Error | undefined = Protocol.validate(obj, EventA.scheme);
                    return error instanceof Error ? error : new EventA({
                        uuid: obj.uuid,
                        field_a: obj.field_a,
                        field_b: obj.field_b,
                    });
                }
            }

            public uuid!: string;
            public field_a!: GroupB.GroupC.StructA;
            public field_b!: GroupB.GroupC.StructB;
            public static getSignature(): string { return 'EventA'; }
            public static getId(): number { return 151; }


            constructor(params: IEventA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    (this as any)[key] = (params as any)[key];
                });
            }

            public signature(): number { return 0; }

            public getSignature(): string { return 'EventA'; }

            public get(): EventA { return this; }

            public getId(): number { return 151; }

            public encode(): ArrayBufferLike {
                return this.collect([
                    () => this.getBufferFromBuf<string>(152, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
                    () => { const buffer = this.field_a.encode(); return this.getBuffer(153, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                    () => { const buffer = this.field_b.encode(); return this.getBuffer(154, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }

            public decode(buffer: ArrayBufferLike): Error | EventA {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const uuid: string | Error = this.getValue<string>(storage, 152, Protocol.Primitives.StrUTF8.decode);
                if (uuid instanceof Error) {
                    return uuid;
                } else {
                    this.uuid = uuid;
                }
                const field_a: GroupB.GroupC.StructA = new GroupB.GroupC.StructA({
                    field_u8: 0,
                    field_u16: 0,
                });
                const field_aBuf: ArrayBufferLike | undefined = storage.get(153);
                if (field_aBuf === undefined) {
                    return new Error(`Fail to find field "field_a" (id=153).`);
                }
                const field_aErr: Error | GroupB.GroupC.StructA = field_a.decode(field_aBuf);
                if (field_aErr instanceof Error) {
                    return field_aErr;
                } else {
                    this.field_a = field_a;
                }
                const field_b: GroupB.GroupC.StructB = new GroupB.GroupC.StructB({
                    field_u8: 0,
                    field_u16: 0,
                    strct: new GroupB.GroupC.StructA({
                        field_u8: 0,
                        field_u16: 0,
                    }),
                });
                const field_bBuf: ArrayBufferLike | undefined = storage.get(154);
                if (field_bBuf === undefined) {
                    return new Error(`Fail to find field "field_b" (id=154).`);
                }
                const field_bErr: Error | GroupB.GroupC.StructB = field_b.decode(field_bBuf);
                if (field_bErr instanceof Error) {
                    return field_bErr;
                } else {
                    this.field_b = field_b;
                }
                return this;
            }

            public defaults(): EventA {
                return EventA.defaults();
            }
        }

    }

}

export namespace InternalServiceGroup {
    export interface IAvailableMessages {
        SelfKeyResponse?: SelfKeyResponse,
        HashRequest?: HashRequest,
        HashResponse?: HashResponse,
        BeaconConfirmation?: BeaconConfirmation,
        ConnectConfirmationBeacon?: ConnectConfirmationBeacon,
    }

    export interface ISelfKeyResponse {
        uuid: string;
    }
    export class SelfKeyResponse extends Protocol.Convertor<SelfKeyResponse> implements ISelfKeyResponse, ISigned<SelfKeyResponse> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'uuid', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): SelfKeyResponse {
            return new InternalServiceGroup.SelfKeyResponse({
                uuid: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<SelfKeyResponse>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof SelfKeyResponse)) {
                                throw new Error(`Expecting instance of SelfKeyResponse on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof SelfKeyResponse ? undefined : new Error(`Expecting instance of SelfKeyResponse`);
                }};
            }
        }

        public static from(obj: any): SelfKeyResponse | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = SelfKeyResponse.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, SelfKeyResponse.scheme);
                return error instanceof Error ? error : new SelfKeyResponse({
                    uuid: obj.uuid,
                });
            }
        }

        public uuid!: string;
        public static getSignature(): string { return 'SelfKeyResponse'; }
        public static getId(): number { return 156; }


        constructor(params: ISelfKeyResponse)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'SelfKeyResponse'; }

        public get(): SelfKeyResponse { return this; }

        public getId(): number { return 156; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(157, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.uuid),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | SelfKeyResponse {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const uuid: string | Error = this.getValue<string>(storage, 157, Protocol.Primitives.StrUTF8.decode);
            if (uuid instanceof Error) {
                return uuid;
            } else {
                this.uuid = uuid;
            }
            return this;
        }

        public defaults(): SelfKeyResponse {
            return SelfKeyResponse.defaults();
        }
    }

    export interface IHashRequest {
        protocol: string;
        workflow: string;
    }
    export class HashRequest extends Protocol.Convertor<HashRequest> implements IHashRequest, ISigned<HashRequest> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'protocol', types: Protocol.Primitives.StrUTF8, optional: false, },
            { prop: 'workflow', types: Protocol.Primitives.StrUTF8, optional: false, },
        ];

        public static defaults(): HashRequest {
            return new InternalServiceGroup.HashRequest({
                protocol: '',
                workflow: '',
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<HashRequest>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof HashRequest)) {
                                throw new Error(`Expecting instance of HashRequest on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof HashRequest ? undefined : new Error(`Expecting instance of HashRequest`);
                }};
            }
        }

        public static from(obj: any): HashRequest | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = HashRequest.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, HashRequest.scheme);
                return error instanceof Error ? error : new HashRequest({
                    protocol: obj.protocol,
                    workflow: obj.workflow,
                });
            }
        }

        public protocol!: string;
        public workflow!: string;
        public static getSignature(): string { return 'HashRequest'; }
        public static getId(): number { return 158; }


        constructor(params: IHashRequest)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'HashRequest'; }

        public get(): HashRequest { return this; }

        public getId(): number { return 158; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<string>(159, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.protocol),
                () => this.getBufferFromBuf<string>(160, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.workflow),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | HashRequest {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const protocol: string | Error = this.getValue<string>(storage, 159, Protocol.Primitives.StrUTF8.decode);
            if (protocol instanceof Error) {
                return protocol;
            } else {
                this.protocol = protocol;
            }
            const workflow: string | Error = this.getValue<string>(storage, 160, Protocol.Primitives.StrUTF8.decode);
            if (workflow instanceof Error) {
                return workflow;
            } else {
                this.workflow = workflow;
            }
            return this;
        }

        public defaults(): HashRequest {
            return HashRequest.defaults();
        }
    }

    export interface IHashResponse {
        error: string | undefined;
    }
    export class HashResponse extends Protocol.Convertor<HashResponse> implements IHashResponse, ISigned<HashResponse> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'error', types: Protocol.Primitives.StrUTF8, optional: true, },
        ];

        public static defaults(): HashResponse {
            return new InternalServiceGroup.HashResponse({
                error: undefined,
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<HashResponse>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof HashResponse)) {
                                throw new Error(`Expecting instance of HashResponse on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof HashResponse ? undefined : new Error(`Expecting instance of HashResponse`);
                }};
            }
        }

        public static from(obj: any): HashResponse | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = HashResponse.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, HashResponse.scheme);
                return error instanceof Error ? error : new HashResponse({
                    error: obj.error,
                });
            }
        }

        public error!: string | undefined;
        public static getSignature(): string { return 'HashResponse'; }
        public static getId(): number { return 161; }


        constructor(params: IHashResponse)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'HashResponse'; }

        public get(): HashResponse { return this; }

        public getId(): number { return 161; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.error === undefined ? this.getBuffer(162, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(162, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.error),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | HashResponse {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const errorBuf: ArrayBufferLike | undefined = storage.get(162);
            if (errorBuf === undefined) {
                return new Error(`Fail to get property error (id=162)`);
            }
            if (errorBuf.byteLength === 0) {
                this.error = undefined;
            } else {
                const error: string | Error = this.getValue<string>(storage, 162, Protocol.Primitives.StrUTF8.decode);
                if (error instanceof Error) {
                    return error;
                } else {
                    this.error = error;
                }
            }
            return this;
        }

        public defaults(): HashResponse {
            return HashResponse.defaults();
        }
    }

    export interface IBeaconConfirmation {
        error: string | undefined;
    }
    export class BeaconConfirmation extends Protocol.Convertor<BeaconConfirmation> implements IBeaconConfirmation, ISigned<BeaconConfirmation> {

        public static scheme: Protocol.IPropScheme[] = [
            { prop: 'error', types: Protocol.Primitives.StrUTF8, optional: true, },
        ];

        public static defaults(): BeaconConfirmation {
            return new InternalServiceGroup.BeaconConfirmation({
                error: undefined,
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<BeaconConfirmation>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof BeaconConfirmation)) {
                                throw new Error(`Expecting instance of BeaconConfirmation on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof BeaconConfirmation ? undefined : new Error(`Expecting instance of BeaconConfirmation`);
                }};
            }
        }

        public static from(obj: any): BeaconConfirmation | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = BeaconConfirmation.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, BeaconConfirmation.scheme);
                return error instanceof Error ? error : new BeaconConfirmation({
                    error: obj.error,
                });
            }
        }

        public error!: string | undefined;
        public static getSignature(): string { return 'BeaconConfirmation'; }
        public static getId(): number { return 163; }


        constructor(params: IBeaconConfirmation)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'BeaconConfirmation'; }

        public get(): BeaconConfirmation { return this; }

        public getId(): number { return 163; }

        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.error === undefined ? this.getBuffer(164, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(164, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.error),
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | BeaconConfirmation {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const errorBuf: ArrayBufferLike | undefined = storage.get(164);
            if (errorBuf === undefined) {
                return new Error(`Fail to get property error (id=164)`);
            }
            if (errorBuf.byteLength === 0) {
                this.error = undefined;
            } else {
                const error: string | Error = this.getValue<string>(storage, 164, Protocol.Primitives.StrUTF8.decode);
                if (error instanceof Error) {
                    return error;
                } else {
                    this.error = error;
                }
            }
            return this;
        }

        public defaults(): BeaconConfirmation {
            return BeaconConfirmation.defaults();
        }
    }

    export interface IConnectConfirmationBeacon {
    }
    export class ConnectConfirmationBeacon extends Protocol.Convertor<ConnectConfirmationBeacon> implements IConnectConfirmationBeacon, ISigned<ConnectConfirmationBeacon> {

        public static scheme: Protocol.IPropScheme[] = [
        ];

        public static defaults(): ConnectConfirmationBeacon {
            return new InternalServiceGroup.ConnectConfirmationBeacon({
            });
        }

        public static getValidator(array: boolean): { validate(value: any): Error | undefined } {
            if (array) {
                return { validate(obj: any): Error | undefined {
                    if (!(obj instanceof Array)) {
                        return new Error(`Expecting Array<ConnectConfirmationBeacon>`);
                    }
                    try {
                        obj.forEach((o, index: number) => {
                            if (!(o instanceof ConnectConfirmationBeacon)) {
                                throw new Error(`Expecting instance of ConnectConfirmationBeacon on index #${index}`);
                            }
                        });
                    } catch (err) {
                        return err instanceof Error ? err : new Error(`Unknown error: ${err}`);
                    }
                }};
            } else {
                return { validate(obj: any): Error | undefined {
                    return obj instanceof ConnectConfirmationBeacon ? undefined : new Error(`Expecting instance of ConnectConfirmationBeacon`);
                }};
            }
        }

        public static from(obj: any): ConnectConfirmationBeacon | Error {
            if (obj instanceof Buffer || obj instanceof ArrayBuffer || obj instanceof Uint8Array) {
                const inst = ConnectConfirmationBeacon.defaults();
                const err = inst.decode(obj);
                return err instanceof Error ? err : inst;
            } else {
                const error: Error | undefined = Protocol.validate(obj, ConnectConfirmationBeacon.scheme);
                return error instanceof Error ? error : new ConnectConfirmationBeacon({
                });
            }
        }

        public static getSignature(): string { return 'ConnectConfirmationBeacon'; }
        public static getId(): number { return 165; }


        constructor(params: IConnectConfirmationBeacon)  {
            super();
            Object.keys(params).forEach((key: string) => {
                (this as any)[key] = (params as any)[key];
            });
        }

        public signature(): number { return 0; }

        public getSignature(): string { return 'ConnectConfirmationBeacon'; }

        public get(): ConnectConfirmationBeacon { return this; }

        public getId(): number { return 165; }

        public encode(): ArrayBufferLike {
            return this.collect([
            ]);
        }

        public decode(buffer: ArrayBufferLike): Error | ConnectConfirmationBeacon {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            return this;
        }

        public defaults(): ConnectConfirmationBeacon {
            return ConnectConfirmationBeacon.defaults();
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
            case 1:
                instance = new EnumA();
                err = instance.decode(buffer);
                if (err instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { EnumA: instance }, getRef: () => instance };
            case 2:
                instance = new EnumB();
                err = instance.decode(buffer);
                if (err instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { EnumB: instance }, getRef: () => instance };
            case 3:
                instance = new EnumC();
                err = instance.decode(buffer);
                if (err instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { EnumC: instance }, getRef: () => instance };
            case 106:
                instance = new GroupA.EnumA();
                err = instance.decode(buffer);
                if (err instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupA: { EnumA: instance } }, getRef: () => instance };
            case 132:
                instance = new GroupD.EnumP();
                err = instance.decode(buffer);
                if (err instanceof Error) { return err; }
                enum_instance = instance.get();
                instance = enum_instance;
                return { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupD: { EnumP: instance } }, getRef: () => instance };
            case 4:
                instance = StructA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructA: instance }, getRef: () => instance };
            case 18:
                instance = StructB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructB: instance }, getRef: () => instance };
            case 45:
                instance = StructC.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructC: instance }, getRef: () => instance };
            case 58:
                instance = StructD.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructD: instance }, getRef: () => instance };
            case 71:
                instance = StructE.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructE: instance }, getRef: () => instance };
            case 75:
                instance = StructF.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructF: instance }, getRef: () => instance };
            case 79:
                instance = StructG.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructG: instance }, getRef: () => instance };
            case 82:
                instance = TriggerBeaconsEmitter.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { TriggerBeaconsEmitter: instance }, getRef: () => instance };
            case 84:
                instance = StructEmpty.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructEmpty: instance }, getRef: () => instance };
            case 85:
                instance = StructEmptyA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructEmptyA: instance }, getRef: () => instance };
            case 86:
                instance = StructEmptyB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructEmptyB: instance }, getRef: () => instance };
            case 87:
                instance = StructJ.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { StructJ: instance }, getRef: () => instance };
            case 91:
                instance = TriggerBeacons.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { TriggerBeacons: instance }, getRef: () => instance };
            case 92:
                instance = FinishConsumerTest.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { FinishConsumerTest: instance }, getRef: () => instance };
            case 94:
                instance = FinishConsumerTestBroadcast.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { FinishConsumerTestBroadcast: instance }, getRef: () => instance };
            case 95:
                instance = BeaconA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { BeaconA: instance }, getRef: () => instance };
            case 98:
                instance = Beacons.ShutdownServer.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Beacons: { ShutdownServer: instance } }, getRef: () => instance };
            case 99:
                instance = Beacons.BeaconA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Beacons: { BeaconA: instance } }, getRef: () => instance };
            case 100:
                instance = Beacons.BeaconB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Beacons: { BeaconB: instance } }, getRef: () => instance };
            case 103:
                instance = Beacons.Sub.BeaconA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Beacons: { Sub: { BeaconA: instance } } }, getRef: () => instance };
            case 107:
                instance = GroupA.StructA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupA: { StructA: instance } }, getRef: () => instance };
            case 111:
                instance = GroupA.StructB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupA: { StructB: instance } }, getRef: () => instance };
            case 116:
                instance = GroupB.StructA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupB: { StructA: instance } }, getRef: () => instance };
            case 120:
                instance = GroupB.GroupC.StructA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupB: { GroupC: { StructA: instance } } }, getRef: () => instance };
            case 123:
                instance = GroupB.GroupC.StructB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupB: { GroupC: { StructB: instance } } }, getRef: () => instance };
            case 128:
                instance = GroupD.StructP.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { GroupD: { StructP: instance } }, getRef: () => instance };
            case 133:
                instance = EventA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { EventA: instance }, getRef: () => instance };
            case 137:
                instance = EventB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { EventB: instance }, getRef: () => instance };
            case 141:
                instance = Events.EventA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Events: { EventA: instance } }, getRef: () => instance };
            case 145:
                instance = Events.EventB.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Events: { EventB: instance } }, getRef: () => instance };
            case 151:
                instance = Events.Sub.EventA.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { Events: { Sub: { EventA: instance } } }, getRef: () => instance };
            case 156:
                instance = InternalServiceGroup.SelfKeyResponse.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { InternalServiceGroup: { SelfKeyResponse: instance } }, getRef: () => instance };
            case 158:
                instance = InternalServiceGroup.HashRequest.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { InternalServiceGroup: { HashRequest: instance } }, getRef: () => instance };
            case 161:
                instance = InternalServiceGroup.HashResponse.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { InternalServiceGroup: { HashResponse: instance } }, getRef: () => instance };
            case 163:
                instance = InternalServiceGroup.BeaconConfirmation.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { InternalServiceGroup: { BeaconConfirmation: instance } }, getRef: () => instance };
            case 165:
                instance = InternalServiceGroup.ConnectConfirmationBeacon.defaults();
                err = instance.decode(buffer);
                return err instanceof Error ? err : { header: { id: header.id, sequence: header.sequence, timestamp: header.ts }, msg: { InternalServiceGroup: { ConnectConfirmationBeacon: instance } }, getRef: () => instance };
            default: throw new Error(`Unknown message id=${header.id}`);
        }
    }
}

export function hash(): string { return `2FE9D6137375F6B74B81143B6CA65EEAE6124B6C03C78937C4583DF0B0EF757A`; }
