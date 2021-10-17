// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import * as Tools from "./tools/index";

import { StrUTF8 } from "./protocol.primitives.string.utf8";
import { u32 } from "./protocol.primitives.u32";
import { Primitive } from "./protocol.primitives.interface";

// injectable
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
