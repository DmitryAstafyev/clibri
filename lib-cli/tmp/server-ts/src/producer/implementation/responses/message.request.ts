import {
	Producer,
	Identification,
	Filter,
	broadcastAll,
	Context,
	Protocol,
} from "./index";
import { response } from "../../responses/message.request";

export class Response {
	private _response!:
		| Protocol.Message.Accepted
		| Protocol.Message.Denied
		| Protocol.Message.Err;
	private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

	constructor(
		res:
			| Protocol.Message.Accepted
			| Protocol.Message.Denied
			| Protocol.Message.Err
	) {
		this._response = res;
	}

	public broadcast(uuids: string[]): {
		message(msg: Protocol.Events.Message): Response;
	} {
		const self = this;
		return {
			message(msg: Protocol.Events.Message): Response {
				if (!(self._response instanceof Protocol.Message.Accepted)) {
					throw new Error(
						`Message "Protocol.Events.Message" can be used only with "Protocol.Message.Accepted"`
					);
				}
				if (
					self._broadcasts.find(
						(b) => b[1] instanceof Protocol.Events.Message
					) !== undefined
				) {
					throw new Error(
						`Broadcast Protocol.Events.Message already has been defined.`
					);
				}
				self._broadcasts.push([uuids, msg]);
				return self;
			},
		};
	}

	public error(): Error | undefined {
		if (this._response instanceof Protocol.Message.Accepted) {
			if (this._broadcasts.length !== 1) {
				return new Error(
					`For "Protocol.Message.Accepted" should be defined next broadcasts:\n - Protocol.Events.Message`
				);
			}
		}
		return undefined;
	}

	public pack(sequence: number, uuid: string): ArrayBufferLike {
		return this._response.pack(sequence, uuid);
	}

	public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
		return this._broadcasts;
	}
}

export function handler(
	request: Protocol.Message.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer,
	sequence: number
): Promise<void> {
	return response(request, consumer, filter, context, producer).then(
		(res) => {
			const error: Error | undefined = res.error();
			if (error instanceof Error) {
				return Promise.reject(error);
			}
			return producer
				.send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
				.then(() => {
					return broadcastAll(producer, res.broadcasts());
				});
		}
	);
}
