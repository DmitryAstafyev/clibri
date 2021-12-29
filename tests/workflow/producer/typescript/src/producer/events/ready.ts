import { Producer, Context } from "../implementation/events";

// it should be constructed in implementation
export function emit(context: Context, producer: Producer): Promise<void> {
	return Promise.resolve();
}
