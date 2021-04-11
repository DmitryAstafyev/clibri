import * as Protocol from './protocol';
import * as fs from 'fs';
import * as path from 'path';

import { state } from './state';

export const usecases: Array<{ name: string, entity: Protocol.Convertor | Protocol.Enum<any> }> = [];

usecases.push({ name: 'EnumExampleA.a', entity: (() => {
    const EnumExampleA = new Protocol.EnumExampleA();
    EnumExampleA.set({ Option_a: 'Option_a' });
    return EnumExampleA;
})()});
usecases.push({ name: 'EnumExampleA.b', entity: (() => {
    const EnumExampleA = new Protocol.EnumExampleA();
    EnumExampleA.set({ Option_b: 'Option_b' });
    return EnumExampleA;
})()});
usecases.push({ name: 'EnumExampleB.str', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_str: 'Option_str' });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u8', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_u8: 8 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u16', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_u16: 16 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u32', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_u32: 32 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.u64', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_u64: BigInt(64) });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i8', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_i8: -8 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i16', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_i16: -16 });
    return EnumExampleB;})()});
usecases.push({ name: 'EnumExampleB.i32', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_i32: -32 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.i64', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_i64: -BigInt(64) });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.f32', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_f32: 0.02 });
    return EnumExampleB;
})()});
usecases.push({ name: 'EnumExampleB.f64', entity: (() => {
    const EnumExampleB = new Protocol.EnumExampleB();
    EnumExampleB.set({ Option_f64: 0.02 });
    return EnumExampleB;
})()});

usecases.push({ name: 'GroupD.EnumExampleP.Option_a', entity: (() => {
    const EnumInts = new Protocol.GroupD.EnumExampleP();
    EnumInts.set({ Option_a: new Protocol.StructExampleA({
        field_str: 'test',
        field_str_empty: '',
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
    }) });
    return EnumInts;
})()});

usecases.push({ name: 'GroupD.EnumExampleP.Option_b', entity: (() => {
    const EnumInts = new Protocol.GroupD.EnumExampleP();
    EnumInts.set({ Option_b: new Protocol.GroupD.StructExampleP({
        field_a: new Protocol.StructExampleA({
            field_str: 'test',
            field_str_empty: '',
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
        field_b: new Protocol.GroupB.StructExampleA({
            field_u8: 1,
            field_u16: 2,
        }),
        field_c: new Protocol.GroupB.GroupC.StructExampleA({
            field_u8: 1,
            field_u16: 2,
        })
    }) });
    return EnumInts;
})()});

usecases.push({ name: 'GroupD.EnumExampleP.Option_c', entity: (() => {
    const EnumInts = new Protocol.GroupD.EnumExampleP();
    EnumInts.set({ Option_c: new Protocol.GroupB.StructExampleA({
        field_u8: 1,
        field_u16: 2,
    }) });
    return EnumInts;
})()});

usecases.push({ name: 'GroupD.EnumExampleP.Option_d', entity: (() => {
    const EnumInts = new Protocol.GroupD.EnumExampleP();
    EnumInts.set({ Option_d: new Protocol.GroupB.GroupC.StructExampleA({
        field_u8: 1,
        field_u16: 2,
    })});
    return EnumInts;
})()});

usecases.push(...[
    { name: 'StructExampleA' , entity: new Protocol.StructExampleA({
        field_str: 'test',
        field_str_empty: '',
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
        field_struct: [
            new Protocol.StructExampleA({
                field_str: 'test',
                field_str_empty: '',
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
            new Protocol.StructExampleA({
                field_str: 'test',
                field_str_empty: '',
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
            new Protocol.StructExampleA({
                field_str: 'test',
                field_str_empty: '',
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
            })
        ],
        field_str_empty: [],
        field_u8_empty: [],
        field_u16_empty: [],
        field_u32_empty: [],
        field_u64_empty: [],
        field_i8_empty: [],
        field_i16_empty: [],
        field_i32_empty: [],
        field_i64_empty: [],
        field_f32_empty: [],
        field_f64_empty: [],
        field_bool_empty: [],
        field_struct_empty: [],
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
            field_str_empty: '',
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
            field_struct: [
                new Protocol.StructExampleA({
                    field_str: 'test',
                    field_str_empty: '',
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
                new Protocol.StructExampleA({
                    field_str: 'test',
                    field_str_empty: '',
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
                new Protocol.StructExampleA({
                    field_str: 'test',
                    field_str_empty: '',
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
                })
            ],
            field_str_empty: [],
            field_u8_empty: [],
            field_u16_empty: [],
            field_u32_empty: [],
            field_u64_empty: [],
            field_i8_empty: [],
            field_i16_empty: [],
            field_i32_empty: [],
            field_i64_empty: [],
            field_f32_empty: [],
            field_f64_empty: [],
            field_bool_empty: [],
            field_struct_empty: [],
        }),
    }) },
    { name: 'StructExampleJ' , entity: new Protocol.StructExampleJ({
        field_a: new Protocol.StructExampleA({
            field_str: 'test',
            field_str_empty: '',
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
        field_c: new Protocol.StructExampleEmpty({}),
    }) },
    { name: 'StructExampleEmpty' , entity: new Protocol.StructExampleEmpty({
        
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
    { name: 'GroupDStructExampleP' , entity: new Protocol.GroupD.StructExampleP({
        field_a: new Protocol.StructExampleA({
            field_str: 'test',
            field_str_empty: '',
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
        field_b: new Protocol.GroupB.StructExampleA({
            field_u8: 1,
            field_u16: 2,
        }),
        field_c: new Protocol.GroupB.GroupC.StructExampleA({
            field_u8: 1,
            field_u16: 2,
        })
    }) }
]);

export function write(): Promise<void> {
    function wr(name: string, entity: Protocol.Convertor | Protocol.Enum<any> | Buffer): Promise<void> {
        return new Promise((resolve, reject) => {
            if ((entity instanceof Protocol.Convertor || entity instanceof Protocol.Enum) && state.getMiddleware()) {
                buffers.push(Buffer.from(entity.pack(0)));
                return resolve();
            }
            const target = path.resolve(dest, `${name}.prot.${state.getMiddleware() ? 'middleware' : 'bin'}`);
            if (fs.existsSync(target)) {
                fs.unlinkSync(target);
            }
            fs.open(target, 'w', (errOpen, file) => {
                if (errOpen) {
                    return reject(errOpen);
                }
                const buf = entity instanceof Buffer ? entity : Buffer.from(entity.encode());
                if (entity instanceof Protocol.Convertor || entity instanceof Protocol.Enum) {
                    buffers.push(Buffer.from(entity.pack(0)));
                }
                fs.writeFile(file, buf, (errWrite: Error | undefined) => {
                    if (errWrite) {
                        return reject(errWrite);
                    }
                    console.log(`[OK]\t[TS] File: ${target} has beed written: ${buf.byteLength} bytes.`);
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
        }).catch((e) => {
            console.log(e);
        });
    });
}