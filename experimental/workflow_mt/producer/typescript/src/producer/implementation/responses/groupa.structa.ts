import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/groupa.structa";

export class Response {    
    static REQUIRED_ROOTA = [    
        Protocol.StructD,
    ];
    private _response!: Protocol.StructA | Protocol.StructB | Protocol.GroupA.StructB;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.StructA | Protocol.StructB | Protocol.GroupA.StructB
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
        StructD(msg: Protocol.StructD): Response;
    } {
        const self = this;
        return {            
            StructD(msg: Protocol.StructD): Response {
                if (
                    self._response.getSignature() !==
                    Protocol.StructA.getSignature()
                ) {
                    throw new Error(
                        `Message "Protocol.StructD" can be used only with "Protocol.StructA"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getSignature() ===
                            Protocol.StructD.getSignature()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.StructD already has been defined.`
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
            this._response.getSignature() ===
            Protocol.StructA.getSignature()
        ) {
            Response.REQUIRED_ROOTA.forEach((ref) => {
                if (error !== undefined) {
                    return;
                }
                if (
                    this._broadcasts.find((msg) => {
                        return msg[1].getSignature() === ref.getSignature();
                    }) === undefined
                ) {
                    error = new Error(
                        `Broadcast ${ref.getSignature()} is required, but hasn't been found`
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
    request: Protocol.GroupA.StructA,
    consumer: Identification,
    filter: Filter,
    context: Context,
    producer: Producer,
    sequence: number
): Promise<void> {
    return response(request, consumer, filter, context, producer).then(
        (res) => {
            const error: Error | undefined = res.error();
            if (error instanceof Error) {
                return Promise.reject(error);
            }
            return producer
                .send(consumer.uuid(), res.pack(sequence, consumer.uuid()))
                .then(() => {
                    return broadcastAll(producer, res.broadcasts());
                });
        }
    );
}