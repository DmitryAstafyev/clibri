import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.sub.eventa";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.Events.Sub.EventA,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const stat = context.getStat(event.uuid);
	stat.case(Alias.GroupBGroupCStructA);
	stat.case(Alias.GroupBGroupCStructB);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.GroupBGroupCStructA(Protocol.GroupB.GroupC.StructA.defaults())
			.broadcast([event.uuid])
			.GroupBGroupCStructB(Protocol.GroupB.GroupC.StructB.defaults())
	);
}
