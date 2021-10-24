import * as Protocol from "implementation/protocol";
import { Response } from "implementation/responses/messages.request";
import {
	Context,
	Producer,
	Identification,
	Filter,
} from "implementation/responses";

export function response(
	reqeust: Protocol.Messages.Request,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<Response> {
	return Promise.reject(
		new Error(`Handler for Protocol.Messages.Request isn't implemented.`)
	);
}
