import { Producer, Context } from "../implementation/events";

export function emit(context: Context, producer: Producer): Promise<void> {
	console.log(`Handler for event "shutdown" isn't implemented`);
	return Promise.resolve();
}
