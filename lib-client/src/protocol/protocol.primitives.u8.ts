// tslint:disable: class-name
// tslint:disable: max-classes-per-file

const CBits = 8;

export class u8 {

    public static getSize(): number {
        return 8 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(this.getSize());
        try {
            buffer.writeUInt8(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}
