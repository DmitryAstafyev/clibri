import { Response } from "../implementation/responses/users.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";

export function response(
	request: Protocol.Users.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	return Promise.resolve(
		new Response(
			new Protocol.Users.Response({
				users: context.getUsers(),
			})
		)
	);
	// return Promise.reject(
	// 	new Error(`Handler for Protocol.Users.Request isn't implemented.`)
	// );
}
