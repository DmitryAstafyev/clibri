import { IServerError } from "clibri";
import { Producer, Identification, Filter, Context } from "./index";
import { emit } from "../../events/error";

export enum ProducerErrorType {
	Connection = "Connection",
	Disconnection = "Disconnection",
	ProtocolHash = "ProtocolHash",
	WorkflowHash = "WorkflowHash",
	HashError = "HashError",
	KeyError = "KeyError",
	AssignedKeyError = "AssignedKeyError",
	ProcessingIncomeData = "ProcessingIncomeData",
}

export class ProducerError extends Error {
	private _type: ProducerErrorType;

	constructor(msg: string, type: ProducerErrorType) {
		super(msg);
		this._type = type;
	}

	public getErrType(): ProducerErrorType {
		return this._type;
	}
}

export function handler(
	error: ProducerError | IServerError,
	context: Context,
	producer: Producer,
	consumer: Identification | undefined,
	filter: Filter | undefined
): Promise<void> {
	return emit(error, context, producer, consumer, filter);
}
