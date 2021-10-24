import { Context, Producer, Identification, Filter } from "./index";
import { emit } from "../../events/connected";

export function handler(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<void> {
	return emit(consumer, filter, context, producer);
}
