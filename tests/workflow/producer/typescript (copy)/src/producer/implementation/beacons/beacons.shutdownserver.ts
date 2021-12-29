import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { emit } from "../../beacons/beacons.shutdownserver";

export function handler(
    beacon: Protocol.Beacons.ShutdownServer,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return emit(beacon, consumer, filter, context, producer).then(() => {
        const confirmation =
            new Protocol.InternalServiceGroup.BeaconConfirmation({
                error: undefined,
            });
        return producer.send(
            consumer.uuid(),
            confirmation.pack(sequence, consumer.uuid())
        );
    });
}