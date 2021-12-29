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
import * as Samples from "../../samples";

export function response(
	request: Protocol.StructA,
	consumer: Identification,
	filter: Filter,
	context: Context,
	producer: Producer
): Promise<Response> {
	const stat = context.getStat(consumer.uuid());
	stat.case(Alias.StructA);
	const index = stat.index(Alias.StructA);
	if (index === 0) {
		return Promise.resolve(new Response(Samples.StructB.instance()));
	} else if (index === 1) {
		return Promise.resolve(new Response(Samples.StructC.instance()));
	} else if (index === 2) {
		return Promise.resolve(new Response(Samples.StructD.instance()));
	} else if (index === 3) {
		return Promise.resolve(new Response(Samples.StructC.instance()));
	} else {
		panic(`Unexpected index for Protocol.StructA request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructA isn't implemented.`)
		);
	}
}
