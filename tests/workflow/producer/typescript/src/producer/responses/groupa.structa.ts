import { Response } from "../implementation/responses/groupa.structa";
import {
	Context,
	Producer,
	Identification,
	Filter,
	Protocol,
} from "../implementation/responses";
import { Scope } from "../implementation/scope";
import { Alias } from "../../stat";
import { panic } from "../../tools";

export function response(
	request: Protocol.GroupA.StructA,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.GroupAStructA);
	const index = stat.index(Alias.GroupAStructA);
	if (index === 0) {
		return Promise.resolve(
			new Response(Protocol.StructA.defaults())
				.broadcast([scope.consumer.uuid()])
				.StructD(Protocol.StructD.defaults())
		);
	} else if (index === 1) {
		return Promise.resolve(new Response(Protocol.StructB.defaults()));
	} else if (index === 2) {
		return Promise.resolve(
			new Response(Protocol.GroupA.StructB.defaults())
		);
	} else {
		panic(`Unexpected index for Protocol.GroupA.StructA request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.GroupA.StructA isn't implemented.`)
		);
	}
}
