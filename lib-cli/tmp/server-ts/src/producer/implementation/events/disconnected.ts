import * as Protocol from "implementation/protocol";
import { Producer, Identification, Filter, broadcastAll } from "./index";
import { emit } from "../../events/disconnected";

export interface Output {
	message?: [string[], Protocol.Events.Message];
	disconnected: [string[], Protocol.Events.UserDisconnected];
}

export function handler<C>(
	consumer: Identification,
	filter: Filter,
	context: C,
	producer: Producer<C>
): Promise<void> {
	const broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];
	return emit(consumer, filter, context, producer).then((output) => {
		broadcasts.push(output.disconnected);
		if (output.message !== undefined) {
			broadcasts.push(output.message);
		}
		return broadcastAll(producer, broadcasts);
	});
}
