import { Response } from "../implementation/responses/messages.request";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";

export function response(
    request: Protocol.Messages.Request,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.Messages.Request isn't implemented.`)
    );
}