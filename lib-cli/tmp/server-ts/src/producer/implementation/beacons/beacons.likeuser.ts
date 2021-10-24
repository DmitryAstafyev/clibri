import * as Protocol from "../protocol";
import { Producer, Identification, Filter } from "./index";
import { emit } from "../../beacons/beacons.likeuser";

export function handler<C>(
	beacon: Protocol.Beacons.LikeUser,
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
		producer.send(
			consumer.uuid(),
			confirmation.pack(sequence, consumer.uuid())
		);
	});
}
