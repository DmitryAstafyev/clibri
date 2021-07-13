declare type Window = any;

declare var window: Window | undefined;
declare var global: any | undefined;

export default function globals(): Window | any | Error {
    if (typeof window === 'object' && window !== null) {
        return window;
    } else if (typeof global === 'object' && global !== null) {
        return global;
    } else {
        return new Error(`Fail to find global namespece ()`);
    }
}
