import {
	Identification,
	Filter,
	Producer,
	Context,
	Protocol,
} from "../implementation/events";
import { Alias } from "../../stat";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<void> {
	context.connected(consumer.uuid());
	const stat = context.getStat(consumer.uuid());
	stat.case(Alias.Connected);
	return Promise.resolve();
}
