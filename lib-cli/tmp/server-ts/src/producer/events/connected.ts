import {
	Identification,
	Filter,
	Producer,
	Context,
	Protocol,
} from "../implementation/events";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<void> {
	return Promise.resolve();
	// return Promise.reject(
	// 	new Error(`Handler for event "connected" isn't implemented`)
	// );
}
