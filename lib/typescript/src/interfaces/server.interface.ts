// tslint:disable: max-classes-per-file

import Subject from "../tools/tools.subject";
import { Logger, DefaultLogger } from "./logger.interface";

declare class Event {}

export interface IServerReceivedEvent {
	uuid: string;
	buffer: Buffer;
}

export enum EServerErrorType {
	ConnectionError = "ConnectionError",
	ConnectionFail = "ConnectionFail",
	UnexpectedDataFormat = "UnexpectedDataFormat",
	DataExtractingError = "DataExtractingError",
	ServerError = "ServerError",
}

export enum EServerErrorContext {
	server = "server",
	connection = "connection",
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
	shutdown: Subject<void>;
	connected: Subject<string>;
	disconnected: Subject<string>;
	received: Subject<IServerReceivedEvent>;
	error: Subject<IServerError>;
}

export enum ProducerIdentificationStrategy {
	// Put warning into logs
	Log,
	// Emit error (would not stop producer)
	EmitError,
	// Disconnect consumer
	Disconnect,
	// Disconnect consumer and emit error
	EmitErrorAndDisconnect,
	// Ignore if consumer doesn't have producer identification
	Ignore,
}

export enum ConsumerErrorHandelingStrategy {
	// Emit error (would not stop producer)
	EmitError,
	// Disconnect consumer
	Disconnect,
	// Disconnect consumer and emit error
	EmitErrorAndDisconnect,
	// Put warning into logs
	Log,
}

export interface IOptions {
	producerIndentificationStrategy?: ProducerIdentificationStrategy;
	consumerErrorHandelingStrategy?: ConsumerErrorHandelingStrategy;
	logger?: Logger;
}

export class Options {
	public readonly producerIndentificationStrategy: ProducerIdentificationStrategy;
	public readonly consumerErrorHandelingStrategy: ConsumerErrorHandelingStrategy;
	public readonly logger: Logger;

	constructor(options: IOptions) {
		this.producerIndentificationStrategy =
			options.producerIndentificationStrategy === undefined
				? ProducerIdentificationStrategy.Log
				: options.producerIndentificationStrategy;
		this.consumerErrorHandelingStrategy =
			options.consumerErrorHandelingStrategy === undefined
				? ConsumerErrorHandelingStrategy.EmitErrorAndDisconnect
				: options.consumerErrorHandelingStrategy;
		this.logger =
			options.logger !== undefined ? options.logger : new DefaultLogger();
	}
}

export abstract class Server {
	public abstract listen(): Promise<void>;
	public abstract disconnect(uuid: string): Promise<void>;
	public abstract shutdown(): Promise<void>;
	public abstract send(buffer: ArrayBufferLike, uuid?: string): Promise<void>;
	public abstract getEvents(): IServerSubjects;
}
