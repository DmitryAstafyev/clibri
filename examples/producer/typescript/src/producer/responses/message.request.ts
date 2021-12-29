import { Response } from "../implementation/responses/message.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";

export function response(
	request: Protocol.Message.Request,
	scope: Scope
): Promise<Response> {
	const msg = scope.context.addMessage(
		new Protocol.Messages.Message({
			user: request.user,
			uuid: scope.consumer.uuid(),
			message: request.message,
			timestamp: BigInt(Date.now()),
		})
	);
	return Promise.resolve(
		new Response(
			new Protocol.Message.Accepted({
				uuid: scope.consumer.uuid(),
			})
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
	);
}
