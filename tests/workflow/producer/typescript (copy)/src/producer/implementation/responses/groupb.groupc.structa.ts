import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { response } from "../../responses/groupb.groupc.structa";

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
    return response(request, consumer, filter, context, producer).then(
        (res) => {
            return producer.send(
                consumer.uuid(),
                res.pack(sequence, consumer.uuid())
            );
        }
    );
}