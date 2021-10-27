import {
	Identification,
	Filter,
	Producer,
	Context,
	Protocol,
} from "../implementation/events";
import { Output } from "../implementation/events/disconnected";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	return Promise.reject(
		new Error(`Handler for event "disconnected" isn't implemented`)
	);
}
