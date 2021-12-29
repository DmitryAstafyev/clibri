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
	context.connected(consumer.uuid());
	return Promise.resolve();
}
