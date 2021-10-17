// tslint:disable: class-name
// tslint:disable: max-classes-per-file
import * as Tools from "./tools/index";
import * as Primitives from "./protocol.primitives";

import { ISigned } from "./protocol.primitives.interface";
import { u16 } from "./protocol.primitives.u16";
import { getPackingMiddleware, PackingMiddleware } from "./packing.middleware";

// injectable
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
