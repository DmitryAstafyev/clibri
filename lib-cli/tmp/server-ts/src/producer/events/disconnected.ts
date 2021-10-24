import {
	Identification,
	Filter,
	Producer,
	Context,
	Protocol,
} from "../implementation/events";
import { Output } from "../implementation/events/disconnected";

export function emit(
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Output> {
	const user = context.removeUser(consumer.uuid());
	if (user instanceof Error) {
		return Promise.reject(user);
	}
	return Promise.resolve({
		message: [
			filter.except(consumer.uuid()),
			new Protocol.Events.Message({
				user: user.name,
				uuid: consumer.uuid(),
				message: `User ${user.name} has been left chat`,
				timestamp: BigInt(Date.now()),
			}),
		],
		disconnected: [
			filter.except(consumer.uuid()),
			new Protocol.Events.UserDisconnected({
				username: user.name,
				uuid: consumer.uuid(),
			}),
		],
	});
	// return Promise.reject(
	// 	new Error(`Handler for event "disconnected" isn't implemented`)
	// );
}
