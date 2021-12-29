import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/beacons";

export function emit(
    beacon: Protocol.StructB,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return Promise.reject(
        new Error(`Handler for Protocol.StructB isn't implemented.`)
    );
}