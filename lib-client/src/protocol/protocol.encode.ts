import { Sizes } from './protocol.sizes';

export abstract class Encode {

    public u8(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.u8);
        try {
            buffer.writeUInt8(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public u16(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.u16);
        try {
            buffer.writeUInt16LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public u32(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.u32);
        try {
            buffer.writeUInt32LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public u64(value: bigint): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.u64);
        try {
            buffer.writeBigUInt64LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public i8(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.i8);
        try {
            buffer.writeInt8(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public i16(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.i16);
        try {
            buffer.writeInt16LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public i32(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.i32);
        try {
            buffer.writeInt32LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public i64(value: bigint): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.i64);
        try {
            buffer.writeBigInt64LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public f32(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.f32);
        try {
            buffer.writeFloatLE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

    public f64(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(Sizes.f64);
        try {
            buffer.writeDoubleLE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}