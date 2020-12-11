import * as Protocol from './protocol';
import * as fs from 'fs';
import * as path from 'path';

const usecases: Array<{ name: string, entity: Protocol.Convertor }> = [
    { name: 'User' , entity: new Protocol.User({
        username: ['ffd'],
        email: 'fdfd',
        usertype: {
            PointA: [3],
        },
        info: new Protocol.StructName({
            age: 3,
            name: 'ff'
        }),
    }) }
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
                    resolve();
                });
            });
        });
    })).then(() => {
        return Promise.resolve();
    });
}