import * as Protocol from "implementation/protocol";
import {
	Identification,
	Filter,
	Producer,
	Context,
} from "implementation/events";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<void> {
	return Promise.reject(
		new Error(`Handler for event "connected" isn't implemented`)
	);
}
