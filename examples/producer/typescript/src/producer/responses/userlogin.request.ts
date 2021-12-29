import { Response } from "../implementation/responses/userlogin.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";

export function response(
	request: Protocol.UserLogin.Request,
	scope: Scope
): Promise<Response> {
	scope.context.addUser(scope.consumer.uuid(), request.username);
	const msg = scope.context.addMessage(
		new Protocol.Messages.Message({
			user: request.username,
			uuid: scope.consumer.uuid(),
			message: `User ${request.username} has been join to chat`,
			timestamp: BigInt(Date.now()),
		})
	);
	return Promise.resolve(
		new Response(
			new Protocol.UserLogin.Accepted({ uuid: scope.consumer.uuid() })
		)
			.broadcast(scope.filter.except(scope.consumer.uuid()))
			.EventsMessage(
				new Protocol.Events.Message({
					uuid: msg.uuid,
					user: msg.user,
					message: msg.message,
					timestamp: msg.timestamp,
				})
			)
			.broadcast(scope.filter.except(scope.consumer.uuid()))
			.EventsUserConnected(
				new Protocol.Events.UserConnected({
					username: request.username,
					uuid: scope.consumer.uuid(),
				})
			)
	);
}
