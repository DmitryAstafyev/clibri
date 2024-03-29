// tslint:disable: class-name
// tslint:disable: max-classes-per-file

import { u32 } from "./protocol.primitives.u32";
import { Primitive } from "./protocol.primitives.interface";

// injectable
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
