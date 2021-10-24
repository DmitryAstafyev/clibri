import * as Protocol from "implementation/protocol";
import { Filter, Producer, Context } from "implementation/events";
import { Output } from "implementation/events/serverevents.userkickoff";

export function emit(
	event: Protocol.ServerEvents.UserKickOff,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<Output> {
	return Promise.reject(
		new Error(`Handler for event "userkickoff" isn't implemented`)
	);
}
