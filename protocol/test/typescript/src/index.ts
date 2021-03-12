import * as ProtocolImpl from './protocol';
import { write } from './writer';
import { read } from './reader';

export { ProtocolImpl };

if (process.argv.indexOf('write') !== -1) {
    write().then(() => {
        console.log(`[OK]\tAll usecases are written`);
    }).catch((err: Error) => {
        console.error(`Fail to write usecases due error: ${err.message}`);
    });
} else if (process.argv.indexOf('read') !== -1) {
    read().then(() => {
        console.log(`[OK]\tAll usecases are read`);
    }).catch((err: Error) => {
        console.error(`Fail to read usecases due error: ${err.message}`);
    });
}
