import * as Protocol from "implementation/protocol";
import {
	Identification,
	Filter,
	Producer,
	Context,
} from "implementation/events";
import { Output } from "implementation/events/disconnected";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<Output> {
	return Promise.reject(
		new Error(`Handler for event "disconnected" isn't implemented`)
	);
}
