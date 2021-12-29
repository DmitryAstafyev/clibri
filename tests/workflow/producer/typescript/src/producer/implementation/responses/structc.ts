import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/structc";
import { Scope } from "../scope";

export class Response {    
    private _response!: Protocol.StructB | Protocol.StructF | Protocol.StructD | Protocol.StructE;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.StructB | Protocol.StructF | Protocol.StructD | Protocol.StructE
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
    } {
        const self = this;
        return {            
        };
    }

    public error(): Error | undefined {
        let error: Error | undefined;        
        return error;
    }

    public pack(sequence: number, uuid: string): ArrayBufferLike {
        return this._response.pack(sequence, uuid);
    }

    public broadcasts(): Array<[string[], Protocol.Convertor<any>]> {
        return this._broadcasts;
    }
}

export function handler(
    request: Protocol.StructC,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
	return new Promise((resolve, reject) => {
        const scope = new Scope(consumer, filter, context, producer);
		response(request, scope)
			.then((res) => {
				const error: Error | undefined = res.error();
				if (error instanceof Error) {
					return reject(error);
				}
				producer
					.send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
					.then(() => {
						broadcastAll(producer, res.broadcasts()).then(() => {
                            scope.call();
                            resolve();
                        }).catch(reject);
					}).catch(reject);
			})
			.catch(reject);
	});
}