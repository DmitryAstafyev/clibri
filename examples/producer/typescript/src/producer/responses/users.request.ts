import { Response } from "../implementation/responses/users.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";

export function response(
	request: Protocol.Users.Request,
	scope: Scope
): Promise<Response> {
	return Promise.resolve(
		new Response(
			new Protocol.Users.Response({
				users: scope.context.getUsers(),
			})
		)
	);
}
