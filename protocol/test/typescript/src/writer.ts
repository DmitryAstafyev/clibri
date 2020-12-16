import * as Protocol from './protocol';
import * as fs from 'fs';
import * as path from 'path';

export const usecases: Array<{ name: string, entity: Protocol.Convertor }> = [
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
];

export function write(): Promise<void> {
    const dest: string = path.resolve(path.dirname(module.filename), '../binary');
    if (!fs.existsSync(dest)) {
        fs.mkdirSync(dest);
    }
    return Promise.all(usecases.map((usecase) => {
        return new Promise((resolve, reject) => {
            const target = path.resolve(dest, `${usecase.name}.prot.bin`);
            if (fs.existsSync(target)) {
                fs.unlinkSync(target);
            }
            fs.open(target, 'w', (errOpen, file) => {
                if (errOpen) {
                    return reject(errOpen);
                }
                fs.writeFile(file, Buffer.from(usecase.entity.encode()), (errWrite: Error | undefined) => {
                    if (errWrite) {
                        return reject(errWrite);
                    }
                    console.log(`[TS] File: ${target} has beed written.`);
                    resolve(undefined);
                });
            });
        });
    })).then(() => {
        return Promise.resolve();
    });
}