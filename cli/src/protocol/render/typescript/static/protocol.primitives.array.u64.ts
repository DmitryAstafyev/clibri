// tslint:disable: class-name
// tslint:disable: max-classes-per-file

import { u64 } from "./protocol.primitives.u64";
import { Primitive } from "./protocol.primitives.interface";

// injectable
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
