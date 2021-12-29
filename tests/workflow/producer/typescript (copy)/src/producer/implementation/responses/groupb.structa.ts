import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/groupb.structa";

export class Response {    
    private _response!: Protocol.GroupB.StructA | Protocol.GroupB.GroupC.StructA | Protocol.GroupB.GroupC.StructB;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.GroupB.StructA | Protocol.GroupB.GroupC.StructA | Protocol.GroupB.GroupC.StructB
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
    } {
        const self = this;
        return {            
        };
    }

    public error(): Error | undefined {
        let error: Error | undefined;        
        return error;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }

    public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
        return this._broadcasts;
    }
}

export function handler(
    request: Protocol.GroupB.StructA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return response(request, consumer, filter, context, producer).then(
        (res) => {
            const error: Error | undefined = res.error();
            if (error instanceof Error) {
                return Promise.reject(error);
            }
            return producer
                .send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
                .then(() => {
                    return broadcastAll(producer, res.broadcasts());
                });
        }
    );
}