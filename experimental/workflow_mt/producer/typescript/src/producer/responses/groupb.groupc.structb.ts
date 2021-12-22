import { Response } from "../implementation/responses/groupb.groupc.structb";
import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/responses";

export function response(
    request: Protocol.GroupB.GroupC.StructB,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Response> {
    return Promise.reject(
    	new Error(`Handler for Protocol.GroupB.GroupC.StructB isn't implemented.`)
    );
}