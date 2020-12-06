import * as Primitives from './protocol.primitives';
import { Convertor } from './protocol.convertor';
import { ESize } from './protocol.sizes';

export { ESize } from './protocol.sizes';

export { Primitives };

export { Convertor } from './protocol.convertor';

// injectable
const Protocol: {
    Convertor: typeof Convertor,
    Primitives: typeof Primitives,
    ESize: typeof ESize,
} = {
    Convertor: Convertor,
    Primitives: Primitives,
    ESize: ESize,
};
