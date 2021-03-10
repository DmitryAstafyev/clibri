import Subject from '../tools/tools.subject';

export interface IClient {
    connected: Subject<void>;
    disconnected: Subject<void>;
    error: Subject<Error>;
}

export abstract class Client implements IClient {
    
    public connected: Subject<void> = new Subject(`connected`);
    public disconnected: Subject<void> = new Subject(`disconnected`);
    public error: Subject<Error> = new Subject(`error`);
    public data: Subject<ArrayBufferLike> = new Subject(`chunk`);

    public abstract send(buffer: ArrayBufferLike): Error | undefined;
    public abstract connect(): Promise<void>;
    public abstract disconnect(): Promise<void>;
    public abstract destroy(): Promise<void>;

}