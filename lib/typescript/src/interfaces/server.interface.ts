import Subject from '../tools/tools.subject';

declare class Event {}

export interface IServerReceivedEvent {
    uuid: string;
    buffer: Buffer;
}

export enum EServerErrorType {
    ConnectionError = 'ConnectionError',
    ConnectionFail = 'ConnectionFail',
    UnexpectedDataFormat = 'UnexpectedDataFormat',
    DataExtractingError = 'DataExtractingError',
    ServerError = 'ServerError'
}

export enum EServerErrorContext {
    server = 'server',
    connection = 'connection',
}

export interface IServerError {
    uuid?: string;
    type: EServerErrorType;
    context: EServerErrorContext;
    message: string;
    event?: Event;
    error?: Error;
}

export interface IServerSubjects {
    ready: Subject<void>;
    shutdown: Subject<void>,
    connected: Subject<string>;
    disconnected: Subject<string>;
    received: Subject<IServerReceivedEvent>;
    error: Subject<IServerError>;
}

export abstract class Server {

    public abstract listen(): Promise<void>;
    public abstract disconnect(uuid: string): Promise<void>;
    public abstract shutdown(): Promise<void>;
    public abstract send(buffer: ArrayBufferLike, uuid?: string): Promise<void>;
    public abstract getEvents(): IServerSubjects;

}