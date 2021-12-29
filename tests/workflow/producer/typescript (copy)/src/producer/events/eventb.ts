import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/eventb";

export function emit(
    event: Protocol.EventB,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "EventB" isn't implemented`)
    );
}