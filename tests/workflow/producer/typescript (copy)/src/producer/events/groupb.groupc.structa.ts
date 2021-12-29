import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/groupb.groupc.structa";

export function emit(
    event: Protocol.GroupB.GroupC.StructA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "GroupB.GroupC.StructA" isn't implemented`)
    );
}