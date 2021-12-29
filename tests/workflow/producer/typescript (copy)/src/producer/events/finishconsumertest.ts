import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/finishconsumertest";

export function emit(
    event: Protocol.FinishConsumerTest,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "FinishConsumerTest" isn't implemented`)
    );
}