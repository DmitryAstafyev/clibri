import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.eventb";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.Events.EventB,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const stat = context.getStat(event.uuid);
	stat.case(Alias.GroupAStructA);
	stat.case(Alias.GroupAStructB);
	stat.case(Alias.GroupBStructA);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.GroupAStructA(Protocol.GroupA.StructA.defaults())
			.broadcast([event.uuid])
			.GroupAStructB(Protocol.GroupA.StructB.defaults())
			.broadcast([event.uuid])
			.GroupBStructA(Protocol.GroupB.StructA.defaults())
	);
}
