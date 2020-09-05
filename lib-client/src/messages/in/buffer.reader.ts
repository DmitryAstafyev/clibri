import { Emitter } from '../../tools/tools.emitter';
import { MessageHolder, IMessage } from './message.holder';
import { MessageHeader } from './message.header';
import * as Income from './msgs/index';

export enum EEvents {
    message = 'message',
    error = 'error',
}

export class BufferReader extends Emitter<EEvents>{

    public static readonly events = EEvents;

    private _buffer: Buffer = new Buffer(0);

    constructor() {
        super();
    }

    public proceed(buffer: Buffer | ArrayBuffer | ArrayBufferLike) {
        this._buffer = Buffer.concat([this._buffer, buffer instanceof Buffer ? buffer : Buffer.from(buffer)]);
        let message: IMessage | Error;
        do {
            message = this._next();
            if (message instanceof Error) {
                // Error during parsing
                this.emit(EEvents.error, message);
                break;
            }
            const Msg = Income.getMsgClass(message);
            if (Msg instanceof Error) {
                this.emit(EEvents.error, Msg);
                break;
            }
            const inst = new Msg(message);
            if (inst.validate() instanceof Error) {
                this.emit(EEvents.error, inst.validate());
                break;
            }
            // Trigger event
            this.emit(EEvents.message, inst);
            if (!MessageHeader.enow(this._buffer)) {
                // Wait for more data
                break;
            }
        } while (true);
    }

    public destroy() {
        // Drop buffer
        this._buffer = new Buffer(0);
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