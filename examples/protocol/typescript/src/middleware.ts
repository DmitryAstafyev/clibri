import * as Protocol from "./protocol";

export class Middleware extends Protocol.PackingMiddleware {
	public decode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		// Do some manipulations with buffer. For example compress.
		return buffer;
	}

	public encode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		// Do some manipulations with buffer. For example decompress.
		return buffer;
	}
}

new Middleware();
