import { Buffer } from "buffer";
import { MessageHeader } from "./packing.header";
import { getPackingMiddleware, PackingMiddleware } from "./packing.middleware";
// injectable

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
