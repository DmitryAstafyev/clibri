import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/groupa.structb";
import { Scope } from "../scope";

export class Response {    
    static REQUIRED_GROUPBSTRUCTA = [    
        Protocol.GroupB.GroupC.StructB,
    ];
    private _response!: Protocol.GroupB.StructA | Protocol.GroupB.GroupC.StructA | Protocol.GroupA.StructB;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.GroupB.StructA | Protocol.GroupB.GroupC.StructA | Protocol.GroupA.StructB
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
        GroupBGroupCStructB(msg: Protocol.GroupB.GroupC.StructB): Response;
    } {
        const self = this;
        return {            
            GroupBGroupCStructB(msg: Protocol.GroupB.GroupC.StructB): Response {
                if (
                    self._response.getId() !==
                    Protocol.GroupB.StructA.getId()
                ) {
                    throw new Error(
                        `Message "Protocol.GroupB.GroupC.StructB" can be used only with "Protocol.GroupB.StructA"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.GroupB.GroupC.StructB.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.GroupB.GroupC.StructB already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
        };
    }

    public error(): Error | undefined {
        let error: Error | undefined;        
        if (
            error === undefined &&
            this._response.getId() ===
            Protocol.GroupB.StructA.getId()
        ) {
            Response.REQUIRED_GROUPBSTRUCTA.forEach((ref) => {
                if (error !== undefined) {
                    return;
                }
                if (
                    this._broadcasts.find((msg) => {
                        return msg[1].getId() === ref.getId();
                    }) === undefined
                ) {
                    error = new Error(
                        `Broadcast ${ref.getSignature()}/${ref.getId()} is required for ${this._response.getSignature()}/${this._response.getId()}, but hasn't been found`
                    );
                }
            });
        }
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
    request: Protocol.GroupA.StructB,
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