import * as Protocol from "../protocol";
import { Producer, Identification, Filter } from "./index";
import { response } from "../../responses/users.request";

export class Response {
	private _response!: Protocol.Users.Response | Protocol.Users.Err;

	constructor(res: Protocol.Users.Response | Protocol.Users.Err) {
		this._response = res;
	}

	public pack(sequence: number, uuid: string): ArrayBufferLike {
		return this._response.pack(sequence, uuid);
	}
}

export function handler<C>(
	request: Protocol.Users.Request,
	consumer: Identification,
	filter: Filter,
	context: C,
	producer: Producer<C>,
	sequence: number
): Promise<void> {
	return response(request, consumer, filter, context, producer).then(
		(res) => {
			return producer.send(
				consumer.uuid(),
				res.pack(sequence, consumer.uuid())
			);
		}
	);
}
