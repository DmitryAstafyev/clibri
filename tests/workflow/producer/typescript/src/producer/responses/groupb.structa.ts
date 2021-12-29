import { Response } from "../implementation/responses/groupb.structa";
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
	request: Protocol.GroupB.StructA,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.GroupBStructA);
	const index = stat.index(Alias.GroupBStructA);
	if (index === 0) {
		return Promise.resolve(
			new Response(Protocol.GroupB.StructA.defaults())
		);
	} else if (index === 1) {
		return Promise.resolve(
			new Response(Protocol.GroupB.GroupC.StructA.defaults())
		);
	} else if (index === 2) {
		return Promise.resolve(
			new Response(Protocol.GroupB.GroupC.StructB.defaults())
		);
	} else {
		panic(`Unexpected index for Protocol.GroupB.StructA request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.GroupB.StructA isn't implemented.`)
		);
	}
}
