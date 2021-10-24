import * as Protocol from "../protocol";
import { Producer, Identification, Filter } from "./index";
import { response } from "../../responses/messages.request";

export class Response {
	private _response!: Protocol.Messages.Response | Protocol.Messages.Err;

	constructor(res: Protocol.Messages.Response | Protocol.Messages.Err) {
		this._response = res;
	}

	public pack(sequence: number, uuid: string): ArrayBufferLike {
		return this._response.pack(sequence, uuid);
	}
}

export function handler<C>(
	request: Protocol.Messages.Request,
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
