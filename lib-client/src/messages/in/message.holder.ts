import { MessageBufferReader } from './message.buffer';
import { MessageHeader } from './message.header';

export interface IStruct {
    [key: string]: any;
}

export interface IMessage {
    id: number;
    timestamp: number;
    struct: IStruct;
}

export class MessageHolder {

    private _header: MessageHeader;
    private _buffer: Buffer;
    private _struct: IStruct;

    constructor(buffer: Buffer, MSBF: boolean = false) {
        this._buffer = buffer;
        this._header = new MessageHeader(this._buffer);
        if (this._header.getLen() > this._buffer.byteLength - this._header.size()) {
            throw new Error(`Fail to get a payload because size problem. Buffer (without header): ${this._buffer.byteLength - this._header.size()} bytes; payload size: ${this._header.getLen()} bytes`);
        }
        const struct: IStruct | Error = this._getStruct();
        if (struct instanceof Error) {
            throw struct;
        }
        this._struct = struct;
    }

    public static enow(buffer: Buffer): boolean {
        if (!MessageHeader.enow(buffer)) {
            return false;
        }
        const header: MessageHeader = new MessageHeader(buffer);
        if (header.getLen() > buffer.byteLength - header.size()) {
            return false;
        }
        return true;
    }

    public getId(): number {
        return this._header.getId();
    }

    public getTimestamp(): number {
        return this._header.getTimestamp();
    }

    public getStruct(): IStruct {
        return this._struct;
    }

    public getMessage(): IMessage {
        return {
            id: this.getId(),
            timestamp: this.getTimestamp(),
            struct: this.getStruct(),
        };
    }

    public crop() {
        const cropped = this._header.crop();
        return cropped.slice(this._header.getLen(), this._buffer.length);
    }

    private _getStruct(): IStruct | Error {
        const cropped = this._header.crop();
        const payload: Uint8Array = new Uint8Array(this._header.getLen());
        cropped.copy(payload, 0, 0, this._header.getLen());
        try {
            return JSON.parse(Buffer.from(payload).toString());
        } catch (err) {
            return new Error(`Fail convert buffer to JSON due error: ${err.message}`);
        }
    }

}