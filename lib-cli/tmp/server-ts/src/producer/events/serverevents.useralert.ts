import { Filter, Producer, Context, Protocol } from "@implementation/events";
import { Output } from "@implementation/events/serverevents.useralert";

export function emit(
	event: Protocol.ServerEvents.UserAlert,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	return Promise.reject(
		new Error(`Handler for event "useralert" isn't implemented`)
	);
}
