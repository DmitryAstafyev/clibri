import { Response } from "../implementation/responses/structc";
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
	request: Protocol.StructC,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.StructC);
	const index = stat.index(Alias.StructC);
	if (index === 0) {
		return Promise.resolve(new Response(Protocol.StructB.defaults()));
	} else if (index === 1) {
		return Promise.resolve(new Response(Protocol.StructF.defaults()));
	} else if (index === 2) {
		return Promise.resolve(new Response(Protocol.StructD.defaults()));
	} else if (index === 3) {
		stat.case(Alias.StructE);
		return Promise.resolve(new Response(Protocol.StructE.defaults()));
	} else {
		panic(`Unexpected index for Protocol.StructC request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructC isn't implemented.`)
		);
	}
}
