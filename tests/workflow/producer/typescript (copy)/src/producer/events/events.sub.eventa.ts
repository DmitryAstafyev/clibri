import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.sub.eventa";

export function emit(
    event: Protocol.Events.Sub.EventA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "Events.Sub.EventA" isn't implemented`)
    );
}