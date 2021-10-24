import * as Protocol from "implementation/protocol";
import { Response } from "implementation/responses/userlogin.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
} from "implementation/responses";

export function response(
	reqeust: Protocol.UserLogin.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	return Promise.reject(
		new Error(`Handler for Protocol.UserLogin.Request isn't implemented.`)
	);
}
