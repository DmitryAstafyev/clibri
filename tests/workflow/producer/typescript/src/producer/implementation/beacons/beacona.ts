import { Producer, Identification, Filter, Context, Protocol } from "./index";
import { emit } from "../../beacons/beacona";
import { Scope } from "../scope";

export function handler(
    beacon: Protocol.BeaconA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
	const scope = new Scope(consumer, filter, context, producer);
	return new Promise((resolve, reject) => {
		emit(beacon, scope)
			.then(() => {
				const confirmation =
					new Protocol.InternalServiceGroup.BeaconConfirmation({
						error: undefined,
					});
				producer
					.send(
						consumer.uuid(),
						confirmation.pack(sequence, consumer.uuid())
					)
					.then(() => {
						scope.call();
						resolve();
					})
					.catch(reject);
			})
			.catch(reject);
	});
}