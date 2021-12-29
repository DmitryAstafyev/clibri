import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/beacons";
import { Scope } from "../implementation/scope";

export function emit(
	beacon: Protocol.Beacons.LikeUser,
	scope: Scope
): Promise<void> {
	return Promise.reject(
		new Error(`Handler for Protocol.Beacons.LikeUser isn't implemented.`)
	);
}
