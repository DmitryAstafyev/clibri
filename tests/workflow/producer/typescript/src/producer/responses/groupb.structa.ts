import { Response } from "../implementation/responses/groupb.structa";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";

export function response(
    request: Protocol.GroupB.StructA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.GroupB.StructA isn't implemented.`)
    );
}