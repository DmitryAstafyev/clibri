import * as Protocol from "../protocol/protocol";

import { Consumer } from "../index";
import { ERequestState } from "../interfaces/request";

export type TStructFResolver = Protocol.StructE | Protocol.StructF;
export type TResponseHandler = (response: Protocol.StructF) => void;
export type TErrHandler = (response: Protocol.StructE) => void;

export class StructF extends Protocol.StructF {
	private _state: ERequestState = ERequestState.Ready;
	private _handlers: {
		response: TResponseHandler | undefined;
		err: TErrHandler | undefined;
	} = {
		response: undefined,
		err: undefined,
	};
	constructor(request: Protocol.IStructF) {
		super(request);
	}

	public destroy() {
		this._state = ERequestState.Destroyed;
		this._handlers = {
			response: undefined,
			err: undefined,
		};
	}

	public send(): Promise<TStructFResolver> {
		const consumer: Consumer | Error = Consumer.get();
		if (consumer instanceof Error) {
			return Promise.reject(consumer);
		}
		if (this._state === ERequestState.Pending) {
			return Promise.reject(
				new Error(`Cannot send request while previous isn't finished`)
			);
		}
		if (this._state === ERequestState.Destroyed) {
			return Promise.reject(
				new Error(`Cannot send request as soon as it's destroyed`)
			);
		}
		const sequence: number = consumer.getSequence();
		this._state = ERequestState.Pending;
		return new Promise((resolve, reject) => {
			consumer
				.request(this.pack(sequence), sequence)
				.then((message: Protocol.IAvailableMessages) => {
					console.log(message);
					switch (this._state) {
						case ERequestState.Pending:
							this._state = ERequestState.Ready;
							if (message === undefined) {
								return reject(
									new Error(
										`Expecting message from "message" group.`
									)
								);
							} else if (message.StructF !== undefined) {
								this._handlers.response !== undefined &&
									this._handlers.response(message.StructF);
								return resolve(message.StructF);
							} else if (message.StructE !== undefined) {
								this._handlers.err !== undefined &&
									this._handlers.err(message.StructE);
								return resolve(message.StructE);
							} else {
								return reject(
									new Error(`No message in "message" group.`)
								);
							}
						case ERequestState.Destroyed:
							return reject(
								new Error(
									`Request "StructF" has been destroyed. Response would not be processed.`
								)
							);
						case ERequestState.Pending:
							return reject(
								new Error(
									`Unexpected state for request "StructF".`
								)
							);
					}
				})
				.catch((err: Error) => {
					reject(err);
				});
		});
	}

	public response(handler: TResponseHandler): StructF {
		this._handlers.response = handler;
		return this;
	}

	public err(handler: TErrHandler): StructF {
		this._handlers.err = handler;
		return this;
	}
}
