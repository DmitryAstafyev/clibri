import { Producer, Filter, broadcastAll, Context, Protocol } from "./index";
import { emit } from "../../events/serverevents.userkickoff";

export interface Output {
	message: [string[], Protocol.Events.Message];
	disconnected: [string[], Protocol.Events.UserDisconnected];
}

export function handler(
	event: Protocol.ServerEvents.UserKickOff,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<void> {
	const broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];
	return emit(event, filter, context, producer).then((output) => {
		broadcasts.push(output.disconnected);
		if (output.message !== undefined) {
			broadcasts.push(output.message);
		}
		return broadcastAll(producer, broadcasts);
	});
}
