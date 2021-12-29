import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/events.eventa";

export function emit(
    event: Protocol.Events.EventA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "Events.EventA" isn't implemented`)
    );
}