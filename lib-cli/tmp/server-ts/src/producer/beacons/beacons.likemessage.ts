import * as Protocol from "implementation/protocol";
import {
	Context,
	Producer,
	Identification,
	Filter,
} from "implementation/beacons";

export function emit(
	beacon: Protocol.Beacons.LikeMessage,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer<Context>
): Promise<void> {
	return Promise.reject(
		new Error(`Handler for Protocol.Beacons.LikeMessage isn't implemented.`)
	);
}
