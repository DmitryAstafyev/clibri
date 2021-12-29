import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { emit } from "../../beacons/beacona";

export function handler(
    beacon: Protocol.BeaconA,
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