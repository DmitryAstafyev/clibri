export function append(parts: ArrayBufferLike[]): ArrayBufferLike {
    if (parts.length === 0) {
        return (new Uint8Array()).buffer;
    }
    const tmp = new Uint8Array(parts.map(arr => arr.byteLength).reduce((acc, cur) => acc + cur));
    let cursor = 0;
    parts.forEach((arr) => {
        tmp.set( new Uint8Array(arr), cursor);
        cursor += arr.byteLength;
    });
    return tmp.buffer;
}
