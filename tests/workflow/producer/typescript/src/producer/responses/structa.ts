import { Response } from "../implementation/responses/structa";
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
	request: Protocol.StructA,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.StructA);
	const index = stat.index(Alias.StructA);
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
		stat.case(Alias.StructE);
		return Promise.resolve(new Response(Protocol.StructE.defaults()));
	} else {
		panic(`Unexpected index for Protocol.StructA request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructA isn't implemented.`)
		);
	}
}
