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
        const ts: ArrayBufferLike = this._bigIntToBuff(BigInt((new Date()).getTime()));
        const len: Uint32Array = Uint32Array.from([buffer.buffer.byteLength]);
        return Tools.append([id.buffer, ts, len.buffer, buffer.buffer]);
    }

    private _bigIntToBuff(int: bigint): ArrayBufferLike {
        if (window !== undefined && (window as any).BigInt64Array !== undefined) {
            return (window as any).BigInt64Array.from([int]).buffer;
        } else {
            let hex: string = BigInt(int).toString(16);
            if (hex.length % 2) { hex = '0' + hex; }
            const len = hex.length / 2;
            const u8 = new Uint8Array(len);
            let i = 0;
            let j = 0;
            while (i < len) {
                u8[i] = parseInt(hex.slice(j, j+2), 16);
                i += 1;
                j += 2;
            }
            return u8;
        }
    }

}