export interface ILogger {
    warm: (msg: string) => void;
    debug: (msg: string) => void;
    verb: (msg: string) => void;
    err: (msg: string) => void;
    info: (msg: string) => void;
}