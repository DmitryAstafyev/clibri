import { Response } from "../implementation/responses/groupa.structb";
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
	request: Protocol.GroupA.StructB,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.GroupAStructB);
	const index = stat.index(Alias.GroupAStructB);
	if (index === 0) {
		return Promise.resolve(
			new Response(Protocol.GroupB.StructA.defaults())
				.broadcast([scope.consumer.uuid()])
				.GroupBGroupCStructB(Protocol.GroupB.GroupC.StructB.defaults())
		);
	} else if (index === 1) {
		return Promise.resolve(
			new Response(Protocol.GroupB.GroupC.StructA.defaults())
		);
	} else if (index === 2) {
		return Promise.resolve(
			new Response(Protocol.GroupA.StructB.defaults())
		);
	} else {
		panic(`Unexpected index for Protocol.GroupA.StructB request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.GroupA.StructB isn't implemented.`)
		);
	}
}
