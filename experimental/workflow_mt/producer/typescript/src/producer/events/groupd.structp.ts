import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/groupd.structp";

export function emit(
    event: Protocol.GroupD.StructP,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "GroupD.StructP" isn't implemented`)
    );
}