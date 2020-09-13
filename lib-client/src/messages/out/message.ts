import * as Tools from '../../tools';

export interface OutgoingMessage {
    encode(): ArrayBufferLike;
}

export abstract class Message<T> implements OutgoingMessage {

    public readonly abstract id: number;

    private readonly message: T;

    constructor(message: T) {
        this.message = message;
    }

    public getMessage(): T {
        return this.message;
    }

    public encode(): ArrayBufferLike {
        const encoder: TextEncoder = new TextEncoder();
        const buffer: Uint8Array = encoder.encode(JSON.stringify(this.getMessage()));
        const id: Uint32Array = Uint32Array.from([this.id]);
        const ts: BigInt64Array = BigInt64Array.from([BigInt((new Date()).getTime())]);
        const len: Uint32Array = Uint32Array.from([buffer.buffer.byteLength]);
        return Tools.append([id.buffer, ts.buffer, len.buffer, buffer.buffer]);
    }

}