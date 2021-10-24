import { Producer, Identification, Filter } from "./index";
import { emit } from "../../events/connected";

export function handler<C>(
	consumer: Identification,
	filter: Filter,
	context: C,
	producer: Producer<C>
): Promise<void> {
	return emit(consumer, filter, context, producer);
}
