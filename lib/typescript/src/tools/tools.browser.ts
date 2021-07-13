import { Buffer } from 'buffer';

declare class Window {}
declare var window: Window | undefined;

const INJECTING_BUFFER_FLAG = '___INJECTING_BUFFER_FLAG___';

export function init() {
    if (typeof window === 'object' && window !== null) {
        if ((window as any)[INJECTING_BUFFER_FLAG]) {
            return;
        }
        (window as any).Buffer = Buffer;
        (window as any)[INJECTING_BUFFER_FLAG] = true;
    }
}