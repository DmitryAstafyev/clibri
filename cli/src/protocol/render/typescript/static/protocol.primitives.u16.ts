// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from "./protocol.primitives.interface";
import { CBits } from "./protocol.sizes";

// injectable
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
