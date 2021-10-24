import { IServerError } from "fiber";
import {
	Identification,
	Filter,
	Producer,
	Context,
	ProducerError,
} from "implementation/events";

// it should be constructed in implementation
export function emit(
	error: ProducerError | IServerError,
	context: Context,
	producer: Producer,
	consumer: Identification | undefined,
	filter: Filter | undefined
): Promise<void> {
	console.log(`Handler for event "error" isn't implemented`);
	return Promise.resolve();
}
