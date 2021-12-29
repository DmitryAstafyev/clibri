import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { response } from "../../responses/groupb.groupc.structa";
import { Scope } from "../scope";

export class Response {
    private _response!: Protocol.GroupB.GroupC.StructB | Protocol.GroupA.StructB;

    constructor(res: Protocol.GroupB.GroupC.StructB | Protocol.GroupA.StructB) {
        this._response = res;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }
}

export function handler(
    request: Protocol.GroupB.GroupC.StructA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
	const scope = new Scope(consumer, filter, context, producer);
	return new Promise((resolve, reject) => {
		response(request, scope)
			.then((res) => {
				producer
					.send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
					.then(() => {
						scope.call();
						resolve();
					})
					.catch(reject);
			})
			.catch(reject);
	});
}