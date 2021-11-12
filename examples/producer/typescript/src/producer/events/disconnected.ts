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
	return Promise.resolve(
		new Output()
			.broadcast(filter.except(consumer.uuid()))
			.EventsMessage(
				new Protocol.Events.Message({
					user: user.name,
					uuid: consumer.uuid(),
					message: `User ${user.name} has been left chat`,
					timestamp: BigInt(Date.now()),
				})
			)
			.broadcast(filter.except(consumer.uuid()))
			.EventsUserDisconnected(
				new Protocol.Events.UserDisconnected({
					username: user.name,
					uuid: consumer.uuid(),
				})
			)
	);
}