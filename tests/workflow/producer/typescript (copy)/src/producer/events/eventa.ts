import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/eventa";

export function emit(
    event: Protocol.EventA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "EventA" isn't implemented`)
    );
}