import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/beacons";

export function emit(
    beacon: Protocol.GroupA.StructA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return Promise.reject(
        new Error(`Handler for Protocol.GroupA.StructA isn't implemented.`)
    );
}