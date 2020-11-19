// tslint:disable: class-name
// tslint:disable: max-classes-per-file

export class StrAscii {

    public static encode(value: string): ArrayBufferLike | Error {
        const result = new Uint8Array(value.length);
        Array.prototype.forEach.call(value, (char: string, i: number) => {
            result[i] = char.charCodeAt(0);
        });
        return result;
    }

    public static decode(bytes: ArrayBufferLike): string | Error {
        let value = '';
        (new Uint8Array(bytes)).map((code: number) => {
            value += String.fromCharCode(code);
            return code;
        });
        return value;
    }

}
