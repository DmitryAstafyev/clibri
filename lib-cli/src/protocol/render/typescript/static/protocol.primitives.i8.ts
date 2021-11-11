// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from "./protocol.primitives.interface";
import { CBits } from "./protocol.sizes";

// injectable
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
