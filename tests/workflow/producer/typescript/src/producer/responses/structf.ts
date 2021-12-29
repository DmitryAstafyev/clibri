import { Response } from "../implementation/responses/structf";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Alias } from "../../stat";
import { panic } from "../../tools";
import { Scope } from "../implementation/scope";

export function response(
	request: Protocol.StructF,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.StructF);
	const index = stat.index(Alias.StructF);
	if (index === 0) {
		return Promise.resolve(new Response(Protocol.StructF.defaults()));
	} else if (index === 1) {
		const event = Protocol.TriggerBeaconsEmitter.defaults();
		event.uuid = scope.consumer.uuid();
		scope.producer.events.triggerBeaconsEmitter.emit(event);
		stat.case(Alias.StructE);
		return Promise.resolve(new Response(Protocol.StructE.defaults()));
	} else {
		panic(`Unexpected index for Protocol.StructF request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructF isn't implemented.`)
		);
	}
}
