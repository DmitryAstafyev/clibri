import { Filter, Producer, Context, Protocol } from "../implementation/events";
import { Output } from "../implementation/events/triggerbeaconsemitter";

export function emit(
    event: Protocol.TriggerBeaconsEmitter,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<Output> {
    return Promise.reject(
        new Error(`Handler for event "TriggerBeaconsEmitter" isn't implemented`)
    );
}