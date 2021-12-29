import * as Protocol from "../protocol/protocol";

import { Consumer } from "../index";
import { ERequestState } from "../interfaces/request";

export type TStructAResolver =
	| Protocol.StructE
	| Protocol.StructB
	| Protocol.StructC
	| Protocol.StructD;
export type TCaseBHandler = (response: Protocol.StructB) => void;
export type TCaseCHandler = (response: Protocol.StructC) => void;
export type TCaseDHandler = (response: Protocol.StructD) => void;
export type TErrHandler = (response: Protocol.StructE) => void;

export class StructA extends Protocol.StructA {
	private _state: ERequestState = ERequestState.Ready;
	private _handlers: {
		caseb: TCaseBHandler | undefined;
		casec: TCaseCHandler | undefined;
		cased: TCaseDHandler | undefined;
		err: TErrHandler | undefined;
	} = {
		caseb: undefined,
		casec: undefined,
		cased: undefined,
		err: undefined,
	};
	constructor(request: Protocol.IStructA) {
		super(request);
	}

	public destroy() {
		this._state = ERequestState.Destroyed;
		this._handlers = {
			caseb: undefined,
			casec: undefined,
			cased: undefined,
			err: undefined,
		};
	}

	public send(): Promise<TStructAResolver> {
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
							} else if (message.StructB !== undefined) {
								this._handlers.caseb !== undefined &&
									this._handlers.caseb(message.StructB);
								return resolve(message.StructB);
							} else if (message.StructC !== undefined) {
								this._handlers.casec !== undefined &&
									this._handlers.casec(message.StructC);
								return resolve(message.StructC);
							} else if (message.StructD !== undefined) {
								this._handlers.cased !== undefined &&
									this._handlers.cased(message.StructD);
								return resolve(message.StructD);
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
									`Request "StructA" has been destroyed. Response would not be processed.`
								)
							);
						case ERequestState.Pending:
							return reject(
								new Error(
									`Unexpected state for request "StructA".`
								)
							);
					}
				})
				.catch((err: Error) => {
					reject(err);
				});
		});
	}

	public caseb(handler: TCaseBHandler): StructA {
		this._handlers.caseb = handler;
		return this;
	}
	public casec(handler: TCaseCHandler): StructA {
		this._handlers.casec = handler;
		return this;
	}
	public cased(handler: TCaseDHandler): StructA {
		this._handlers.cased = handler;
		return this;
	}
	public err(handler: TErrHandler): StructA {
		this._handlers.err = handler;
		return this;
	}
}
