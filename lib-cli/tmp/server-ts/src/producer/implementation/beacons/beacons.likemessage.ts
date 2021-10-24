import * as Protocol from "../protocol";
import { Producer, Identification, Filter } from "./index";
import { emit } from "../../beacons/beacons.likemessage";

export function handler<C>(
	beacon: Protocol.Beacons.LikeMessage,
	consumer: Identification,
	filter: Filter,
	context: C,
	producer: Producer<C>,
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
