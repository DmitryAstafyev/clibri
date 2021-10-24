import { Producer, Context } from "implementation/events";

// it should be constructed in implementation
export function emit(
	context: Context,
	producer: Producer<Context>
): Promise<void> {
	console.log(`Handler for event "ready" isn't implemented`);
	return Promise.resolve();
}
