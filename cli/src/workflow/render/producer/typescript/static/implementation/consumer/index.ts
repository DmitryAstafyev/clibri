import { Options, Logger } from "clibri";
import { Identification } from "./identification";
import * as Protocol from "../protocol";

export { Identification } from "./identification";
export { Filter } from "./filter";

export class Consumer {
	private readonly _options: Options;
	private readonly _identification: Identification;
	private readonly _buffer: Protocol.BufferReaderMessages =
		new Protocol.BufferReaderMessages();
	private _logger: Logger;
	private _hash: boolean = false;

	constructor(uuid: string, options: Options, logger: Logger) {
		this._logger = logger.clone(`[${uuid}][Consumer]`);
		this._options = options;
		this._identification = new Identification(
			uuid,
			options.producerIndentificationStrategy,
			logger
		);
	}

	public getIdentification(): Identification {
		return this._identification;
	}

	public chunk(buffer: ArrayBufferLike): Error | undefined {
		const errors = this._buffer.chunk(buffer);
		if (errors !== undefined) {
			return new Error(errors.map((err) => err.message).join("\n"));
		}
		return undefined;
	}

	public message():
		| Protocol.IAvailableMessage<Protocol.IAvailableMessages>
		| undefined {
		return this._buffer.next();
	}

	public acceptHash() {
		this._hash = true;
		this._logger.debug(`hash has been accepted`);
	}

	public isHashAccepted(): boolean {
		return this._hash;
	}
}
