import * as ProtocolImpl from './protocol';
import { state } from './state';

export class Middleware extends ProtocolImpl.PackingMiddleware {

    constructor() {
        super();
    }
    public decode(buffer: ArrayBufferLike, id: number, sequence: number, uuid?: string): ArrayBufferLike | Error {
        if (state.getMiddleware()) {
            return buffer.slice(0, Math.round(buffer.byteLength / 2));
        }
        return buffer;
    }

    public encode(buffer: ArrayBufferLike, id: number, sequence: number, uuid?: string): ArrayBufferLike | Error {
        if (state.getMiddleware()) {
            return ProtocolImpl.append([buffer, buffer]);
        }
        return buffer;
    }

}
