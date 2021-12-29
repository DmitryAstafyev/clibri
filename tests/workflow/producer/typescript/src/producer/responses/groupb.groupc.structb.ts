import { Response } from "../implementation/responses/groupb.groupc.structb";
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
	request: Protocol.GroupB.GroupC.StructB,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.GroupBGroupCStructB);
	const index = stat.index(Alias.GroupBGroupCStructB);
	if (index === 0) {
		return Promise.resolve(
			new Response(Protocol.StructB.defaults())
				.broadcast([scope.consumer.uuid()])
				.StructD(Protocol.StructD.defaults())
				.broadcast([scope.consumer.uuid()])
				.StructF(Protocol.StructF.defaults())
		);
	} else if (index === 1) {
		return Promise.resolve(new Response(Protocol.StructC.defaults()));
	} else if (index === 2) {
		stat.case(Alias.StructJ);
		return Promise.resolve(
			new Response(Protocol.StructD.defaults())
				.broadcast([scope.consumer.uuid()])
				.StructJ(Protocol.StructJ.defaults())
		);
	} else if (index === 3) {
		return Promise.resolve(
			new Response(Protocol.GroupB.GroupC.StructA.defaults())
		);
	} else {
		panic(
			`Unexpected index for Protocol.GroupB.GroupC.StructB request: ${index}`
		);
		return Promise.reject(
			new Error(
				`Handler for Protocol.GroupB.GroupC.StructB isn't implemented.`
			)
		);
	}
}
