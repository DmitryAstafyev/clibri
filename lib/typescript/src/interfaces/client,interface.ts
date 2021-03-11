import Subject from '../tools/tools.subject';

export interface IClientSubjects {
    connected: Subject<void>;
    disconnected: Subject<void>;
    error: Subject<Error>;
    data: Subject<ArrayBufferLike>;
}

export abstract class Client {

    public abstract connect(): Promise<void>;
    public abstract disconnect(): Promise<void>;
    public abstract destroy(): Promise<void>;
    public abstract send(buffer: ArrayBufferLike): Error | undefined;
    public abstract getEvents(): IClientSubjects;

}