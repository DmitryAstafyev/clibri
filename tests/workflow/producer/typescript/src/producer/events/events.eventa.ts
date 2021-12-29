import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.eventa";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.Events.EventA,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const stat = context.getStat(event.uuid);
	stat.case(Alias.StructB);
	stat.case(Alias.StructA);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.StructB(Protocol.StructB.defaults())
			.broadcast([event.uuid])
			.StructA(Protocol.StructA.defaults())
	);
}
