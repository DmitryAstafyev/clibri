import { Emitter } from '../../tools/tools.emitter';
import { MessageHolder, IMessage } from './message.holder';
import { MessageHeader } from './message.header';
import { Protocol } from '../../protocol';

export enum EBufferEvents {
    message = 'message',
    error = 'error',
}

export class BufferReader<TIncomeMessages> extends Emitter<EBufferEvents> {

    public static readonly events = EBufferEvents;

    private _buffer: Buffer = Buffer.alloc(0);
    private _protocol: Protocol<TIncomeMessages>;

    constructor(protocol: Protocol<TIncomeMessages>) {
        super();
        this._protocol = protocol;
    }

    public proceed(buffer: Buffer | ArrayBuffer | ArrayBufferLike) {
        this._buffer = Buffer.concat([this._buffer, buffer instanceof Buffer ? buffer : Buffer.from(buffer)]);
        let message: IMessage | Error;
        do {
            message = this._next();
            if (message instanceof Error) {
                // Error during parsing
                this.emit(EBufferEvents.error, message);
                break;
            }
            const Msg: any = this._protocol.getMsgClass(message);
            if (Msg instanceof Error) {
                this.emit(EBufferEvents.error, Msg);
                break;
            }
            const inst = new Msg(message);
            if (inst.validate() instanceof Error) {
                this.emit(EBufferEvents.error, inst.validate());
                break;
            }
            // Trigger event
            this.emit(EBufferEvents.message, inst);
            if (!MessageHeader.enow(this._buffer)) {
                // Wait for more data
                break;
            }
        } while (true);
    }

    public destroy() {
        // Drop buffer
        this._buffer = Buffer.alloc(0);
    }

    public size(): number {
        return this._buffer.byteLength;
    }

    private _next(): Error | IMessage {
        try {
            const holder: MessageHolder = new MessageHolder(this._buffer);
            this._buffer = holder.crop();
            return holder.getMessage();
        } catch (err) {
            return err;
        }
    }
}