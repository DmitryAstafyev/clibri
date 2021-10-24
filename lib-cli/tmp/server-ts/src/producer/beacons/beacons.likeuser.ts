import * as Protocol from "implementation/protocol";
import {
	Context,
	Producer,
	Identification,
	Filter,
} from "implementation/beacons";

export function emit(
	beacon: Protocol.Beacons.LikeUser,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<void> {
	return Promise.reject(
		new Error(`Handler for Protocol.Beacons.LikeUser isn't implemented.`)
	);
}
