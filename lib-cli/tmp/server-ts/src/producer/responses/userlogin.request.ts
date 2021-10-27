import { Response } from "../implementation/responses/userlogin.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";

export function response(
	request: Protocol.UserLogin.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	context.addUser(consumer.uuid(), request.username);
	const msg = context.addMessage(
		new Protocol.Messages.Message({
			user: request.username,
			uuid: consumer.uuid(),
			message: `User ${request.username} has been join to chat`,
			timestamp: BigInt(Date.now()),
		})
	);
	return Promise.resolve(
		new Response(new Protocol.UserLogin.Accepted({ uuid: consumer.uuid() }))
			.broadcast(filter.except(consumer.uuid()))
			.EventsMessage(
				new Protocol.Events.Message({
					uuid: msg.uuid,
					user: msg.user,
					message: msg.message,
					timestamp: msg.timestamp,
				})
			)
			.broadcast(filter.except(consumer.uuid()))
			.EventsUserConnected(
				new Protocol.Events.UserConnected({
					username: request.username,
					uuid: consumer.uuid(),
				})
			)
	);
}
