// tslint:disable: class-name
// tslint:disable: max-classes-per-file

const CBits = 8;

export class i64 {

    public static getSize(): number {
        return 64 / CBits;
    }

    public static encode(value: bigint): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(this.getSize());
        try {
            buffer.writeBigInt64LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}
