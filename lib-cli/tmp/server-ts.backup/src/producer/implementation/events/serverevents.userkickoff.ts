import { Producer, Filter, broadcastAll, Context, Protocol } from "./index";
import { emit } from "../../events/serverevents.userkickoff";

export class Output {
	static REQUIRED = [
		Protocol.Events.Message,
		Protocol.Events.UserDisconnected,
	];
	private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

	public broadcast(uuids: string[]): {
		message(msg: Protocol.Events.Message): Output;
		disconnected(msg: Protocol.Events.UserDisconnected): Output;
	} {
		const self = this;
		return {
			message(msg: Protocol.Events.Message): Output {
				if (
					self._broadcasts.find(
						(b) =>
							b[1].getSignature() ===
							Protocol.Events.Message.getSignature()
					) !== undefined
				) {
					throw new Error(
						`Broadcast Protocol.Events.Message already has been defined.`
					);
				}
				self._broadcasts.push([uuids, msg]);
				return self;
			},
			disconnected(msg: Protocol.Events.UserDisconnected): Output {
				if (
					self._broadcasts.find(
						(b) =>
							b[1].getSignature() ===
							Protocol.Events.UserDisconnected.getSignature()
					) !== undefined
				) {
					throw new Error(
						`Broadcast Protocol.Events.UserDisconnected already has been defined.`
					);
				}
				self._broadcasts.push([uuids, msg]);
				return self;
			},
		};
	}

	public error(): Error | undefined {
		let error: Error | undefined;
		Output.REQUIRED.forEach((ref) => {
			if (error !== undefined) {
				return;
			}
			if (
				this._broadcasts.find((msg) => {
					return msg[1].getSignature() === ref.getSignature();
				}) === undefined
			) {
				error = new Error(
					`Broadcast ${ref.getSignature()} is required, but hasn't been found`
				);
			}
		});
		return error;
	}

	public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
		return this._broadcasts;
	}
}

export function handler(
	event: Protocol.ServerEvents.UserKickOff,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<void> {
	return emit(event, filter, context, producer).then((output) => {
		const error: Error | undefined = output.error();
		if (error instanceof Error) {
			return Promise.reject(error);
		}
		return broadcastAll(producer, output.broadcasts());
	});
}
