import { Response } from "../implementation/responses/message.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";

export function response(
	request: Protocol.Message.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	const msg = context.addMessage(
		new Protocol.Messages.Message({
			user: request.user,
			uuid: consumer.uuid(),
			message: request.message,
			timestamp: BigInt(Date.now()),
		})
	);
	return Promise.resolve(
		new Response(
			new Protocol.Message.Accepted({
				uuid: consumer.uuid(),
			})
		)
			.broadcast(filter.except(consumer.uuid()))
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
