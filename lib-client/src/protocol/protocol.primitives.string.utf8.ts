// tslint:disable: class-name
// tslint:disable: max-classes-per-file

import { TextEncoder } from "util";

export class StrUTF8 {

    public static encode(value: string): ArrayBufferLike | Error {
        const encoder = new TextEncoder();
        return encoder.encode(value);
    }

    public static decode(bytes: ArrayBufferLike): string | Error {
        const decoder = new TextDecoder();
        return decoder.decode(bytes);
    }

}
