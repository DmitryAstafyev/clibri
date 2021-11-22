import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { response } from "../../responses/structd";

export class Response {
    private _response!: Protocol.StructA | Protocol.StructC;

    constructor(res: Protocol.StructA | Protocol.StructC) {
        this._response = res;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }
}

export function handler(
    request: Protocol.StructD,
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