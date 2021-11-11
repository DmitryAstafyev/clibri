export { append } from './tools.arraybuffer';

import { append } from './tools.arraybuffer';

// injectable
const Tools: {
    append: typeof append;
} = {
    append: append,
};
