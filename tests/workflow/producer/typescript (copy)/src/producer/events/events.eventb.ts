import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.eventb";

export function emit(
    event: Protocol.Events.EventB,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "Events.EventB" isn't implemented`)
    );
}