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
	const stat = context.getStat(consumer.uuid());
	stat.case(Alias.Disconnected);
	context.disconnected(consumer.uuid());
	return Promise.resolve();
}
