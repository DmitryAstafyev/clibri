import { Filter, Identification } from "../consumer";
import { Producer } from "../index";
import { Context } from "../../context";

type Handler = () => void;

export class Scope {
	public readonly consumer: Identification;
	public readonly filter: Filter;
	public readonly context: Context;
	public readonly producer: Producer;

	private _deferred: Handler | undefined;

	constructor(
		consumer: Identification,
		filter: Filter,
		context: Context,
		producer: Producer
	) {
		this.consumer = consumer;
		this.filter = filter;
		this.context = context;
		this.producer = producer;
	}

	public deferred(cb: Handler) {
		this._deferred = cb;
	}

	public call() {
		this._deferred !== undefined && this._deferred();
	}
}
