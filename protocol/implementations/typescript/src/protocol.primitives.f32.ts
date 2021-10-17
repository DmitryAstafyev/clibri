// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import { Primitive } from "./protocol.primitives.interface";
import { CBits } from "./protocol.sizes";

// injectable
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
