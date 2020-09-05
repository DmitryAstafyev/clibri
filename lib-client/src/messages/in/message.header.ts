import { MessageBufferReader } from './message.buffer';

export class MessageHeader extends MessageBufferReader {

    public static readonly ID_LENGTH = 4;
    public static readonly TS_LENGTH = 8;
    public static readonly LEN_LENGTH = 4;

    public readonly ID_LENGTH = 4;
    public readonly TS_LENGTH = 8;
    public readonly LEN_LENGTH = 4;

    public id: number | undefined;
    public ts: number | undefined;
    public len: number | undefined;

    constructor(buffer: Buffer) {
        super(buffer, true);
        if (MessageHeader.enow(this._buffer) === false) {
            throw new Error(`Cannot parse header because size problem. Buffer: ${this._buffer.byteLength} bytes; header size: ${this.size()} bytes`);
        } else {
            this.id = super.readInt32();
            this.ts = super.readInt(8);
            this.len = super.readInt32();
        }
    }

    public static enow(buffer: Buffer): boolean {
        return buffer.byteLength >= (MessageHeader.ID_LENGTH + MessageHeader.TS_LENGTH + MessageHeader.LEN_LENGTH);
    }

    public read(): void {
        this.id = super.readInt32();
        this.ts = super.readInt(8);
        this.len = super.readInt32();
    }

    public getId(): number {
        return this.id;
    }

    public getTimestamp(): number {
        return this.ts;
    }

    public getLen(): number {
        return this.len;
    }

    public size(): number {
        return this.ID_LENGTH + this.TS_LENGTH + this.LEN_LENGTH;
    }

    public crop(): Buffer {
        return this._buffer.slice(this.size(), this._buffer.length);
    }

}