import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/structb";

export function emit(
    event: Protocol.StructB,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "StructB" isn't implemented`)
    );
}