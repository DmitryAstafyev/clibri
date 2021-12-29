import {
    Context,
    Producer,
    Identification,
    Filter,
    Protocol,
} from "../implementation/beacons";

export function emit(
    beacon: Protocol.Beacons.BeaconA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer
): Promise<void> {
    return Promise.reject(
        new Error(`Handler for Protocol.Beacons.BeaconA isn't implemented.`)
    );
}