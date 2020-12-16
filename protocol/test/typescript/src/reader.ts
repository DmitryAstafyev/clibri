import * as fs from 'fs';
import * as path from 'path';
import * as Protocol from './protocol';
import * as _ from 'lodash';
import { usecases as samples } from './writer';

const usecases: Array<{ name: string, entity: any }> = [
    { name: 'StructExampleA' , entity: Protocol.StructExampleA },
    { name: 'StructExampleB' , entity: Protocol.StructExampleB },
    { name: 'StructExampleC' , entity: Protocol.StructExampleC },
    { name: 'StructExampleD' , entity: Protocol.StructExampleD },
    { name: 'StructExampleE' , entity: Protocol.StructExampleE },
    { name: 'StructExampleF' , entity: Protocol.StructExampleF },
    { name: 'StructExampleG' , entity: Protocol.StructExampleG },
    { name: 'StructExampleJ' , entity: Protocol.StructExampleJ },
    { name: 'GroupAStructExampleA' , entity: Protocol.GroupA.StructExampleA },
    { name: 'GroupAStructExampleB' , entity: Protocol.GroupA.StructExampleB },
    { name: 'GroupBStructExampleA' , entity: Protocol.GroupB.StructExampleA },
    { name: 'GroupCStructExampleA' , entity: Protocol.GroupB.GroupC.StructExampleA },
    { name: 'GroupCStructExampleB' , entity: Protocol.GroupB.GroupC.StructExampleB },
];

export function read(): Promise<void> {
    const dest: string = path.resolve(path.dirname(module.filename), '../../rust/binary');
    return new Promise((resolve, reject) => {
        if (!fs.existsSync(dest)) {
            return reject(new Error(`Fail to find dest: ${dest}`));
        }
        return Promise.all(usecases.map((usecase, index) => {
            return new Promise((res, rej) => {
                const target = path.resolve(dest, `${usecase.name}.prot.bin`);
                fs.open(target, 'r', (errOpen, file) => {
                    if (errOpen) {
                        return rej(new Error(`Fail to open file ${target} due error: ${errOpen.message}`));
                    }
                    fs.readFile(file, (errWrite: Error | undefined, buffer: Buffer) => {
                        if (errWrite) {
                            return rej(new Error(`Fail to read file ${target} due error: ${errWrite.message}`));
                        }
                        const inst = usecase.entity.from(buffer);
                        if (inst instanceof Error) {
                            return rej(new Error(`Fail to parse usecase "${usecase.name}": ${inst.message}`));
                        }
                        const sample = samples[index].entity;
                        console.log(`[TS] File: ${target} has beed read.`);
                        /*
                        if (!_.isEqual(inst, sample)) {
                            console.log(sample);
                            console.log(inst);
                        }
                        */
                        res(undefined);
                    });
                });
            });
        })).then(() => {
            return resolve();
        }).catch(reject);
    });
}