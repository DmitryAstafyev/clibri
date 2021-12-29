import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/beacons";
import { Scope } from "../implementation/scope";
import { Alias } from "../../stat";

export function emit(beacon: Protocol.BeaconA, scope: Scope): Promise<void> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.BeaconA);
	scope.context.checkBeacons(scope.consumer.uuid(), scope.producer);
	return Promise.resolve();
}
