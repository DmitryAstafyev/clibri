import { Response } from "../implementation/responses/structempty";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";

export function response(
    request: Protocol.StructEmpty,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.StructEmpty isn't implemented.`)
    );
}