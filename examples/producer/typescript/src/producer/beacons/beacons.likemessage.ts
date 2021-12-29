import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/beacons";
import { Scope } from "../implementation/scope";

export function emit(
	beacon: Protocol.Beacons.LikeMessage,
	scope: Scope
): Promise<void> {
	return Promise.reject(
		new Error(`Handler for Protocol.Beacons.LikeMessage isn't implemented.`)
	);
}
