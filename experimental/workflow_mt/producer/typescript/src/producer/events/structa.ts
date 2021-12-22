import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/structa";

export function emit(
    event: Protocol.StructA,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "StructA" isn't implemented`)
    );
}