import { Response } from "../implementation/responses/groupb.groupc.structa";
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
	request: Protocol.GroupB.GroupC.StructA,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.GroupBGroupCStructA);
	const index = stat.index(Alias.GroupBGroupCStructA);
	if (index === 0) {
		return Promise.resolve(
			new Response(Protocol.GroupA.StructB.defaults())
		);
	} else if (index === 1) {
		return Promise.resolve(
			new Response(Protocol.GroupB.GroupC.StructB.defaults())
		);
	} else {
		panic(
			`Unexpected index for Protocol.GroupB.GroupC.StructA request: ${index}`
		);
		return Promise.reject(
			new Error(
				`Handler for Protocol.GroupB.GroupC.StructA isn't implemented.`
			)
		);
	}
}
