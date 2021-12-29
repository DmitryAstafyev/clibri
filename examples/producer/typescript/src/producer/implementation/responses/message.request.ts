import {
    Producer,
    Identification,
    Filter,
    broadcastAll,
    Context,
    Protocol,
} from "./index";
import { response } from "../../responses/message.request";
import { Scope } from "../scope";

export class Response {    
    static REQUIRED_ACCEPT = [    
        Protocol.Events.Message,
    ];
    private _response!: Protocol.Message.Accepted | Protocol.Message.Denied | Protocol.Message.Err;
    private _broadcasts: Array<[string[], Protocol.Convertor<any>]> = [];

    constructor(
        res: Protocol.Message.Accepted | Protocol.Message.Denied | Protocol.Message.Err
    ) {
        this._response = res;
    }

    public broadcast(uuids: string[]): {        
        EventsMessage(msg: Protocol.Events.Message): Response;
    } {
        const self = this;
        return {            
            EventsMessage(msg: Protocol.Events.Message): Response {
                if (
                    self._response.getId() !==
                    Protocol.Message.Accepted.getId()
                ) {
                    throw new Error(
                        `Message "Protocol.Events.Message" can be used only with "Protocol.Message.Accepted"`
                    );
                }
                if (
                    self._broadcasts.find(
                        (b) =>
                            b[1].getId() ===
                            Protocol.Events.Message.getId()
                    ) !== undefined
                ) {
                    throw new Error(
                        `Broadcast Protocol.Events.Message already has been defined.`
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
            Protocol.Message.Accepted.getId()
        ) {
            Response.REQUIRED_ACCEPT.forEach((ref) => {
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
    request: Protocol.Message.Request,
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