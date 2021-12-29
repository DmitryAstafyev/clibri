import { Response } from "../implementation/responses/messages.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";

export function response(
	request: Protocol.Messages.Request,
	scope: Scope
): Promise<Response> {
	return Promise.resolve(
		new Response(
			new Protocol.Messages.Response({
				messages: scope.context.getMessages(),
			})
		)
	);
}
