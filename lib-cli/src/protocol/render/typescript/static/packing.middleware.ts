import { Buffer } from "buffer";
import { MessageHeader } from "./packing.header";

// injectable
declare var window: Window | undefined;
declare var global: any | undefined;

export function globals(): Window | any | Error {
	if (typeof window === "object" && window !== null) {
		return window;
	} else if (typeof global === "object" && global !== null) {
		return global;
	} else {
		return new Error(`Fail to find global namespece ()`);
	}
}

export function getPackingMiddleware(): PackingMiddleware | undefined {
	const space = globals();
	if (space instanceof Error) {
		return undefined;
	}
	return space[PackingMiddleware.GUID];
}

export abstract class PackingMiddleware {
	static GUID: string = "___FiberPackingMiddleware___";

	constructor() {
		const space = globals();
		if (space instanceof Error) {
			console.error(
				`Fail to bind PackingMiddleware as soon as fail to find global object (window or NodeJS global)`
			);
			return;
		}
		if (space[PackingMiddleware.GUID] !== undefined) {
			console.warn(`PackingMiddleware instance is overwritten.`);
		}
		space[PackingMiddleware.GUID] = this;
	}

	public decode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		return buffer;
	}

	public encode(
		buffer: ArrayBufferLike,
		id: number,
		sequence: number,
		uuid?: string
	): ArrayBufferLike | Error {
		return buffer;
	}
}
