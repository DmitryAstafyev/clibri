import { Response } from "../implementation/responses/structempty";
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
	request: Protocol.StructEmpty,
	scope: Scope
): Promise<Response> {
	const stat = scope.context.getStat(scope.consumer.uuid());
	stat.case(Alias.StructEmpty);
	const index = stat.index(Alias.StructEmpty);
	if (index === 0) {
		stat.case(Alias.StructEmptyA);
		return Promise.resolve(new Response(Protocol.StructEmptyA.defaults()));
	} else if (index === 1) {
		scope.producer.events.eventA.emit(
			(() => {
				const event = Protocol.EventA.defaults();
				event.uuid = scope.consumer.uuid();
				return event;
			})()
		);
		scope.producer.events.eventB.emit(
			(() => {
				const event = Protocol.EventB.defaults();
				event.uuid = scope.consumer.uuid();
				return event;
			})()
		);
		scope.producer.events.eventsEventA.emit(
			(() => {
				const event = Protocol.Events.EventA.defaults();
				event.uuid = scope.consumer.uuid();
				return event;
			})()
		);
		scope.producer.events.eventsEventB.emit(
			(() => {
				const event = Protocol.Events.EventB.defaults();
				event.uuid = scope.consumer.uuid();
				return event;
			})()
		);
		scope.producer.events.eventsSubEventA.emit(
			(() => {
				const event = Protocol.Events.Sub.EventA.defaults();
				event.uuid = scope.consumer.uuid();
				return event;
			})()
		);
		stat.case(Alias.StructEmptyB);
		return Promise.resolve(new Response(Protocol.StructEmptyB.defaults()));
	} else {
		panic(`Unexpected index for Protocol.StructEmpty request: ${index}`);
		return Promise.reject(
			new Error(`Handler for Protocol.StructEmpty isn't implemented.`)
		);
	}
}
