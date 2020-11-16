// tslint:disable: class-name
// tslint:disable: max-classes-per-file

const CBits = 8;

export class u16 {

    public static getSize(): number {
        return 16 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(this.getSize());
        try {
            buffer.writeUInt16LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}
