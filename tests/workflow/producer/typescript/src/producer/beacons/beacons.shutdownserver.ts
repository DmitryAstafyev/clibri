import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/beacons";
import { panic } from "../../tools";
import { Scope } from "../implementation/scope";

export function emit(
	beacon: Protocol.Beacons.ShutdownServer,
	scope: Scope
): Promise<void> {
	scope.context.ignore(scope.consumer.uuid());
	scope.deferred(() => {
		setTimeout(() => {
			scope.producer.destroy().catch((error: Error) => {
				panic(`Fail to destroy producer: ${error.message}`);
			});
		}, 1000);
	});
	return Promise.resolve();
}
