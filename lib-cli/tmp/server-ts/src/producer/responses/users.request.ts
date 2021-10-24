import * as Protocol from "implementation/protocol";
import { Response } from "implementation/responses/users.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
} from "implementation/responses";

export function response(
	reqeust: Protocol.Users.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	return Promise.reject(
		new Error(`Handler for Protocol.Users.Request isn't implemented.`)
	);
}
