import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/triggerbeaconsemitter";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.TriggerBeaconsEmitter,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const stat = context.getStat(event.uuid);
	stat.case(Alias.TriggerBeacons);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.TriggerBeacons(Protocol.TriggerBeacons.defaults())
	);
}
