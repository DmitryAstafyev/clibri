import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/structa";
import { Scope } from "../scope";

export class Response {    
    static REQUIRED_CASEB = [    
        Protocol.StructD,
        Protocol.StructF,
    ];
    static REQUIRED_CASED = [    
        Protocol.StructJ,
    ];
    private _response!: Protocol.StructB | Protocol.StructC | Protocol.StructD | Protocol.StructE;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.StructB | Protocol.StructC | Protocol.StructD | Protocol.StructE
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
        StructD(msg: Protocol.StructD): Response;
        StructF(msg: Protocol.StructF): Response;
        StructJ(msg: Protocol.StructJ): Response;
    } {
        const self = this;
        return {            
            StructD(msg: Protocol.StructD): Response {
                if (
                    self._response.getId() !==
                    Protocol.StructB.getId()
                ) {
                    throw new Error(
                        `Message "Protocol.StructD" can be used only with "Protocol.StructB"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructD.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructD already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            StructF(msg: Protocol.StructF): Response {
                if (
                    self._response.getId() !==
                    Protocol.StructB.getId()
                ) {
                    throw new Error(
                        `Message "Protocol.StructF" can be used only with "Protocol.StructB"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructF.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructF already has been defined.`
                    );
                }
                self._broadcasts.push([uuids, msg]);
                return self;
            },
            StructJ(msg: Protocol.StructJ): Response {
                if (
                    self._response.getId() !==
                    Protocol.StructD.getId()
                ) {
                    throw new Error(
                        `Message "Protocol.StructJ" can be used only with "Protocol.StructD"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.StructJ.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructJ already has been defined.`
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
            Protocol.StructB.getId()
        ) {
            Response.REQUIRED_CASEB.forEach((ref) => {
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
        if (
            error === undefined &&
            this._response.getId() ===
            Protocol.StructD.getId()
        ) {
            Response.REQUIRED_CASED.forEach((ref) => {
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
    request: Protocol.StructA,
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