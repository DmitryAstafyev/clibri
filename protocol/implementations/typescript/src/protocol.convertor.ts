import * as Primitives from './protocol.primitives';
import * as Tools from './tools/index';

import { ESize, CBits } from './protocol.sizes';
import { Storage } from './protocol.convertor.storage';
import { u32 } from './protocol.primitives.u32';
import { u64 } from './protocol.primitives.u64';

// injectable
export abstract class Convertor {

    public collect(getters: Array<() => ArrayBufferLike | Error>): ArrayBufferLike {
        const buffers: ArrayBufferLike[] = [];
        try {
            getters.forEach((getter: () => ArrayBufferLike | Error) => {
                const buf: ArrayBufferLike | Error = getter();
                if (buf instanceof Error) {
                    throw buf;
                }
                buffers.push(buf);
            });
        } catch (e) {
            return e;
        }
        return Tools.append(buffers);
    }

    public getBuffer(id: number, esize: ESize, size: number | bigint, value: ArrayBufferLike | Error): ArrayBufferLike | Error {
        if (value instanceof Error) {
            return value;
        }
        const idBuf: ArrayBufferLike | Error = Primitives.u16.encode(id);
        if (idBuf instanceof Error) {
            return idBuf;
        }
        let sizeType: ArrayBufferLike | Error;
        let sizeValue: ArrayBufferLike | Error;
        if (esize === ESize.u64 && typeof size !== 'bigint') {
            return new Error(`For size ${ESize.u64}, size should be defined as BigInt`);
        } else if ((esize === ESize.u8 || esize === ESize.u16 || esize === ESize.u32) && typeof size === 'bigint') {
            return new Error(`For sizes ${ESize.u8}, ${ESize.u16}, ${ESize.u32}, size should be defined as Number`);
        }
        switch(esize) {
            case ESize.u8:
                sizeType = Primitives.u8.encode(Primitives.u8.getSize() * CBits);
                sizeValue = Primitives.u8.encode(size as number);
                break;
            case ESize.u16:
                sizeType = Primitives.u8.encode(Primitives.u16.getSize() * CBits);
                sizeValue = Primitives.u16.encode(size as number);
                break;
            case ESize.u32:
                sizeType = Primitives.u8.encode(Primitives.u32.getSize() * CBits);
                sizeValue = Primitives.u32.encode(size as number);
                break;
            case ESize.u64:
                sizeType = Primitives.u8.encode(Primitives.u64.getSize() * CBits);
                sizeValue = Primitives.u64.encode(BigInt(size));
                break;
        }
        if (sizeType instanceof Error) {
            return sizeType;
        }
        if (sizeValue instanceof Error) {
            return sizeValue;
        }
        if (sizeType === undefined || sizeValue === undefined) {
            return new Error(`Size type or size value aren't defined`);
        }
        return Tools.append([idBuf, sizeType, sizeValue, value]);
    }

    public getBufferFromBuf<T>(id: number, esize: ESize, encoder: (...args: any[]) => ArrayBufferLike | Error, value: T): ArrayBufferLike | Error {
        const buffer = encoder(value);
        if (buffer instanceof Error) {
            return buffer;
        }
        return this.getBuffer(id, esize, esize === ESize.u64 ? BigInt(buffer.byteLength) : buffer.byteLength, buffer);
    }

    public getStorage(buffer: ArrayBufferLike): Storage | Error {
        const storage: Storage = new Storage();
        const error: Error | undefined = storage.read(buffer);
        if (error instanceof Error) {
            return error;
        }
        return storage;
    }

    public getValue<T>(storage: Storage, id: number, decoder: (buf: ArrayBufferLike) => T | Error): T | Error {
        const buffer = storage.get(id);
        if (buffer === undefined) {
            return new Error(`Fail to find field with ID "${id}"`);
        }
        return decoder(buffer);
    }

    public encodeSelfArray(items: Array<Required<Convertor>>): ArrayBufferLike | Error {
        let error: Error | undefined;
        const buffers: ArrayBufferLike[] = [];
        items.forEach((item: Required<Convertor>) => {
            if (error !== undefined) {
                return;
            }
            const buffer = item.encode();
            if (buffer instanceof Error) {
                error = buffer;
                return;
            }
            const len = u64.encode(BigInt(buffer.byteLength));
            if (len instanceof Error) {
                error = len;
                return;
            }
            buffers.push(len);
            buffers.push(buffer);
        });
        if (error !== undefined) {
            return error;
        }
        return Tools.append(buffers);
    }

    public decodeSelfArray(bytes: ArrayBufferLike): Array<Required<Convertor>> | Error {
        const buffer = Buffer.from(bytes);
        const selfs: Array<Required<Convertor>> = [];
        let offset: number = 0;
        do {
            const len = buffer.readUInt32LE(offset);
            if (isNaN(len) || !isFinite(len)) {
                return new Error(`Invalid length of ${this.getSignature()}/${this.getId()} in an array`);
            }
            offset += u32.getSize();
            const body = buffer.slice(offset, offset + len);
            const self = this.defaults();
            const err = self.decode(body);
            if (err instanceof Error) {
                return err;
            }
            selfs.push(self);
            offset += body.byteLength;
        } while (offset < buffer.byteLength);
        return selfs;
    }

    public pack(sequence: number): ArrayBufferLike {
        const id: ArrayBufferLike | Error = Primitives.u32.encode(this.getId());
        const signature: ArrayBufferLike | Error = Primitives.u16.encode(this.signature());
        const seq: ArrayBufferLike | Error = Primitives.u32.encode(sequence);
        const ts = BigInt((new Date()).getTime());
        const timestamp: ArrayBufferLike | Error = Primitives.u64.encode(ts);
        if (id instanceof Error) {
            throw new Error(`Fail to encode id (${this.getId()}) due error: ${id.message}`);
        }
        if (signature instanceof Error) {
            throw new Error(`Fail to encode signature (${this.signature()}) due error: ${signature.message}`);
        }
        if (seq instanceof Error) {
            throw new Error(`Fail to encode seq (${this.getId()}) due error: ${seq.message}`);
        }
        if (timestamp instanceof Error) {
            throw new Error(`Fail to encode timestamp (${ts}) due error: ${timestamp.message}`);
        }
        const buffer: ArrayBufferLike = this.encode();
        const len: ArrayBufferLike | Error = Primitives.u64.encode(BigInt(buffer.byteLength));
        if (len instanceof Error) {
            throw new Error(`Fail to encode len (${ts}) due error: ${len.message}`);
        }
        return Tools.append([id, signature, seq, timestamp, len, buffer]);
    }

    public abstract getSignature(): string;
    public abstract signature(): number;
    public abstract getId(): number;
    public abstract encode(): ArrayBufferLike;
    public abstract decode(buffer: ArrayBufferLike): Error | undefined;
    public abstract defaults(): Convertor;

}
