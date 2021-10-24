import * as Protocol from "../protocol";
import { Context, Producer, Identification, Filter } from "./index";
import { emit } from "../../beacons/beacons.likemessage";

export function handler(
	beacon: Protocol.Beacons.LikeMessage,
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
