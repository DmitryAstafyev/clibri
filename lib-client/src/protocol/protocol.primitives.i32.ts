// tslint:disable: class-name
// tslint:disable: max-classes-per-file

const CBits = 8;

export class i32 {

    public static getSize(): number {
        return 32 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(this.getSize());
        try {
            buffer.writeInt32LE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}
