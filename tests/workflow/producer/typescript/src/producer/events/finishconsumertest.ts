import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/finishconsumertest";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.FinishConsumerTest,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const stat = context.getStat(event.uuid);
	stat.case(Alias.FinishConsumerTestBroadcast);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.FinishConsumerTestBroadcast(
				Protocol.FinishConsumerTestBroadcast.defaults()
			)
	);
}
