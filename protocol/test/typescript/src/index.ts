import * as ProtocolImpl from './protocol';
import { write } from './writer';
import { read } from './reader';
import { state } from './state';
import { Middleware } from './middleware';

export { ProtocolImpl };

const middleware: Middleware = new Middleware();

if (process.argv.indexOf('write') !== -1) {
    write().then(() => {
        state.setMiddleware(true);
        write().then(() => {
            console.log(`[OK]\tAll usecases are written`);
        }).catch((err: Error) => {
            console.error(`Fail to write usecases due error: ${err.message}`);
        });
    }).catch((err: Error) => {
        console.error(`Fail to write usecases due error: ${err.message}`);
    });
} else if (process.argv.indexOf('read') !== -1) {
    read().then(() => {
        state.setMiddleware(true);
        read().then(() => {
            console.log(`[OK]\tAll usecases are read`);
        }).catch((err: Error) => {
            console.error(`Fail to read usecases due error: ${err.message}`);
        });
    }).catch((err: Error) => {
        console.error(`Fail to read usecases due error: ${err.message}`);
    });
}
