declare var window: Window | undefined;
declare var global: NodeJS.Global | undefined;

export default function globals(): Window | NodeJS.Global | Error {
    if (typeof window === 'object' && window !== null) {
        return window;
    } else if (typeof global === 'object' && global !== null) {
        return global;
    } else {
        return new Error(`Fail to find global namespece ()`);
    }
}
