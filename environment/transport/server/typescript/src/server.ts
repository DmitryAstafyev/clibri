import { WebSocketServer, WebSocket } from "ws";
import { IncomingMessage } from "http";
import { Connection } from "./connection";
import {
	Server as ServerInterface,
	Subject,
	IServerError,
	IServerReceivedEvent,
	IServerSubjects,
	EServerErrorType,
	EServerErrorContext,
	guid,
	Logger,
	DefaultLogger,
} from "fiber";

export interface IServerOptions {
	logger?: Logger;
}

export class Server extends ServerInterface {
	private readonly _addr: {
		port: number;
		host: string;
		src: string;
	};
	private readonly _subjects: IServerSubjects = {
		ready: new Subject<void>(),
		shutdown: new Subject<void>(),
		connected: new Subject<string>(),
		disconnected: new Subject<string>(),
		received: new Subject<IServerReceivedEvent>(),
		error: new Subject<IServerError>(),
	};
	private readonly _connections: Map<string, Connection> = new Map();
	private readonly _logger: Logger;
	private readonly _extLogger: Logger | undefined;
	private readonly _pending: {
		rejector: undefined | ((err: Error) => void);
	} = {
		rejector: undefined,
	};
	private _server: WebSocketServer | undefined;

	constructor(addr: string, options: IServerOptions = {}) {
		super();
		const portMatch: RegExpMatchArray | null = addr.match(/:\d{1,}$/gi);
		if (portMatch === null || portMatch.length !== 1) {
			throw new Error(`Fail to get port value from "${addr}"`);
		}
		const port: number = parseInt(portMatch[0].replace(":", ""), 10);
		if (isNaN(port) || !isFinite(port)) {
			throw new Error(`Invalid port value for "${addr}"`);
		}
		const host: string = addr.replace(/:\d{1,}$/gi, "");
		this._addr = {
			port,
			host,
			src: addr,
		};
		this._logger =
			options.logger !== undefined
				? options.logger
				: new DefaultLogger(`Server`);
		this._extLogger = options.logger;
	}

	public listen(): Promise<void> {
		return new Promise((resolve, reject) => {
			this._pending.rejector = reject;
			try {
				this._server = new WebSocketServer({
					host: this._addr.host,
					port: this._addr.port,
				});
			} catch (err) {
				return reject(
					new Error(
						`Fail start server. Error: ${
							err instanceof Error ? err.message : err
						}`
					)
				);
			}
			this._server.on("error", this._onError.bind(this));
			this._server.on("connection", this._onConnection.bind(this));
			this._server.on("close", this._onClose.bind(this));
			this._logger.debug(
				`server is listening ${this._addr.host} on ${this._addr.port}`
			);
			resolve();
			this._pending.rejector = undefined;
			this._subjects.ready.emit();
		});
	}

	public disconnect(uuid: string): Promise<void> {
		const connection: Connection | undefined = this._connections.get(uuid);
		if (connection === undefined) {
			return Promise.reject(new Error(`Connection doesn't exist`));
		}
		connection.destroy();
		this._onConnectionDisconnected(connection);
		return Promise.resolve();
	}

	public shutdown(): Promise<void> {
		this._connections.forEach((connection: Connection) => {
			connection.destroy();
			this._onConnectionDisconnected(connection);
		});
		if (this._server !== undefined) {
			this._server.removeAllListeners();
			this._server.close();
		}
		return Promise.resolve();
	}

	public send(buffer: ArrayBufferLike, uuid?: string): Promise<void> {
		if (uuid !== undefined) {
			const connection: Connection | undefined =
				this._connections.get(uuid);
			if (connection === undefined) {
				return Promise.reject(new Error(`Connection doesn't exist`));
			}
			connection.send(buffer);
		} else {
			this._connections.forEach((connection: Connection) => {
				connection.send(buffer);
			});
		}
		return Promise.resolve();
	}

	public getEvents(): IServerSubjects {
		return this._subjects;
	}

	public getStats(): {
		connections: number;
		clients: number;
	} {
		return {
			connections: this._connections.size,
			clients: this._server === undefined ? 0 : this._server.clients.size,
		};
	}

	private _onClose(): void {
		this._logger.debug(`Server is closed`);
	}

	private _onError(err: Error): void {
		this._logger.debug(`Server error: ${err.message}`);
		if (this._pending.rejector !== undefined) {
			this._pending.rejector(err);
			this._pending.rejector = undefined;
		} else {
			this._subjects.error.emit({
				type: EServerErrorType.ServerError,
				context: EServerErrorContext.server,
				message: err.message,
				error: err,
			});
		}
	}

	private _onConnection(socket: WebSocket): void {
		const connection: Connection = new Connection(socket, this._extLogger);
		connection
			.established()
			.then(() => {
				this._logger.debug(
					`New connection established: ${connection.getGuid()}`
				);
				this._connections.set(connection.getGuid(), connection);
				connection
					.getSubjects()
					.error.subscribe(
						this._onConnectionError.bind(this, connection)
					);
				connection
					.getSubjects()
					.data.subscribe(
						this._onConnectionData.bind(this, connection)
					);
				connection
					.getSubjects()
					.disconnected.subscribe(
						this._onConnectionDisconnected.bind(this, connection)
					);
				this._subjects.connected.emit(connection.getGuid());
			})
			.catch((err: Error | IServerError) => {
				this._logger.warn(
					`Fail to add connection. Error: ${err.message}`
				);
				if (err instanceof Error) {
					this._subjects.error.emit({
						type: EServerErrorType.ConnectionFail,
						context: EServerErrorContext.connection,
						message: err.message,
						error: err,
					});
				} else {
					this._subjects.error.emit(err);
				}
			});
	}

	private _onConnectionError(
		connection: Connection,
		error: IServerError
	): void {
		this._subjects.error.emit(error);
		this._logger.warn(
			`Error on connection "${connection.getGuid()}". Error [${
				error.type
			}]: ${error.message}`
		);
	}

	private _onConnectionData(connection: Connection, data: Buffer): void {
		this._subjects.received.emit({
			uuid: connection.getGuid(),
			buffer: data,
		});
	}

	private _onConnectionDisconnected(connection: Connection): void {
		this._connections.delete(connection.getGuid());
		this._subjects.disconnected.emit(connection.getGuid());
		this._logger.debug(`Connection "${connection.getGuid()}" is closed`);
	}
}
