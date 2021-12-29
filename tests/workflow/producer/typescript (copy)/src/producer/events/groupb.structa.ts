import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/groupb.structa";

export function emit(
    event: Protocol.GroupB.StructA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "GroupB.StructA" isn't implemented`)
    );
}