import { Response } from "../implementation/responses/message.request";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";

export function response(
    request: Protocol.Message.Request,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.Message.Request isn't implemented.`)
    );
}