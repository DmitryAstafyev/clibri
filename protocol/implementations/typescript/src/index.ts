import * as Primitives from './protocol.primitives';
import { Convertor } from './protocol.convertor';
import { validate } from './protocol.validator';
import { ESize } from './protocol.sizes';

export { ESize } from './protocol.sizes';
export { Primitives };
export { Convertor } from './protocol.convertor';
export { validate } from './protocol.validator';

// injectable
const Protocol: {
    Convertor: typeof Convertor,
    Primitives: typeof Primitives,
    ESize: typeof ESize,
    validate: typeof validate,
} = {
    Convertor: Convertor,
    Primitives: Primitives,
    ESize: ESize,
    validate: validate,
};
