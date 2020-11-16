// tslint:disable: class-name
// tslint:disable: max-classes-per-file

const CBits = 8;

export class f64 {

    public static getSize(): number {
        return 64 / CBits;
    }

    public static encode(value: number): ArrayBufferLike | Error {
        const buffer: Buffer = Buffer.alloc(this.getSize());
        try {
            buffer.writeDoubleLE(value);
            return buffer.buffer;
        } catch (err) {
            return err;
        }
    }

}
