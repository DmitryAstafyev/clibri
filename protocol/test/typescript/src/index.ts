import * as ProtocolImpl from './protocol';
import { write } from './writer';

export { ProtocolImpl };

write().then(() => {
    console.log(`All usecases are written`);
}).catch((err: Error) => {
    console.error(`Fail to write usecases due error: ${err.message}`);
});
