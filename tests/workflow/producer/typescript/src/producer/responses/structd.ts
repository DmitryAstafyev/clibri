import { Response } from "../implementation/responses/structd";
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
	request: Protocol.StructD,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.StructD);
	const index = stat.index(Alias.StructD);
	if (index === 0) {
		return Promise.resolve(new Response(Protocol.StructC.defaults()));
	} else if (index === 1) {
		return Promise.resolve(new Response(Protocol.StructA.defaults()));
	} else {
		panic(`Unexpected index for Protocol.StructD request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructD isn't implemented.`)
		);
	}
}
