// injectable
export class MessageHeader {
    public static readonly ID_LENGTH = 4;
    public static readonly SIGN_LENGTH = 2;
    public static readonly TS_LENGTH = 8;
    public static readonly LEN_LENGTH = 4;
    public static readonly SIZE =
        MessageHeader.ID_LENGTH +
        MessageHeader.SIGN_LENGTH +
        MessageHeader.TS_LENGTH +
        MessageHeader.LEN_LENGTH;

    public readonly id: number;
    public readonly signature: number;
    public readonly ts: BigInt;
    public readonly len: number;

    constructor(buffer: Buffer) {
        if (MessageHeader.enow(buffer) === false) {
            throw new Error(
                `Cannot parse header because size problem. Buffer: ${buffer.byteLength} bytes; header size: ${MessageHeader.SIZE} bytes`
            );
        } else {
            this.id = buffer.readUInt32LE(0);
            this.signature = buffer.readUInt16LE(MessageHeader.ID_LENGTH);
            this.ts = buffer.readBigUInt64LE(MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH);
            this.len = buffer.readUInt32LE(MessageHeader.ID_LENGTH + MessageHeader.SIGN_LENGTH + MessageHeader.TS_LENGTH);
        }
    }

    public static enow(buffer: Buffer): boolean {
        return buffer.byteLength >= MessageHeader.SIZE;
    }

}
