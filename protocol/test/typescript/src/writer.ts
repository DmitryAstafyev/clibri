import * as Protocol from './protocol';
import * as fs from 'fs';
import * as path from 'path';

function EnumExampleAFactory(): Protocol.Primitives.Enum {
    return new Protocol.Primitives.Enum([
        Protocol.Primitives.StrUTF8.getSignature(),
        Protocol.Primitives.StrUTF8.getSignature(),
    ], (id: number): Protocol.ISigned<any> | undefined => {
        switch (id) {
            case 0: return new Protocol.Primitives.StrUTF8('');
            case 1: return new Protocol.Primitives.StrUTF8('');
        }
    });
}

function EnumExampleBFactory(): Protocol.Primitives.Enum {
    return new Protocol.Primitives.Enum([
        Protocol.Primitives.StrUTF8.getSignature(),
        Protocol.Primitives.u8.getSignature(),
        Protocol.Primitives.u16.getSignature(),
        Protocol.Primitives.u32.getSignature(),
        Protocol.Primitives.u64.getSignature(),
        Protocol.Primitives.i8.getSignature(),
        Protocol.Primitives.i16.getSignature(),
        Protocol.Primitives.i32.getSignature(),
        Protocol.Primitives.i64.getSignature(),
        Protocol.Primitives.f32.getSignature(),
        Protocol.Primitives.f64.getSignature(),
    ], (id: number): Protocol.ISigned<any> | undefined => {
        switch (id) {
            case 0: return new Protocol.Primitives.StrUTF8('');
            case 1: return new Protocol.Primitives.u8(0);
            case 2: return new Protocol.Primitives.u16(0);
            case 3: return new Protocol.Primitives.u32(0);
            case 4: return new Protocol.Primitives.u64(BigInt(0));
            case 5: return new Protocol.Primitives.i8(0);
            case 6: return new Protocol.Primitives.i16(0);
            case 7: return new Protocol.Primitives.i32(0);
            case 8: return new Protocol.Primitives.i64(BigInt(0));
            case 9: return new Protocol.Primitives.f32(0);
            case 10: return new Protocol.Primitives.f64(0);
        }
    });
}

export const usecases: Array<{ name: string, entity: Protocol.Convertor | Protocol.Enum }> = [];

usecases.push({ name: 'EnumExampleA.a', entity: (() => {
    const EnumExampleA = EnumExampleAFactory();
    EnumExampleA.set(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(`Option_a`)));
    return EnumExampleA;
})()});
usecases.push({ name: 'EnumExampleA.b', entity: (() => {
    const EnumExampleA = EnumExampleAFactory();
    EnumExampleA.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(`Option_b`)));
    return EnumExampleA;
})()});
usecases.push({ name: 'EnumExampleB.str', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<string>(0, new Protocol.Primitives.StrUTF8(`Option_str`)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u8', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(1, new Protocol.Primitives.u8(8)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u16', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(16)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u32', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(3, new Protocol.Primitives.u32(32)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u64', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<bigint>(4, new Protocol.Primitives.u64(BigInt(64))));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i8', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(5, new Protocol.Primitives.i8(-8)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i16', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(6, new Protocol.Primitives.i16(-16)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i32', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(7, new Protocol.Primitives.i32(-32)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i64', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<bigint>(8, new Protocol.Primitives.i64(-BigInt(64))));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.f32', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(9, new Protocol.Primitives.f32(0.02)));
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.f64', entity: (() => {
    const EnumExampleB = EnumExampleBFactory();
    EnumExampleB.set(new Protocol.Primitives.Option<number>(10, new Protocol.Primitives.f64(0.02)));
    return EnumExampleB;
})()});


usecases.push(...[
    { name: 'StructExampleA' , entity: new Protocol.StructExampleA({
        field_str: 'test',
        field_u8: 1,
        field_u16: 2,
        field_u32: 3,
        field_u64: BigInt(4),
        field_i8: -1,
        field_i16: -2,
        field_i32: -3,
        field_i64: -BigInt(4),
        field_f32: 0.1,
        field_f64: 0.2,
        field_bool: true,
    }) },
    { name: 'StructExampleB' , entity: new Protocol.StructExampleB({
        field_str: ['test_a', 'test_b'],
        field_u8: [1, 2, 3, 4],
        field_u16: [1, 2, 3, 4],
        field_u32: [1, 2, 3, 4],
        field_u64: [BigInt(1), BigInt(2)],
        field_i8: [-1, -2, -3, -4],
        field_i16: [-1, -2, -3, -4],
        field_i32: [-1, -2, -3, -4],
        field_i64: [-BigInt(1), -BigInt(2)],
        field_f32: [0.1, 0.2, 0.3, 0.4],
        field_f64: [0.1, 0.2, 0.3, 0.4],
        field_bool: [true, false, true],
    }) },
    { name: 'StructExampleC' , entity: new Protocol.StructExampleC({
        field_str: 'test',
        field_u8: 1,
        field_u16: 2,
        field_u32: 3,
        field_u64: BigInt(4),
        field_i8: undefined,
        field_i16: undefined,
        field_i32: undefined,
        field_i64: undefined,
        field_f32: undefined,
        field_f64: undefined,
        field_bool: undefined,
    }) },
    { name: 'StructExampleD' , entity: new Protocol.StructExampleD({
        field_str: ['test_a', 'test_b'],
        field_u8: [1, 2, 3, 4],
        field_u16: [1, 2, 3, 4],
        field_u32: [1, 2, 3, 4],
        field_u64: [BigInt(1), BigInt(2)],
        field_i8: undefined,
        field_i16: undefined,
        field_i32: undefined,
        field_i64: undefined,
        field_f32: undefined,
        field_f64: undefined,
        field_bool: undefined,
    }) },
    { name: 'StructExampleE' , entity: new Protocol.StructExampleE({
        field_a: {
            Option_a: 'Option_a'
        },
        field_b: {
            Option_u8: 1,
        },
        field_c: {
            Option_u8: [1],
        }
    }) },
    { name: 'StructExampleF' , entity: new Protocol.StructExampleF({
        field_a: undefined,
        field_b: undefined,
        field_c: {
            Option_u8: [1],
        }
    }) },
    { name: 'StructExampleG' , entity: new Protocol.StructExampleG({
        field_a: new Protocol.StructExampleA({
            field_str: 'test',
            field_u8: 1,
            field_u16: 2,
            field_u32: 3,
            field_u64: BigInt(4),
            field_i8: -1,
            field_i16: -2,
            field_i32: -3,
            field_i64: -BigInt(4),
            field_f32: 0.1,
            field_f64: 0.2,
            field_bool: true,
        }),
        field_b: new Protocol.StructExampleB({
            field_str: ['test_a', 'test_b'],
            field_u8: [1, 2, 3, 4],
            field_u16: [1, 2, 3, 4],
            field_u32: [1, 2, 3, 4],
            field_u64: [BigInt(1), BigInt(2)],
            field_i8: [-1, -2, -3, -4],
            field_i16: [-1, -2, -3, -4],
            field_i32: [-1, -2, -3, -4],
            field_i64: [-BigInt(1), -BigInt(2)],
            field_f32: [0.1, 0.2, 0.3, 0.4],
            field_f64: [0.1, 0.2, 0.3, 0.4],
            field_bool: [true, false, true],
        }),
    }) },
    { name: 'StructExampleJ' , entity: new Protocol.StructExampleJ({
        field_a: new Protocol.StructExampleA({
            field_str: 'test',
            field_u8: 1,
            field_u16: 2,
            field_u32: 3,
            field_u64: BigInt(4),
            field_i8: -1,
            field_i16: -2,
            field_i32: -3,
            field_i64: -BigInt(4),
            field_f32: 0.1,
            field_f64: 0.2,
            field_bool: true,
        }),
        field_b: undefined,
    }) },
    { name: 'GroupAStructExampleA' , entity: new Protocol.GroupA.StructExampleA({
        field_u8: 1,
        field_u16: 2,
        opt: {
            Option_a: 'Option_a',
        }
    }) },
    { name: 'GroupAStructExampleB' , entity: new Protocol.GroupA.StructExampleB({
        field_u8: 1,
        field_u16: 2,
        strct: new Protocol.GroupA.StructExampleA({
            field_u8: 1,
            field_u16: 2,
            opt: {
                Option_a: 'Option_a',
            }
        })
    }) },
    { name: 'GroupBStructExampleA' , entity: new Protocol.GroupB.StructExampleA({
        field_u8: 1,
        field_u16: 2,
    }) },
    { name: 'GroupCStructExampleA' , entity: new Protocol.GroupB.GroupC.StructExampleA({
        field_u8: 1,
        field_u16: 2,
    }) },
    { name: 'GroupCStructExampleB' , entity: new Protocol.GroupB.GroupC.StructExampleB({
        field_u8: 1,
        field_u16: 2,
        strct: new Protocol.GroupB.GroupC.StructExampleA({
            field_u8: 1,
            field_u16: 2,
        })
    }) },
]);

export function write(): Promise<void> {
    function wr(name: string, entity: Protocol.Convertor | Protocol.Enum | Buffer): Promise<void> {
        return new Promise((resolve, reject) => {
            const target = path.resolve(dest, `${name}.prot.bin`);
            if (fs.existsSync(target)) {
                fs.unlinkSync(target);
            }
            fs.open(target, 'w', (errOpen, file) => {
                if (errOpen) {
                    return reject(errOpen);
                }
                const buf = entity instanceof Buffer ? entity : Buffer.from(entity.encode());
                buffers.push(buf);
                fs.writeFile(file, buf, (errWrite: Error | undefined) => {
                    if (errWrite) {
                        return reject(errWrite);
                    }
                    console.log(`[TS] File: ${target} has beed written.`);
                    resolve(undefined);
                });
            });
        });
    }
    const dest: string = path.resolve(path.dirname(module.filename), '../binary');
    const buffers: Buffer[] = [];
    if (!fs.existsSync(dest)) {
        fs.mkdirSync(dest);
    }
    return new Promise((resolve) => {
        Promise.all(usecases.map((usecase) => {
            return wr(usecase.name, usecase.entity);
        })).then(() => {
            wr('buffer', Buffer.concat(buffers)).then(resolve);
        });
    });
}