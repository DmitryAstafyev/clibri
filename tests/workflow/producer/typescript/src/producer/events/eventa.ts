import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/eventa";
import { Alias } from "../../stat";

export function emit(
	event: Protocol.EventA,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
    const stat = context.getStat(event.uuid);
	stat.case(Alias.StructB);
	stat.case(Alias.StructC);
	return Promise.resolve(
		new Output()
			.broadcast([event.uuid])
			.StructB(Protocol.StructB.defaults())
			.broadcast([event.uuid])
			.StructC(Protocol.StructC.defaults())
	);
}
