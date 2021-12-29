import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/beacons";
import { Alias } from "../../stat";
import { Scope } from "../implementation/scope";

export function emit(
	beacon: Protocol.Beacons.Sub.BeaconA,
	scope: Scope
): Promise<void> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.BeaconsSubBeaconA);
	scope.context.checkBeacons(scope.consumer.uuid(), scope.producer);
	return Promise.resolve();
}
